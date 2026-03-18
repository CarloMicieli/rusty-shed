//! Use case to delete a track inventory.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
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
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        id: &TrackInventoryId,
    ) -> Result<(), DomainError> {
        // Verify inventory exists
        let mut repo = unit_of_work.track_inventories_repo();
        let _inventory = repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "TrackInventory".to_string(),
                identifier: id.to_string(),
            })?;

        drop(repo); // Release the repository before making direct SQL calls

        // Delete cascades via database constraints:
        // - track_inventory_items (ON DELETE CASCADE)
        // - track_purchases (ON DELETE CASCADE)
        let sql = "DELETE FROM track_inventories WHERE id = ?1";
        sqlx::query(sql)
            .bind(id)
            .execute(&mut *unit_of_work.tx)
            .await
            .map_err(DomainError::from)?;

        Ok(())
    }
}
