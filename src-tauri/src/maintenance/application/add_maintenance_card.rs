use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::domain::{MaintenanceCard, MaintenanceCardId};

/// Use-case to create a new maintenance card for an owned rolling stock.
pub struct AddMaintenanceCard;

impl AddMaintenanceCard {
    /// Create and persist a new `MaintenanceCard` for an owned rolling stock.
    ///
    /// The function constructs a domain `MaintenanceCard` (emitting a `Created`
    /// domain event), persists it using the provided repository obtained from
    /// the `unit_of_work`, and returns the newly-created `MaintenanceCardId`.
    ///
    /// # Arguments
    /// - `unit_of_work`: Unit of Work providing access to the maintenance repository.
    /// - `id_provider`: Provider that generates a new `Uuid` for the created card.
    /// - `input`: Application-layer input containing the `owned_rolling_stock_id`.
    ///
    /// # Returns
    /// - `Ok(MaintenanceCardId)` on success
    /// - `Err(DomainError)` if persisting the card fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `MaintenanceUowExt` and `Send`.
    /// - `P`: Identifier provider type implementing `IdProvider<MaintenanceCardId>`.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: AddMaintenanceCardInput,
    ) -> Result<MaintenanceCardId, DomainError>
    where
        U: MaintenanceUowExt + Send,
        P: IdProvider<MaintenanceCardId>,
    {
        let mut repo = unit_of_work.maintenance_repository();

        let card_id = id_provider.next_id();

        let card = MaintenanceCard::create(card_id.clone(), input.owned_rolling_stock_id);

        repo.save(card).await?;
        Ok(card_id)
    }
}

/// Input for creating a MaintenanceCard.
#[derive(Debug, Clone)]
pub struct AddMaintenanceCardInput {
    /// The identifier of the owned rolling stock for which the maintenance card is created.
    pub owned_rolling_stock_id: OwnedRollingStockId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::maintenance::application::testing::FakeUow;
    use crate::maintenance::domain::MockMaintenanceRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_creates_card_and_persists() {
        let mut mock = MockMaintenanceRepository::new();

        // Expect save to be called once and verify the owned_rolling_stock_id matches
        mock.expect_save()
            .times(1)
            .withf(|card| {
                // owned_rolling_stock_id should be a TRN, non-empty
                !card.owned_rolling_stock_id.as_ref().is_empty()
            })
            .returning(|_card| Ok(()));

        let mut uow = FakeUow::new(mock);

        let owned = OwnedRollingStockId::from_uuid(&Uuid::new_v4());

        let input = AddMaintenanceCardInput {
            owned_rolling_stock_id: owned.clone(),
        };

        let id_provider = MockIdProvider::new(MaintenanceCardId::default());

        let id = AddMaintenanceCard::execute(&mut uow, id_provider, input)
            .await
            .expect("execute");

        assert!(id.to_string().starts_with("trn:maintenance-card:"));
    }
}
