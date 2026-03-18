// Remove Extra Budget Use Case
// Feature: 001-budget-tracking - Phase 6 (US4)

use crate::budget::domain::{BudgetRepository, ExtraBudgetId};
use crate::budget::infrastructure::BudgetUowExt;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

pub struct RemoveExtraBudgetUseCase;

impl RemoveExtraBudgetUseCase {
    /// Remove an extra budget entry.
    ///
    /// # Arguments
    /// * `uow` - Unit of work for database access
    /// * `id` - ID of the extra budget entry to remove
    ///
    /// # Errors
    /// - `NotFound`: If the extra budget entry doesn't exist
    /// - `Infrastructure`: If database operation fails
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        id: ExtraBudgetId,
    ) -> Result<(), DomainError> {
        let mut repo = uow.budget_repo();

        // Check if entry exists
        let entry = repo
            .get_extra_budget_by_id(&id)
            .await
            .map_err(DomainError::Infrastructure)?;

        if entry.is_none() {
            return Err(DomainError::NotFound {
                resource: "ExtraBudget".to_string(),
                identifier: id.as_ref().to_string(),
            });
        }

        // Remove entry
        repo.remove_extra_budget(&id)
            .await
            .map_err(DomainError::Infrastructure)?;

        Ok(())
    }
}
