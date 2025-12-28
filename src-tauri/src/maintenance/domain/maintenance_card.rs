use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

/// Domain model representing a maintenance card for owned rolling stock.
///
/// This struct contains parsed, strongly-typed fields suitable for use in
/// application logic and for returning to the frontend via Specta.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct MaintenanceCard {
    /// Unique identifier for the maintenance card.
    pub id: Uuid,

    /// FK referencing the owned rolling stock.
    pub owned_rolling_stock_id: Uuid,

    /// The date the last maintenance was performed, if any.
    pub last_maintenance_date: Option<NaiveDate>,

    /// The scheduled next maintenance date, if any.
    pub next_maintenance_date: Option<NaiveDate>,

    /// Created timestamp parsed from the database (if present).
    pub created_at: Option<NaiveDateTime>,

    /// Updated timestamp parsed from the database (if present).
    pub updated_at: Option<NaiveDateTime>,
}
