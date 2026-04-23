use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{FormationCategoryView, TrainsUowExt};

pub struct GetFormationCategoriesUseCase;

impl GetFormationCategoriesUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
    ) -> Result<Vec<FormationCategoryView>, DomainError> {
        uow.trains_repo().get_all_categories().await
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
    async fn returns_categories() {
        let cat = make_category("cat-1", "EMU Trains");
        let mut repo = MockTrainsRepository::new();
        repo.expect_get_all_categories()
            .times(1)
            .return_once(|| Ok(vec![cat]));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetFormationCategoriesUseCase::execute(&mut uow).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
