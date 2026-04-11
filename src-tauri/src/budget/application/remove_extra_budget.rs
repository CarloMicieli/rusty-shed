// Remove Extra Budget Use Case
// Feature: 001-budget-tracking - Phase 6 (US4)

use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::ExtraBudgetId;
use crate::core::domain::domain_error::DomainError;

pub struct RemoveExtraBudgetUseCase;

impl RemoveExtraBudgetUseCase {
    /// Remove an extra budget entry.
    ///
    /// # Arguments
    /// * `uow` - Unit of work for database access
    /// * `id` - ID of the extra budget entry to remove
    ///
    /// # Errors
    /// - `NotFound`: If the budget configuration or extra budget entry doesn't exist
    /// - `Infrastructure`: If database operation fails
    pub async fn execute<U>(uow: &mut U, id: ExtraBudgetId) -> Result<(), DomainError>
    where
        U: BudgetUowExt + Send,
    {
        let mut repo = uow.budget_repo();

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

        repo.remove_extra_budget(&id)
            .await
            .map_err(DomainError::Infrastructure)?;

        Ok(())
    }
}
