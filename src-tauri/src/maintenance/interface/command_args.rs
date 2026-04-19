use crate::core::domain::validation::validate_not_future_date;
use crate::core::infrastructure::error::CommandError;
use crate::maintenance::application::add_maintenance_event::AddMaintenanceEventInput;
use crate::maintenance::domain::validate_maintenance_card_id;
use crate::maintenance::domain::{MaintenanceCardId, MaintenanceType};
use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

/// Arguments for the `AddMaintenanceEvent` use-case.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddMaintenanceEventArgs {
    /// The maintenance card this event belongs to (non-empty TRN string).
    #[garde(length(min = 1), custom(validate_maintenance_card_id))]
    pub maintenance_card_id: String,

    /// Date the maintenance was performed (date-only).
    #[garde(custom(validate_not_future_date))]
    pub date_performed: NaiveDate,

    /// Optional maintenance type.
    #[garde(skip)]
    pub maintenance_type: Option<MaintenanceType>,

    /// Optional free-text notes.
    #[garde(length(max = 2000))]
    pub notes: Option<String>,

    /// Optional scheduled date for the next maintenance event.
    #[garde(skip)]
    pub next_maintenance_date: Option<NaiveDate>,
}

fn validate_opt_maintenance_type_parse(value: &Option<String>, _: &()) -> garde::Result {
    match value {
        None => Ok(()),
        Some(s) => s
            .parse::<MaintenanceType>()
            .map(|_| ())
            .map_err(|_| garde::Error::new("error_invalid_maintenance_type")),
    }
}

/// Arguments for adding a maintenance record.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddMaintenanceArgs {
    /// The ID of the maintenance card.
    #[garde(length(min = 1), custom(validate_maintenance_card_id))]
    pub maintenance_card_id: String,
    /// The date the maintenance was performed (YYYY-MM-DD).
    #[garde(custom(validate_not_future_date))]
    pub date_performed: NaiveDate,
    /// The type of maintenance performed (optional).
    #[garde(custom(validate_opt_maintenance_type_parse))]
    pub maintenance_type: Option<String>,
    /// Additional notes about the maintenance (optional).
    #[garde(length(max = 2000))]
    pub notes: Option<String>,

    /// Optional scheduled date for the next maintenance event.
    #[garde(skip)]
    pub next_maintenance_date: Option<NaiveDate>,
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
            next_maintenance_date: value.next_maintenance_date,
        })
    }
}

#[cfg(test)]
mod garde_tests {
    use super::*;
    use crate::maintenance::domain::MaintenanceCardId;
    use chrono::NaiveDate;
    use garde::Validate;
    use uuid::Uuid;

    fn valid_add_event() -> AddMaintenanceEventArgs {
        let id = Uuid::new_v4();
        AddMaintenanceEventArgs {
            maintenance_card_id: MaintenanceCardId::from_uuid(&id).to_string(),
            date_performed: NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
            maintenance_type: None,
            notes: None,
            next_maintenance_date: None,
        }
    }

    #[test]
    fn add_maintenance_event_valid_passes() {
        assert!(valid_add_event().validate().is_ok());
    }

    #[test]
    fn add_maintenance_event_empty_card_id_fails() {
        let args = AddMaintenanceEventArgs {
            maintenance_card_id: String::new(),
            ..valid_add_event()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "maintenance_card_id"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_maintenance_event_invalid_card_id_format_fails() {
        let args = AddMaintenanceEventArgs {
            maintenance_card_id: "not-a-trn".to_string(),
            ..valid_add_event()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "maintenance_card_id"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_maintenance_event_notes_too_long_fails() {
        let args = AddMaintenanceEventArgs {
            notes: Some("x".repeat(2001)),
            ..valid_add_event()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "notes"),
            "{errors:?}"
        );
    }

    fn valid_add_maintenance() -> AddMaintenanceArgs {
        let id = Uuid::new_v4();
        AddMaintenanceArgs {
            maintenance_card_id: MaintenanceCardId::from_uuid(&id).to_string(),
            date_performed: NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
            maintenance_type: None,
            notes: None,
            next_maintenance_date: None,
        }
    }

    #[test]
    fn add_maintenance_valid_passes() {
        assert!(valid_add_maintenance().validate().is_ok());
    }

    #[test]
    fn add_maintenance_empty_card_id_fails() {
        let args = AddMaintenanceArgs {
            maintenance_card_id: String::new(),
            ..valid_add_maintenance()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "maintenance_card_id"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_maintenance_invalid_card_id_format_fails() {
        let args = AddMaintenanceArgs {
            maintenance_card_id: "not-a-trn".to_string(),
            ..valid_add_maintenance()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "maintenance_card_id"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_maintenance_invalid_type_string_fails() {
        let args = AddMaintenanceArgs {
            maintenance_type: Some("unknown-type".to_string()),
            ..valid_add_maintenance()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "maintenance_type"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_maintenance_notes_too_long_fails() {
        let args = AddMaintenanceArgs {
            notes: Some("y".repeat(2001)),
            ..valid_add_maintenance()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "notes"),
            "{errors:?}"
        );
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
            next_maintenance_date: Some(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()),
        };

        let input: AddMaintenanceEventInput = args.try_into().expect("conversion");
        assert_eq!(input.maintenance_card_id, MaintenanceCardId::from_uuid(&id));
        assert_eq!(input.notes.unwrap(), "note");
        assert_eq!(
            input.next_maintenance_date,
            Some(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap())
        );
    }
}
