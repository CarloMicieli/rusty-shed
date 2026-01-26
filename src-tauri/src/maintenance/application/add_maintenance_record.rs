use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceCard;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::interface::AddMaintenanceRecordArgs;
use uuid::Uuid;

/// Use-case responsible for adding a maintenance record and updating the card.
pub struct AddMaintenanceRecord;

impl AddMaintenanceRecord {
    /// Execute the use-case within the provided Unit of Work using a typed input.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the maintenance repository.
    /// - `id_provider`: The identifier provider for generating new IDs.
    /// - `input`: The input data required to add a maintenance record.
    ///
    /// # Returns
    /// - `Ok(())` if the operation was successful.
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `MaintenanceUowExt` and `Send`.
    /// - `P`: Identifier provider type implementing `IdProvider<Uuid>`.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: AddMaintenanceRecordArgs,
    ) -> Result<(), DomainError>
    where
        U: MaintenanceUowExt + Send,
        P: IdProvider<Uuid>,
    {
        let mut repo = unit_of_work.maintenance_repository();

        let card_id = Uuid::parse_str(&input.maintenance_card_id)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let mut card = MaintenanceCard::from_id(card_id);

        // Generate a new id for the maintenance record using the provided IdProvider.
        let id = id_provider.next_id();

        card.record_maintenance(
            id,
            input.date_performed,
            input.maintenance_type,
            input.notes.clone(),
        );

        // pass the card (containing pending events) to the repository to persist
        repo.save(card).await?;

        Ok(())
    }
}
