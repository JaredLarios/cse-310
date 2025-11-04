use sqlx::{SqlitePool, Pool, Sqlite, query, sqlite};
use std::fs;
use std::path::PathBuf;

pub async fn create_items_table(pool: &Pool<Sqlite>) {
    println!("Creating Table...");
    query("
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE NOT NULL,
            name TEXT,
            description TEXT
        )
    ")
    .execute(pool)
    .await
    .unwrap();
}

pub async fn db() -> Result<Pool<Sqlite>, sqlx::Error> {
    println!("Starting local SQLite database...");

    // Ensure the folder exists (optional if you want a folder)
    fs::create_dir_all("data").expect("Failed to create database directory");

    // Connect to local SQLite database file
    let db_file_name = "my_database.db";
    let db_path = PathBuf::from(db_file_name);
    let options = sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true);
    
    let pool = SqlitePool::connect_with(options).await?;

    println!("✅ SQLite database ready!");
    Ok(pool)
}