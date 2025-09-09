use axum::{Router, routing::get};
use tower_http::services::ServeDir;

mod photos;
mod posts;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // serve assets for the blog posts in the /assets folder
        .nest_service("/assets", ServeDir::new("../assets"))
        .nest_service("/album", ServeDir::new("../album"))
        .route("/posts", get(posts::get_posts))
        .route("/photos", get(photos::get_photos));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
