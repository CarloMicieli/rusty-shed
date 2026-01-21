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
    use crate::core::domain::currency::Currency;
    use crate::core::domain::monetary_amount::MonetaryAmount;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::sellers::domain::seller_id::SellerId;
    use crate::tracks_inventory::domain::{TrackId, TrackPurchaseId};
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn it_should_add_purchase(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");

        let inventory_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .unwrap();

        let input = AddTrackPurchaseInput {
            id: inventory_id.clone(),
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 2,
            price: MonetaryAmount::new(1234, Currency::EUR),
            seller_id: Some(SellerId::try_from("trn:seller:model-train-shop").unwrap()),
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let fixed_purchase_id =
            TrackPurchaseId::try_from("trn:track-purchase:00000000-0000-0000-0000-000000000009")
                .unwrap();
        let purchase_id_provider = MockIdProvider::new(fixed_purchase_id.clone());

        let returned =
            AddTrackPurchaseUseCase::execute(&mut unit_of_work, purchase_id_provider, input)
                .await
                .expect("execute should succeed");

        assert_eq!(returned, fixed_purchase_id);

        // Verify the purchase is present by reloading the aggregate using the same UnitOfWork
        let mut repo = unit_of_work.track_inventories_repo();
        let reloaded = repo
            .find_by_id(&inventory_id)
            .await
            .expect("find")
            .expect("inventory exists");

        assert!(
            reloaded
                .purchase_history
                .iter()
                .any(|p| p.track_id.to_string() == "trn:track:acme:60100" && p.quantity == 2)
        );
    }
}
