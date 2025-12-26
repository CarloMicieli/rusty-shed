//! Database utilities for the Tauri backend.
//!
//! This module provides helpers to initialize a connection pool to a
//! SQLite database used by the application. Migrations are embedded at
//! compile time and can be run by code that uses the provided
//! `MIGRATOR` value.

use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{Sqlite, migrate::MigrateDatabase};
use std::path::PathBuf;
use log::error;
use thiserror::Error;
use xdg::BaseDirectories;
use uuid::Uuid;

/// Embedded SQL migrations for the application.
///
/// These migrations are compiled into the binary using `sqlx::migrate!`.
/// The path is relative to the crate root (the `Cargo.toml` of this crate).
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Initialize and return a SQLite connection pool for the application.
///
/// This function performs the following steps:
///
/// - Determine an application data file location using the XDG Base
///   Directories standard (via the `xdg` crate) with the prefix
///   `rusty_shed`. If that fails it falls back to `./rusty_shed.db`.
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
pub async fn init_db_pool() -> Result<SqlitePool, SqliteDbError> {
    // Determine app data directory using xdg crate
    let db_path = {
        let bd = BaseDirectories::with_prefix("rusty_shed");
        bd.place_data_file("rusty_shed.db").unwrap_or_else(|e| {
            error!("Failed to determine data file path via XDG: {e}");
            PathBuf::from("rusty_shed.db")
        })
    };

    // Ensure parent directory exists so SQLite can create the file
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    let db_url = format!("sqlite:{}", db_path.display());
    error!("Opening SQLite DB at {}", db_url);

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

/// Initialize and return an in-memory SQLite connection pool for tests.
///
/// This creates a unique, named in-memory database using a generated UUID
/// (for example: `sqlite:file:memdb-<uuid>?mode=memory&cache=shared`).
/// Because the database has a unique name each call returns an isolated
/// database instance. `cache=shared` allows multiple connections from the
/// returned pool to observe the same in-memory DB (so the pool's internal
/// connections see a consistent state).
///
/// The function will:
/// - generate a UUID-based name for the in-memory DB,
/// - construct the named in-memory SQLite URL with `mode=memory&cache=shared`,
/// - create a `SqlitePool` (max 5 connections),
/// - run the embedded migrations (`MIGRATOR`) against that pool, and
/// - return the migrated, ready-to-use pool.
///
/// This design avoids cross-test interference and migration races when
/// multiple tests run in parallel because each call targets a separate
/// in-memory database. Use this helper in tests that require an isolated
/// transient database. Note that in-memory databases are scoped to the
/// process — they are not shared across processes.
///
/// Returns `Ok(SqlitePool)` on success or a `SqliteDbError` on failure.
#[allow(dead_code)]
pub async fn init_in_memory_db_pool() -> Result<SqlitePool, SqliteDbError> {
    // Use a per-call UUID-backed named in-memory database so multiple
    // tests running in the same process get isolated databases while
    // still allowing the pool's connections to share the same DB.
    let id = Uuid::new_v4();
    let db_url = format!("sqlite:file:memdb-{}?mode=memory&cache=shared", id);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Run migrations against the in-memory database before returning.
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn in_memory_db_pool_runs_migrations_and_queries() {
        // Initialize an in-memory pool which will run migrations.
        let pool = init_in_memory_db_pool().await.expect("init in-memory pool");

        // Run a simple query to ensure the pool is usable.
        let row = sqlx::query("SELECT 1 as v").fetch_one(&pool).await.expect("select 1");
        let v: i64 = row.get("v");
        assert_eq!(v, 1);
    }
}
