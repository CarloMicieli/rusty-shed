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
