//! Use case: delete a train formation by ID.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;

pub struct DeleteTrainFormationUseCase;

impl DeleteTrainFormationUseCase {
    pub async fn execute(uow: &mut SqliteUnitOfWork, id: String) -> Result<(), DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.delete(&id).await
    }
}
