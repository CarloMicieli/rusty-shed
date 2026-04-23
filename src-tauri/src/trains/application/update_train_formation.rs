use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{TrainFormationView, TrainsUowExt};

pub struct UpdateTrainFormationUseCase;

impl UpdateTrainFormationUseCase {
    #[allow(clippy::too_many_arguments)]
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        id: String,
        name: Option<String>,
        category_id: Option<String>,
        start_year: Option<i32>,
        end_year: Option<i32>,
        epoch: Option<String>,
        notes: Option<String>,
    ) -> Result<TrainFormationView, DomainError> {
        let mut repo = uow.trains_repo();
        let mut formation = repo.find_formation_by_id(&id).await?;

        if let Some(name) = name {
            formation.rename(name)?;
        }

        formation.update_metadata(category_id, start_year, end_year, epoch, notes)?;

        repo.save_formation(&formation).await?;
        repo.get_formation_view(&id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::metadata::Metadata;
    use crate::trains::domain::formation::train_formation::TrainFormation;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::TrainFormationView;

    fn empty_formation(id: &str) -> TrainFormation {
        TrainFormation {
            id: id.into(),
            name: "Test Formation".into(),
            category_id: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            elements: vec![],
            pending_events: vec![],
            metadata: Metadata::default(),
        }
    }

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

    #[tokio::test]
    async fn not_found_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_find_formation_by_id().times(1).returning(|id| {
            Err(DomainError::NotFound {
                resource: "TrainFormation".into(),
                identifier: id.to_string(),
            })
        });

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = UpdateTrainFormationUseCase::execute(
            &mut uow,
            "non-existent".into(),
            Some("New Name".into()),
            None,
            None,
            None,
            None,
            None,
        )
        .await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "must propagate NotFound, got {result:?}"
        );
    }

    #[tokio::test]
    async fn happy_path_returns_updated_view() {
        let formation = empty_formation("f-1");

        let mut repo = MockTrainsRepository::new();
        repo.expect_find_formation_by_id()
            .times(1)
            .return_once(|_| Ok(formation));
        repo.expect_save_formation().times(1).returning(|_| Ok(()));
        repo.expect_get_formation_view()
            .times(1)
            .returning(|_| Ok(make_view("f-1", "Renamed")));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = UpdateTrainFormationUseCase::execute(
            &mut uow,
            "f-1".into(),
            Some("Renamed".into()),
            None,
            None,
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap().name, "Renamed");
    }
}
