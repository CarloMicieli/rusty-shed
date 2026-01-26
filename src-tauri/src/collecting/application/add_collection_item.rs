use crate::catalog::application::GetRailwayModelById;
use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::collecting::application::AddCollectionItemInput;
use crate::collecting::domain::{CollectionId, NewCollectionItem};
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
        U: CollectionUowExt + RailwayModelUowExt + Send,
        P: IdProvider<CollectionItemId>,
        Q: IdProvider<PurchaseInfoId>,
    {
        let railway_model =
            { GetRailwayModelById::execute(unit_of_work, &input.railway_model_id).await? };

        let railway_model = railway_model.ok_or(DomainError::NotFound {
            resource: "RailwayModel".to_string(),
            identifier: input.railway_model_id.to_string(),
        })?;

        let mut repo = unit_of_work.collections_repository();
        let collection_id = CollectionId::default();
        let collection = repo.find_by_id(&collection_id).await?;

        let mut collection = collection.ok_or(DomainError::NotFound {
            resource: "Collection".to_string(),
            identifier: collection_id.to_string(),
        })?;

        let collection_item_id = collection_item_id_provider.next_id();
        let purchase_info_id = purchase_info_id_provider.next_id();

        let new_item = NewCollectionItem {
            collection_item_id: collection_item_id.clone(),
            purchase_info_id: purchase_info_id.clone(),
            railway_model,
            price: input.price,
            seller_id: input.seller_id,
            added_date: input.added_date,
            purchase_date: input.purchase_date,
            purchase_condition: input.purchase_condition,
            model_condition: input.model_condition,
            box_condition: input.box_condition,
            notes: input.notes,
        };

        let item_id = collection.add_item(new_item);

        repo.save(&mut collection).await?;

        Ok(item_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
        RailwayModelId,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{
        Collection, CollectionId, CollectionSummary, MockCollectionRepository,
    };
    use crate::core::domain::test_utils::DefaultMockIdProvider;
    use crate::core::domain::{Currency, MonetaryAmount};

    #[tokio::test]
    async fn it_should_add_collection_items() {
        let mut mock = MockCollectionRepository::new();
        mock.expect_find_by_id().times(1).returning(move |_| {
            let collection = Collection {
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

        mock.expect_save()
            .times(1)
            .returning(move |_collection| Ok(()));

        let mut railway_mock = MockRailwayModelRepository::new();
        let railway_model_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        let railway_model = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer_id: ManufacturerId::new("not-a-trn"),
            product_code: ProductCode::try_from("P100").unwrap(),
            description: "Test model".to_string(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: Vec::new(),
        };

        railway_mock
            .expect_find_by_id()
            .withf(move |id| *id == railway_model_id)
            .times(1)
            .returning(move |_| Ok(Some(railway_model.clone())));

        let mut unit_of_work = FakeUow::new(mock, railway_mock);

        let date = chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let add_item = AddCollectionItemInput {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:rm:test").unwrap(),
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
