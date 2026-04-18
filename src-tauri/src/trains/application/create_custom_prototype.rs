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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::PrototypeView;

    async fn execute_with_defaults<U: TrainsUowExt + Send>(
        uow: &mut U,
    ) -> Result<PrototypeView, DomainError> {
        CreateCustomPrototypeUseCase::execute(
            uow,
            "co-1".into(),
            "Re 460".into(),
            Some("Lok 2000".into()),
            true,
            false,
            Some("custom notes".into()),
            "LOCOMOTIVE".into(),
            Some("ELECTRIC_LOCOMOTIVE".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
    }

    #[tokio::test]
    async fn happy_path_creates_custom_prototype() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_create_prototype().times(1).returning(|input| {
            Ok(PrototypeView {
                id: input.id,
                railway_company_id: input.railway_company_id,
                company_name: "SBB".into(),
                series_code: input.series_code,
                friendly_name: input.friendly_name,
                is_motorized: input.is_motorized,
                default_is_dummy: input.default_is_dummy,
                is_custom: true,
                specification_type: input.specification_type,
                locomotive_type: input.locomotive_type,
                locomotive_series: input.locomotive_series,
                service_level: input.service_level,
                passenger_car_type: input.passenger_car_type,
                freight_car_type: input.freight_car_type,
                railcar_type: input.railcar_type,
                electric_multiple_unit_type: input.electric_multiple_unit_type,
                elements_count: input.elements_count,
                is_permanently_coupled: input.is_permanently_coupled,
            })
        });

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = execute_with_defaults(&mut uow).await;

        assert!(result.is_ok(), "{result:?}");
        let prototype = result.unwrap();
        assert!(!prototype.id.is_empty());
        assert!(prototype.is_custom);
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_create_prototype()
            .times(1)
            .returning(|_| Err(DomainError::Infrastructure("db down".into())));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = execute_with_defaults(&mut uow).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
