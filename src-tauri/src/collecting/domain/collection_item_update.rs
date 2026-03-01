use crate::collecting::domain::{
    BoxCondition, CollectionItemId, ModelCondition, PurchaseCondition,
};
use crate::core::domain::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;

/// Domain command payload describing a single collection-item field mutation.
///
/// This is intentionally modeled as a tagged enum so callers can issue one
/// focused update per command invocation while still using a single IPC command.
#[derive(Debug, Clone)]
pub enum CollectionItemUpdate {
    /// Update or clear seller in purchase info.
    Seller(Option<SellerId>),
    /// Update or clear purchased price in purchase info.
    Price(Option<MonetaryAmount>),
    /// Update or clear purchase date in purchase info.
    PurchaseDate(Option<NaiveDate>),
    /// Update or clear added date on collection item.
    AddedDate(Option<NaiveDate>),
    /// Update or clear free-form notes on collection item.
    Notes(Option<String>),
    /// Update or clear purchase condition on collection item.
    PurchaseCondition(Option<PurchaseCondition>),
    /// Update or clear model condition on collection item.
    ModelCondition(Option<ModelCondition>),
    /// Update or clear box condition on collection item.
    BoxCondition(Option<BoxCondition>),
}

/// Input for the update-collection-item use case.
#[derive(Debug, Clone)]
pub struct UpdateCollectionItemInput {
    /// The collection item identifier to mutate.
    pub collection_item_id: CollectionItemId,
    /// The concrete field change to apply.
    pub update: CollectionItemUpdate,
}
