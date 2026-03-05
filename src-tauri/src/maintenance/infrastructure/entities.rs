//! SQL row mappers for the `maintenance` module.
//!
//! These types mirror the columns in the SQLite tables and provide
//! parsing helpers to convert TEXT TRNs and TEXT dates into
//! domain types and `chrono::NaiveDate` respectively.

use crate::collecting::domain::OwnedRollingStockId;
use crate::maintenance::domain::{MaintenanceCardId, MaintenanceEventId};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Row mapper for the `maintenance_cards` table.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, FromRow)]
pub struct MaintenanceCardRow {
    /// Primary key stored as TEXT (TRN string).
    pub id: MaintenanceCardId,

    /// Foreign key to owned rolling stock, stored as TEXT (TRN string).
    pub owned_rolling_stock_id: OwnedRollingStockId,

    /// Last maintenance date stored as TEXT (YYYY-MM-DD) or NULL.
    pub last_maintenance_date: Option<NaiveDate>,

    /// Next maintenance date stored as TEXT (YYYY-MM-DD) or NULL.
    pub next_maintenance_date: Option<NaiveDate>,

    /// Created timestamp stored as TEXT (kept as String to tolerate any stored format).
    pub created_at: Option<String>,

    /// Updated timestamp stored as TEXT (kept as String to tolerate any stored format).
    pub updated_at: Option<String>,

    /// Row version for optimistic concurrency control.
    pub version: i64,
}

/// Row mapper for the `maintenance_cards` table joined with catalog tables to include display info.
#[derive(Debug, Clone, FromRow)]
pub struct MaintenanceCardWithDisplayInfoRow {
    pub id: MaintenanceCardId,
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub last_maintenance_date: Option<NaiveDate>,
    pub next_maintenance_date: Option<NaiveDate>,
    pub manufacturer_name: Option<String>,
    pub product_code: Option<String>,
    pub series_code: Option<String>,
    pub road_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, FromRow)]
/// Row mapper for the `maintenance_events` table.
pub struct MaintenanceEventRow {
    /// Primary key stored as TEXT (TRN string).
    pub id: MaintenanceEventId,

    /// FK to maintenance_card.id stored as TEXT (TRN string).
    pub maintenance_card_id: MaintenanceCardId,

    /// Date performed stored as TEXT (YYYY-MM-DD).
    pub date_performed: NaiveDate,

    /// Optional free-text notes.
    pub notes: Option<String>,

    /// Optional maintenance type stored as TEXT.
    pub maintenance_type: Option<String>,
}
