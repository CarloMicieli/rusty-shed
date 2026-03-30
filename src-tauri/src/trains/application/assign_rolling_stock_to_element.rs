//! Use case: assign or unassign an owned rolling stock to a formation element.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::FormationElementView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::AssignRollingStockToElementArgs;

pub struct AssignRollingStockToElementUseCase;

impl AssignRollingStockToElementUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        element_id: String,
        args: AssignRollingStockToElementArgs,
    ) -> Result<FormationElementView, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.assign_rolling_stock(&element_id, args.owned_rolling_stock_id.as_deref())
            .await
    }
}
