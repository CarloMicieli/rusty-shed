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

        let _inventory = repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "TrackInventory".to_string(),
                identifier: id.to_string(),
            })?;

        repo.delete(id).await?;

        Ok(())
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
        TrackInventory::new(id, "My Inventory".to_string(), None)
    }

    #[tokio::test]
    async fn it_deletes_existing_inventory() {
        let id = TrackInventoryId::default();
        let id_clone = id.clone();
        let inventory = make_inventory(id.clone());

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(inventory.clone())));
        repo.expect_delete().times(1).returning(|_| Ok(()));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = DeleteTrackInventoryUseCase::execute(&mut uow_box, &id_clone).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn it_returns_not_found_when_inventory_missing() {
        let id = TrackInventoryId::default();
        let id_clone = id.clone();

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id().times(1).returning(|_| Ok(None));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = DeleteTrackInventoryUseCase::execute(&mut uow_box, &id_clone).await;

        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}
