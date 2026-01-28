use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::DigitalRollingStockView;
use crate::dcc_inventory::domain::DccInventoryUowExt;

/// Use case to fetch all digital rolling stocks as views.
pub struct GetDigitalRollingStocksUseCase;

impl GetDigitalRollingStocksUseCase {
    pub async fn execute<U>(
        unit_of_work: &mut U,
    ) -> Result<Vec<DigitalRollingStockView>, DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repo();
        repo.find_all_digital_rolling_stocks().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::dcc_inventory::application::DigitalRollingStockView;
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
    async fn it_should_return_all_digital_rolling_stocks_views() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let view = DigitalRollingStockView {
            id: DigitalRollingStockId::from_uuid(Uuid::new_v4()),
            owned_rolling_stock_id: OwnedRollingStockId::from(Uuid::new_v4()),
            dcc_address: DccAddress::new(1).unwrap(),
            decoder_id: DecoderId::try_from("trn:decoder:acme:d-100").unwrap(),
        };

        mock.expect_find_all_digital_rolling_stocks()
            .times(1)
            .returning(move || Ok(vec![view.clone()]));

        let mut uow = FakeUow::new(mock);

        let result = GetDigitalRollingStocksUseCase::execute(&mut uow)
            .await
            .expect("query should succeed");

        assert_eq!(result.len(), 1);
    }
}
