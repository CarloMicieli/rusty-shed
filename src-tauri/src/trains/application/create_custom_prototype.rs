//! Use case: create a new custom prototype.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::trains::infrastructure::mappers::PrototypeView;
use crate::trains::infrastructure::train_formation_repo::{
    SavePrototypeParams, SqlxTrainFormationRepository,
};
use crate::trains::interface::command_args::CreateCustomPrototypeArgs;

pub struct CreateCustomPrototypeUseCase;

impl CreateCustomPrototypeUseCase {
    pub async fn execute(
        uow: &mut SqliteUnitOfWork<'_>,
        args: CreateCustomPrototypeArgs,
    ) -> Result<PrototypeView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.save_prototype(SavePrototypeParams {
            id: &id,
            railway_company_id: &args.railway_company_id,
            series_code: &args.series_code,
            car_type: &args.car_type,
            service_level: args.service_level.as_deref(),
            category: &args.category,
            is_motorized: args.is_motorized,
            default_is_dummy: args.default_is_dummy,
            notes: args.notes.as_deref(),
        })
        .await
    }
}
