use crate::core::infrastructure::seeder;
use log::debug;
use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
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
    pub async fn new_sqlite_pool(db_path: &Path) -> Result<SqlitePool, SqliteDbError> {
        let db_url = format!("sqlite:{}", db_path.display());
        debug!("Opening SQLite DB at {}", db_url);

        // 1. Create the connection options
        let options = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            // --- PERFORMANCE & CONCURRENCY ---
            // Sets WAL mode so reads don't block writes
            .journal_mode(SqliteJournalMode::Wal)
            // Optimization: In WAL mode, 'Normal' is safe and much faster than 'Full'
            .synchronous(SqliteSynchronous::Normal)
            // --- RELIABILITY ---
            // If the DB is locked, wait 5 seconds before giving up (prevents crashes)
            .busy_timeout(Duration::from_secs(5))
            // Ensures database-level data integrity
            .foreign_keys(true);

        let pool = SqlitePool::connect_with(options).await?;
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
    pub async fn run_initial_seed(pool: &SqlitePool) -> Result<(), anyhow::Error> {
        seeder::seed_railway_companies(pool).await?;
        seeder::seed_manufacturers(pool).await?;
        seeder::seed_track_products(pool).await?;
        seeder::seed_sellers(pool).await?;
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

#[cfg(test)]
pub async fn fk_violations(pool: &SqlitePool) -> Result<Vec<String>, anyhow::Error> {
    let sql = r#"PRAGMA foreign_key_check;"#;
    let violations = sqlx::query(sql).fetch_all(pool).await?;

    let mut results = Vec::new();
    use sqlx::Row;
    for row in violations {
        let table: String = row.get(0);
        let row_id: i64 = row.get(1);
        let target_table: String = row.get(2);
        let fk_id: i32 = row.get(3);

        results.push(format!(
            "FK(id={}) violation in table '{}' at rowid {}. Points to '{}'",
            fk_id, table, row_id, target_table
        ));
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[sqlx::test]
    async fn test_ensure_foreign_keys_are_enabled(pool: sqlx::SqlitePool) {
        let is_enabled: i32 = sqlx::query_scalar("PRAGMA foreign_keys")
            .fetch_one(&pool)
            .await
            .expect("Failed to query PRAGMA");

        assert_eq!(
            is_enabled, 1,
            "Foreign keys are DISABLED. Check your connection string or options."
        );
    }

    #[sqlx::test]
    async fn test_fk_constraint_behavior(pool: sqlx::SqlitePool) {
        let insert_cmd = r#"
        INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, series_code) 
        VALUES ('id1', 'non_existent_model_id', 'LOCOMOTIVE', 'non_existent_company_id', 'series');"#;

        let result = sqlx::query(insert_cmd).execute(&pool).await;

        // The test passes if the insertion fails with a Database error
        assert!(
            result.is_err(),
            "The database should have blocked the orphaned record"
        );

        // Optional: Check if the error is specifically an FK violation
        let err = result.unwrap_err();
        if let Some(sqlite_err) = err.as_database_error() {
            assert_eq!(sqlite_err.message(), "FOREIGN KEY constraint failed");
        }
    }
}
