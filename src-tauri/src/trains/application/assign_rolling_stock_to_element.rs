//! Use case: assign or unassign an owned rolling stock to a formation element.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{FormationElementView, TrainsUowExt};

pub struct AssignRollingStockToElementUseCase;

impl AssignRollingStockToElementUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        element_id: String,
        owned_rolling_stock_id: Option<String>,
    ) -> Result<FormationElementView, DomainError> {
        uow.trains_repo()
            .assign_rolling_stock_to_element(&element_id, owned_rolling_stock_id)
            .await
    }
}
