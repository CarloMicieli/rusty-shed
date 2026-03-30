//! Use case: reorder elements within a formation.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::TrainFormationDetail;
use crate::trains::infrastructure::train_formation_repo::SqlxTrainFormationRepository;
use crate::trains::interface::command_args::ReorderFormationElementsArgs;

pub struct ReorderFormationElementsUseCase;

impl ReorderFormationElementsUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        formation_id: String,
        args: ReorderFormationElementsArgs,
    ) -> Result<TrainFormationDetail, DomainError> {
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        // Verify formation exists and element IDs belong to it
        let formation = repo.find_by_id_raw(&formation_id).await?;
        let existing_ids: std::collections::HashSet<&str> =
            formation.elements.iter().map(|e| e.id.as_str()).collect();

        for eid in &args.element_ids {
            if !existing_ids.contains(eid.as_str()) {
                return Err(DomainError::NotFound {
                    resource: "FormationElement".into(),
                    identifier: eid.clone(),
                });
            }
        }

        if args.element_ids.len() != formation.elements.len() {
            return Err(DomainError::BusinessRule(
                "element_ids must contain exactly all elements of the formation".into(),
            ));
        }

        repo.bulk_reorder(&formation_id, &args.element_ids).await?;
        repo.get_detail(&formation_id).await
    }
}
