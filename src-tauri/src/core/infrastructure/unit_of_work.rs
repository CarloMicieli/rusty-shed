use crate::app_uow::{AppUnitOfWork, AppUowFactory};
use crate::core::infrastructure::error::CommandError;

/// A specialized implementation of the Unit of Work pattern for SQLite.
///
/// The `SqliteUnitOfWork` manages a single database transaction, ensuring that
/// all repository operations performed within its scope are atomic. It acts as
/// a container for the transaction and provides a point of entry for accessing
/// various repositories through extension traits.
///
/// ### Design Rationale
/// This struct wraps an `sqlx::Transaction`. By owning the transaction, it
/// enforces a clear lifecycle: the work must either be explicitly committed
/// via `.commit()` or it will be automatically rolled back when the struct
/// is dropped (a safety feature of `sqlx`).
pub struct SqliteUnitOfWork {
    /// The underlying SQLite transaction.
    pub tx: sqlx::Transaction<'static, sqlx::Sqlite>,
    pool: sqlx::SqlitePool,
}

impl SqliteUnitOfWork {
    /// Creates a new Unit of Work by starting a transaction on the provided pool.
    ///
    /// This is the entry point for a business transaction. Once created, the
    /// Unit of Work "locks" the connection for exclusive use until it is consumed.
    pub async fn new(pool: &sqlx::SqlitePool) -> Result<Self, sqlx::Error> {
        Ok(Self {
            tx: pool.clone().begin().await?,
            pool: pool.clone(),
        })
    }

    /// Returns a cloned handle to the backing SQLite pool.
    ///
    /// `SqlitePool` is an internally shared handle, so cloning is cheap.
    pub fn pool(&self) -> sqlx::SqlitePool {
        self.pool.clone()
    }

    /// Commits the atomic transaction to the database.
    ///
    /// This method consumes the `SqliteUnitOfWork`, ensuring that no further
    /// operations can be performed after the transaction is finalized.
    /// If this method is not called, the transaction will roll back on drop.
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await
    }
}

#[async_trait::async_trait]
impl AppUnitOfWork for SqliteUnitOfWork {
    async fn commit(self: Box<Self>) -> Result<(), CommandError> {
        (*self)
            .commit()
            .await
            .map_err(|e| CommandError::DatabaseError(e.to_string()))
    }
}

/// Production `AppUowFactory` that creates a `SqliteUnitOfWork` backed by a
/// real SQLite connection pool.
pub struct SqliteUowFactory {
    pool: sqlx::SqlitePool,
}

impl SqliteUowFactory {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AppUowFactory for SqliteUowFactory {
    async fn create_uow(&self) -> Result<Box<dyn AppUnitOfWork>, CommandError> {
        let uow = SqliteUnitOfWork::new(&self.pool)
            .await
            .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
        Ok(Box::new(uow))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_migrations_compile_and_run() {
        // 1. Create a pristine, isolated in-memory database connection pool
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to in-memory SQLite database");

        // 2. Point to your migrations folder relative to the src-tauri root
        // This macro validates SQL syntax and runs them in order
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run database migrations successfully");

        // 3. Optional: Verify a table exists to ensure it executed completely
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM manufacturers")
            .fetch_one(&pool)
            .await
            .expect("Failed to query the migrated manufacturers table");

        assert_eq!(row.0, 0, "The newly initialized database should be empty.");
    }
}
