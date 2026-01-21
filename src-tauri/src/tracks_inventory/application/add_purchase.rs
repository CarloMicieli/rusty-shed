use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::{
    TrackInventoryEvent, TrackInventoryId, TrackPurchase, TracksInventoryUowExt,
};

/// Use case to add a purchase record to an inventory.
#[allow(dead_code)]
pub struct AddTrackPurchaseUseCase;

impl AddTrackPurchaseUseCase {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        inventory_id: &TrackInventoryId,
        purchase: TrackPurchase,
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

        // update in-memory history
        inventory.purchase_history.push(purchase.clone());

        inventory.push_event(TrackInventoryEvent::PurchaseAdded { purchase });

        repo.save(inventory).await
    }
}
