use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::{
    Category, Epoch, PowerMethod, ProductCode, RailwayModelId, RailwayModelParams,
    RailwayModelUowExt,
};
use crate::catalog::domain::scale::Scale;
use crate::collecting::domain::{
    CollectionId, CollectionItemId, CollectionUowExt, NewCollectionItem, PurchaseInfoId,
};
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
                .find_by_id(&model_id, "en")
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
                .find_by_id(&model_id, "en")
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
