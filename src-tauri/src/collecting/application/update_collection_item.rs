use crate::collecting::domain::{CollectionUowExt, UpdateCollectionItemInput};
use crate::core::domain::domain_error::DomainError;

/// Use case to update mutable fields on a collection item.
pub struct UpdateCollectionItem;

impl UpdateCollectionItem {
    /// Execute a single-field update for an existing collection item.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateCollectionItemInput,
    ) -> Result<(), DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();
        repo.update_item(&input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{
        CollectionItemId, CollectionItemUpdate, MockCollectionRepository,
    };
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::identifiers::Identifier;

    fn make_input() -> UpdateCollectionItemInput {
        UpdateCollectionItemInput {
            collection_item_id: CollectionItemId::new_from_parts(&["item-1"]),
            update: CollectionItemUpdate::Notes(Some("updated notes".into())),
        }
    }

    #[tokio::test]
    async fn happy_path_updates_item() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_update_item()
            .times(1)
            .returning(|_| Ok(()));

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = UpdateCollectionItem::execute(&mut uow, make_input()).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_update_item()
            .times(1)
            .returning(|_| Err(DomainError::Infrastructure("write failed".into())));

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = UpdateCollectionItem::execute(&mut uow, make_input()).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
