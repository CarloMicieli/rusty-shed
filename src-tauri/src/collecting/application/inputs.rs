use crate::catalog::domain::railway_model::RailwayModelId;
use crate::collecting::domain::{BoxCondition, ModelCondition, PurchaseCondition};
use crate::core::domain::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;

/// Input structure for adding an item to the collection.
#[derive(Debug, Clone)]
pub struct AddCollectionItemInput {
    /// The railway model ID of the item to add.
    pub railway_model_id: RailwayModelId,
    /// The price of the item.
    pub price: MonetaryAmount,
    /// The seller ID (optional).
    pub seller_id: Option<SellerId>,
    /// The date the item was added to the collection.
    pub added_date: chrono::NaiveDate,
    /// The date the item was purchased.
    pub purchase_date: chrono::NaiveDate,
    /// The purchase condition (optional).
    pub purchase_condition: Option<PurchaseCondition>,
    /// The model condition (optional).
    pub model_condition: Option<ModelCondition>,
    /// The box condition (optional).
    pub box_condition: Option<BoxCondition>,
    /// Additional notes (optional).
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemoveCollectionItemInput {
    pub collection_item_id: crate::collecting::domain::CollectionItemId,
    pub category: crate::catalog::domain::railway_model::Category,
    pub removed_date: chrono::NaiveDate,
}
