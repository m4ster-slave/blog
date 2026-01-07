use axum::extract::State;
use axum::{Json, extract::Query, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SlugPageQuery {
    slug: Option<String>,
    page: Option<usize>,
}

pub async fn get_posts(
    Query(query): Query<SlugPageQuery>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<Post>>) {
    let posts = &state.posts;

    match query.slug {
        None => match query.page {
            None => (
                StatusCode::OK,
                Json(posts.iter().take(5).cloned().collect()),
            ),

            Some(page) => {
                let start = page * 5;
                let end = (start + 5).min(posts.len());
                let page_posts = if start >= posts.len() {
                    vec![]
                } else {
                    posts[start..end].to_vec()
                };

                (StatusCode::OK, Json(page_posts))
            }
        },
        Some(slug) => {
            if let Some(found) = posts.iter().find(|&p| p.slug == slug) {
                (StatusCode::OK, Json(vec![found.clone()]))
            } else {
                (StatusCode::NOT_FOUND, Json(vec![]))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    title: String,
    date: String,
    summary: String,
    slug: String,
    content: Option<String>,
}

pub fn read_markdown_files<P: AsRef<Path>>(
    dir_path: P,
) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let mut posts = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path)?;

            let parsed =
                gray_matter::Matter::<gray_matter::engine::YAML>::new().parse::<Post>(&content)?;

            let mut post = parsed
                .data
                .expect("Should have been able to parse metadata");

            post.content = Some(parsed.content);

            posts.push(post);
        }
    }

    posts.sort_by(|a, b| {
        let da: DateTime<Utc> = a.date.parse().unwrap();
        let db: DateTime<Utc> = b.date.parse().unwrap();
        db.cmp(&da) // reverse order: newest first
    });

    Ok(posts)
}
