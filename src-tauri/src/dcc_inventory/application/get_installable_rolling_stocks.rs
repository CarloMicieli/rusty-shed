use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::InstallableRollingStockView;
use crate::dcc_inventory::domain::DccInventoryUowExt;

/// Use case to fetch all rolling stocks that can have a decoder installed.
pub struct GetInstallableRollingStocksUseCase;

impl GetInstallableRollingStocksUseCase {
    /// Execute the use case to fetch installable rolling stocks.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the query.
    ///
    /// # Returns
    /// - `Ok(Vec<InstallableRollingStockView>)` containing installable rolling stocks on success.
    /// - `Err(DomainError)` when the repository query fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
    ) -> Result<Vec<InstallableRollingStockView>, DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();
        repo.find_installable_rolling_stocks().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::RollingStockCategory;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::dcc_inventory::application::InstallableRollingStockView;
    use crate::dcc_inventory::application::testing::FakeUow;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_should_return_installable_rolling_stocks() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let view = InstallableRollingStockView {
            owned_rolling_stock_id: OwnedRollingStockId::from(Uuid::new_v4()),
            category: RollingStockCategory::Locomotive,
            railway_company_name: Some("Test Railway".to_string()),
            road_number: Some("1001".to_string()),
            series_code: Some("E.656".to_string()),
            has_decoder: false,
            dcc_interface: None,
        };

        mock.expect_find_installable_rolling_stocks()
            .times(1)
            .returning(move || Ok(vec![view.clone()]));

        let mut uow = FakeUow::new(mock);

        let result = GetInstallableRollingStocksUseCase::execute(&mut uow)
            .await
            .expect("query should succeed");

        assert_eq!(result.len(), 1);
        assert!(!result[0].has_decoder);
    }
}
