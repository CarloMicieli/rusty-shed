use crate::collecting::domain::Collection;
use crate::core::domain::domain_error::DomainError;

/// A domain-agnostic interface for collection data access.
///
/// This trait abstracts away the database engine (SQLite, Postgres, etc.),
/// allowing the business logic to remain decoupled from infrastructure details.
#[async_trait::async_trait]
pub trait CollectionRepository: Send + Sync {
    /// Retrieves a collection from the underlying data store.
    ///
    /// If no collection is found, a default, empty `Collection` is automatically
    /// created and returned. Therefore, on success this function will return a
    /// `Collection` value. Database errors (I/O, query failures, etc.) are not
    /// swallowed and will be returned as `Err`.
    async fn get_collection(&mut self) -> Result<Collection, DomainError>;
}
