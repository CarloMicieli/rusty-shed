use crate::core::domain::domain_error::DomainError;
use crate::dashboard::domain::DashboardSummary;

/// Repository trait for accessing dashboard-related data.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait DashboardRepository: Send + Sync {
    /// Retrieves a comprehensive summary for the dashboard.
    ///
    /// # Arguments
    /// - `params: QueryParams` - Parameters to customize the summary retrieval.
    ///
    /// # Returns
    /// - `Ok(DashboardSummary)` containing the dashboard data on success.
    /// - `Err(DomainError)` if there is an issue retrieving the data.
    async fn find_summary(&mut self, params: QueryParams) -> Result<DashboardSummary, DomainError>;
}

/// An extension trait that provides access to the `DashboardRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait DashboardUowExt: Send {
    /// Returns a trait object for interacting with dashboard data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn dashboard_repository(&mut self) -> Box<dyn DashboardRepository + '_>;
}

/// Query parameters for retrieving the dashboard summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryParams {
    /// Number of recent items to retrieve for the dashboard.
    pub number_of_recent_items: u8,
    /// Path to the directory where model images are stored.
    pub models_dir: std::path::PathBuf,
}
