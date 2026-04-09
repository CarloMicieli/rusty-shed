//! Use case: create a custom formation category.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::FormationCategoryView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::CreateFormationCategoryArgs;

pub struct CreateFormationCategoryUseCase;

impl CreateFormationCategoryUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork,
        args: CreateFormationCategoryArgs,
    ) -> Result<FormationCategoryView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.create_category(&id, &args.name).await
    }
}
