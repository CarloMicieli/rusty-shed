use crate::maintenance::domain::maintenance_card::MaintenanceCard;
use crate::maintenance::domain::maintenance_event::MaintenanceEvent;
use crate::maintenance::domain::maintenance_type::MaintenanceType;
use crate::maintenance::infrastructure::entities::{MaintenanceCardRow, MaintenanceEventRow};

impl TryFrom<MaintenanceCardRow> for MaintenanceCard {
    type Error = String;

    fn try_from(row: MaintenanceCardRow) -> Result<Self, Self::Error> {
        let id = row.id;
        let owned = row.owned_rolling_stock_id;

        let last = row.last_maintenance_date;
        let next = row.next_maintenance_date;

        let created = row.created_at;
        let updated = row.updated_at;

        Ok(MaintenanceCard {
            id,
            owned_rolling_stock_id: owned,
            last_maintenance_date: last,
            next_maintenance_date: next,
            created_at: created,
            updated_at: updated,
        })
    }
}

impl TryFrom<MaintenanceEventRow> for MaintenanceEvent {
    type Error = String;

    fn try_from(row: MaintenanceEventRow) -> Result<Self, Self::Error> {
        let id = row.id;
        let card_id = row.maintenance_card_id;
        let date = row.date_performed;

        let maintenance_type = match row.maintenance_type {
            Some(s) => s
                .parse::<MaintenanceType>()
                .unwrap_or(MaintenanceType::Other),
            None => MaintenanceType::Other,
        };

        Ok(MaintenanceEvent {
            id,
            maintenance_card_id: card_id,
            date_performed: date,
            maintenance_type,
            notes: row.notes,
        })
    }
}
