use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // serve assets for the blog posts in the /assets folder
        .nest_service("/assets", ServeDir::new("../assets"))
        .route("/posts", get(get_posts));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct SlugQuery {
    slug: Option<String>,
}

async fn get_posts(Query(query): Query<SlugQuery>) -> (StatusCode, Json<Vec<Post>>) {
    match query.slug {
        None => {
            let posts = read_markdown_files("../posts");

            match posts {
                Ok(p) => (StatusCode::OK, Json(p)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
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

#[derive(Serialize, Deserialize)]
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
