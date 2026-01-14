use crate::collecting::domain::Collection;
use crate::collecting::domain::collection_view::CollectionView;
use crate::collecting::domain::depot_view::DepotView;
use crate::core::domain::domain_error::DomainError;

/// A domain-agnostic interface for collection data access.
///
/// This trait abstracts away the database engine (SQLite, Postgres, etc.),
/// allowing the business logic to remain decoupled from infrastructure details.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait CollectionRepository: Send + Sync {
    /// Retrieves a collection from the underlying data store.
    ///
    /// If no collection is found, a default, empty `Collection` is automatically
    /// created and returned. Therefore, on success this function will return a
    /// `Collection` value. Database errors (I/O, query failures, etc.) are not
    /// swallowed and will be returned as `Err`.
    async fn find_view(&mut self) -> Result<CollectionView, DomainError>;

    /// Persists the current state of the collection to the data store.
    ///
    /// # Arguments
    /// * `collection` - The `Collection` instance to be saved.
    ///
    /// If the save operation fails, a `DomainError` is returned.
    async fn save(&mut self, collection: &mut Collection) -> Result<(), DomainError>;

    /// Retrieves a `DepotView` representation listing owned rolling stocks
    /// suitable for the UI depot listing.
    ///
    /// Returns a `DepotView` on success or a `DomainError` on failure.
    async fn find_depot_view(&mut self) -> Result<DepotView, DomainError>;
}

/// An extension trait that provides access to the `CollectionRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait CollectionUowExt: Send {
    /// Returns a trait object for interacting with collections data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_>;
}
