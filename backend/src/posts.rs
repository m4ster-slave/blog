use crate::auth::AuthUser;
use crate::models::post::{Post, PostSummary};
use axum::extract::{Path, Query, State};
use axum::{Json, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

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
        println!("Fetch error on posts: {}", e);
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
            updated_at
        FROM posts
        WHERE slug = $1
            AND archived = false
        ORDER BY created_at DESC
        "#,
    )
    .bind(slug)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
        println!("Error deleting post: {}", e);
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
        archived = $1,
        title = $2,
        slug = $3,
        summary = $4,
        content = $5,
        updated_at = NOW()
    WHERE id = $6
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

    Ok((StatusCode::OK, Json(id)))
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
        println!("Fetch error on posts: {}", e);
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
            updated_at
        FROM posts
        WHERE id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        println! {"Admin route failed to get post by id: {}", e};
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let post = match post {
        Some(p) => p,
        None => return Err(StatusCode::NOT_FOUND),
    };

    Ok((StatusCode::OK, Json(post)))
}
