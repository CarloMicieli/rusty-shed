//! SQL row mappers for the `maintenance` module.
//!
//! These types mirror the columns in the SQLite tables and provide
//! parsing helpers to convert TEXT UUIDs and TEXT dates into
//! `uuid::Uuid` and `chrono::NaiveDate` respectively.

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Row mapper for the `maintenance_cards` table.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, FromRow)]
pub struct MaintenanceCardRow {
    /// Primary key stored as TEXT (UUID string).
    pub id: Uuid,

    /// Foreign key to owned rolling stock, stored as TEXT (UUID string).
    pub owned_rolling_stock_id: Uuid,

    /// Last maintenance date stored as TEXT (YYYY-MM-DD) or NULL.
    pub last_maintenance_date: Option<NaiveDate>,

    /// Next maintenance date stored as TEXT (YYYY-MM-DD) or NULL.
    pub next_maintenance_date: Option<NaiveDate>,

    /// Created timestamp stored as TEXT (optional, kept as TEXT here).
    pub created_at: Option<NaiveDateTime>,

    /// Updated timestamp stored as TEXT (optional).
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, FromRow)]
/// Row mapper for the `maintenance_events` table.
pub struct MaintenanceEventRow {
    /// Primary key stored as TEXT (UUID string).
    pub id: Uuid,

    /// FK to maintenance_card.id stored as TEXT.
    pub maintenance_card_id: Uuid,

    /// Date performed stored as TEXT (YYYY-MM-DD).
    pub date_performed: NaiveDate,

    /// Optional free-text notes.
    pub notes: Option<String>,

    /// Optional maintenance type stored as TEXT.
    pub maintenance_type: Option<String>,
}
