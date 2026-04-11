//! Use case: delete a train formation by ID.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::TrainsUowExt;

pub struct DeleteTrainFormationUseCase;

impl DeleteTrainFormationUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        id: String,
    ) -> Result<(), DomainError> {
        uow.trains_repo().delete_formation(&id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;

    #[tokio::test]
    async fn not_found_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_delete_formation().times(1).returning(|id| {
            Err(DomainError::NotFound {
                resource: "TrainFormation".into(),
                identifier: id.to_string(),
            })
        });

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = DeleteTrainFormationUseCase::execute(&mut uow, "non-existent".into()).await;
        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "must propagate NotFound"
        );
    }

    #[tokio::test]
    async fn happy_path_returns_unit() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_delete_formation()
            .times(1)
            .returning(|_| Ok(()));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = DeleteTrainFormationUseCase::execute(&mut uow, "f-1".into()).await;
        assert!(result.is_ok());
    }
}
