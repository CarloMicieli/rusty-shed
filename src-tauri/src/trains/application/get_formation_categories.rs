//! Use case: list all formation categories.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::FormationCategoryView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;

pub struct GetFormationCategoriesUseCase;

impl GetFormationCategoriesUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork,
    ) -> Result<Vec<FormationCategoryView>, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.list_categories().await
    }
}
