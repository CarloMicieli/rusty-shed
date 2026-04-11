//! Use case: create a custom formation category.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{FormationCategoryView, TrainsUowExt};

pub struct CreateFormationCategoryUseCase;

impl CreateFormationCategoryUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        name: String,
    ) -> Result<FormationCategoryView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        uow.trains_repo()
            .create_formation_category(&id, &name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::FormationCategoryView;

    fn make_category(id: &str, name: &str) -> FormationCategoryView {
        FormationCategoryView {
            id: id.into(),
            name: name.into(),
            is_custom: true,
        }
    }

    #[tokio::test]
    async fn happy_path_returns_category_view() {
        let cat = make_category("cat-new", "Freight Consists");
        let mut repo = MockTrainsRepository::new();
        repo.expect_create_formation_category()
            .times(1)
            .return_once(|_, _| Ok(cat));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result =
            CreateFormationCategoryUseCase::execute(&mut uow, "Freight Consists".into()).await;
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap().name, "Freight Consists");
    }
}
