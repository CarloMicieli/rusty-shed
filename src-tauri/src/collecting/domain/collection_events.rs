use crate::catalog::domain::railway_model::{Category, RailwayModelId, RollingStockId};
use crate::collecting::domain::{
    BoxCondition, CollectionId, CollectionItemId, ModelCondition, OwnedRollingStockId,
    PurchaseCondition, PurchaseInfoId,
};
use crate::core::domain::MonetaryAmount;
use crate::dcc_inventory::domain::DecoderId;
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionEvent {
    /// A new collection was created.
    CollectionCreated {
        aggregate_id: CollectionId,
        name: String,
    },
    /// A railway model was added to the collection.
    RailwayModelAdded {
        aggregate_id: CollectionId,
        collection_item_id: CollectionItemId,
        railway_model_id: RailwayModelId,
        category: Category,
        rolling_stock: Vec<OwnedRollingStockIds>,
        price: MonetaryAmount,
        seller_id: Option<SellerId>,
        added_date: NaiveDate,
        purchase_info_id: PurchaseInfoId,
        purchase_date: NaiveDate,
        purchase_condition: Option<PurchaseCondition>,
        model_condition: Option<ModelCondition>,
        box_condition: Option<BoxCondition>,
        notes: Option<String>,
    },
    /// A railway model was removed from the collection.
    RailwayModelRemoved {
        aggregate_id: CollectionId,
        collection_item_id: CollectionItemId,
        removed_date: NaiveDate,
        category: Category,
    },
    /// A railway model was sold from the collection.
    RailwayModelSold {
        aggregate_id: CollectionId,
        collection_item_id: CollectionItemId,
        removed_date: NaiveDate,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRollingStockIds {
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub rolling_stock_id: RollingStockId,
    pub installed_decoder_id: Option<DecoderId>,
}
