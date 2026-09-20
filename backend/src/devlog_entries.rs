use crate::auth::AuthUser;
use crate::models::devlog::Devlog;
use axum::extract::State;
use axum::{Json, extract::Query, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateDevlogBody {
    content: String,
}

pub async fn get_devlog_entries(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Devlog>>), StatusCode> {
    let entries = sqlx::query_as::<_, Devlog>(
        r#"
        SELECT
            id,
            content,
            created_at,
            updated_at
        FROM devlog_entries 
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(entries)))
}

pub async fn create_devlog_entry(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(entry_data): Json<CreateDevlogBody>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, created a devlog entry", auth.user.username);

    let id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
    INSERT INTO devlog_entries(
        id,
        content
    )
    VALUES ($1, $2)
    "#,
    )
    .bind(id)
    .bind(entry_data.content)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Errror: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(id)))
}
