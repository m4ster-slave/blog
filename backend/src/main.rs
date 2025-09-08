use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // or restrict to specific origins with .allow_origin("http://0.0.0.0:3001".parse().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // serve assets for the blog posts in the /assets folder
        .nest_service("/assets", ServeDir::new("../assets"))
        .route("/posts", get(get_posts))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct SlugPageQuery {
    slug: Option<String>,
    page: Option<usize>,
}

async fn get_posts(Query(query): Query<SlugPageQuery>) -> (StatusCode, Json<Vec<Post>>) {
    match query.slug {
        None => {
            let posts = match read_markdown_files("../posts") {
                Ok(p) => p,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
            };

            // TODO sort posts by date

            match query.page {
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
            }
        }
        Some(p) => {
            let post = read_post_by_slug("../posts", &p);

            match post {
                Ok(p) => (StatusCode::OK, Json(vec![p])),
                Err(_) => (StatusCode::NOT_FOUND, Json(vec![])),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    title: String,
    date: String,
    summary: String,
    slug: String,
    content: Option<String>,
}

fn read_markdown_files<P: AsRef<Path>>(
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

            // post.content = Some(parsed.content);
            post.content = None;

            posts.push(post);
        }
    }

    Ok(posts)
}

fn read_post_by_slug<P: AsRef<Path>>(
    dir_path: P,
    slug: &str,
) -> Result<Post, Box<dyn std::error::Error>> {
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

            if post.slug == slug {
                post.content = Some(parsed.content);
                return Ok(post);
            }
        }
    }

    Err(format!("Post with slug '{}' not found", slug).into())
}
