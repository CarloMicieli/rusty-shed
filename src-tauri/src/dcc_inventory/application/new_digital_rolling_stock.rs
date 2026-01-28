use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::NewDigitalRollingStockInput;
use crate::dcc_inventory::domain::{
    DccInventoryUowExt, DigitalRollingStock, DigitalRollingStockId,
};

/// Use case to create a new DigitalRollingStock aggregate.
pub struct NewDigitalRollingStockUseCase;

impl NewDigitalRollingStockUseCase {
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: NewDigitalRollingStockInput,
    ) -> Result<DigitalRollingStockId, DomainError>
    where
        U: DccInventoryUowExt + Send,
        P: IdProvider<DigitalRollingStockId>,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repo();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use crate::dcc_inventory::domain::{DccAddress, DecoderId};
    use crate::dcc_inventory::domain::{DccInventoryUowExt, DigitalRollingStockId};
    use uuid::Uuid;

    struct FakeUow {
        repo: Option<MockDigitalRollingStockRepository>,
    }

    impl FakeUow {
        fn new(repo: MockDigitalRollingStockRepository) -> Self {
            Self { repo: Some(repo) }
        }
    }

    impl DccInventoryUowExt for FakeUow {
        fn digital_rolling_stocks_repo(
            &mut self,
        ) -> Box<dyn crate::dcc_inventory::domain::DigitalRollingStockRepository + '_> {
            Box::new(self.repo.take().expect("repo already taken"))
        }
    }

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
