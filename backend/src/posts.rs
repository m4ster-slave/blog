use crate::auth::AuthUser;
use crate::models::post::{Post, PostSummary};
use axum::extract::{Path, Query, State};
use axum::{Json, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct EditPostQuery {
    title: String,
    slug: String,
    summary: String,
    content: String,
    archived: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostBody {
    title: String,
    slug: String,
    summary: String,
    content: String,
}

fn count_words(markdown: &str) -> i32 {
    use pulldown_cmark::{Event, Parser};

    let sum: usize = Parser::new(markdown)
        .filter_map(|event| match event {
            Event::Text(text) => Some(text),
            _ => None,
        })
        .map(|text| text.split_whitespace().count())
        .sum();

    sum as i32
}

pub async fn get_posts(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<crate::models::pagination::Pagination>,
) -> Result<(StatusCode, Json<Vec<PostSummary>>), StatusCode> {
    let page = pagination.page.unwrap_or(0).max(0);
    let limit = pagination.limit.unwrap_or(20).clamp(1, 100);
    let offset = page * limit;

    let posts = sqlx::query_as::<_, PostSummary>(
        r#"
        SELECT
            id,
            title,
            slug,
            summary,
            archived,
            published_at,
            created_at,
            updated_at
        FROM posts
        WHERE archived = false
        ORDER BY published_at DESC
        LIMIT $1
        OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!(error = %e, status = 500, "failed to fetch published posts");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn get_post_by_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let post = sqlx::query_as::<_, Post>(
        r#"
        SELECT
            id,
            title,
            slug,
            summary,
            archived,
            content,
            published_at,
            created_at,
            updated_at,
            word_count,
            modify_count
        FROM posts
        WHERE slug = $1
            AND archived = false
        ORDER BY created_at DESC
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        error!(slug = %slug, error = %e, status = 500, "failed to fetch post");

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let post = match post {
        Some(p) => p,
        None => return Err(StatusCode::NOT_FOUND),
    };

    Ok((StatusCode::OK, Json(post)))
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
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
        word_count,
        content
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#,
    )
    .bind(id)
    .bind(true)
    .bind(post_data.title)
    .bind(post_data.slug)
    .bind(post_data.summary)
    .bind(count_words(&post_data.content))
    .bind(post_data.content)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!(user = %auth.user.username, post_id = %id, error = %e, status = 500, "failed to create post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(user = %auth.user.username, post_id = %id, status = 201, "created post");

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
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
        error!(user = %auth.user.username, post_id = %id, error = %e, status = 500, "failed to delete post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(user = %auth.user.username, post_id = %id, status = 204, "deleted post");

    Ok((StatusCode::NO_CONTENT, Json(id)))
}

pub async fn edit_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(post_data): Json<EditPostQuery>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    sqlx::query(
        r#"
    UPDATE posts
    SET 
        archived = $1,
        title = $2,
        slug = $3,
        summary = $4,
        word_count = $5,
        content = $6,
        updated_at = NOW()
    WHERE id = $7
    "#,
    )
    .bind(post_data.archived)
    .bind(post_data.title)
    .bind(post_data.slug)
    .bind(post_data.summary)
    .bind(count_words(&post_data.content))
    .bind(post_data.content)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!(user = %auth.user.username, post_id = %id, error = %e, status = 500, "failed to update post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(user = %auth.user.username, post_id = %id, status = 200, "updated post");

    Ok((StatusCode::OK, Json(id)))
}

pub async fn publish_post(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
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
        error!(user = %auth.user.username, post_id = %id, error = %e, status = 500, "failed to publish post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    info!(user = %auth.user.username, post_id = %id, status = 200, "published post");

    Ok(StatusCode::OK)
}

pub async fn get_posts_admin(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
) -> Result<(StatusCode, Json<Vec<PostSummary>>), StatusCode> {
    let posts = sqlx::query_as::<_, PostSummary>(
        r#"
        SELECT
            id,
            title,
            slug,
            summary,
            archived,
            published_at,
            created_at,
            updated_at
        FROM posts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!(error = %e, status = 500, "failed to fetch admin posts");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn get_post_by_slug_admin(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let post = sqlx::query_as::<_, Post>(
        r#"
        SELECT
            id,
            title,
            slug,
            summary,
            archived,
            content,
            published_at,
            created_at,
            updated_at,
            word_count,
            modify_count
        FROM posts
        WHERE id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        error!(post_id = %id, error = %e, status = 500, "failed to fetch admin post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let post = match post {
        Some(p) => p,
        None => return Err(StatusCode::NOT_FOUND),
    };

    Ok((StatusCode::OK, Json(post)))
}
