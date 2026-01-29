use crate::core::infrastructure::error::CommandError;
use crate::maintenance::application::add_maintenance_event::AddMaintenanceEventInput;
use crate::maintenance::domain::{MaintenanceCardId, MaintenanceType};
use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

/// Arguments for the `AddMaintenanceEvent` use-case.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AddMaintenanceEventArgs {
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

impl TryFrom<AddMaintenanceEventArgs> for AddMaintenanceEventInput {
    type Error = CommandError;

    fn try_from(value: AddMaintenanceEventArgs) -> Result<Self, Self::Error> {
        let maintenance_card_id = MaintenanceCardId::try_from(value.maintenance_card_id.clone())
            .map_err(|_e| CommandError::ValidationError(HashMap::new()))?;

        Ok(AddMaintenanceEventInput {
            maintenance_card_id,
            date_performed: value.date_performed,
            maintenance_type: value.maintenance_type,
            notes: value.notes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maintenance::domain::MaintenanceCardId;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[test]
    fn add_maintenance_event_args_converts_to_input() {
        let id = Uuid::new_v4();
        let trn = MaintenanceCardId::from_uuid(&id).to_string();
        let args = AddMaintenanceEventArgs {
            maintenance_card_id: trn,
            date_performed: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            maintenance_type: None,
            notes: Some("note".to_string()),
        };

        let input: AddMaintenanceEventInput = args.try_into().expect("conversion");
        assert_eq!(input.maintenance_card_id, MaintenanceCardId::from_uuid(&id));
        assert_eq!(input.notes.unwrap(), "note");
    }
}
