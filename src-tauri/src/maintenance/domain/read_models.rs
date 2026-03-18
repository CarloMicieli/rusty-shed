use crate::collecting::domain::OwnedRollingStockId;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceType;
use chrono::NaiveDate;
use serde::Serialize;

/// Lightweight view representation of a maintenance event for UI consumption.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceCardEventView {
    /// Unique identifier for the maintenance event.
    pub id: uuid::Uuid,
    /// Date when the maintenance was performed.
    pub date_performed: NaiveDate,
    /// Optional type of maintenance performed.
    pub maintenance_type: Option<MaintenanceType>,
    /// Optional notes associated with the maintenance event.
    pub notes: Option<String>,
}

/// Human-readable identity information sourced from the catalog rolling stock
/// and railway model tables. All fields are optional because a rolling stock
/// may not have a catalog entry.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RollingStockDisplayInfo {
    /// Manufacturer display name (e.g. "Bachmann", "Märklin").
    pub manufacturer_name: Option<String>,
    /// Catalog product code (e.g. "32-504").
    pub product_code: Option<String>,
    /// Series code from the rolling stock record (e.g. "Class 66").
    pub series_code: Option<String>,
    /// Road number / running number (e.g. "66001").
    pub road_number: Option<String>,
}

/// Lightweight view representation of a maintenance card intended for the frontend.
/// Does not include metadata or pending events.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceCardView {
    /// Unique identifier for the maintenance card.
    pub id: MaintenanceCardId,
    /// The owned rolling stock associated with this maintenance card.
    pub owned_rolling_stock_id: OwnedRollingStockId,
    /// Date of the last maintenance performed, if any.
    pub last_maintenance_date: Option<NaiveDate>,
    /// Scheduled date for the next maintenance, if any.
    pub next_maintenance_date: Option<NaiveDate>,
    /// Historical maintenance events associated with this card.
    pub events: Vec<MaintenanceCardEventView>,
    /// Human-readable identity derived from the catalog at query time.
    /// None when the owned rolling stock has no catalog entry.
    pub display_info: Option<RollingStockDisplayInfo>,
}
