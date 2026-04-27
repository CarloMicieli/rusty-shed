use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::{
    Category, Epoch, PowerMethod, ProductCode, RailwayModelId, RailwayModelParams,
    RailwayModelUowExt,
};
use crate::catalog::domain::scale::Scale;
use crate::collecting::domain::{
    CollectionId, CollectionItemId, CollectionUowExt, NewCollectionItem, PurchaseInfoId,
};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{IdProvider, MonetaryAmount};
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;

/// Input for the RecordAcquisition use case.
#[derive(Debug, Clone)]
pub struct RecordAcquisitionInput {
    pub seller_id: Option<SellerId>,
    pub purchase_date: NaiveDate,
    pub items: Vec<AcquisitionItemInput>,
}

/// Input for a single item within a RecordAcquisition operation.
#[derive(Debug, Clone)]
pub struct AcquisitionItemInput {
    pub manufacturer_id: ManufacturerId,
    pub product_code: String,
    pub description: String,
    pub category: Category,
    pub scale: Scale,
    pub epoch: Epoch,
    pub power_method: PowerMethod,
    pub price: MonetaryAmount,
}

/// Use case: record a batch acquisition (one or more catalog items + collection items).
pub struct RecordAcquisition;

impl RecordAcquisition {
    /// Execute the record acquisition use case.
    ///
    /// For each item:
    /// 1. Derives a deterministic `RailwayModelId`.
    /// 2. Creates the catalog entry if it does not exist.
    /// 3. Loads the `RailwayModel` aggregate (needed by `Collection::add_item`).
    /// 4. Adds a new `CollectionItem` (with purchase info) to the default collection.
    ///
    /// A single `save` on the collection repository is performed after all items
    /// are processed so that the whole batch commits atomically.
    pub async fn execute<U, P, Q>(
        unit_of_work: &mut U,
        collection_item_id_provider: P,
        purchase_info_id_provider: Q,
        input: RecordAcquisitionInput,
    ) -> Result<Vec<CollectionItemId>, DomainError>
    where
        U: CollectionUowExt + RailwayModelUowExt + Send,
        P: IdProvider<CollectionItemId>,
        Q: IdProvider<PurchaseInfoId>,
    {
        // Load (or auto-create) the default collection
        let collection_id = CollectionId::default();
        let mut collection = unit_of_work
            .collections_repository()
            .find_by_id(&collection_id)
            .await?
            .unwrap_or_default();

        let today = chrono::Local::now().date_naive();
        let mut ids: Vec<CollectionItemId> = Vec::with_capacity(input.items.len());

        for item in input.items {
            // 1. Derive deterministic model ID
            let model_id = RailwayModelId::new(&item.manufacturer_id, &item.product_code)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

            // 2. Upsert catalog entry (create only if absent)
            let existing = unit_of_work
                .railway_model_repository()
                .find_by_id(&model_id, Language::English)
                .await?;

            if existing.is_none() {
                let product_code = ProductCode::try_from(item.product_code.as_str())
                    .map_err(|e| DomainError::Validation(e.to_string()))?;

                let params = RailwayModelParams {
                    manufacturer_id: item.manufacturer_id.clone(),
                    product_code,
                    description: item.description.clone(),
                    details: None,
                    power_method: item.power_method,
                    scale: item.scale,
                    epoch: item.epoch.clone(),
                    category: item.category,
                    delivery_date: None,
                    availability_status: None,
                    rolling_stocks: vec![],
                };

                unit_of_work
                    .railway_model_repository()
                    .create(&params)
                    .await?;
            }

            // 3. Load the full aggregate (required by NewCollectionItem)
            let railway_model = unit_of_work
                .railway_model_repository()
                .find_by_id(&model_id, Language::English)
                .await?
                .ok_or_else(|| DomainError::NotFound {
                    resource: "RailwayModel".to_string(),
                    identifier: model_id.to_string(),
                })?;

            // 4. Record the purchase
            let collection_item_id = collection_item_id_provider.next_id();
            let purchase_info_id = purchase_info_id_provider.next_id();

            let new_item = NewCollectionItem {
                collection_item_id: collection_item_id.clone(),
                purchase_info_id,
                railway_model,
                price: item.price,
                seller_id: input.seller_id.clone(),
                added_date: today,
                purchase_date: input.purchase_date,
                purchase_condition: None,
                model_condition: None,
                box_condition: None,
                notes: None,
            };

            collection.add_item(new_item);
            ids.push(collection_item_id);
        }

        // 5. Persist the collection once for the entire batch
        unit_of_work
            .collections_repository()
            .save(&mut collection)
            .await?;

        Ok(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        MockRailwayModelRepository, ProductCode, RailwayModel,
    };
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{Collection, MockCollectionRepository};
    use crate::core::domain::Currency;
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::test_utils::SequentialIdProvider;

    fn manufacturer_id() -> ManufacturerId {
        ManufacturerId::from_string_unchecked("trn:manufacturer:acme".to_string())
    }

    fn railway_model() -> RailwayModel {
        let model_id = RailwayModelId::new(&manufacturer_id(), "60100").expect("valid model id");

        RailwayModel {
            id: model_id,
            manufacturer_id: manufacturer_id(),
            product_code: ProductCode::try_from("60100").expect("valid product code"),
            description: LocalizedField {
                lang: Language::English,
                value: "ACME test model".to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: vec![],
        }
    }

    fn input_with_product_code(product_code: &str) -> RecordAcquisitionInput {
        RecordAcquisitionInput {
            seller_id: None,
            purchase_date: NaiveDate::from_ymd_opt(2026, 2, 1).expect("valid date"),
            items: vec![AcquisitionItemInput {
                manufacturer_id: manufacturer_id(),
                product_code: product_code.to_string(),
                description: "ACME test model".to_string(),
                category: Category::Locomotives,
                scale: Scale::H0,
                epoch: "IV".into(),
                power_method: PowerMethod::DC,
                price: MonetaryAmount::new(12_500, Currency::EUR),
            }],
        }
    }

    #[tokio::test]
    async fn it_should_create_missing_catalog_model_and_save_collection() {
        let model = railway_model();
        let model_id = model.id.clone();

        let mut collections_find = MockCollectionRepository::new();
        collections_find
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(None));

        let mut collections_save = MockCollectionRepository::new();
        collections_save
            .expect_save()
            .once()
            .returning(|collection| {
                assert_eq!(collection.items.len(), 1);
                Ok(())
            });

        let mut railway_find_missing = MockRailwayModelRepository::new();
        railway_find_missing
            .expect_find_by_id()
            .once()
            .returning(|_, _| Ok(None));

        let model_id_for_create = model_id.clone();
        let mut railway_create = MockRailwayModelRepository::new();
        railway_create
            .expect_create()
            .once()
            .returning(move |_| Ok(model_id_for_create.clone()));

        let mut railway_find_created = MockRailwayModelRepository::new();
        railway_find_created
            .expect_find_by_id()
            .once()
            .returning(move |_, _| Ok(Some(model.clone())));

        let mut uow = FakeUow::default()
            .with_collection_repo(collections_find)
            .with_collection_repo(collections_save)
            .with_railway_repo(railway_find_missing)
            .with_railway_repo(railway_create)
            .with_railway_repo(railway_find_created);

        let expected_item_id = CollectionItemId::new_from_parts(&["test-item-id"]);
        let expected_purchase_id = PurchaseInfoId::new_from_parts(&["test-purchase-id"]);

        let item_id_provider = SequentialIdProvider::new(vec![expected_item_id.clone()]);
        let purchase_id_provider = SequentialIdProvider::new(vec![expected_purchase_id]);

        let result = RecordAcquisition::execute(
            &mut uow,
            item_id_provider,
            purchase_id_provider,
            input_with_product_code("60100"),
        )
        .await
        .expect("record acquisition should succeed");

        assert_eq!(result, vec![expected_item_id]);
    }

    #[tokio::test]
    async fn it_should_return_not_found_when_model_is_not_retrievable_after_create() {
        let mut collections_find = MockCollectionRepository::new();
        collections_find
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(Some(Collection::default())));

        let mut railway_find_missing = MockRailwayModelRepository::new();
        railway_find_missing
            .expect_find_by_id()
            .once()
            .returning(|_, _| Ok(None));

        let model_id = RailwayModelId::new(&manufacturer_id(), "60100").expect("valid model id");
        let model_id_for_create = model_id.clone();
        let mut railway_create = MockRailwayModelRepository::new();
        railway_create
            .expect_create()
            .once()
            .returning(move |_| Ok(model_id_for_create.clone()));

        let mut railway_find_missing_again = MockRailwayModelRepository::new();
        railway_find_missing_again
            .expect_find_by_id()
            .once()
            .returning(|_, _| Ok(None));

        let mut uow = FakeUow::default()
            .with_collection_repo(collections_find)
            .with_railway_repo(railway_find_missing)
            .with_railway_repo(railway_create)
            .with_railway_repo(railway_find_missing_again);

        let item_id_provider = SequentialIdProvider::new(vec![CollectionItemId::default()]);
        let purchase_id_provider = SequentialIdProvider::new(vec![PurchaseInfoId::default()]);

        let result = RecordAcquisition::execute(
            &mut uow,
            item_id_provider,
            purchase_id_provider,
            input_with_product_code("60100"),
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }

    #[tokio::test]
    async fn it_should_map_invalid_product_code_to_validation_error() {
        let mut collections_find = MockCollectionRepository::new();
        collections_find
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(Some(Collection::default())));

        let mut railway_find_missing = MockRailwayModelRepository::new();
        railway_find_missing
            .expect_find_by_id()
            .once()
            .returning(|_, _| Ok(None));

        let mut uow = FakeUow::default()
            .with_collection_repo(collections_find)
            .with_railway_repo(railway_find_missing);

        let item_id_provider = SequentialIdProvider::new(vec![CollectionItemId::default()]);
        let purchase_id_provider = SequentialIdProvider::new(vec![PurchaseInfoId::default()]);

        let result = RecordAcquisition::execute(
            &mut uow,
            item_id_provider,
            purchase_id_provider,
            input_with_product_code(""),
        )
        .await;

        assert!(matches!(result, Err(DomainError::Validation(_))));
    }
}
