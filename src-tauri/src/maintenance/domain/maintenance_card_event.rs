use crate::maintenance::domain::maintenance_type::MaintenanceType;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain events for the maintenance aggregate.
///
/// This enum represents the immutable facts that occurred within the
/// maintenance bounded context. Event-driven persistence stores these
/// events and replays or projects them into read models.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub enum MaintenanceCardEvent {
    /// A maintenance record was created for a maintenance card.
    MaintenanceRecorded {
        id: Uuid,
        maintenance_card_id: Uuid,
        date_performed: NaiveDate,
        maintenance_type: Option<MaintenanceType>,
        notes: Option<String>,
    },
}

impl MaintenanceCardEvent {
    /// Helper to convert the domain event into a tuple useful for persistence.
    pub fn as_persistence_tuple(&self) -> (Uuid, Uuid, NaiveDate, Option<String>, Option<String>) {
        match self {
            MaintenanceCardEvent::MaintenanceRecorded {
                id,
                maintenance_card_id,
                date_performed,
                maintenance_type,
                notes,
            } => (
                *id,
                *maintenance_card_id,
                *date_performed,
                maintenance_type.as_ref().map(|t| t.to_string()),
                notes.clone(),
            ),
        }
    }
}
