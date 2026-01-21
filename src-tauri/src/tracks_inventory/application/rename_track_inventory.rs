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
    use crate::core::domain::IdProvider;
    use crate::core::domain::test_utils::{DefaultMockIdProvider, MockIdProvider};
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn it_should_rename_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let fixed_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();
        let id_provider = MockIdProvider::new(fixed_id.clone());
        let inventory_id = id_provider.next_id();

        let new_name = "Renamed Inventory".to_string();

        let input = RenameTrackInventoryInput {
            id: inventory_id.clone(),
            new_name: new_name.clone(),
        };

        RenameTrackInventoryUseCase::execute(&mut unit_of_work, input)
            .await
            .expect("rename should succeed");

        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&inventory_id)
            .await
            .expect("find")
            .expect("inventory exists");

        assert_eq!(reloaded.name, new_name);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_not_found_when_inventory_missing(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let missing_provider = DefaultMockIdProvider::default();
        let missing_id: TrackInventoryId = missing_provider.value();

        let input = RenameTrackInventoryInput {
            id: missing_id.clone(),
            new_name: "No such inventory".to_string(),
        };

        let res = RenameTrackInventoryUseCase::execute(&mut unit_of_work, input).await;

        assert!(matches!(res, Err(DomainError::NotFound { .. })));
    }
}
