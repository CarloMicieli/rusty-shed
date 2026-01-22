use crate::collecting::application::AddCollectionItemInput;
use crate::collecting::domain::{Collection, CollectionItem, OwnedRollingStock};
use crate::collecting::domain::{CollectionItemId, CollectionUowExt, PurchaseInfoId};
use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;

/// Command handler for adding an item to the collection.
pub struct AddCollectionItemUseCase;

impl AddCollectionItemUseCase {
    /// Execute the add collection item use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `collection_item_id_provider`: provider for generating new collection item IDs.
    /// - `purchase_info_id_provider`: provider for generating new purchase info IDs.
    /// - `input`: command carrying the details of the item to add.
    ///
    /// # Returns
    /// * the `CollectionItemId` of the new item on success
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `CollectionUowExt` and `Send`.
    /// - `P`: Identifier provider type for `CollectionItemId`.
    /// - `Q`: Identifier provider type for `PurchaseInfoId`.
    pub async fn execute<U, P, Q>(
        unit_of_work: &mut U,
        collection_item_id_provider: P,
        purchase_info_id_provider: Q,
        input: AddCollectionItemInput,
    ) -> Result<CollectionItemId, DomainError>
    where
        U: CollectionUowExt + Send,
        P: IdProvider<CollectionItemId>,
        Q: IdProvider<PurchaseInfoId>,
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

        let collection_item_id = collection_item_id_provider.next_id();
        let purchase_info_id = purchase_info_id_provider.next_id();

        let item_id =
            collection.add_item(input, collection_item_id.clone(), purchase_info_id.clone());

        repo.save(&mut collection).await?;

        Ok(item_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::{Category, RailwayModelId};
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{
        CollectionId, CollectionSummary, CollectionView, MockCollectionRepository,
    };
    use crate::core::domain::test_utils::DefaultMockIdProvider;
    use crate::core::domain::{Currency, MonetaryAmount};

    #[tokio::test]
    async fn it_should_add_collection_items() {
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

        mock.expect_save()
            .times(1)
            .returning(move |_collection| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let date = chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let add_item = AddCollectionItemInput {
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

        let id_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let _ = AddCollectionItemUseCase::execute(
            &mut unit_of_work,
            id_provider,
            purchase_info_provider,
            add_item,
        )
        .await
        .expect("Failed to add collection item");
    }
}
