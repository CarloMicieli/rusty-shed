use crate::catalog::domain::manufacturer::ManufacturerId;
use serde::{Deserialize, Serialize};

/// A `RailwayModelManufacturer` represents the manufacturer of a railway model.
/// It contains the unique identifier and display name of the manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct RailwayModelManufacturer {
    /// The unique identifier of the manufacturer.
    pub manufacturer_id: ManufacturerId,
    /// The manufacturer of the model (e.g. Bachmann, Märklin).
    pub display: String,
}
