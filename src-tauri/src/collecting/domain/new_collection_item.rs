use crate::catalog::domain::railway_model::RailwayModel;
use crate::collecting::domain::{CollectionItemId, PurchaseInfoId};
use crate::core::domain::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;

/// Helper struct representing a new collection item prepared for addition.
#[derive(Debug, Clone)]
pub struct NewCollectionItem {
    pub collection_item_id: CollectionItemId,
    pub purchase_info_id: PurchaseInfoId,
    pub railway_model: RailwayModel,
    pub price: MonetaryAmount,
    pub seller_id: Option<SellerId>,
    pub added_date: chrono::NaiveDate,
    pub purchase_date: chrono::NaiveDate,
    pub purchase_condition: Option<crate::collecting::domain::PurchaseCondition>,
    pub model_condition: Option<crate::collecting::domain::ModelCondition>,
    pub box_condition: Option<crate::collecting::domain::BoxCondition>,
    pub notes: Option<String>,
}
