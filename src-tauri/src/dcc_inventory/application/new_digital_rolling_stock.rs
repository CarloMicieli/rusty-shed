use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::domain::{
    DccAddress, DccInventoryUowExt, DecoderId, DigitalRollingStock, DigitalRollingStockId,
};

/// Use case to create a new DigitalRollingStock aggregate.
pub struct NewDigitalRollingStockUseCase;

impl NewDigitalRollingStockUseCase {
    /// Execute the use case to create a new `DigitalRollingStock` aggregate.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the use case.
    /// - `id_provider`: Provider used to obtain a new `DigitalRollingStockId`.
    /// - `input`: `NewDigitalRollingStockInput` containing initial aggregate data.
    ///
    /// # Returns
    /// - `Ok(DigitalRollingStockId)` with the created aggregate id on success.
    /// - `Err(DomainError)` when persistence fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    /// - `P`: `IdProvider` implementation that yields `DigitalRollingStockId` values.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: NewDigitalRollingStockInput,
    ) -> Result<DigitalRollingStockId, DomainError>
    where
        U: DccInventoryUowExt + Send,
        P: IdProvider<DigitalRollingStockId>,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();

        let id = id_provider.next_id();

        let drs = DigitalRollingStock::new(
            id.clone(),
            input.owned_rolling_stock_id,
            input.dcc_address,
            input.decoder_id,
        );

        repo.save(drs).await.map(|_| id)
    }
}

/// Input for creating a new digital rolling stock
#[derive(Debug, Clone)]
pub struct NewDigitalRollingStockInput {
    /// The owned rolling stock id associated with the digital rolling stock
    pub owned_rolling_stock_id: OwnedRollingStockId,
    /// The DCC address assigned to the digital rolling stock
    pub dcc_address: DccAddress,
    /// The decoder id assigned to the digital rolling stock
    pub decoder_id: DecoderId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::dcc_inventory::domain::DigitalRollingStockId;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use crate::dcc_inventory::domain::{DccAddress, DecoderId};
    use uuid::Uuid;

    use crate::dcc_inventory::application::testing::FakeUow;

    #[tokio::test]
    async fn it_should_create_new_digital_rolling_stock() {
        let mut mock = MockDigitalRollingStockRepository::new();

        mock.expect_save()
            .times(1)
            .withf(|drs: &DigitalRollingStock| drs.pending_events.is_empty())
            .returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);

        let fixed_id = DigitalRollingStockId::from_uuid(Uuid::new_v4());
        let id_provider = MockIdProvider::new(fixed_id.clone());

        let input = NewDigitalRollingStockInput {
            owned_rolling_stock_id: OwnedRollingStockId::from(Uuid::new_v4()),
            dcc_address: DccAddress::new(123).unwrap(),
            decoder_id: DecoderId::try_from("trn:decoder:acme:d-100").unwrap(),
        };

        let returned = NewDigitalRollingStockUseCase::execute(&mut uow, id_provider, input)
            .await
            .expect("execute should succeed");

        assert_eq!(returned, fixed_id);
    }
}
