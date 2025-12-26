use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{Sqlite, migrate::MigrateDatabase};
use std::path::PathBuf;
use thiserror::Error;
use xdg::BaseDirectories;

// Embed migrations at compile time
// The path is relative to Cargo.toml of this crate
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db_pool() -> Result<SqlitePool, SqliteDbError> {
    // Determine app data directory using xdg crate
    let db_path = {
        let bd = BaseDirectories::with_prefix("rusty_shed");
        bd.place_data_file("rusty_shed.db").unwrap_or_else(|e| {
            eprintln!("Failed to determine data file path via XDG: {e}");
            PathBuf::from("rusty_shed.db")
        })
    };

    // Ensure parent directory exists so SQLite can create the file
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    let db_url = format!("sqlite:{}", db_path.display());
    eprintln!("Opening SQLite DB at {}", db_url);

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

#[derive(Error, Debug)]
pub enum SqliteDbError {
    #[error("database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}
