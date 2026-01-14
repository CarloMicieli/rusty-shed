use crate::collecting::domain::Collection;
use crate::collecting::domain::CollectionView;
use crate::collecting::domain::{AddCollectionItem, CollectionUowExt};
use crate::core::domain::domain_error::DomainError;

pub struct AddCollectionItemCommand;

impl AddCollectionItemCommand {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        add_cmd: AddCollectionItem,
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
                        .map(|ov| crate::collecting::domain::OwnedRollingStock {
                            id: ov.id,
                            rolling_stock_id: ov.rolling_stock_id,
                            notes: ov.notes,
                            installed_decoder_id: ov.digital.map(|d| d.installed_decoder_id),
                        })
                        .collect();

                    crate::collecting::domain::CollectionItem {
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

        collection.add_item(add_cmd);

        repo.save(&mut collection).await?;

        // Return the refreshed view after persistence
        let updated = repo.find_view().await?;
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::{Category, RailwayModelId};
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{CollectionId, CollectionSummary, MockCollectionRepository};
    use crate::core::domain::{Currency, MonetaryAmount};

    #[tokio::test]
    async fn it_should_add_collection_items() {
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

        mock.expect_save()
            .times(1)
            .returning(move |_collection| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let date = chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let add_item = AddCollectionItem {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:rm:test").unwrap(),
            category: Category::Locomotives,
            rolling_stock_ids: vec![],
            price: MonetaryAmount::new(100, Currency::USD),
            seller_id: None,
            added_date: date,
            purchase_date: date,
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
            notes: Some("Test note".to_string()),
        };

        let _ = AddCollectionItemCommand::execute(&mut unit_of_work, add_item)
            .await
            .expect("Failed to add collection item");
    }
}
