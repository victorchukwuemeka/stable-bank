use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use anyhow::Result;

pub async fn init_db() -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:stablebank.db?mode=rwc")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS deposits (
            id TEXT PRIMARY KEY,
            pubkey TEXT NOT NULL,
            amount_sol REAL NOT NULL,
            signature TEXT NOT NULL,
            created_at TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            pubkey TEXT PRIMARY KEY,
            total_deposited REAL NOT NULL DEFAULT 0,
            total_withdrawn REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    println!("Database ready");
    Ok(pool)
}