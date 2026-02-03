use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::DigitalSummary;
use crate::dcc_inventory::domain::DccInventoryUowExt;

/// Use case to fetch digital rolling stock summary statistics.
pub struct GetDigitalSummaryUseCase;

impl GetDigitalSummaryUseCase {
    /// Execute the use case to fetch summary statistics.
    ///
    /// # Parameters
    /// - `unit_of_work`: Unit of work providing repository access required by the query.
    ///
    /// # Returns
    /// - `Ok(DigitalSummary)` containing summary statistics on success.
    /// - `Err(DomainError)` when the repository query fails.
    ///
    /// # Type Parameters
    /// - `U`: Unit-of-work type that implements `DccInventoryUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<DigitalSummary, DomainError>
    where
        U: DccInventoryUowExt + Send,
    {
        let mut repo = unit_of_work.digital_rolling_stocks_repository();
        repo.get_digital_summary().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dcc_inventory::application::DigitalSummary;
    use crate::dcc_inventory::application::testing::FakeUow;
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;

    #[tokio::test]
    async fn it_should_return_digital_summary() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let summary = DigitalSummary {
            total_non_dummy: 10,
            digital_count: 5,
            percentage: 50.0,
        };

        mock.expect_get_digital_summary()
            .times(1)
            .returning(move || Ok(summary.clone()));

        let mut uow = FakeUow::new(mock);

        let result = GetDigitalSummaryUseCase::execute(&mut uow)
            .await
            .expect("query should succeed");

        assert_eq!(result.total_non_dummy, 10);
        assert_eq!(result.digital_count, 5);
        assert_eq!(result.percentage, 50.0);
    }

    #[tokio::test]
    async fn it_should_calculate_zero_percentage_when_no_rolling_stocks() {
        let mut mock = MockDigitalRollingStockRepository::new();

        let summary = DigitalSummary {
            total_non_dummy: 0,
            digital_count: 0,
            percentage: 0.0,
        };

        mock.expect_get_digital_summary()
            .times(1)
            .returning(move || Ok(summary.clone()));

        let mut uow = FakeUow::new(mock);

        let result = GetDigitalSummaryUseCase::execute(&mut uow)
            .await
            .expect("query should succeed");

        assert_eq!(result.percentage, 0.0);
    }
}
