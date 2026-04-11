//! Use case: override traction status of a formation element.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{FormationElementView, TrainsUowExt};

pub struct SetTractionOverrideUseCase;

impl SetTractionOverrideUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        element_id: String,
        traction_override: i32,
    ) -> Result<FormationElementView, DomainError> {
        uow.trains_repo()
            .set_element_traction_override(&element_id, traction_override)
            .await
    }
}
