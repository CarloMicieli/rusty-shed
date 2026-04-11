//! Use case: remove an element from a formation.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::TrainsUowExt;

pub struct RemoveFormationElementUseCase;

impl RemoveFormationElementUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        element_id: String,
    ) -> Result<(), DomainError> {
        uow.trains_repo()
            .remove_formation_element(&element_id)
            .await
    }
}
