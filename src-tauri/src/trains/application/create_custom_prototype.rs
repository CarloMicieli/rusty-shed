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
        uow: &mut SqliteUnitOfWork,
        args: CreateCustomPrototypeArgs,
    ) -> Result<PrototypeView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);
        repo.save_prototype(SavePrototypeParams {
            id: &id,
            railway_company_id: &args.railway_company_id,
            series_code: &args.series_code,
            friendly_name: args.friendly_name.as_deref(),
            is_motorized: args.is_motorized,
            default_is_dummy: args.default_is_dummy,
            notes: args.notes.as_deref(),
            specification_type: &args.specification_type,
            locomotive_type: args.locomotive_type.as_deref(),
            locomotive_series: args.locomotive_series.as_deref(),
            service_level: args.service_level.as_deref(),
            passenger_car_type: args.passenger_car_type.as_deref(),
            freight_car_type: args.freight_car_type.as_deref(),
            railcar_type: args.railcar_type.as_deref(),
            electric_multiple_unit_type: args.electric_multiple_unit_type.as_deref(),
            elements_count: args.elements_count,
            is_permanently_coupled: args.is_permanently_coupled,
        })
        .await
    }
}
