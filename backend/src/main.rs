use anyhow::Ok;
use axum::{
    Router, middleware,
    routing::{get, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;

mod auth;
mod database;
mod devlog_entries;
mod models;
mod posts;
mod stats;

struct AppState {
    pool: PgPool,
    request_stats: stats::RequestStats,
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

    let app_state = Arc::new(AppState {
        pool,
        request_stats: stats::RequestStats::default(),
    });

    let app = Router::new()
        .route("/posts", get(posts::get_posts))
        .route("/posts/{slug}", get(posts::get_post_by_slug))
        .route(
            "/admin/posts",
            post(posts::create_post).get(posts::get_posts_admin),
        )
        .route(
            "/admin/posts/{id}",
            put(posts::edit_post)
                .delete(posts::delete_post)
                .get(posts::get_post_by_slug_admin),
        )
        .route("/admin/posts/{id}/publish", post(posts::publish_post))
        .route("/login", post(auth::login))
        .route("/stats", get(stats::stats_aggregator))
        .route("/logout", post(auth::logout))
        .route("/admin/check", get(auth::check_session))
        .route(
            "/devlog/entries",
            get(devlog_entries::get_devlog_entries).post(devlog_entries::create_devlog_entry),
        )
        .route(
            "/devlog/entries/{id}",
            put(devlog_entries::edit_devlog_entry).delete(devlog_entries::delete_devlog_entry),
        )
        .with_state(app_state.clone())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            stats::request_middleware,
        ));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let stats_state = app_state.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        println!("tokio task spawned");

        interval.tick().await;

        loop {
            interval.tick().await;
            println!("flushing stats");

            stats::flush(&stats_state.pool, &stats_state.request_stats).await;
        }
    });

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
