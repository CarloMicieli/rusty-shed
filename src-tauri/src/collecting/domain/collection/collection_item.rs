use crate::catalog::domain::{Epoch, PowerMethod, ProductCode, Scale};
use crate::collecting::domain::collection::{OwnedRollingStock, PurchaseInfo};
use serde::{Deserialize, Serialize};

/// A single item within a user's collection.
///
/// A `CollectionItem` represents a reference to a catalog `RailwayModel` along
/// with ownership-specific data such as the rolling stock instances owned by
/// the collector and purchase information. It is intentionally a lightweight
/// entity that is meaningful only inside the context of its parent
/// `Collection` (the aggregate root).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CollectionItem {
    /// Unique identifier for this collection item (e.g. UUID).
    pub id: String,

    /// Link to the corresponding catalog `RailwayModel` this item represents.
    ///
    /// This is a reference to the canonical model in the catalog; use this
    /// to look up full catalog details (manufacturer, product codes, etc.).
    pub railway_model_id: String,

    /// Manufacturer name of the model (copied or stored for quick access).
    pub manufacturer: String,

    /// Manufacturer product code for the model (kept as a value object).
    pub product_code: ProductCode,

    /// Human-readable description of this item or model.
    pub description: String,

    /// The power method for the model (e.g. AC, DC, None).
    pub power_method: PowerMethod,

    /// The scale of the model (e.g. H0, N).
    pub scale: Scale,

    /// The historical epoch associated with the model.
    pub epoch: Epoch,

    /// The specific rolling stock instances owned that correspond to this model.
    pub rolling_stocks: Vec<OwnedRollingStock>,

    /// Optional purchase information associated with this collection item.
    pub purchase_info: Option<PurchaseInfo>,
}
