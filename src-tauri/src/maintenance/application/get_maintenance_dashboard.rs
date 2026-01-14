use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::domain::maintenance_card::MaintenanceCard;

/// Use-case to retrieve maintenance cards that are due or overdue.
pub struct GetMaintenanceDashboardUseCase;

impl GetMaintenanceDashboardUseCase {
    /// Execute the use-case using the provided Unit of Work.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the maintenance repository.
    ///
    /// # Returns
    /// - `Ok(Vec<MaintenanceCard>)` containing due or overdue maintenance cards.
    /// - `Err(DomainError)` if an error occurred during the operation.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<MaintenanceCard>, DomainError>
    where
        U: MaintenanceUowExt + Send,
    {
        let mut repo = unit_of_work.maintenance_repository();

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
