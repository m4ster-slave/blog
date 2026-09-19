use anyhow::Ok;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use std::sync::Arc;

mod database;
mod models;
mod posts;

struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;
    let pool: PgPool = database::establish_connection(&db_url)
        .await
        .expect("Database connection failed");

    database::run_migrations(&pool)
        .await
        .expect("Some migrations failed");

    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/posts", get(posts::get_posts))
        .route("/posts", post(posts::create_post))
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
