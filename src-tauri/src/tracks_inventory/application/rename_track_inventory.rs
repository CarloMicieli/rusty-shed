use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::{
    TrackInventoryEvent, TrackInventoryId, TracksInventoryUowExt,
};

/// Use case to rename an existing `TrackInventory` aggregate.
#[allow(dead_code)]
pub struct RenameTrackInventoryUseCase;

impl RenameTrackInventoryUseCase {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        id: &TrackInventoryId,
        new_name: String,
    ) -> Result<(), DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let maybe = repo.find_by_id(id).await?;
        let mut inventory = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "TrackInventory".to_string(),
            identifier: id.to_string(),
        })?;

        inventory.push_event(TrackInventoryEvent::Renamed { name: new_name });

        repo.save(inventory).await
    }
}
