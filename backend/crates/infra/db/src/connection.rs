use anyhow::Result;
use log;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn get_connection() -> Result<PgPool> {
    let db_url = get_db_url().await;
    log::trace!("Connecting to database: {}", db_url);
    let pool = PgPoolOptions::new().connect(&db_url).await?;
    Ok(pool)
}

pub async fn get_db_url() -> String {
    log::info!(
        "RUST_LOG: {}",
        env::var("RUST_LOG").expect("RUST_LOG is not set")
    );
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER is not set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD is not set");
    let host = env::var("DB_HOST").expect("DB_HOST is not set");
    let port = env::var("DB_PORT").expect("DB_PORT is not set");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB is not set");

    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db)
}
