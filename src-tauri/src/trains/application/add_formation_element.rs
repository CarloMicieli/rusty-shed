//! Use case: add a prototype element to a formation.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::formation::formation_element::FormationElement;
use crate::trains::domain::{FormationElementView, TrainsUowExt};
use uuid::Uuid;

pub struct AddFormationElementUseCase;

impl AddFormationElementUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        formation_id: String,
        prototype_id: String,
        owned_rolling_stock_id: Option<String>,
    ) -> Result<FormationElementView, DomainError> {
        let element_id = Uuid::new_v4().to_string();

        let mut repo = uow.trains_repo();

        // Verify formation exists and determine element position
        let formation = repo.find_formation_by_id(&formation_id).await?;

        let element = FormationElement {
            id: element_id.clone(),
            prototype_id,
            owned_rolling_stock_id,
            position_order: formation.elements.len() as i32,
            traction_override: 0,
        };

        repo.add_formation_element(&formation_id, element).await?;
        repo.get_formation_element_view(&element_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::metadata::Metadata;
    use crate::trains::domain::formation::train_formation::TrainFormation;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::{FormationElementView, PrototypeView};

    fn empty_formation(id: &str) -> TrainFormation {
        TrainFormation {
            id: id.into(),
            name: "Test".into(),
            category_id: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            elements: vec![],
            pending_events: vec![],
            metadata: Metadata::default(),
        }
    }

    fn make_element_view(id: &str) -> FormationElementView {
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
            traction_override: 0,
            is_traction_slot: true,
        }
    }

    #[tokio::test]
    async fn happy_path_returns_element_view() {
        let formation = empty_formation("f-1");
        let element_view = make_element_view("el-new");

        let mut repo = MockTrainsRepository::new();
        repo.expect_find_formation_by_id()
            .times(1)
            .return_once(|_| Ok(formation));
        repo.expect_add_formation_element()
            .times(1)
            .returning(|_, _| Ok(()));
        repo.expect_get_formation_element_view()
            .times(1)
            .return_once(|_| Ok(element_view));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result =
            AddFormationElementUseCase::execute(&mut uow, "f-1".into(), "pt-1".into(), None).await;
        assert!(result.is_ok(), "{result:?}");
    }
}
