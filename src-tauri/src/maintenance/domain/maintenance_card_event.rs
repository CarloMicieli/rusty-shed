use crate::maintenance::domain::maintenance_type::MaintenanceType;
use chrono::NaiveDate;
use uuid::Uuid;

/// Domain events for the maintenance aggregate.
///
/// This enum represents the immutable facts that occurred within the
/// maintenance bounded context. Event-driven persistence stores these
/// events and replays or projects them into read models.
///
/// Domain events carry pure business concepts; serialization for persistence
/// (e.g., converting MaintenanceType to string) is handled exclusively in the
/// infrastructure layer.
#[derive(Debug, Clone)]
pub enum MaintenanceCardEvent {
    /// A maintenance card aggregate was created.
    Created {
        id: Uuid,
        maintenance_card_id: Uuid,
        created_at: NaiveDate,
    },
    /// A maintenance record was created for a maintenance card.
    MaintenanceRecorded {
        id: Uuid,
        maintenance_card_id: Uuid,
        date_performed: NaiveDate,
        maintenance_type: Option<MaintenanceType>,
        notes: Option<String>,
    },
}
