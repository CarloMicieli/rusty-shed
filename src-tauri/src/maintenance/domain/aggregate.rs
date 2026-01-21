use crate::maintenance::domain::events::MaintenanceEvent;
use crate::maintenance::domain::maintenance_card::MaintenanceCard;
use crate::maintenance::domain::maintenance_type::MaintenanceType;
use chrono::NaiveDate;
use uuid::Uuid;

/// Aggregate root for maintenance domain. Holds current state and emitted events.
#[derive(Debug, Clone)]
pub struct MaintenanceAggregate {
    pub card: MaintenanceCard,
    /// Events that have been produced by operations on the aggregate but
    /// not yet persisted/applied to the event store.
    pub pending_events: Vec<MaintenanceEvent>,
}

impl MaintenanceAggregate {
    /// Create an aggregate from an existing maintenance card.
    pub fn from_card(card: MaintenanceCard) -> Self {
        Self {
            card,
            pending_events: Vec::new(),
        }
    }

    /// Create a minimal aggregate when only the maintenance card id is known.
    ///
    /// Note: This creates a skeleton `MaintenanceCard` with `owned_rolling_stock_id`
    /// set to the same value as `id` as a sensible default for callers that only
    /// have the card id. Prefer `from_card` when the full card projection is available.
    pub fn from_id(id: uuid::Uuid) -> Self {
        let card = MaintenanceCard {
            id,
            owned_rolling_stock_id: id,
            last_maintenance_date: None,
            next_maintenance_date: None,
            created_at: None,
            updated_at: None,
        };

        Self::from_card(card)
    }

    /// Record a maintenance activity: update aggregate state and emit an event.
    pub fn record_maintenance(
        &mut self,
        id: Uuid,
        date_performed: NaiveDate,
        maintenance_type: Option<MaintenanceType>,
        notes: Option<String>,
    ) {
        // update state
        self.card.last_maintenance_date = Some(date_performed);

        // push domain event
        let evt = MaintenanceEvent::MaintenanceRecorded {
            id,
            maintenance_card_id: self.card.id,
            date_performed,
            maintenance_type,
            notes,
        };

        self.pending_events.push(evt);
    }

    /// Take pending events and clear the list.
    pub fn take_events(&mut self) -> Vec<MaintenanceEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
