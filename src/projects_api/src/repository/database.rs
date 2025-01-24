use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;
use std::{env, fs};

pub async fn setup() -> Result<SqlitePool, sqlx::Error> {
    let database_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("Using database: {}", database_path);

    if !Path::new(&database_path).exists() {
        log::info!("Creating database: {}", database_path);
        fs::File::create(&database_path).expect("Failed to create database file");
    }

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", database_path))
        .await?;

    sqlx::query(include_str!("../../migrations/001_create_tables.sql"))
        .execute(&pool)
        .await?;

    Ok(pool)
}
