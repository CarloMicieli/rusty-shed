//! Use case: search prototypes, optionally grouped by company.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::PrototypeGroupView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;

pub struct GetPrototypesUseCase;

impl GetPrototypesUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        query: Option<String>,
    ) -> Result<Vec<PrototypeGroupView>, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.search_prototypes(query.as_deref()).await
    }
}
