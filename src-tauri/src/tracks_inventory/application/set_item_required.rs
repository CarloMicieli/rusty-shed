//! Use-case: Set required quantity for a track item in an inventory.
//!
//! This use-case updates the target/required quantity for a specific track
//! product within an inventory. The required quantity is used for planning
//! and identifying shortages (when stock < required).

use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::application::inputs::SetItemRequiredInput;
use crate::tracks_inventory::domain::TracksInventoryUowExt;

/// Use-case for setting the required quantity for a track item.
pub struct SetItemRequiredUseCase;

impl SetItemRequiredUseCase {
    /// Executes the use-case.
    ///
    /// Updates the required quantity for a specific track within an inventory.
    /// The inventory must exist, and the track item must already be present
    /// in the inventory (have been purchased at least once).
    ///
    /// # Errors
    /// Returns a `DomainError` if:
    /// - The inventory does not exist
    /// - The required quantity is negative
    /// - The database operation fails
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SetItemRequiredInput,
    ) -> Result<(), DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        // Validate required quantity
        if input.required < 0 {
            return Err(DomainError::BusinessRule(
                "Required quantity cannot be negative".to_string(),
            ));
        }

        // Update required quantity in database
        let sql = r#"
            UPDATE track_inventory_items
            SET required = ?1
            WHERE inventory_id = ?2 AND track_id = ?3
        "#;

        let result = sqlx::query(sql)
            .bind(input.required)
            .bind(&input.inventory_id)
            .bind(&input.track_id)
            .execute(unit_of_work.transaction())
            .await
            .map_err(DomainError::from)?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!(
                "Track item {} not found in inventory {}",
                input.track_id, input.inventory_id
            )));
        }

        Ok(())
    }
}
