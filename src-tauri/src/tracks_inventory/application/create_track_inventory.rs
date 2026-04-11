use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{IdProvider, metadata::Metadata};
use crate::tracks_inventory::application::NewTrackInventoryInput;
use crate::tracks_inventory::domain::TracksInventoryUowExt;
use crate::tracks_inventory::domain::{TrackInventory, TrackInventoryEvent, TrackInventoryId};
use std::collections::HashMap;

/// Use case to create a new `TrackInventory` aggregate.
pub struct CreateTrackInventoryUseCase;

impl CreateTrackInventoryUseCase {
    /// Executes the use case to create a new track inventory.
    ///
    /// # Arguments
    /// - `unit_of_work` - The unit of work to manage the transaction.
    /// - `id_provider` - The provider to generate unique identifiers for the new track inventory.
    /// - `input` - The input data required to create the new track inventory.
    ///
    /// # Returns
    /// * A result containing the identifier of the newly created track inventory or a domain error.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `TracksInventoryUowExt` and be `Send`.
    /// * `P` - The type of the ID provider, which must implement `IdProvider<TrackInventoryId>`.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: NewTrackInventoryInput,
    ) -> Result<TrackInventoryId, DomainError>
    where
        U: TracksInventoryUowExt + Send,
        P: IdProvider<TrackInventoryId>,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let id = id_provider.next_id();

        let aggregate = TrackInventory::new(id.clone(), input.name, input.description);

        repo.save(aggregate).await.map(|_| id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::tracks_inventory::domain::MockTrackInventoryRepository;

    #[tokio::test]
    async fn it_creates_inventory_and_returns_id() {
        let fixed_id = TrackInventoryId::default();
        let fixed_id_clone = fixed_id.clone();

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_save().times(1).returning(|_| Ok(()));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let id_provider = MockIdProvider::new(fixed_id.clone());
        let input = NewTrackInventoryInput {
            name: "Test Inventory".to_string(),
            description: Some("A test".to_string()),
        };

        let result = CreateTrackInventoryUseCase::execute(&mut uow_box, id_provider, input).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), fixed_id_clone);
    }
}
