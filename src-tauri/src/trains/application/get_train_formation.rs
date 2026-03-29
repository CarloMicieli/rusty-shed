//! Use case: get a single train formation with full element detail.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::TrainFormationDetail;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;

pub struct GetTrainFormationUseCase;

impl GetTrainFormationUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        id: String,
    ) -> Result<TrainFormationDetail, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.get_detail(&id).await
    }
}
