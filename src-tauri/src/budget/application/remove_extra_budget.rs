use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::ExtraBudgetId;
use crate::core::domain::domain_error::DomainError;

pub struct RemoveExtraBudgetUseCase;

impl RemoveExtraBudgetUseCase {
    /// Remove an extra budget entry.
    ///
    /// # Arguments
    /// * `unit_of_work` - Unit of work for database access
    /// * `id` - ID of the extra budget entry to remove
    ///
    /// # Errors
    /// - `NotFound`: If the budget configuration or extra budget entry doesn't exist
    /// - `Infrastructure`: If database operation fails
    pub async fn execute<U>(unit_of_work: &mut U, id: ExtraBudgetId) -> Result<(), DomainError>
    where
        U: BudgetUowExt + Send,
    {
        let mut repo = unit_of_work.budget_repo();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::application::testing::{
        FakeBudgetUow, sample_extra_budget_entry, sample_extra_budget_id,
    };
    use crate::budget::domain::repository::MockBudgetRepository;

    #[tokio::test]
    async fn it_should_remove_extra_budget_successfully() {
        let entry = sample_extra_budget_entry();
        let id = sample_extra_budget_id();

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_extra_budget_by_id()
            .once()
            .returning(move |_| Ok(Some(entry.clone())));
        mock.expect_remove_extra_budget()
            .once()
            .returning(|_| Ok(()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);

        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
    }

    #[tokio::test]
    async fn it_should_fail_when_entry_not_found() {
        let id = sample_extra_budget_id();
        let id_display = id.as_ref().to_string();

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_extra_budget_by_id()
            .once()
            .returning(|_| Ok(None));

        let mut uow = FakeBudgetUow::new().with_repo(mock);

        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "Expected NotFound for id {id_display}, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_propagate_remove_error() {
        let entry = sample_extra_budget_entry();
        let id = sample_extra_budget_id();

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_extra_budget_by_id()
            .once()
            .returning(move |_| Ok(Some(entry.clone())));
        mock.expect_remove_extra_budget()
            .once()
            .returning(|_| Err("database error".to_string()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);

        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        assert!(
            matches!(result, Err(DomainError::Infrastructure(_))),
            "Expected Infrastructure error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_propagate_lookup_error() {
        let id = sample_extra_budget_id();

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_extra_budget_by_id()
            .once()
            .returning(|_| Err("lookup failed".to_string()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);

        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        assert!(
            matches!(result, Err(DomainError::Infrastructure(_))),
            "Expected Infrastructure error, got: {:?}",
            result
        );
    }
}
