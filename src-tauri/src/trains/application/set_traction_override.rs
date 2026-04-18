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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::{FormationElementView, PrototypeView};

    fn make_element_view(id: &str, traction_override: i32) -> FormationElementView {
        FormationElementView {
            id: id.into(),
            position_order: 0,
            prototype: PrototypeView {
                id: "pt-1".into(),
                railway_company_id: "co-1".into(),
                company_name: "Test Co".into(),
                series_code: "Re 4/4".into(),
                friendly_name: None,
                is_motorized: true,
                default_is_dummy: false,
                is_custom: false,
                specification_type: "LOCOMOTIVE".into(),
                locomotive_type: Some("ELECTRIC_LOCOMOTIVE".into()),
                locomotive_series: None,
                service_level: None,
                passenger_car_type: None,
                freight_car_type: None,
                railcar_type: None,
                electric_multiple_unit_type: None,
                elements_count: None,
                is_permanently_coupled: None,
            },
            owned_rolling_stock_id: None,
            snapshot_series_code: None,
            snapshot_company_name: None,
            stock_not_found: false,
            owned_count_for_prototype: 0,
            traction_override,
            is_traction_slot: traction_override >= 0,
        }
    }

    #[tokio::test]
    async fn happy_path_sets_override_and_returns_view() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_set_element_traction_override()
            .times(1)
            .returning(|_, value| Ok(make_element_view("el-1", value)));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = SetTractionOverrideUseCase::execute(&mut uow, "el-1".into(), -1).await;

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap().traction_override, -1);
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_set_element_traction_override()
            .times(1)
            .returning(|_, _| Err(DomainError::Infrastructure("write failed".into())));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = SetTractionOverrideUseCase::execute(&mut uow, "el-1".into(), 1).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
