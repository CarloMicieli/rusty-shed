//! Use case to delete a track inventory.

use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::{TrackInventoryId, TracksInventoryUowExt};

/// Use case to delete a track inventory and its associated data.
pub struct DeleteTrackInventoryUseCase;

impl DeleteTrackInventoryUseCase {
    /// Executes the use case to delete a track inventory.
    ///
    /// This will cascade delete all associated:
    /// - Inventory items
    /// - Purchase history
    ///
    /// # Arguments
    /// - `unit_of_work` - The unit of work to manage the transaction.
    /// - `id` - The identifier of the inventory to delete.
    ///
    /// # Returns
    /// * A result indicating success or a domain error.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `TracksInventoryUowExt` and be `Send`.
    pub async fn execute<U>(unit_of_work: &mut U, id: &TrackInventoryId) -> Result<(), DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        // Verify inventory exists
        let _inventory = repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "TrackInventory".to_string(),
                identifier: id.to_string(),
            })?;

        // Delete cascades via database constraints:
        // - track_inventory_items (ON DELETE CASCADE)
        // - track_purchases (ON DELETE CASCADE)
        repo.delete(id).await?;

        Ok(())
    }
}
