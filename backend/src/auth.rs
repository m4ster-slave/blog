use argon2::{Argon2, PasswordHash, password_hash::PasswordVerifier};
use axum::Json;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{
    extract::FromRequestParts,
    http::{Method, StatusCode, request::Parts},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::{AppState, models::user::User};
use tracing::{error, info, warn};

/// Holding struct for authenticated user data inside protected handlers
#[derive(Debug)]
pub struct AuthUser {
    pub user: User,
}

pub struct LoginAttempt {
    pub window_started: Instant,
    pub failures: u32,
}

const LOGIN_WINDOW: Duration = Duration::from_secs(15 * 60);
const MAX_LOGIN_FAILURES: u32 = 5;

pub async fn csrf_middleware(request: Request, next: Next) -> axum::response::Response {
    let is_state_changing = matches!(
        *request.method(),
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );

    let cookie_header = request
        .headers()
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok());
    let cookie_token = cookie_header.and_then(|cookies| {
        cookies.split(';').find_map(|cookie| {
            cookie
                .trim()
                .strip_prefix("csrf_token=")
                .map(str::to_owned)
        })
    });

    if is_state_changing && request.uri().path() != "/login" {
        let header_token = request
            .headers()
            .get("x-csrf-token")
            .and_then(|value| value.to_str().ok());

        if cookie_token.as_deref() != header_token {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    let mut response = next.run(request).await;
    if cookie_token.is_none() {
        let mut csrf_cookie = Cookie::new("csrf_token", Uuid::new_v4().to_string());
        csrf_cookie.set_path("/");
        csrf_cookie.set_secure(false); // TODO when production is HTTPS-only
        csrf_cookie.set_same_site(SameSite::Lax);
        if let Ok(value) = csrf_cookie.to_string().parse() {
            response
                .headers_mut()
                .append(axum::http::header::SET_COOKIE, value);
        }
    }

    response
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
            .map_err(|error| {
                error!(error = ?error, status = 500, "failed to parse authentication cookies");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

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
            error!(error = %e, "auth database lookup failed");
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

async fn find_user(credentials: &CredentialBody, pool: &PgPool) -> Result<Option<User>, StatusCode> {
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
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!(user = %credentials.username, error = %e, "user lookup failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

fn login_is_rate_limited(state: &AppState, username: &str) -> bool {
    let mut attempts = state
        .login_attempts
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let now = Instant::now();

    match attempts.get(username) {
        Some(attempt)
            if now.duration_since(attempt.window_started) < LOGIN_WINDOW
                && attempt.failures >= MAX_LOGIN_FAILURES => true,
        Some(attempt) if now.duration_since(attempt.window_started) >= LOGIN_WINDOW => {
            attempts.remove(username);
            false
        }
        _ => false,
    }
}

fn record_login_failure(state: &AppState, username: &str) {
    let mut attempts = state
        .login_attempts
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let now = Instant::now();
    let attempt = attempts.entry(username.to_owned()).or_insert(LoginAttempt {
        window_started: now,
        failures: 0,
    });

    if now.duration_since(attempt.window_started) >= LOGIN_WINDOW {
        attempt.window_started = now;
        attempt.failures = 0;
    }
    attempt.failures = attempt.failures.saturating_add(1);
}

fn clear_login_failures(state: &AppState, username: &str) {
    let mut attempts = state
        .login_attempts
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    attempts.remove(username);
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
    if login_is_rate_limited(&state, &credentials.username) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let user = match find_user(&credentials, &state.pool).await? {
        Some(user) => user,
        None => {
            record_login_failure(&state, &credentials.username);
            warn!(user = %credentials.username, "login failed");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if !argon2_verify(&credentials.password, &user.password_hash)
        .await
        .map_err(|e| {
            error!(user = %credentials.username, error = %e, "password verification failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    {
        record_login_failure(&state, &credentials.username);
        warn!(user = %credentials.username, "login failed");
        return Err(StatusCode::UNAUTHORIZED);
    }

    clear_login_failures(&state, &credentials.username);

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
        error!(user = %user.username, error = %e, "session creation failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut cookie = Cookie::new("session", session_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(false); //TODO when production set to true
    cookie.set_same_site(SameSite::Lax);

    let mut csrf_cookie = Cookie::new("csrf_token", uuid::Uuid::new_v4().to_string());
    csrf_cookie.set_path("/");
    csrf_cookie.set_secure(false); // TODO when production set to true
    csrf_cookie.set_same_site(SameSite::Lax);

    let jar = jar.add(cookie).add(csrf_cookie);

    info!(user = %user.username, status = 200, "login succeeded");

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
            error!(error = %e, "session deletion failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    let mut removal_cookie = Cookie::from("session");
    removal_cookie.set_path("/");

    let jar = jar.remove(removal_cookie);

    Ok((jar, StatusCode::OK))
}
pub async fn check_session(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<StatusCode, StatusCode> {
    let session_id = jar
        .get("session")
        .and_then(|c| Uuid::parse_str(c.value()).ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM sessions WHERE id = $1)")
        .bind(session_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "session lookup failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if exists {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
