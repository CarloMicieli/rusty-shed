use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::{
    TrackId, TrackInventoryEvent, TrackInventoryId, TrackQuantity, TracksInventoryUowExt,
};

/// Use case to set the quantity for a specific track product in an inventory.
#[allow(dead_code)]
pub struct SetTrackItemQuantityUseCase;

impl SetTrackItemQuantityUseCase {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        inventory_id: &TrackInventoryId,
        track_id: TrackId,
        quantity: i64,
    ) -> Result<(), DomainError>
    where
        U: TracksInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let maybe = repo.find_by_id(inventory_id).await?;
        let mut inventory = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "TrackInventory".to_string(),
            identifier: inventory_id.to_string(),
        })?;

        if quantity <= 0 {
            inventory.inventory.remove(&track_id);
        } else {
            inventory.inventory.insert(
                track_id.clone(),
                TrackQuantity {
                    track_id: track_id.clone(),
                    quantity,
                },
            );
        }

        inventory.push_event(TrackInventoryEvent::ItemQuantitySet { track_id, quantity });

        repo.save(inventory).await
    }
}
