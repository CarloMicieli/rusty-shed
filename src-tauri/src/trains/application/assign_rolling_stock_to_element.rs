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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::{FormationElementView, PrototypeView};

    fn make_element_view(id: &str, owned_rolling_stock_id: Option<String>) -> FormationElementView {
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
            owned_rolling_stock_id,
            snapshot_series_code: None,
            snapshot_company_name: None,
            stock_not_found: false,
            owned_count_for_prototype: 1,
            traction_override: 0,
            is_traction_slot: true,
        }
    }

    #[tokio::test]
    async fn happy_path_assigns_stock_and_returns_view() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_assign_rolling_stock_to_element()
            .times(1)
            .returning(|_, _| Ok(make_element_view("el-1", Some("ors-1".into()))));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = AssignRollingStockToElementUseCase::execute(
            &mut uow,
            "el-1".into(),
            Some("ors-1".into()),
        )
        .await;

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(
            result.unwrap().owned_rolling_stock_id.as_deref(),
            Some("ors-1")
        );
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_assign_rolling_stock_to_element()
            .times(1)
            .returning(|_, _| Err(DomainError::Infrastructure("db down".into())));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result =
            AssignRollingStockToElementUseCase::execute(&mut uow, "el-1".into(), None).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
