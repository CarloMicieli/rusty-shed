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
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::domain::TrackInventoryId;
    use pretty_assertions::assert_eq;

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_create_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let name = "My Inventory".to_string();
        let description = Some("Created by test".to_string());

        let fixed_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000002")
                .unwrap();
        let id_provider = MockIdProvider::new(fixed_id.clone());

        let input = NewTrackInventoryInput {
            name: name.clone(),
            description: description.clone(),
        };

        let id = CreateTrackInventoryUseCase::execute(&mut unit_of_work, id_provider, input)
            .await
            .expect("create should succeed");

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&id)
            .await
            .expect("find")
            .expect("inventory exists");

        assert_eq!(reloaded.name, name);
        assert_eq!(reloaded.description, description);
    }
}
