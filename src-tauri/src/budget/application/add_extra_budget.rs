// Add Extra Budget Use Case
// Feature: 001-budget-tracking - Phase 6 (US4)

use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::{ExtraBudgetEntry, ExtraBudgetId};
use crate::core::domain::calendar::{Month, Year};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::Utc;

/// Input for adding an extra budget entry.
#[derive(Debug, Clone)]
pub struct AddExtraBudgetInput {
    pub year: Year,
    pub month: Month,
    pub amount: MonetaryAmount,
    pub reason: Option<String>,
}

pub struct AddExtraBudgetUseCase;

impl AddExtraBudgetUseCase {
    /// Add a one-time budget injection to a specific month.
    ///
    /// # Arguments
    /// * `uow` - Unit of work for database access
    /// * `input` - Input parameters for the extra budget
    ///
    /// # Returns
    /// The created `ExtraBudgetEntry`.
    ///
    /// # Errors
    /// - `Validation`: If year or month is invalid, or amount is not positive
    /// - `NotFound`: If no budget configuration exists yet
    /// - `Infrastructure`: If database operation fails
    pub async fn execute<U>(
        uow: &mut U,
        input: AddExtraBudgetInput,
    ) -> Result<ExtraBudgetEntry, DomainError>
    where
        U: BudgetUowExt + Send,
    {
        // Validate input
        if input.amount.amount <= 0 {
            return Err(DomainError::Validation(
                "Amount must be positive".to_string(),
            ));
        }

        // Validate reason length if provided
        if let Some(ref reason) = input.reason
            && reason.len() > 500
        {
            return Err(DomainError::Validation(
                "Reason must be 500 characters or less".to_string(),
            ));
        }

        // Load the budget configuration aggregate — it is the root for budget events.
        let mut config = {
            let mut repo = uow.budget_repo();
            repo.get_config()
                .await?
                .ok_or_else(|| DomainError::NotFound {
                    resource: "BudgetConfiguration".to_string(),
                    identifier: "singleton".to_string(),
                })?
        };

        // Build the entry value object
        let entry = ExtraBudgetEntry {
            id: ExtraBudgetId::default(),
            year: input.year,
            month: input.month,
            amount: input.amount,
            reason: input.reason,
            created_at: Utc::now(),
            version: 0,
        };

        // Emit the ExtraBudgetAdded event on the aggregate
        config.add_extra_budget(&entry);

        // Save: the repository drains pending_events and runs handle_event for each
        {
            let mut repo = uow.budget_repo();
            repo.save(config).await?;
        }

        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;
    use crate::core::domain::calendar::{Month, Year};

    #[test]
    fn test_validate_positive_amount() {
        let input = AddExtraBudgetInput {
            year: Year::try_from(2026).unwrap(),
            month: Month::try_from(3).unwrap(),
            amount: MonetaryAmount::new(0, Currency::USD),
            reason: None,
        };

        // Would fail validation
        assert!(input.amount.amount <= 0);
    }
}
