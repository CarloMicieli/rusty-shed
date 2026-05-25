use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::views::{
    TrackInventoryListItem, TrackInventoryView, TrackProductView,
};
use crate::tracks_inventory::domain::{TrackId, TrackInventory, TrackInventoryId, TrackProduct};

/// Repository trait for accessing track product master data.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TrackProductRepository: Send + Sync {
    /// Find a product by its canonical `TrackId`.
    async fn find_by_id(&mut self, id: &TrackId) -> Result<Option<TrackProduct>, DomainError>;

    /// Find a product by manufacturer and product code.
    async fn find_by_product_code(
        &mut self,
        manufacturer_id: &ManufacturerId,
        product_code: &str,
    ) -> Result<Option<TrackProduct>, DomainError>;

    /// Insert a `TrackProduct` master record.
    async fn insert_track(&mut self, track: &TrackProduct) -> Result<(), DomainError>;

    /// Update a `TrackProduct` master record.
    async fn update_track(&mut self, track: &TrackProduct) -> Result<(), DomainError>;

    /// Delete a `TrackProduct` master record.
    async fn delete_track(&mut self, track_id: &TrackId) -> Result<(), DomainError>;

    /// Create or update a translation row for the given product language.
    async fn upsert_translation(
        &mut self,
        track_id: &TrackId,
        lang: Language,
        description: Option<String>,
        details: Option<String>,
    ) -> Result<(), DomainError>;

    /// Delete one translation row for the given product language.
    async fn delete_translation(
        &mut self,
        track_id: &TrackId,
        lang: Language,
    ) -> Result<(), DomainError>;

    /// Return all track products as display views (joined with manufacturer name).
    async fn find_all_views(
        &mut self,
        lang: Language,
    ) -> Result<Vec<TrackProductView>, DomainError>;
}

/// Repository trait for accessing and persisting track inventories.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TrackInventoryRepository: Send + Sync {
    /// Find an inventory aggregate by its id.
    async fn find_by_id(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventory>, DomainError>;

    /// Persist a `TrackInventory` aggregate.
    async fn save(&mut self, inventory: TrackInventory) -> Result<(), DomainError>;

    /// Delete a track inventory (cascades to items and purchases via DB constraints).
    async fn delete(&mut self, id: &TrackInventoryId) -> Result<(), DomainError>;

    /// Update the `required` quantity for a specific track item in an inventory.
    ///
    /// Returns `true` if the row was found and updated, `false` otherwise.
    async fn set_item_required(
        &mut self,
        inventory_id: &TrackInventoryId,
        track_id: &TrackId,
        required: i64,
    ) -> Result<bool, DomainError>;

    /// Return all inventories as summary list items (with item/quantity counts).
    async fn find_all_summaries(&mut self) -> Result<Vec<TrackInventoryListItem>, DomainError>;

    /// Return a fully-populated view for a single inventory, or `None` if not found.
    async fn find_view_by_id(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventoryView>, DomainError>;
}

/// Unit of Work extension providing access to track-related repositories.
pub trait TracksInventoryUowExt: Send {
    /// Returns a repository for track product master data.
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_>;

    /// Returns a repository for inventory aggregates.
    fn track_inventories_repo(&mut self) -> Box<dyn TrackInventoryRepository + '_>;
}

/// Focused Unit-of-Work extension exposing only the TrackProduct repository.
///
/// This mirrors the project's per-repository UoW extension pattern (for example
/// `ManufacturerUowExt`) and allows code that only needs product master data to
/// request a narrower interface from a unit of work implementation.
pub trait TrackProductUowExt: Send {
    /// Returns a repository for track product master data bound to the UoW
    /// lifetime.
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_>;
}
