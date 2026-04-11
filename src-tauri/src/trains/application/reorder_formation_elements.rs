//! Use case: reorder elements within a formation.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::{TrainFormationDetail, TrainsUowExt};

pub struct ReorderFormationElementsUseCase;

impl ReorderFormationElementsUseCase {
    pub async fn execute<U: TrainsUowExt + Send>(
        uow: &mut U,
        formation_id: String,
        element_ids: Vec<String>,
    ) -> Result<TrainFormationDetail, DomainError> {
        let mut repo = uow.trains_repo();

        // Verify formation exists and element IDs belong to it
        let formation = repo.find_formation_by_id(&formation_id).await?;
        let existing_ids: std::collections::HashSet<&str> =
            formation.elements.iter().map(|e| e.id.as_str()).collect();

        for eid in &element_ids {
            if !existing_ids.contains(eid.as_str()) {
                return Err(DomainError::NotFound {
                    resource: "FormationElement".into(),
                    identifier: eid.clone(),
                });
            }
        }

        if element_ids.len() != formation.elements.len() {
            return Err(DomainError::BusinessRule(
                "element_ids must contain exactly all elements of the formation".into(),
            ));
        }

        repo.reorder_formation_elements(&formation_id, element_ids)
            .await?;
        repo.get_formation_detail(&formation_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::metadata::Metadata;
    use crate::trains::domain::formation::formation_element::FormationElement;
    use crate::trains::domain::formation::train_formation::TrainFormation;
    use crate::trains::domain::repository::MockTrainsRepository;
    use crate::trains::domain::views::TrainFormationDetail;

    fn make_formation(id: &str, element_ids: &[&str]) -> TrainFormation {
        let elements = element_ids
            .iter()
            .enumerate()
            .map(|(i, eid)| FormationElement {
                id: eid.to_string(),
                prototype_id: "pt-1".into(),
                owned_rolling_stock_id: None,
                position_order: i as i32,
                traction_override: 0,
            })
            .collect();
        TrainFormation {
            id: id.into(),
            name: "Test".into(),
            category_id: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            elements,
            pending_events: vec![],
            metadata: Metadata::default(),
        }
    }

    fn make_detail(id: &str) -> TrainFormationDetail {
        TrainFormationDetail {
            id: id.into(),
            name: "Test".into(),
            category: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            elements: vec![],
            has_traction: false,
        }
    }

    #[tokio::test]
    async fn unknown_element_id_is_rejected() {
        let formation = make_formation("f-1", &["el-1", "el-2"]);
        let mut repo = MockTrainsRepository::new();
        repo.expect_find_formation_by_id()
            .times(1)
            .return_once(|_| Ok(formation));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = ReorderFormationElementsUseCase::execute(
            &mut uow,
            "f-1".into(),
            vec!["el-1".into(), "el-UNKNOWN".into()],
        )
        .await;

        assert!(
            matches!(result, Err(DomainError::NotFound { .. })),
            "unknown element must return NotFound, got {result:?}"
        );
    }

    #[tokio::test]
    async fn count_mismatch_is_rejected() {
        let formation = make_formation("f-1", &["el-1", "el-2"]);
        let mut repo = MockTrainsRepository::new();
        repo.expect_find_formation_by_id()
            .times(1)
            .return_once(|_| Ok(formation));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result =
            ReorderFormationElementsUseCase::execute(&mut uow, "f-1".into(), vec!["el-1".into()])
                .await;

        assert!(
            matches!(result, Err(DomainError::BusinessRule(_))),
            "partial element list must return BusinessRule, got {result:?}"
        );
    }

    #[tokio::test]
    async fn valid_reorder_returns_detail() {
        let formation = make_formation("f-1", &["el-1", "el-2"]);
        let detail = make_detail("f-1");

        let mut repo = MockTrainsRepository::new();
        repo.expect_find_formation_by_id()
            .times(1)
            .return_once(|_| Ok(formation));
        repo.expect_reorder_formation_elements()
            .times(1)
            .returning(|_, _| Ok(()));
        repo.expect_get_formation_detail()
            .times(1)
            .return_once(|_| Ok(detail));

        let mut uow = MockAppUow::new().with_trains_repo(repo);
        let result = ReorderFormationElementsUseCase::execute(
            &mut uow,
            "f-1".into(),
            vec!["el-2".into(), "el-1".into()],
        )
        .await;

        assert!(result.is_ok(), "valid reorder must succeed: {result:?}");
    }
}
