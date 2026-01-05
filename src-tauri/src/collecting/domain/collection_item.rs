use crate::catalog::domain::railway_model::RailwayModelId;
use crate::collecting::domain::CollectionRailwayModel;
use crate::collecting::domain::OwnedRollingStock;
use crate::collecting::domain::PurchaseInfo;
use crate::collecting::domain::{
    BoxCondition, CollectionItemId, ModelCondition, PurchaseCondition,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A single item within a user's collection.
///
/// A `CollectionItem` represents a reference to a catalog `RailwayModel` along
/// with ownership-specific data such as the rolling stock instances owned by
/// the collector and purchase information.
///
/// It captures the state and details of a specific model as it exists within
/// the collector's personal collection.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CollectionItem {
    /// Unique identifier for this collection item.
    pub id: CollectionItemId,

    /// Link to the corresponding catalog `RailwayModel` this item represents.
    ///
    /// This is a reference to the canonical model in the catalog; use this
    /// to look up full catalog details (manufacturer, product codes, etc.).
    pub railway_model_id: RailwayModelId,

    /// A lightweight view of the railway model details
    pub railway_model: Option<CollectionRailwayModel>,

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
    pub rolling_stocks: Vec<OwnedRollingStock>,

    /// Optional purchase information associated with this collection item.
    pub purchase_info: Option<PurchaseInfo>,
}
