use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::DigitalRollingStockView;
use crate::dcc_inventory::domain::DccInventoryUowExt;

/// Use case to fetch all digital rolling stocks as views.
pub struct GetDigitalRollingStocksUseCase;

impl GetDigitalRollingStocksUseCase {
    /// Execute the use case to fetch all digital rolling stocks as view objects.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the query.
    ///
    /// # Returns
    /// - `Ok(Vec<DigitalRollingStockView>)` containing the views on success.
    /// - `Err(DomainError)` when the repository query fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
    ) -> Result<Vec<DigitalRollingStockView>, DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();
        repo.find_all_digital_rolling_stocks().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::dcc_inventory::application::DigitalRollingStockView;
    use crate::dcc_inventory::domain::DigitalRollingStockId;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use crate::dcc_inventory::domain::{DccAddress, DecoderId};
    use uuid::Uuid;

    use crate::dcc_inventory::application::testing::FakeUow;

    #[tokio::test]
    async fn it_should_return_all_digital_rolling_stocks_views() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let view = DigitalRollingStockView {
            id: DigitalRollingStockId::from_uuid(Uuid::new_v4()),
            owned_rolling_stock_id: OwnedRollingStockId::from(Uuid::new_v4()),
            dcc_address: DccAddress::new(1).unwrap(),
            decoder: crate::dcc_inventory::application::DecoderView {
                id: DecoderId::try_from("trn:decoder:acme:d-100").unwrap(),
                manufacturer: "ACME".to_string(),
                product_code: "d-100".to_string(),
                decoder_type: crate::dcc_inventory::domain::DecoderType::Plain,
                protocol: crate::dcc_inventory::domain::DigitalProtocol::Dcc,
                decoder_interface: crate::catalog::domain::railway_model::DccInterface::Nem651,
            },
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
