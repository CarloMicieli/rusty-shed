use std::path::PathBuf;
use log::debug;
use sqlx::{Sqlite, SqlitePool};
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::sqlite::SqlitePoolOptions;
use thiserror::Error;

/// Embedded SQL migrations for the application.
///
/// These migrations are compiled into the binary using `sqlx::migrate!`.
/// The path is relative to the crate root (the `Cargo.toml` of this crate).
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Application-level helper for managing the SQLite database lifecycle.
///
/// `Database` provides convenience functions to create a connection pool,
/// run the embedded migrations, and perform initial seeding. It centralizes
/// the common DB setup tasks used by the Tauri backend.
pub struct Database;

impl Database {
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
    /// Returns `Ok(SqlitePool)` on success or a `SqliteDbError` on failure.
    pub async fn new_sqlite_pool(db_path: &PathBuf) -> Result<SqlitePool, SqliteDbError> {
        // Ensure parent directory exists so SQLite can create the file
        //if let Some(parent) = db_path.parent() {
        //    std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
        //}

        let db_url = format!("sqlite:{}", db_path.display());
        debug!("Opening SQLite DB at {}", db_url);

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(pool)
    }

    /// Run the embedded SQL migrations that are compiled into the binary.
    ///
    /// This applies any pending migrations (as provided by `MIGRATOR`) to the
    /// database referenced by `pool`. Use this during startup after opening
    /// the connection pool to ensure schema is up-to-date.
    pub async fn run_migrations(pool: &SqlitePool) -> Result<(), SqliteDbError> {
        debug!("Running migrations...");
        MIGRATOR.run(pool).await?;
        Ok(())
    }

    /// Perform initial seeding of application data.
    ///
    /// This function is a placeholder for any one-time insertions or default
    /// data the application requires after a fresh database is created or
    /// after migrations. Implement seeding logic here as needed.
    pub async fn run_initial_seed(_pool: &SqlitePool) -> Result<(), SqliteDbError> {
        debug!("Running initial data seeding...");
        // Implement initial seeding logic here if needed
        Ok(())
    }
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
