use crate::collecting::domain::CollectionItem;
use crate::collecting::domain::CollectionView;
use crate::collecting::domain::OwnedRollingStock;
use crate::collecting::domain::RemoveCollectionItem;
use crate::collecting::domain::{Collection, CollectionUowExt};
use crate::core::domain::domain_error::DomainError;

pub struct RemoveCollectionItemCommand;

impl RemoveCollectionItemCommand {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        remove_cmd: RemoveCollectionItem,
    ) -> Result<CollectionView, DomainError>
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

        collection.remove_item(remove_cmd);

        repo.save(&mut collection).await?;

        // Return the refreshed view after persistence
        let updated = repo.find_view().await?;
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::Category;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{
        CollectionId, CollectionItemId, CollectionSummary, MockCollectionRepository,
    };

    #[tokio::test]
    async fn it_should_return_collection_view() {
        let mut mock = MockCollectionRepository::new();
        mock.expect_find_view().times(2).returning(move || {
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
        let remove_cmd = RemoveCollectionItem {
            collection_item_id: CollectionItemId::try_from(
                "trn:collection-item:89df34a4-ffee-49a2-9406-955264dea4f8",
            )
            .unwrap(),
            category: Category::Locomotives,
            removed_date: date,
        };
        let result = RemoveCollectionItemCommand::execute(&mut unit_of_work, remove_cmd)
            .await
            .expect("Failed to remove collection item");

        assert_eq!(result.items.len(), 0);
    }
}
