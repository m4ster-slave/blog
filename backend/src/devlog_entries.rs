use crate::auth::AuthUser;
use crate::models::devlog::Devlog;
use axum::extract::{Path, Query, State};
use axum::{Json, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct DevlogBody {
    content: String,
}

pub async fn get_devlog_entries(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<crate::models::pagination::Pagination>,
) -> Result<(StatusCode, Json<Vec<Devlog>>), StatusCode> {
    let page = pagination.page.unwrap_or(0).max(0);
    let limit = pagination.limit.unwrap_or(50).clamp(1, 100);
    let offset = page * limit;

    let entries = sqlx::query_as::<_, Devlog>(
        r#"
        SELECT
            id,
            content,
            created_at,
            updated_at
        FROM devlog_entries 
        ORDER BY created_at DESC
        LIMIT $1
        OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(entries)))
}

pub async fn create_devlog_entry(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(entry_data): Json<DevlogBody>,
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

pub async fn delete_devlog_entry(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, deleted a devlog entry", auth.user.username);

    sqlx::query(
        r#"
            DELETE FROM devlog_entries 
            WHERE id = $1
            "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error deleting devlog_entry: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::NO_CONTENT, Json(id)))
}

pub async fn edit_devlog_entry(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
    Json(entry_data): Json<DevlogBody>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, edited a devlog entry", auth.user.username);

    sqlx::query(
        r#"
    UPDATE devlog_entries
    SET 
        content = $1,
        updated_at = NOW()
    WHERE id = $2
    "#,
    )
    .bind(entry_data.content)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error updating devlog_entry: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(id)))
}
