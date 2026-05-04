use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::CollectionUowExt;
use crate::core::domain::domain_error::DomainError;
use chrono::NaiveDate;

/// Application use case: mark a pre-ordered item as received.
///
/// Converts the `purchase_type` from `PREORDER` to `PURCHASED` for the
/// given collection item, using `received_date` as the new `purchase_date`.
/// After the update the collection's summary and total value are recalculated
/// so the item is now counted in the active inventory.
pub struct ReceivePreorder;

impl ReceivePreorder {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: ReceivePreorderInput,
    ) -> Result<(), DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();
        repo.receive_preorder(&input.collection_item_id, input.received_date)
            .await
    }
}

/// Input for the [`ReceivePreorder`] use case.
#[derive(Debug, Clone)]
pub struct ReceivePreorderInput {
    /// The item that has been physically received.
    pub collection_item_id: CollectionItemId,
    /// The date the item arrived (used as the new purchase_date).
    pub received_date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{CollectionItemId, MockCollectionRepository};
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::identifiers::Identifier;

    fn make_input() -> ReceivePreorderInput {
        ReceivePreorderInput {
            collection_item_id: CollectionItemId::new_from_parts(&["item-1"]),
            received_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
        }
    }

    #[tokio::test]
    async fn happy_path_marks_preorder_received() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_receive_preorder()
            .times(1)
            .returning(|_, _| Ok(()));

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = ReceivePreorder::execute(&mut uow, make_input()).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn repo_not_found_error_propagates() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_receive_preorder()
            .times(1)
            .returning(|_, _| {
                Err(DomainError::NotFound {
                    resource: "PreorderPurchaseInfo".to_string(),
                    identifier: "item-1".to_string(),
                })
            });

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = ReceivePreorder::execute(&mut uow, make_input()).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }

    #[tokio::test]
    async fn repo_infrastructure_error_propagates() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_receive_preorder()
            .times(1)
            .returning(|_, _| Err(DomainError::Infrastructure("db error".into())));

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = ReceivePreorder::execute(&mut uow, make_input()).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
