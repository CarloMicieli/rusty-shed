use crate::catalog::domain::railway_model::{Control, Epoch, ProductCode, RollingStockCategory};
use crate::collecting::domain::OwnedRollingStockId;
use serde::Serialize;

/// A read-only representation of the depot contents.
///
/// This view is intended for use by the interface layer (API/IPC/UI) and
/// serializes the minimal information required to display the depot and its
/// rolling stock entries.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DepotView {
    /// A list of rolling stock entries currently associated with the depot.
    pub rolling_stocks: Vec<DepotRollingStockView>,
}

/// Compact information about a single owned rolling stock instance.
///
/// The struct is intentionally small and focused on presentation needs:
/// identity, human-friendly labels, classification and a few display hints
/// (like livery and depot location) used by the frontend.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DepotRollingStockView {
    /// Unique identifier for the owned rolling stock instance.
    pub id: OwnedRollingStockId,

    /// Series or class code for the rolling stock (e.g. "Class 37").
    pub series_code: String,

    /// Optional road number used to disambiguate items in the same series.
    pub road_number: Option<String>,

    /// Optional user-facing friendly name for the item.
    pub friendly_name: Option<String>,

    /// Optional depot/location name where the item is stored.
    pub depot: Option<String>,

    /// The rolling stock category (locomotive, coach, freight, etc.).
    pub category: RollingStockCategory,

    /// Manufacturer display name (e.g. "Hornby").
    pub manufacturer_name: String,

    /// Product code identifying the specific model variant.
    pub product_code: ProductCode,

    /// Optional control type (analogue/DCC/etc.) where known.
    pub control: Option<Control>,

    /// Optional livery or paint scheme string for display purposes.
    pub livery: Option<String>,

    /// Optional railway company name for display purposes.
    pub railway_company_name: Option<String>,

    /// Epoch/era (e.g., "IV", "III/IV", "Vm") for display purposes.
    pub epoch: Option<Epoch>,
}
