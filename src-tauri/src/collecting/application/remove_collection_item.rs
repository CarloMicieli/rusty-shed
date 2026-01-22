use crate::collecting::application::RemoveCollectionItemInput;
use crate::collecting::domain::CollectionItem;
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::OwnedRollingStock;
use crate::collecting::domain::{Collection, CollectionUowExt};
use crate::core::domain::domain_error::DomainError;

/// Use case to remove an item from the collection.
pub struct RemoveCollectionItemUseCase;

impl RemoveCollectionItemUseCase {
    /// Execute the remove collection item use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `remove_cmd`: command carrying the details of the item to remove.
    ///
    /// # Returns
    /// * `CollectionView` on success
    /// * `DomainError` on failure.
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

        // Load current view and rehydrate into domain `Collection` so we can
        // apply domain operations and persist resulting events.
        let view = repo.find_view().await?;

        let mut collection = Collection {
            id: view.id.clone(),
            name: view.name.clone(),
            summary: view.summary,
            total_value: view.total_value,
            items: view
                .items
                .into_iter()
                .map(|iv| {
                    let rolling_stocks = iv
                        .rolling_stocks
                        .into_iter()
                        .map(|ov| OwnedRollingStock {
                            id: ov.id,
                            rolling_stock_id: ov.rolling_stock_id,
                            notes: ov.notes,
                            installed_decoder_id: ov.digital.map(|d| d.installed_decoder_id),
                        })
                        .collect();

                    CollectionItem {
                        id: iv.id,
                        railway_model_id: iv.railway_model.railway_model_id,
                        added_date: iv.added_date,
                        removed_date: iv.removed_date,
                        purchase_condition: iv.purchase_condition,
                        model_condition: iv.model_condition,
                        box_condition: iv.box_condition,
                        notes: iv.notes,
                        rolling_stocks,
                        purchase_info: iv.purchase_info,
                    }
                })
                .collect(),
            pending_events: Vec::new(),
            metadata: Default::default(),
        };

        // capture id before passing command (remove_cmd is consumed)
        let removed_id = remove_cmd.collection_item_id.clone();

        collection.remove_item(remove_cmd);

        repo.save(&mut collection).await?;

        // Return the id of the removed collection item
        Ok(removed_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::Category;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{
        CollectionId, CollectionItemId, CollectionSummary, CollectionView, MockCollectionRepository,
    };

    #[tokio::test]
    async fn it_should_return_collection_view() {
        let mut mock = MockCollectionRepository::new();
        mock.expect_find_view().times(1).returning(move || {
            let view = CollectionView {
                id: CollectionId::default(),
                name: "My Collection".to_string(),
                summary: CollectionSummary::default(),
                total_value: None,
                items: vec![],
            };
            Ok(view.clone())
        });

        mock.expect_save().times(1).returning(move |_| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let date = chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let remove_cmd = RemoveCollectionItemInput {
            collection_item_id: CollectionItemId::try_from(
                "trn:collection-item:89df34a4-ffee-49a2-9406-955264dea4f8",
            )
            .unwrap(),
            category: Category::Locomotives,
            removed_date: date,
        };
        let result = RemoveCollectionItemUseCase::execute(&mut unit_of_work, remove_cmd)
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
