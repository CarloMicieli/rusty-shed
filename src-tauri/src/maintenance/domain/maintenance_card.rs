use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::metadata::Metadata;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::maintenance_card_event::MaintenanceCardEvent;
use crate::maintenance::domain::maintenance_event::MaintenanceEvent;
use crate::maintenance::domain::maintenance_type::MaintenanceType;
use chrono::NaiveDate;
use uuid::Uuid;

/// Represents the primary maintenance record for a specific piece of rolling stock.
///
/// The `MaintenanceCard` acts as an **Aggregate Root** in the domain. It ensures
/// consistency by encapsulating the history of maintenance events and providing
/// methods to update the state of the equipment safely.
///
/// # Invariants
/// * Maintenance events are recorded chronologically.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceCard {
    /// Unique identifier for the maintenance card.
    pub id: MaintenanceCardId,

    /// The owned rolling stock this maintenance card is associated with.
    pub owned_rolling_stock_id: OwnedRollingStockId,

    /// The date the last maintenance was performed, if any.
    pub last_maintenance_date: Option<NaiveDate>,

    /// The scheduled next maintenance date, if any.
    pub next_maintenance_date: Option<NaiveDate>,

    /// Pending domain events produced by operations on this aggregate.
    pub pending_events: Vec<MaintenanceCardEvent>,

    /// Historical events that have been persisted for this maintenance card.
    pub events: Vec<MaintenanceEvent>,

    /// Resource metadata (versioning + timestamps).
    pub metadata: Metadata,
}

impl MaintenanceCard {
    /// Construct a minimal `MaintenanceCard` when only the UUID is known.
    pub fn from_id(id: Uuid) -> Self {
        MaintenanceCard {
            id: MaintenanceCardId::new(&id),
            owned_rolling_stock_id: OwnedRollingStockId::from(id),
            last_maintenance_date: None,
            next_maintenance_date: None,
            pending_events: Vec::new(),
            events: Vec::new(),
            metadata: Metadata::default(),
        }
    }

    /// Record a maintenance activity: update state and emit an event.
    ///
    /// # Arguments
    /// - `id`: Unique identifier for the maintenance event.
    /// - `date_performed`: Date when the maintenance was performed.
    /// - `maintenance_type`: Optional type/category of maintenance performed.
    /// - `notes`: Optional notes or comments about the maintenance.
    pub fn record_maintenance(
        &mut self,
        id: Uuid,
        date_performed: NaiveDate,
        maintenance_type: Option<MaintenanceType>,
        notes: Option<String>,
    ) {
        self.last_maintenance_date = Some(date_performed);

        let evt = MaintenanceCardEvent::MaintenanceRecorded {
            id,
            maintenance_card_id: {
                let s = self.id.to_string();
                let uuid_str = s.trim_start_matches(MaintenanceCardId::TRN_PREFIX);
                Uuid::parse_str(uuid_str).expect("invalid maintenance card id trn")
            },
            date_performed,
            maintenance_type,
            notes,
        };

        self.pending_events.push(evt);
    }

    /// Take pending events and clear the list.
    pub fn take_events(&mut self) -> Vec<MaintenanceCardEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
