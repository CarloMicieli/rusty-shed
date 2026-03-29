//! Use case: list all train formations as summaries.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::TrainFormationSummary;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;

pub struct GetTrainFormationsUseCase;

impl GetTrainFormationsUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<TrainFormationSummary>, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.list_summaries().await
    }
}
