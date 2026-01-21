use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::application::SetTrackItemQuantityInput;
use crate::tracks_inventory::domain::{TrackId, TrackInventoryId, TracksInventoryUowExt};

/// Use case to set the quantity for a specific track product in an inventory.
pub struct SetTrackItemQuantityUseCase;

impl SetTrackItemQuantityUseCase {
    /// Executes the use case to set the quantity of a track item in the specified track inventory.
    ///
    /// # Arguments
    /// - `unit_of_work` - The unit of work to manage the transaction.
    /// - `input` - The input data required to set the track item quantity.
    ///
    /// # Returns
    /// * A result indicating success or failure of the operation.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `TracksInventoryUowExt` and be `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SetTrackItemQuantityInput,
    ) -> Result<(), DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let maybe = repo.find_by_id(&input.inventory_id).await?;
        let mut inventory = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "TrackInventory".to_string(),
            identifier: input.inventory_id.to_string(),
        })?;

        inventory.set_item_quantity(input.track_id, input.quantity);

        repo.save(inventory).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use pretty_assertions::assert_eq;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn it_should_set_item_quantity(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let inventory_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();

        let track_id = TrackId::try_from("trn:track:acme:60100").unwrap();

        // Increase quantity to 5
        let input = SetTrackItemQuantityInput {
            inventory_id: inventory_id.clone(),
            track_id: track_id.clone(),
            quantity: 5,
        };

        SetTrackItemQuantityUseCase::execute(&mut unit_of_work, input)
            .await
            .expect("set quantity should succeed");

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&inventory_id)
            .await
            .expect("find")
            .expect("inventory exists");

        let qty = reloaded
            .inventory
            .get(&track_id)
            .map(|t| t.quantity)
            .unwrap_or_default();

        assert_eq!(qty, 5);
    }
}
