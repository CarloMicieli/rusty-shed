use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::{BudgetConfiguration, BudgetMode};
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
        let existing_config = {
            let mut repo = unit_of_work.budget_repo();
            repo.get_config().await?
        };

        let config = match existing_config {
            Some(mut existing) => {
                existing.update(input.mode, input.base_amount);
                existing
            }
            None => BudgetConfiguration::new(input.mode, input.base_amount),
        };

        {
            let mut repo = unit_of_work.budget_repo();
            repo.save(config.clone()).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::application::testing::{FakeBudgetUow, sample_budget_config};
    use crate::budget::domain::repository::MockBudgetRepository;
    use crate::core::domain::Currency;

    fn valid_input() -> SetBudgetInput {
        SetBudgetInput {
            mode: BudgetMode::Monthly,
            base_amount: MonetaryAmount::new(120_000, Currency::EUR),
        }
    }

    #[tokio::test]
    async fn it_should_create_budget_when_none_exists() {
        let mut mock_get = MockBudgetRepository::new();
        mock_get.expect_get_config().once().returning(|| Ok(None));

        let mut mock_save = MockBudgetRepository::new();
        mock_save.expect_save().once().returning(|_| Ok(()));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_save);

        let result = SetBudgetUseCase::execute(&mut uow, valid_input()).await;

        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        let config = result.unwrap();
        assert_eq!(config.mode, BudgetMode::Monthly);
        assert_eq!(config.base_amount.amount, 120_000);
    }

    #[tokio::test]
    async fn it_should_update_existing_budget() {
        let existing = sample_budget_config(); // Monthly, 100_000 EUR

        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(existing.clone())));

        let mut mock_save = MockBudgetRepository::new();
        mock_save.expect_save().once().returning(|_| Ok(()));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_save);

        let input = SetBudgetInput {
            mode: BudgetMode::Yearly,
            base_amount: MonetaryAmount::new(1_200_000, Currency::EUR),
        };

        let result = SetBudgetUseCase::execute(&mut uow, input).await;

        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        let config = result.unwrap();
        assert_eq!(config.mode, BudgetMode::Yearly);
        assert_eq!(config.base_amount.amount, 1_200_000);
    }

    #[tokio::test]
    async fn it_should_propagate_get_config_error() {
        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(|| Err(DomainError::Infrastructure("db failure".to_string())));

        let mut uow = FakeBudgetUow::new().with_repo(mock_get);

        let result = SetBudgetUseCase::execute(&mut uow, valid_input()).await;

        assert!(
            matches!(result, Err(DomainError::Infrastructure(_))),
            "Expected Infrastructure error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_propagate_save_error() {
        let mut mock_get = MockBudgetRepository::new();
        mock_get.expect_get_config().once().returning(|| Ok(None));

        let mut mock_save = MockBudgetRepository::new();
        mock_save
            .expect_save()
            .once()
            .returning(|_| Err(DomainError::Infrastructure("write failed".to_string())));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_save);

        let result = SetBudgetUseCase::execute(&mut uow, valid_input()).await;

        assert!(
            matches!(result, Err(DomainError::Infrastructure(_))),
            "Expected Infrastructure error, got: {:?}",
            result
        );
    }
}
