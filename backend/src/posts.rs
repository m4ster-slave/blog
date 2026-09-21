use crate::auth::AuthUser;
use crate::models::post::Post;
use axum::extract::{Path, State};
use axum::{Json, extract::Query, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SlugPageQuery {
    slug: Option<String>,
    page: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct EditPostQuery {
    title: String,
    slug: String,
    summary: String,
    content: String,
    archived: bool,
    publish: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostBody {
    title: String,
    slug: String,
    summary: String,
    content: String,
}

pub async fn get_posts(
    Query(_query): Query<SlugPageQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Post>>), StatusCode> {
    let posts = sqlx::query_as::<_, Post>(
        r#"
        SELECT
            id,
            title,
            slug,
            summary,
            content,
            published_at,
            created_at,
            updated_at
        FROM posts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(post_data): Json<CreatePostBody>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, created a post", auth.user.username);

    let id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
    INSERT INTO posts (
        id,
        archived,
        title,
        slug,
        summary,
        content
    )
    VALUES ($1, $2, $3, $4, $5, $6)
    "#,
    )
    .bind(id)
    .bind(true)
    .bind(post_data.title)
    .bind(post_data.slug)
    .bind(post_data.summary)
    .bind(post_data.content)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Errror: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, deleted a post", auth.user.username);

    sqlx::query(
        r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error deleting session: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::NO_CONTENT, Json(id)))
}

pub async fn edit_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(post_data): Json<EditPostQuery>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, edited a post", auth.user.username);

    sqlx::query(
        r#"
    UPDATE posts
    SET 
        archived = $2,
        title = $3,
        slug = $4,
        summary = $5,
        content = $6
        updated_at = NOW()
    WHERE id = $7
    "#,
    )
    .bind(post_data.archived)
    .bind(post_data.title)
    .bind(post_data.slug)
    .bind(post_data.summary)
    .bind(post_data.content)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error updating post: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn publish_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    println!("The user {}, published a post", auth.user.username);

    let result = sqlx::query(
        r#"
        UPDATE posts
        SET 
            archived = false,
            published_at = COALESCE(published_at, NOW()),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error publishing post: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}
