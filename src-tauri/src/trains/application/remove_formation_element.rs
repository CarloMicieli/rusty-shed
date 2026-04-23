use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::TrainsUowExt;

pub struct RemoveFormationElementUseCase;

impl RemoveFormationElementUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        element_id: String,
    ) -> Result<(), DomainError> {
        uow.trains_repo()
            .remove_formation_element(&element_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;

    #[tokio::test]
    async fn happy_path_removes_element() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_remove_formation_element()
            .times(1)
            .returning(|_| Ok(()));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = RemoveFormationElementUseCase::execute(&mut uow, "el-1".into()).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_remove_formation_element()
            .times(1)
            .returning(|_| {
                Err(DomainError::NotFound {
                    resource: "FormationElement".into(),
                    identifier: "el-404".into(),
                })
            });

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = RemoveFormationElementUseCase::execute(&mut uow, "el-404".into()).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }
}
