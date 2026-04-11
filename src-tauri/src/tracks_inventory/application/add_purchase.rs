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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::core::domain::currency::Currency;
    use crate::core::domain::monetary_amount::MonetaryAmount;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::tracks_inventory::domain::{MockTrackInventoryRepository, TrackId, TrackInventory};
    use chrono::NaiveDate;

    fn make_inventory(id: TrackInventoryId) -> TrackInventory {
        TrackInventory::new(id, "Test Inventory".to_string(), None)
    }

    #[tokio::test]
    async fn it_adds_purchase_and_returns_purchase_id() {
        let inventory_id = TrackInventoryId::default();
        let purchase_id = TrackPurchaseId::default();
        let purchase_id_clone = purchase_id.clone();
        let inventory = make_inventory(inventory_id.clone());

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(inventory.clone())));
        repo.expect_save().times(1).returning(|_| Ok(()));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let id_provider = MockIdProvider::new(purchase_id.clone());
        let input = AddTrackPurchaseInput {
            id: inventory_id,
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 2,
            price: MonetaryAmount::new(1234, Currency::EUR),
            seller_id: None,
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let result = AddTrackPurchaseUseCase::execute(&mut uow_box, id_provider, input).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), purchase_id_clone);
    }

    #[tokio::test]
    async fn it_returns_not_found_when_inventory_missing() {
        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_by_id().times(1).returning(|_| Ok(None));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let id_provider = MockIdProvider::new(TrackPurchaseId::default());
        let input = AddTrackPurchaseInput {
            id: TrackInventoryId::default(),
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 1,
            price: MonetaryAmount::new(100, Currency::EUR),
            seller_id: None,
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let result = AddTrackPurchaseUseCase::execute(&mut uow_box, id_provider, input).await;

        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}
