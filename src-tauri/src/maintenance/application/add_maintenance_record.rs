use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::domain::maintenance_type::MaintenanceType;
use crate::maintenance::infrastructure::repository::NewMaintenanceEvent;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Use-case responsible for adding a maintenance record and updating the card.
pub struct AddMaintenanceRecordUseCase {}

impl AddMaintenanceRecordUseCase {
    /// Execute the use-case within the provided Unit of Work using a typed input.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the maintenance repository.
    /// - `input`: The input data required to add a maintenance record.
    ///
    /// # Returns
    /// - `Ok(())` if the operation was successful.
    /// - `Err(DomainError)` if an error occurred during the operation.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: AddMaintenanceRecordInput,
    ) -> Result<(), DomainError>
    where
        U: MaintenanceUowExt + Send,
    {
        let mut repo = unit_of_work.maintenance_repository();

        let event = NewMaintenanceEvent {
            id: input.id,
            maintenance_card_id: input.maintenance_card_id,
            date_performed: input.date_performed,
            maintenance_type: input.maintenance_type.map(|t| t.to_string()),
            notes: input.notes,
        };

        repo.record_event_transaction(event).await?;

        Ok(())
    }
}

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
