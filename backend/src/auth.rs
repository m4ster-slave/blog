use argon2::{Argon2, PasswordHash, password_hash::PasswordVerifier};
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, models::user::User};

/// Holding struct for authenticated user data inside protected handlers
#[derive(Debug)]
pub struct AuthUser {
    pub user: User,
}

impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extract cookies from request headers
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // 2. Read session cookie
        let session_cookie = jar.get("session").ok_or(StatusCode::UNAUTHORIZED)?;
        let session_id =
            Uuid::parse_str(session_cookie.value()).map_err(|_| StatusCode::UNAUTHORIZED)?;

        // 3. Query DB for active, non-expired session joined with user
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT u.id, u.username, u.email, u.password_hash, u.created_at
            FROM users u
            INNER JOIN sessions s ON s.user_id = u.id
            WHERE s.id = $1 AND s.expires_at > NOW()
            "#,
        )
        .bind(session_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            println!("Auth DB Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser { user })
    }
}

#[derive(Debug, Deserialize)]
pub struct CredentialBody {
    username: String,
    password: String,
}

async fn find_user(credentials: &CredentialBody, pool: &PgPool) -> Result<User, StatusCode> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            created_at
        FROM users
        WHERE
            username = $1
        "#,
    )
    .bind(&credentials.username)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("Error finding user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn argon2_verify(
    password: &str,
    password_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(e) => Err(e),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(credentials): Json<CredentialBody>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = find_user(&credentials, &state.pool).await?;

    if !argon2_verify(&credentials.password, &user.password_hash)
        .await
        .map_err(|e| {
            println!("Error verifying hash: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    {
        println!("User unauthorized");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_id = uuid::Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO sessions (id, user_id, expires_at)
        VALUES ($1, $2, now() + interval '30 days')
        "#,
    )
    .bind(session_id)
    .bind(user.id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error creating session: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut cookie = Cookie::new("session", session_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Lax);

    let jar = jar.add(cookie);

    Ok((jar, StatusCode::OK))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let session_cookie = match jar.get("session") {
        Some(cookie) => cookie,
        None => return Ok((jar, StatusCode::OK)),
    };

    if let Ok(session_id) = Uuid::parse_str(session_cookie.value()) {
        sqlx::query(
            r#"
            DELETE FROM sessions 
            WHERE id = $1
            "#,
        )
        .bind(session_id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            println!("Error deleting session: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    let mut removal_cookie = Cookie::from("session");
    removal_cookie.set_path("/");

    let jar = jar.remove(removal_cookie);

    Ok((jar, StatusCode::OK))
}
