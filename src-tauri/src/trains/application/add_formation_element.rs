//! Use case: add a prototype element to a formation.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::FormationElementView;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::AddFormationElementArgs;
use uuid::Uuid;

pub struct AddFormationElementUseCase;

impl AddFormationElementUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        formation_id: String,
        args: AddFormationElementArgs,
    ) -> Result<FormationElementView, DomainError> {
        let element_id = Uuid::new_v4().to_string();
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        // Verify formation exists
        let mut formation = repo.find_by_id_raw(&formation_id).await?;

        use crate::trains::domain::formation::formation_element::FormationElement;
        let element = FormationElement {
            id: element_id.clone(),
            prototype_id: args.prototype_id,
            owned_rolling_stock_id: args.owned_rolling_stock_id,
            position_order: formation.elements.len() as i32,
            traction_override: 0,
        };

        formation.add_element(element.clone());

        repo.add_element(&formation_id, &element).await?;
        repo.get_element_view(&element_id).await
    }
}
