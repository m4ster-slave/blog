use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

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

pub async fn establish_connection_from_env() -> anyhow::Result<PgPool> {
    let host = std::env::var("DATABASE_HOST")?;
    let port = std::env::var("DATABASE_PORT")?.parse::<u16>()?;
    let database = std::env::var("DATABASE_NAME")?;
    let username = std::env::var("DATABASE_USER")?;
    let password = std::env::var("DATABASE_PASSWORD")?;
    let options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .database(&database)
        .username(&username)
        .password(&password);

    PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect_with(options)
        .await
        .map_err(Into::into)
}
