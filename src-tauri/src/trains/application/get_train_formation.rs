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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::{FormationElementView, PrototypeView, TrainFormationDetail};

    fn make_detail() -> TrainFormationDetail {
        TrainFormationDetail {
            id: "f-1".into(),
            name: "EuroCity".into(),
            category: None,
            start_year: None,
            end_year: None,
            epoch: Some("V".into()),
            notes: None,
            elements: vec![FormationElementView {
                id: "el-1".into(),
                position_order: 0,
                prototype: PrototypeView {
                    id: "pt-1".into(),
                    railway_company_id: "co-1".into(),
                    company_name: "SBB".into(),
                    series_code: "Re 460".into(),
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
                traction_override: 0,
                is_traction_slot: true,
            }],
            has_traction: true,
        }
    }

    #[tokio::test]
    async fn returns_formation_detail() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_get_formation_detail()
            .times(1)
            .return_once(|_| Ok(make_detail()));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetTrainFormationUseCase::execute(&mut uow, "f-1".into()).await;

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap().elements.len(), 1);
    }

    #[tokio::test]
    async fn not_found_propagates() {
        let mut repo = MockTrainsRepository::new();
        repo.expect_get_formation_detail()
            .times(1)
            .returning(|id| Err(DomainError::NotFound {
                resource: "TrainFormation".into(),
                identifier: id.into(),
            }));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = GetTrainFormationUseCase::execute(&mut uow, "f-missing".into()).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }
}
