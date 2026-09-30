use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod auth;
mod database;
mod devlog_entries;
mod files;
mod models;
mod logs;
mod posts;
mod stats;

struct AppState {
    pool: PgPool,
    request_stats: stats::RequestStats,
    log_dir: std::path::PathBuf,
    login_attempts: Mutex<HashMap<String, auth::LoginAttempt>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let log_dir = std::env::var_os("LOG_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("./logs"));
    std::fs::create_dir_all(&log_dir)?;
    let file_appender = tracing_appender::rolling::daily(&log_dir, "backend");
    let (file_writer, _log_guard) = tracing_appender::non_blocking(file_appender);
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout).with_ansi(false))
        .with(tracing_subscriber::fmt::layer().with_writer(file_writer).with_ansi(false))
        .init();

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
        log_dir,
        login_attempts: Mutex::new(HashMap::new()),
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
        .route("/admin/logs", get(logs::get_logs))
        .route(
            "/devlog/entries",
            get(devlog_entries::get_devlog_entries).post(devlog_entries::create_devlog_entry),
        )
        .route(
            "/devlog/entries/{id}",
            put(devlog_entries::edit_devlog_entry).delete(devlog_entries::delete_devlog_entry),
        )
        .route(
            "/admin/files",
            get(files::get_file_entries).post(files::create_file),
        )
        .route("/admin/files/{id}", delete(files::delete_file_entry))
        .with_state(app_state.clone())
        .layer(middleware::from_fn(auth::csrf_middleware))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            stats::request_middleware,
        ));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!(%addr, "starting server");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let stats_state = app_state.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        tracing::info!("stats flush task started");

        interval.tick().await;

        loop {
            interval.tick().await;

            stats::flush(&stats_state.pool, &stats_state.request_stats).await;
        }
    });

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
