use axum::extract::State;
use axum::{Json, extract::Query, http::StatusCode};
use chrono::{DateTime, Utc}; // Ensure Utc is imported
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
        None => {
            let to_summary = |p: &Post| Post {
                title: p.title.clone(),
                date: p.date.clone(),
                summary: p.summary.clone(),
                slug: p.slug.clone(),
                content: None,
            };

            match query.page {
                None => {
                    let start = 0;
                    let end = 5.min(posts.len());
                    let summary_list = posts[start..end].iter().map(to_summary).collect();
                    (StatusCode::OK, Json(summary_list))
                }

                Some(page) => {
                    let start = page * 5;
                    let end = (start + 5).min(posts.len());

                    let page_posts = if start >= posts.len() {
                        vec![]
                    } else {
                        posts[start..end].iter().map(to_summary).collect()
                    };

                    (StatusCode::OK, Json(page_posts))
                }
            }
        }
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
    let mut posts_with_date = Vec::new();

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

            // Parse date ONCE here - Schwartzian transform
            let date_parsed = post
                .date
                .parse::<DateTime<Utc>>()
                .unwrap_or_else(|_| Utc::now());
            posts_with_date.push((post, date_parsed));
        }
    }

    posts_with_date.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(posts_with_date.into_iter().map(|(p, _)| p).collect())
}
