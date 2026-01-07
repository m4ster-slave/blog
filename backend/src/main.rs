use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::{
    photos::{Photo, read_photo_files},
    util::migrate_to_webp,
};

mod photos;
mod posts;
mod util;

struct AppState {
    photos: Vec<Photo>,
}

#[tokio::main]
async fn main() {
    if let Err(e) = migrate_to_webp("../album", 1000) {
        eprintln!("Error converting images to thumbnails: {}", e);
    }

    let photo_list = read_photo_files("../album").expect("Error reading album directory");
    let app_state = Arc::new(AppState { photos: photo_list });

    let app = Router::new()
        // serve assets for the blog posts in the /assets folder
        .nest_service("/assets", ServeDir::new("../assets"))
        .nest_service("/album", ServeDir::new("../album"))
        .route("/posts", get(posts::get_posts))
        .route("/photos", get(photos::get_photos))
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
