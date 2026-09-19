use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    // Reads the `./migrations` folder at compile time and bundles it!
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}

pub async fn establish_connection(db_url: &str) -> anyhow::Result<PgPool> {
    // Production-ready pool configuration
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(db_url)
        .await?;

    Ok(pool)
}
