use crate::catalog::domain::railway_model::{RailwayModelId, RollingStockId};
use crate::collecting::domain::Collection;
use crate::collecting::domain::CollectionId;
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::CollectionStats;
use crate::collecting::domain::OwnedRollingStockId;
use crate::collecting::domain::UpdateCollectionItemInput;
use crate::collecting::domain::collection_view::CollectionView;
use crate::collecting::domain::depot_view::DepotView;
use crate::core::domain::MonetaryAmount;
use crate::core::domain::domain_error::DomainError;
use chrono::NaiveDate;

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

    /// Retrieves a collection aggregate by its identifier.
    ///
    /// Returns `Ok(Some(Collection))` if found, `Ok(None)` if no collection
    /// with the given id exists, or `Err(DomainError)` on failure.
    async fn find_by_id(&mut self, id: &CollectionId) -> Result<Option<Collection>, DomainError>;

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

    /// Applies a mutable field update to a single collection item.
    ///
    /// Implementations should persist only the targeted field and keep
    /// collection metadata coherent after the mutation.
    async fn update_item(&mut self, input: &UpdateCollectionItemInput) -> Result<(), DomainError>;

    /// Marks a collection item as sold and persists its sale financial data.
    async fn sell_item(
        &mut self,
        collection_item_id: &CollectionItemId,
        sale_date: NaiveDate,
        sale_price: MonetaryAmount,
        buyer_id: Option<String>,
    ) -> Result<(), DomainError>;

    /// Marks a pre-ordered item as received (converts purchase_type PREORDER → PURCHASED).
    ///
    /// `received_date` is stored as the new `purchase_date`. The collection summary
    /// is recalculated so the item is now included in active counts.
    async fn receive_preorder(
        &mut self,
        collection_item_id: &CollectionItemId,
        received_date: NaiveDate,
    ) -> Result<(), DomainError>;

    /// Converts an existing PURCHASED purchase_info row to PREORDER after creation.
    ///
    /// Called immediately after `AddCollectionItem::execute` when the caller
    /// specified a Preorder purchase type. The deposit / total / expected_date
    /// fields are set and `purchased_price_amount` is cleared (null).
    #[allow(clippy::too_many_arguments)]
    async fn convert_to_preorder(
        &mut self,
        collection_item_id: &CollectionItemId,
        deposit_amount: i64,
        deposit_currency: &str,
        preorder_total_amount: i64,
        preorder_total_currency: &str,
        expected_date: Option<NaiveDate>,
    ) -> Result<(), DomainError>;

    /// Returns lifecycle stats (preordered / active / sold counts + financial aggregates).
    async fn get_stats(&mut self) -> Result<CollectionStats, DomainError>;

    /// Creates an `owned_rolling_stocks` row for every active collection item
    /// that references `railway_model_id`.
    ///
    /// Called after a new rolling stock variant is added to a model that already
    /// belongs to one or more collections, so that the ownership link is created
    /// for every existing collection item automatically.
    async fn add_owned_rolling_stock_for_collection_items(
        &mut self,
        railway_model_id: &RailwayModelId,
        rolling_stock_id: &RollingStockId,
    ) -> Result<Vec<OwnedRollingStockId>, DomainError>;
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
