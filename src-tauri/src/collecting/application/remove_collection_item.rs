use crate::catalog::domain::railway_model::Category;
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::{CollectionId, CollectionUowExt};
use crate::core::domain::domain_error::DomainError;

/// Use case to remove an item from the collection.
pub struct RemoveCollectionItem;

impl RemoveCollectionItem {
    /// Execute the remove collection item use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `remove_cmd`: command carrying the details of the item to remove.
    ///
    /// # Returns
    /// * the `CollectionItemId` of the removed item on success
    ///
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `CollectionUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        remove_cmd: RemoveCollectionItemInput,
    ) -> Result<CollectionItemId, DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();

        let collection_id = CollectionId::default();
        let collection_opt = repo.find_by_id(&collection_id).await?;

        let mut collection = collection_opt.ok_or(DomainError::NotFound {
            resource: "Collection".to_string(),
            identifier: collection_id.to_string(),
        })?;

        // capture id before passing command (remove_cmd is consumed)
        let removed_id = remove_cmd.collection_item_id.clone();

        collection.remove_item(remove_cmd);

        repo.save(&mut collection).await?;

        // Return the id of the removed collection item
        Ok(removed_id)
    }
}

/// Input structure for removing an item from the collection.
#[derive(Debug, Clone)]
pub struct RemoveCollectionItemInput {
    /// The ID of the collection item to remove.
    pub collection_item_id: CollectionItemId,
    /// The category of the item.
    pub category: Category,
    /// The date the item was removed from the collection.
    pub removed_date: chrono::NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::{Category, MockRailwayModelRepository};
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{
        CollectionId, CollectionItemId, CollectionSummary, MockCollectionRepository,
    };

    #[tokio::test]
    async fn it_should_return_collection_view() {
        let mut mock = MockCollectionRepository::new();
        mock.expect_find_by_id().times(1).returning(move |_| {
            let collection = crate::collecting::domain::Collection {
                id: CollectionId::default(),
                name: "My Collection".to_string(),
                summary: CollectionSummary::default(),
                total_value: None,
                items: vec![],
                pending_events: Vec::new(),
                metadata: Default::default(),
            };
            Ok(Some(collection.clone()))
        });

        mock.expect_save().times(1).returning(move |_| Ok(()));

        let mut unit_of_work = FakeUow::new(mock, MockRailwayModelRepository::new());

        let date = chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let remove_cmd = RemoveCollectionItemInput {
            collection_item_id: CollectionItemId::try_from(
                "trn:collection-item:89df34a4-ffee-49a2-9406-955264dea4f8",
            )
            .unwrap(),
            category: Category::Locomotives,
            removed_date: date,
        };
        let result = RemoveCollectionItem::execute(&mut unit_of_work, remove_cmd)
            .await
            .expect("Failed to remove collection item");

        // Expect the returned id to match the requested removed id
        assert_eq!(
            result,
            CollectionItemId::try_from("trn:collection-item:89df34a4-ffee-49a2-9406-955264dea4f8")
                .unwrap()
        );
    }
}
