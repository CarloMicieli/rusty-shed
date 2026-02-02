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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::application::{
        CreateTrackInventoryUseCase, NewTrackInventoryInput,
    };
    use crate::tracks_inventory::domain::TrackInventoryId;

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_delete_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();

        // Create an inventory first
        let fixed_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();
        let id_provider = MockIdProvider::new(fixed_id.clone());

        let input = NewTrackInventoryInput {
            name: "Test Inventory".to_string(),
            description: Some("To be deleted".to_string()),
        };

        let id = CreateTrackInventoryUseCase::execute(&mut unit_of_work, id_provider, input)
            .await
            .unwrap();

        unit_of_work.commit().await.unwrap();

        // Now delete it
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();
        let result = DeleteTrackInventoryUseCase::execute(&mut unit_of_work, &id).await;
        assert!(result.is_ok());

        unit_of_work.commit().await.unwrap();

        // Verify it's gone
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo.find_by_id(&id).await.unwrap();
        assert!(reloaded.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_fail_for_nonexistent_inventory(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();

        let fake_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-999999999999")
                .unwrap();

        let result = DeleteTrackInventoryUseCase::execute(&mut unit_of_work, &fake_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}
