use crate::maintenance::domain::maintenance_type::MaintenanceType;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain model for a maintenance event performed on a maintenance card.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceEvent {
    /// Event identifier.
    pub id: Uuid,

    /// Parent maintenance card id.
    pub maintenance_card_id: Uuid,

    /// Date the event was performed.
    pub date_performed: NaiveDate,

    /// Optional maintenance type.
    pub maintenance_type: MaintenanceType,

    /// Optional notes.
    pub notes: Option<String>,
}
