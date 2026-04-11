//! Use case: get a single train formation with full element detail.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{TrainFormationDetail, TrainsUowExt};

pub struct GetTrainFormationUseCase;

impl GetTrainFormationUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        id: String,
    ) -> Result<TrainFormationDetail, DomainError> {
        uow.trains_repo().get_formation_detail(&id).await
    }
}
