use crate::core::domain::domain_error::DomainError;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;

/// Repository trait for managing sellers in the data store.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait SellersRepository: Send + Sync {
    /// Lists all sellers from the data store.
    async fn list(&mut self) -> Result<Vec<Seller>, DomainError>;

    /// Retrieves a seller by its ID.
    async fn get(&mut self, id: &SellerId) -> Result<Option<Seller>, DomainError>;

    /// Retrieves a presentation `SellerView` by id.
    async fn find_seller_view_by_id(
        &mut self,
        id: &SellerId,
    ) -> Result<Option<SellerView>, DomainError>;

    /// Upserts a seller into the data store.
    async fn upsert(&mut self, seller: &Seller) -> Result<(), DomainError>;

    /// Deletes a seller by its ID.
    async fn delete(&mut self, id: &SellerId) -> Result<u64, DomainError>;

    /// Persist events produced by a `Seller` aggregate.
    ///
    /// Implementations should iterate `seller.pull_events()` and apply the
    /// corresponding database operations. The method takes a mutable reference
    /// so that `pull_events()` can clear the aggregate's pending events.
    async fn save(&mut self, seller: &mut Seller) -> Result<(), DomainError>;

    /// Returns the `(name, is_system_seeded)` pair for the given seller, or
    /// `None` when no matching row exists.
    ///
    /// Used to enforce business rules (e.g. seeded-name immutability) before
    /// mutating a seller without loading the full aggregate.
    async fn find_seeded_and_name(
        &mut self,
        id: &SellerId,
    ) -> Result<Option<(String, bool)>, DomainError>;

    /// Returns the total number of purchase records that reference this party
    /// as either a seller or buyer.
    ///
    /// Used to populate the `usage_count` field in `SellerView` and to guard
    /// against deleting a seller that is still referenced.
    async fn find_usage_count(&mut self, id: &SellerId) -> Result<i64, DomainError>;
}

/// An extension trait that provides access to the `SellersRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait SellersUowExt: Send {
    /// Returns a trait object for interacting with sellers data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn sellers_repository(&mut self) -> Box<dyn SellersRepository + '_>;
}
