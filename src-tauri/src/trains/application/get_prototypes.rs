//! Use case: search prototypes, optionally grouped by company.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{PrototypeGroupView, TrainsUowExt};

pub struct GetPrototypesUseCase;

impl GetPrototypesUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        query: Option<String>,
    ) -> Result<Vec<PrototypeGroupView>, DomainError> {
        uow.trains_repo().find_prototypes_by_query(query).await
    }
}
