use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::dashboard::domain::{DashboardSummary, DashboardUowExt, QueryParams};
use log::info;

/// Query to retrieve the dashboard summary from the database.
pub struct GetDashboardSummaryQuery;

impl GetDashboardSummaryQuery {
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
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        number_of_recent_items: u8,
        number_of_depot_entries: u8,
    ) -> Result<DashboardSummary, DomainError> {
        info!("Retrieving dashboard summary");

        let params = QueryParams {
            number_of_recent_items,
            number_of_depot_entries,
        };

        let mut repository = unit_of_work.dashboard_repository();
        repository.find_summary(params).await
    }
}
