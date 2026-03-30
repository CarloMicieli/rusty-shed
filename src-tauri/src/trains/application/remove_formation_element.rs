//! Use case: remove an element from a formation.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;

pub struct RemoveFormationElementUseCase;

impl RemoveFormationElementUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        element_id: String,
    ) -> Result<(), DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.remove_element_and_shift(&element_id).await
    }
}
