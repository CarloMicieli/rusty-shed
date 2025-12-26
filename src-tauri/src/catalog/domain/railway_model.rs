use crate::catalog::domain::RollingStock;
use crate::catalog::domain::availability_status::AvailabilityStatus;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::{Category, DeliveryDate, Epoch, PowerMethod, ProductCode, Scale};
use serde::{Deserialize, Serialize};

/// A `RailwayModel` represents a manufactured model product in the catalog.
///
/// It contains metadata about the product (manufacturer, product code,
/// scale, epoch, etc.) and a list of `RollingStock` instances that correspond
/// to specific owned or catalogued items of this model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayModel {
    /// Unique identifier for the railway model.
    pub id: RailwayModelId,

    /// The manufacturer of the model (e.g. Bachmann, Märklin).
    pub manufacturer: String,

    /// Manufacturer-assigned product code.
    pub product_code: ProductCode,

    /// Human-readable description of the model.
    pub description: String,

    /// Additional details about the model (e.g. special features, variations).
    pub details: Option<String>,

    /// The power method used by this model (e.g. Diesel, Electric, None for non-powered models).
    pub power_method: PowerMethod,

    /// The scale of the model (e.g. HO, N).
    pub scale: Scale,

    /// The historical epoch the model belongs to.
    pub epoch: Epoch,

    /// Classification category for the model (e.g. locomotive, freight car).
    pub category: Category,

    /// Delivery or release date information for the product.
    pub delivery_date: Option<DeliveryDate>,

    /// the availability status
    pub availability_status: Option<AvailabilityStatus>,

    /// Rolling stock instances (specific vehicles) that correspond to this model.
    pub rolling_stocks: Vec<RollingStock>,
}
