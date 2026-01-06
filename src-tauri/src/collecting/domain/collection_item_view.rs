use crate::collecting::domain::CollectionRailwayModel;
use crate::collecting::domain::OwnedRollingStockView;
use crate::collecting::domain::PurchaseInfo;
use crate::collecting::domain::{
    BoxCondition, CollectionItemId, ModelCondition, PurchaseCondition,
};
use chrono::NaiveDate;
use serde::Serialize;

/// A single item view within a user's collection.
///
/// A `CollectionItemView` represents a reference to a catalog `RailwayModel` along
/// with ownership-specific data such as the rolling stock instances owned by
/// the collector and purchase information.
///
/// It captures the state and details of a specific model as it exists within
/// the collector's personal collection.
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct CollectionItemView {
    /// Unique identifier for this collection item.
    pub id: CollectionItemId,

    /// A lightweight view of the railway model details
    pub railway_model: CollectionRailwayModel,

    /// Date when this item was added to the collection.
    pub added_date: NaiveDate,

    /// Date when this item was removed from the collection, if applicable.
    pub removed_date: Option<NaiveDate>,

    /// Condition of the item as recorded by the owner (e.g. "mint", "used").
    pub purchase_condition: Option<PurchaseCondition>,

    /// Physical and mechanical condition of the model as recorded by the owner.
    pub model_condition: Option<ModelCondition>,

    /// Condition of the original packaging box for this item.
    pub box_condition: Option<BoxCondition>,

    /// Free-form notes provided by the owner for this collection item.
    pub notes: Option<String>,

    /// The specific rolling stock instances owned that correspond to this model.
    pub rolling_stocks: Vec<OwnedRollingStockView>,

    /// Optional purchase information associated with this collection item.
    pub purchase_info: Option<PurchaseInfo>,
}
