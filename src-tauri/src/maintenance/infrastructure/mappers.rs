use crate::core::domain::metadata::Metadata;
use crate::maintenance::domain::MaintenanceCard;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceEvent;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::infrastructure::entities::{MaintenanceCardRow, MaintenanceEventRow};
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

impl TryFrom<MaintenanceCardRow> for MaintenanceCard {
    type Error = String;

    fn try_from(row: MaintenanceCardRow) -> Result<Self, Self::Error> {
        // Extract UUID from TRN (e.g., "trn:maintenance-card:uuid").
        // `MaintenanceCardId` is a newtype; convert to string first.
        let id_trn = row.id.to_string();
        let uuid_str = id_trn.rsplit_once(':').map(|(_, s)| s).unwrap_or(&id_trn);
        let uuid = uuid::Uuid::parse_str(uuid_str)
            .map_err(|e| format!("Invalid maintenance card id: {}", e))?;

        let id = MaintenanceCardId::from_uuid(&uuid);
        // `owned_rolling_stock_id` is already strongly-typed in the row, move it.
        let owned = row.owned_rolling_stock_id;

        let last = row.last_maintenance_date;
        let next = row.next_maintenance_date;

        // Parse the stored TEXT timestamp into DateTime<Utc>. The column may contain either a
        // full datetime ("YYYY-MM-DD HH:MM:SS") or a plain date ("YYYY-MM-DD") depending on
        // how the row was originally inserted. Fall back to Utc::now() for any missing/invalid value.
        let parse_dt = |s: &str| -> DateTime<Utc> {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .or_else(|_| {
                    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                })
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|_| Utc::now())
        };

        let created_at: DateTime<Utc> = row
            .created_at
            .as_deref()
            .map(parse_dt)
            .unwrap_or_else(Utc::now);

        let updated_at: DateTime<Utc> = row
            .updated_at
            .as_deref()
            .map(parse_dt)
            .unwrap_or_else(Utc::now);

        Ok(MaintenanceCard {
            id,
            owned_rolling_stock_id: owned,
            last_maintenance_date: last,
            next_maintenance_date: next,
            metadata: Metadata {
                version: row.version as u8,
                created_at,
                updated_at,
            },
            pending_events: Vec::new(),
            events: Vec::new(),
        })
    }
}

impl TryFrom<MaintenanceEventRow> for MaintenanceEvent {
    type Error = String;

    fn try_from(row: MaintenanceEventRow) -> Result<Self, Self::Error> {
        // Extract UUID from TRN strings (newtype -> string -> uuid).
        let id_trn = row.id.to_string();
        let id_str = id_trn
            .rsplit_once(':')
            .map(|(_, s)| s)
            .unwrap_or(id_trn.as_str());
        let id = uuid::Uuid::parse_str(id_str)
            .map_err(|e| format!("Invalid maintenance event id: {}", e))?;

        let card_trn = row.maintenance_card_id.to_string();
        let card_id_str = card_trn
            .rsplit_once(':')
            .map(|(_, s)| s)
            .unwrap_or(card_trn.as_str());
        let card_id = uuid::Uuid::parse_str(card_id_str)
            .map_err(|e| format!("Invalid maintenance card id: {}", e))?;

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
