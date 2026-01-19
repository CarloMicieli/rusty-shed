use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::domain_error::DomainError;
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

    /// Persist a `TrackProduct` master record.
    async fn save(&mut self, track: TrackProduct) -> Result<(), DomainError>;
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
