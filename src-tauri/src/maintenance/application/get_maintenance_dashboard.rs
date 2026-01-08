use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::maintenance::domain::maintenance_card::MaintenanceCard;
use crate::maintenance::infrastructure::repository::MaintenanceUowExt;

/// Use-case to retrieve maintenance cards that are due or overdue.
pub struct GetMaintenanceDashboardUseCase;

impl GetMaintenanceDashboardUseCase {
    /// Execute the use-case using the provided Unit of Work.
    ///
    /// Returns a vector of domain `MaintenanceCard` items converted from
    /// the infrastructure row mappers.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<MaintenanceCard>, DomainError> {
        let mut repo = unit_of_work.maintenance_repo();

        let rows = repo.list_due_cards().await?;

        // Map infra rows to domain models
        let mut out = Vec::with_capacity(rows.len());
        for r in rows.into_iter() {
            out.push(
                MaintenanceCard::try_from(r).map_err(|e| DomainError::Validation(e.to_string()))?,
            );
        }

        Ok(out)
    }
}
