use crate::catalog::domain::railway_model::{Category, RailwayModelId, RollingStockId};
use crate::collecting::domain::{
    BoxCondition, CollectionId, CollectionItemId, ModelCondition,
    OwnedRollingStockId, PurchaseCondition, PurchaseInfoId,
};
use crate::core::domain::{DomainEvent, MonetaryAmount};
use crate::dcc_inventory::domain::DecoderId;
use crate::sellers::domain::seller_id::SellerId;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionEvent {
    /// A new collection was created.
    CollectionCreated {
        event_id: Uuid,
        aggregate_id: CollectionId,
        timestamp: NaiveDateTime,
    },
    /// A railway model was added to the collection.
    RailwayModelAdded {
        event_id: Uuid,
        aggregate_id: CollectionId,
        timestamp: NaiveDateTime,

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
        event_id: Uuid,
        aggregate_id: CollectionId,
        timestamp: NaiveDateTime,

        collection_item_id: CollectionItemId,
        removed_date: NaiveDate,
    },
    /// A railway model was sold from the collection.
    RailwayModelSold {
        event_id: Uuid,
        aggregate_id: CollectionId,
        timestamp: NaiveDateTime,

        collection_item_id: CollectionItemId,
        removed_date: NaiveDate,
    },
}

impl DomainEvent<CollectionId> for CollectionEvent {
    fn aggregate_id(&self) -> &CollectionId {
        match self {
            CollectionEvent::CollectionCreated { aggregate_id, .. } => aggregate_id,
            CollectionEvent::RailwayModelAdded { aggregate_id, .. } => aggregate_id,
            CollectionEvent::RailwayModelRemoved { aggregate_id, .. } => aggregate_id,
            CollectionEvent::RailwayModelSold { aggregate_id, .. } => aggregate_id,
        }
    }

    fn event_id(&self) -> &Uuid {
        match self {
            CollectionEvent::CollectionCreated { event_id, .. } => event_id,
            CollectionEvent::RailwayModelAdded { event_id, .. } => event_id,
            CollectionEvent::RailwayModelRemoved { event_id, .. } => event_id,
            CollectionEvent::RailwayModelSold { event_id, .. } => event_id,
        }
    }

    fn timestamp(&self) -> NaiveDateTime {
        match self {
            CollectionEvent::CollectionCreated { timestamp, .. } => *timestamp,
            CollectionEvent::RailwayModelAdded { timestamp, .. } => *timestamp,
            CollectionEvent::RailwayModelRemoved { timestamp, .. } => *timestamp,
            CollectionEvent::RailwayModelSold { timestamp, .. } => *timestamp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRollingStockIds {
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub rolling_stock_id: RollingStockId,
    pub installed_decoder_id: Option<DecoderId>,
}
