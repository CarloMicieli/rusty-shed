use crate::catalog::domain::railway_model::{Category, RailwayModelId, RollingStockId};
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::{BoxCondition, ModelCondition, PurchaseCondition};
use crate::core::domain::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;

#[derive(Debug, Clone)]
pub struct AddCollectionItem {
    pub railway_model_id: RailwayModelId,
    pub rolling_stock_ids: Vec<RollingStockId>,
    pub category: Category,
    pub price: MonetaryAmount,
    pub seller_id: Option<SellerId>,
    pub added_date: chrono::NaiveDate,
    pub purchase_date: chrono::NaiveDate,
    pub purchase_condition: Option<PurchaseCondition>,
    pub model_condition: Option<ModelCondition>,
    pub box_condition: Option<BoxCondition>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemoveCollectionItem {
    pub collection_item_id: CollectionItemId,
    pub category: Category,
    pub removed_date: chrono::NaiveDate,
}
