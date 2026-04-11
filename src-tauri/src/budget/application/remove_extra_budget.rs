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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::application::testing::{
        FakeBudgetUow, sample_extra_budget_entry, sample_extra_budget_id,
    };
    use crate::budget::domain::repository::MockBudgetRepository;

    #[tokio::test]
    async fn it_should_remove_extra_budget_successfully() {
        // Arrange – one budget_repo() call; both methods invoked on the same repo
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

        // Act
        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        // Assert
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
    }

    #[tokio::test]
    async fn it_should_fail_when_entry_not_found() {
        // Arrange – get_extra_budget_by_id returns None → NotFound
        let id = sample_extra_budget_id();
        let id_display = id.as_ref().to_string();

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_extra_budget_by_id()
            .once()
            .returning(|_| Ok(None));
        // remove_extra_budget must NOT be called

        let mut uow = FakeBudgetUow::new().with_repo(mock);

        // Act
        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        // Assert
        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "Expected NotFound for id {id_display}, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_propagate_remove_error() {
        // Arrange – entry exists but DELETE fails
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

        // Act
        let result = RemoveExtraBudgetUseCase::execute(&mut uow, id).await;

        // Assert
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
