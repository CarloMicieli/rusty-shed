//! Use case: override traction status of a formation element.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::FormationElementView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::SetTractionOverrideArgs;

pub struct SetTractionOverrideUseCase;

impl SetTractionOverrideUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        element_id: String,
        args: SetTractionOverrideArgs,
    ) -> Result<FormationElementView, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.set_traction_override(&element_id, args.traction_override)
            .await
    }
}
