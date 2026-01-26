use crate::maintenance::domain::MaintenanceType;
use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};

/// Arguments for the `AddMaintenanceRecordUseCase`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AddMaintenanceRecordArgs {
    /// The maintenance card this event belongs to.
    pub maintenance_card_id: String,

    /// Date the maintenance was performed (date-only).
    pub date_performed: NaiveDate,

    /// Optional maintenance type.
    pub maintenance_type: Option<MaintenanceType>,

    /// Optional free-text notes.
    pub notes: Option<String>,
}

/// Arguments for adding a maintenance record.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddMaintenanceArgs {
    /// The unique identifier for the maintenance record.
    pub id: String,
    /// The ID of the maintenance card.
    pub maintenance_card_id: String,
    /// The date the maintenance was performed (YYYY-MM-DD).
    pub date_performed: NaiveDate,
    /// The type of maintenance performed (optional).
    pub maintenance_type: Option<String>,
    /// Additional notes about the maintenance (optional).
    pub notes: Option<String>,
}
