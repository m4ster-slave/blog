use crate::models::post::Post;
use axum::extract::State;
use axum::{Json, extract::Query, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SlugPageQuery {
    slug: Option<String>,
    page: Option<usize>,
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
    Json(post_data): Json<CreatePostBody>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
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
