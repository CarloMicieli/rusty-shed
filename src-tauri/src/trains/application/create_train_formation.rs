use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::formation::train_formation::TrainFormation;
use crate::trains::domain::{TrainFormationView, TrainsUowExt};

pub struct CreateTrainFormationUseCase;

impl CreateTrainFormationUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        name: String,
        category_id: Option<String>,
        start_year: Option<i32>,
        end_year: Option<i32>,
        epoch: Option<String>,
        notes: Option<String>,
    ) -> Result<TrainFormationView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut formation = TrainFormation::create(id.clone(), name)?;

        formation.update_metadata(category_id, start_year, end_year, epoch, notes)?;

        let mut repo = uow.trains_repo();
        repo.save_formation(&formation).await?;
        repo.get_formation_view(&id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::TrainFormationView;

    fn make_view(id: &str, name: &str) -> TrainFormationView {
        TrainFormationView {
            id: id.into(),
            name: name.into(),
            category: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            element_count: 0,
            has_traction: false,
        }
    }

    #[test]
    fn empty_name_is_rejected_by_domain() {
        let result = crate::trains::domain::formation::train_formation::TrainFormation::create(
            "f-1".into(),
            "".into(),
        );
        assert!(result.is_err(), "empty name must fail domain validation");
    }

    #[tokio::test]
    async fn happy_path_returns_view() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_save_formation().times(1).returning(|_| Ok(()));
        repo.expect_get_formation_view()
            .times(1)
            .returning(|_| Ok(make_view("f-1", "EuroCity Gottardo")));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = CreateTrainFormationUseCase::execute(
            &mut uow,
            "EuroCity Gottardo".into(),
            None,
            Some(1975),
            Some(1985),
            Some("IV".into()),
            None,
        )
        .await;

        assert!(result.is_ok(), "valid args must succeed: {result:?}");
        assert_eq!(result.unwrap().name, "EuroCity Gottardo");
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_save_formation().times(1).returning(|_| {
            Err(crate::core::domain::domain_error::DomainError::Infrastructure("db down".into()))
        });

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = CreateTrainFormationUseCase::execute(
            &mut uow,
            "EuroCity".into(),
            None,
            None,
            None,
            None,
            None,
        )
        .await;
        assert!(result.is_err());
    }
}
