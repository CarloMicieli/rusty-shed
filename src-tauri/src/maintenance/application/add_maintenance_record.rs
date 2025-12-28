use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::maintenance::domain::maintenance_type::MaintenanceType;
use crate::maintenance::infrastructure::repository::{MaintenanceUowExt, NewMaintenanceEvent};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Input for the `AddMaintenanceRecordUseCase`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AddMaintenanceRecordInput {
    /// Unique identifier for the new maintenance event.
    pub id: Uuid,

    /// The maintenance card this event belongs to.
    pub maintenance_card_id: Uuid,

    /// Date the maintenance was performed (date-only).
    pub date_performed: NaiveDate,

    /// Optional maintenance type.
    pub maintenance_type: Option<MaintenanceType>,

    /// Optional free-text notes.
    pub notes: Option<String>,
}

/// Use-case responsible for adding a maintenance record and updating the card.
pub struct AddMaintenanceRecordUseCase {}

impl AddMaintenanceRecordUseCase {
    /// Create a new instance of the use-case.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    /// Execute the use-case within the provided Unit of Work using a typed input.
    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        input: AddMaintenanceRecordInput,
    ) -> Result<(), String> {
        let mut repo = uow.maintenance_repo();

        let event = NewMaintenanceEvent {
            id: input.id,
            maintenance_card_id: input.maintenance_card_id,
            date_performed: input.date_performed,
            maintenance_type: input.maintenance_type.map(|t| t.to_string()),
            notes: input.notes,
        };

        repo.record_event_transaction(event)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
