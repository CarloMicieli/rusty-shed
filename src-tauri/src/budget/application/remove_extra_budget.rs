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
    /// - `NotFound`: If the budget configuration or extra budget entry doesn't exist
    /// - `Infrastructure`: If database operation fails
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        id: ExtraBudgetId,
    ) -> Result<(), DomainError> {
        // Load the budget configuration aggregate
        let mut config = {
            let mut repo = uow.budget_repo();
            repo.get_config()
                .await?
                .ok_or_else(|| DomainError::NotFound {
                    resource: "BudgetConfiguration".to_string(),
                    identifier: "singleton".to_string(),
                })?
        };

        // Verify the extra budget entry exists before emitting the event
        {
            let mut repo = uow.budget_repo();
            let entry = repo.get_extra_budget_by_id(&id).await?;
            if entry.is_none() {
                return Err(DomainError::NotFound {
                    resource: "ExtraBudget".to_string(),
                    identifier: id.as_ref().to_string(),
                });
            }
        }

        // Emit the ExtraBudgetRemoved event on the aggregate
        config.remove_extra_budget(id);

        // Save: the repository drains pending_events and runs handle_event for each
        {
            let mut repo = uow.budget_repo();
            repo.save(config).await?;
        }

        Ok(())
    }
}
