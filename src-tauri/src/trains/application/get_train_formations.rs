//! Use case: list all train formations as summaries.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{TrainFormationSummary, TrainsUowExt};

pub struct GetTrainFormationsUseCase;

impl GetTrainFormationsUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
    ) -> Result<Vec<TrainFormationSummary>, DomainError> {
        uow.trains_repo().get_all_formation_summaries().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::TrainFormationSummary;

    fn make_summary(id: &str, name: &str) -> TrainFormationSummary {
        TrainFormationSummary {
            id: id.into(),
            name: name.into(),
            category: None,
            epoch: None,
            element_count: 0,
            has_traction: false,
            owned_count: 0,
            planned_count: 0,
        }
    }

    #[tokio::test]
    async fn returns_empty_list() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_get_all_formation_summaries()
            .times(1)
            .returning(|| Ok(vec![]));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetTrainFormationsUseCase::execute(&mut uow).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn returns_all_summaries() {
        let summary = make_summary("f-1", "Test Formation");
        let mut repo = MockTrainsRepository::new();
        repo.expect_get_all_formation_summaries()
            .times(1)
            .return_once(|| Ok(vec![summary]));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetTrainFormationsUseCase::execute(&mut uow).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
