use crate::core::domain::domain_error::DomainError;
use crate::dashboard::domain::{DashboardSummary, DashboardUowExt, QueryParams};

/// Query to retrieve the dashboard summary from the database.
pub struct GetDashboardSummary;

impl GetDashboardSummary {
    /// Execute the query to retrieve the dashboard summary
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `number_of_recent_items` - The number of recent items to include in the summary.
    /// * `number_of_depot_entries` - The number of depot entries to include in
    ///
    /// # Returns
    /// - `Ok(DashboardSummary)` dashboard summary on success.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `DashboardUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        number_of_recent_items: u8,
        models_dir: std::path::PathBuf,
    ) -> Result<DashboardSummary, DomainError>
    where
        U: DashboardUowExt + Send,
    {
        let params = QueryParams {
            number_of_recent_items,
            models_dir,
        };

        let mut repository = unit_of_work.dashboard_repository();
        repository.find_summary(params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dashboard::domain::{
        DashboardRepository, DashboardSummary, DashboardTotals, MockDashboardRepository,
    };

    struct FakeDashboardUow {
        repo: Option<MockDashboardRepository>,
    }

    impl FakeDashboardUow {
        fn new(repo: MockDashboardRepository) -> Self {
            Self { repo: Some(repo) }
        }
    }

    impl DashboardUowExt for FakeDashboardUow {
        fn dashboard_repository(&mut self) -> Box<dyn DashboardRepository + '_> {
            Box::new(
                self.repo
                    .take()
                    .expect("dashboard repository already taken"),
            )
        }
    }

    fn empty_summary() -> DashboardSummary {
        DashboardSummary {
            totals: DashboardTotals {
                collection_items: 0,
                wishlists: 0,
                maintenance_due: 0,
                total_value: None,
            },
            recent_items: vec![],
            purchase_groups: vec![],
        }
    }

    #[tokio::test]
    async fn it_returns_dashboard_summary_on_success() {
        let summary = empty_summary();

        let mut mock = MockDashboardRepository::new();
        mock.expect_find_summary()
            .times(1)
            .returning(move |_| Ok(summary.clone()));

        let mut uow = FakeDashboardUow::new(mock);
        let result = GetDashboardSummary::execute(&mut uow, 5, std::path::PathBuf::from("/tmp"))
            .await
            .expect("should return dashboard summary");

        assert_eq!(result.totals.collection_items, 0);
        assert!(result.recent_items.is_empty());
    }

    #[tokio::test]
    async fn it_passes_query_params_to_repository() {
        let models_dir = std::path::PathBuf::from("/models");
        let models_dir_clone = models_dir.clone();

        let mut mock = MockDashboardRepository::new();
        mock.expect_find_summary()
            .withf(move |p| p.number_of_recent_items == 10 && p.models_dir == models_dir_clone)
            .times(1)
            .returning(|_| Ok(empty_summary()));

        let mut uow = FakeDashboardUow::new(mock);
        GetDashboardSummary::execute(&mut uow, 10, models_dir)
            .await
            .expect("should succeed");
    }

    #[tokio::test]
    async fn it_propagates_infrastructure_error_from_repository() {
        let mut mock = MockDashboardRepository::new();
        mock.expect_find_summary()
            .times(1)
            .returning(|_| Err(DomainError::Infrastructure("db error".to_string())));

        let mut uow = FakeDashboardUow::new(mock);
        let err = GetDashboardSummary::execute(&mut uow, 5, std::path::PathBuf::from("/tmp"))
            .await
            .expect_err("repository error should propagate");

        assert!(
            matches!(err, DomainError::Infrastructure(_)),
            "expected Infrastructure error, got {err:?}"
        );
    }
}
