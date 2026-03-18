use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::application::AddTrackPurchaseInput;
use crate::tracks_inventory::domain::{
    TrackInventoryEvent, TrackInventoryId, TrackPurchase, TrackPurchaseId, TracksInventoryUowExt,
};

/// Use case to add a purchase record to an inventory.
pub struct AddTrackPurchaseUseCase;

impl AddTrackPurchaseUseCase {
    /// Executes the use case to add a purchase to the specified track inventory.
    ///
    /// # Arguments
    /// - `unit_of_work` - The unit of work to manage the transaction.
    /// - `id_provider` - The provider to generate unique identifiers for the new track purchase.
    /// - `input` - The input data required to add the track purchase.
    ///
    /// # Returns
    /// * A result containing the identifier of the newly added track purchase or a domain error.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `TracksInventoryUowExt` and be `Send`.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: AddTrackPurchaseInput,
    ) -> Result<TrackPurchaseId, DomainError>
    where
        U: TracksInventoryUowExt + Send,
        P: IdProvider<TrackPurchaseId>,
    {
        let mut repo = unit_of_work.track_inventories_repo();

        let inventory_id = &input.id;

        let maybe = repo.find_by_id(inventory_id).await?;
        let mut inventory = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "TrackInventory".to_string(),
            identifier: inventory_id.to_string(),
        })?;

        let purchase = TrackPurchase {
            track_purchase_id: id_provider.next_id(),
            track_id: input.track_id,
            quantity: input.quantity,
            price: input.price,
            seller_id: input.seller_id,
            purchase_date: input.purchase_date,
        };

        let new_purchase_id = purchase.track_purchase_id.clone();

        inventory.add_purchase(purchase);

        repo.save(inventory).await.map(|_| new_purchase_id)
    }
}
