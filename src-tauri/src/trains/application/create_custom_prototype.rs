//! Use case: create a new custom prototype.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::repository::CreatePrototypeInput;
use crate::trains::domain::{PrototypeView, TrainsUowExt};

pub struct CreateCustomPrototypeUseCase;

impl CreateCustomPrototypeUseCase {
    #[allow(clippy::too_many_arguments)]
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        railway_company_id: String,
        series_code: String,
        friendly_name: Option<String>,
        is_motorized: bool,
        default_is_dummy: bool,
        notes: Option<String>,
        specification_type: String,
        locomotive_type: Option<String>,
        locomotive_series: Option<String>,
        service_level: Option<String>,
        passenger_car_type: Option<String>,
        freight_car_type: Option<String>,
        railcar_type: Option<String>,
        electric_multiple_unit_type: Option<String>,
        elements_count: Option<i64>,
        is_permanently_coupled: Option<bool>,
    ) -> Result<PrototypeView, DomainError> {
        let id = uuid::Uuid::new_v4().to_string();
        uow.trains_repo()
            .create_prototype(CreatePrototypeInput {
                id,
                railway_company_id,
                series_code,
                friendly_name,
                is_motorized,
                default_is_dummy,
                notes,
                specification_type,
                locomotive_type,
                locomotive_series,
                service_level,
                passenger_car_type,
                freight_car_type,
                railcar_type,
                electric_multiple_unit_type,
                elements_count,
                is_permanently_coupled,
            })
            .await
    }
}
