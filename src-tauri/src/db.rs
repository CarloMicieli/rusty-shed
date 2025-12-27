use log::debug;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{Sqlite, migrate::MigrateDatabase};
use std::path::PathBuf;
use thiserror::Error;

/// Embedded SQL migrations for the application.
///
/// These migrations are compiled into the binary using `sqlx::migrate!`.
/// The path is relative to the crate root (the `Cargo.toml` of this crate).
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Initialize and return a SQLite connection pool for the application.
///
/// This function performs the following steps:
///
/// - Use the provided `db_path` (file) where the SQLite DB should live.
/// - Ensure the parent directory of the chosen path exists so SQLite can
///   create the file.
/// - Build a `sqlite:` database URL and create the database file if it
///   does not already exist.
/// - Connect a `SqlitePool` (max 5 connections) to the database, run the
///   embedded migrations, and return the pool.
///
/// This function will execute the embedded migrations (from `MIGRATOR`)
/// against the newly-created pool before returning. If migration
/// execution fails the error will be returned.
///
/// Returns `Ok(SqlitePool)` on success or a `SqliteDbError` on failure.
pub async fn init_db_pool(db_path: PathBuf) -> Result<SqlitePool, SqliteDbError> {
    // Ensure parent directory exists so SQLite can create the file
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    let db_url = format!("sqlite:{}", db_path.display());
    debug!("Opening SQLite DB at {}", db_url);

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Run embedded migrations before returning the pool
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}

/// Errors that can occur while preparing or working with the SQLite DB.
#[derive(Error, Debug)]
pub enum SqliteDbError {
    /// A generic database error returned by `sqlx`.
    #[error("database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    /// Errors related to running embedded migrations.
    #[error("migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}
