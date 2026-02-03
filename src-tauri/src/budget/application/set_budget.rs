// Set Budget Use Case
// Feature: 001-budget-tracking

use crate::budget::domain::{BudgetConfiguration, BudgetMode, BudgetRepository};
use crate::budget::infrastructure::BudgetUowExt;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::monetary_amount::MonetaryAmount;

pub struct SetBudgetUseCase;

impl SetBudgetUseCase {
    /// Creates or updates the budget configuration.
    ///
    /// # Arguments
    /// - `unit_of_work`: Unit of work providing access to budget repository.
    /// - `input`: Input data for budget configuration.
    ///
    /// # Returns
    /// - `Ok(BudgetConfiguration)` if successful.
    /// - `Err(DomainError)` if an error occurred.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SetBudgetInput,
    ) -> Result<BudgetConfiguration, DomainError>
    where
        U: BudgetUowExt + Send,
    {
        // Get existing config if any
        let existing_config = {
            let mut repo = unit_of_work.budget_repo();
            repo.get_config().await.map_err(DomainError::BusinessRule)?
        };

        let config = match existing_config {
            Some(mut existing) => {
                // Update existing configuration
                existing.update(input.mode, input.base_amount);
                existing
            }
            None => {
                // Create new configuration
                BudgetConfiguration::new(input.mode, input.base_amount)
            }
        };

        // Save configuration
        {
            let mut repo = unit_of_work.budget_repo();
            repo.save_config(&config)
                .await
                .map_err(DomainError::BusinessRule)?;
        }

        Ok(config)
    }
}

/// Input for setting budget configuration.
#[derive(Debug, Clone)]
pub struct SetBudgetInput {
    pub mode: BudgetMode,
    pub base_amount: MonetaryAmount,
}
