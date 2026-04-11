use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::application::RenameTrackInventoryInput;
use crate::tracks_inventory::domain::{
    TrackInventoryEvent, TrackInventoryId, TracksInventoryUowExt,
};

/// Use case to rename an existing `TrackInventory` aggregate.
pub struct RenameTrackInventoryUseCase;

impl RenameTrackInventoryUseCase {
    /// Executes the use case to rename a track inventory.
    ///
    /// # Arguments
    /// - `unit_of_work` - The unit of work to manage the transaction.
    /// - `input` - The input data required to rename the track inventory.
    ///
    /// # Returns
    /// * A result indicating success or failure of the operation.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `TracksInventoryUowExt` and be `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: RenameTrackInventoryInput,
    ) -> Result<(), DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let maybe = repo.find_by_id(&input.id).await?;
        let mut inventory = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "TrackInventory".to_string(),
            identifier: input.id.to_string(),
        })?;

        inventory.rename(input.new_name);

        repo.save(inventory).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::core::domain::domain_error::DomainError;
    use crate::tracks_inventory::domain::{MockTrackInventoryRepository, TrackInventory};

    fn make_inventory(id: TrackInventoryId) -> TrackInventory {
        TrackInventory::new(id, "Original Name".to_string(), None)
    }

    #[tokio::test]
    async fn it_renames_existing_inventory() {
        let id = TrackInventoryId::default();
        let inventory = make_inventory(id.clone());

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(inventory.clone())));
        repo.expect_save().times(1).returning(|_| Ok(()));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let input = RenameTrackInventoryInput {
            id,
            new_name: "Renamed Inventory".to_string(),
        };

        let result = RenameTrackInventoryUseCase::execute(&mut uow_box, input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn it_returns_not_found_when_inventory_missing() {
        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id().times(1).returning(|_| Ok(None));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let input = RenameTrackInventoryInput {
            id: TrackInventoryId::default(),
            new_name: "New Name".to_string(),
        };

        let result = RenameTrackInventoryUseCase::execute(&mut uow_box, input).await;

        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}
