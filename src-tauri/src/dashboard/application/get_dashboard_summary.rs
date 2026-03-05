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
