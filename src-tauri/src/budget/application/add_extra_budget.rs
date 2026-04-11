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
    use crate::budget::application::testing::{FakeBudgetUow, sample_budget_config};
    use crate::budget::domain::repository::MockBudgetRepository;
    use crate::core::domain::Currency;
    use crate::core::domain::calendar::{Month, Year};

    fn valid_input() -> AddExtraBudgetInput {
        AddExtraBudgetInput {
            year: Year::try_from(2026).unwrap(),
            month: Month::try_from(4).unwrap(),
            amount: MonetaryAmount::new(5_000, Currency::EUR),
            reason: Some("Birthday gift".to_string()),
        }
    }

    #[tokio::test]
    async fn it_should_add_extra_budget_successfully() {
        // Arrange – two budget_repo() calls: get_config then save
        let config = sample_budget_config();
        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(config.clone())));

        let mut mock_save = MockBudgetRepository::new();
        mock_save.expect_save().once().returning(|_| Ok(()));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_save);

        // Act
        let result = AddExtraBudgetUseCase::execute(&mut uow, valid_input()).await;

        // Assert
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        let entry = result.unwrap();
        assert_eq!(entry.year.value(), 2026);
        assert_eq!(entry.month.value(), 4);
        assert_eq!(entry.amount.amount, 5_000);
        assert_eq!(entry.reason.as_deref(), Some("Birthday gift"));
    }

    #[tokio::test]
    async fn it_should_fail_when_amount_is_zero() {
        // Arrange – validation fires before any repo call
        let input = AddExtraBudgetInput {
            amount: MonetaryAmount::new(0, Currency::EUR),
            ..valid_input()
        };
        let mut uow = FakeBudgetUow::new();

        // Act
        let result = AddExtraBudgetUseCase::execute(&mut uow, input).await;

        // Assert
        assert!(
            matches!(result, Err(DomainError::Validation(_))),
            "Expected Validation error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_fail_when_amount_is_negative() {
        let input = AddExtraBudgetInput {
            amount: MonetaryAmount::new(-100, Currency::EUR),
            ..valid_input()
        };
        let mut uow = FakeBudgetUow::new();

        let result = AddExtraBudgetUseCase::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(DomainError::Validation(_))),
            "Expected Validation error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_fail_when_reason_exceeds_500_chars() {
        let long_reason = "x".repeat(501);
        let input = AddExtraBudgetInput {
            reason: Some(long_reason),
            ..valid_input()
        };
        let mut uow = FakeBudgetUow::new();

        let result = AddExtraBudgetUseCase::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(DomainError::Validation(_))),
            "Expected Validation error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_accept_reason_at_exactly_500_chars() {
        let config = sample_budget_config();
        let boundary_reason = "x".repeat(500);

        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(config.clone())));

        let mut mock_save = MockBudgetRepository::new();
        mock_save.expect_save().once().returning(|_| Ok(()));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_save);

        let input = AddExtraBudgetInput {
            reason: Some(boundary_reason),
            ..valid_input()
        };

        let result = AddExtraBudgetUseCase::execute(&mut uow, input).await;
        assert!(result.is_ok(), "Expected Ok at boundary, got: {:?}", result);
    }

    #[tokio::test]
    async fn it_should_fail_when_no_budget_config_exists() {
        // Arrange – only get_config is called; returns None → NotFound
        let mut mock_get = MockBudgetRepository::new();
        mock_get.expect_get_config().once().returning(|| Ok(None));

        let mut uow = FakeBudgetUow::new().with_repo(mock_get);

        // Act
        let result = AddExtraBudgetUseCase::execute(&mut uow, valid_input()).await;

        // Assert
        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "Expected NotFound error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_propagate_get_config_error() {
        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(|| Err(DomainError::Infrastructure("db failure".to_string())));

        let mut uow = FakeBudgetUow::new().with_repo(mock_get);

        let result = AddExtraBudgetUseCase::execute(&mut uow, valid_input()).await;

        assert!(
            matches!(result, Err(DomainError::Infrastructure(_))),
            "Expected Infrastructure error, got: {:?}",
            result
        );
    }
}
