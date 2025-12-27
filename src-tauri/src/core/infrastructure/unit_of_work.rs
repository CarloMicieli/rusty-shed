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
pub struct SqliteUnitOfWork<'conn> {
    /// The underlying SQLite transaction.
    pub tx: sqlx::Transaction<'conn, sqlx::Sqlite>,
}

impl<'conn> SqliteUnitOfWork<'conn> {
    /// Creates a new Unit of Work by starting a transaction on the provided pool.
    ///
    /// This is the entry point for a business transaction. Once created, the
    /// Unit of Work "locks" the connection for exclusive use until it is consumed.
    pub async fn new(pool: &sqlx::SqlitePool) -> Result<Self, sqlx::Error> {
        Ok(Self {
            tx: pool.begin().await?,
        })
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
