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
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::core::domain::domain_error::DomainError;
    use crate::tracks_inventory::domain::{MockTrackInventoryRepository, TrackId, TrackInventory};

    fn make_inventory(id: TrackInventoryId) -> TrackInventory {
        TrackInventory::new(id, "Test Inventory".to_string(), None)
    }

    #[tokio::test]
    async fn it_sets_item_quantity_for_existing_inventory() {
        let inventory_id = TrackInventoryId::default();
        let inventory = make_inventory(inventory_id.clone());

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(inventory.clone())));
        repo.expect_save().times(1).returning(|_| Ok(()));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let input = SetTrackItemQuantityInput {
            inventory_id,
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 5,
        };

        let result = SetTrackItemQuantityUseCase::execute(&mut uow_box, input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn it_returns_not_found_when_inventory_missing() {
        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id().times(1).returning(|_| Ok(None));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let input = SetTrackItemQuantityInput {
            inventory_id: TrackInventoryId::default(),
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 3,
        };

        let result = SetTrackItemQuantityUseCase::execute(&mut uow_box, input).await;

        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}
