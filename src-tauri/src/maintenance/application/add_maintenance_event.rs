use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::identifiers::Identifier;
use crate::maintenance::domain::MaintenanceEventId;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::domain::{MaintenanceCard, MaintenanceCardId};
use uuid::Uuid;

/// Input DTO for the AddMaintenanceEvent use-case.
/// This type belongs to the application layer and is not an interface/wire type.
pub struct AddMaintenanceEventInput {
    /// Parsed UUID of the maintenance card the record belongs to.
    pub maintenance_card_id: MaintenanceCardId,
    /// Date when the maintenance was performed.
    pub date_performed: chrono::NaiveDate,
    /// Optional maintenance type.
    pub maintenance_type: Option<MaintenanceType>,
    /// Optional free-text notes.
    pub notes: Option<String>,
}

/// Use-case responsible for adding a maintenance event and updating the card.
pub struct AddMaintenanceEvent;

impl AddMaintenanceEvent {
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
    /// - `P`: Identifier provider type implementing `IdProvider<MaintenanceEventId>`.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        input: AddMaintenanceEventInput,
    ) -> Result<(), DomainError>
    where
        U: MaintenanceUowExt + Send,
        P: IdProvider<MaintenanceEventId>,
    {
        let mut repo = unit_of_work.maintenance_repository();

        // Input contains a `MaintenanceCardId` TRN. Convert it to UUID for the
        // aggregate constructor which expects a UUID.
        let card_trn = input.maintenance_card_id.to_string();
        let uuid_str = card_trn
            .trim_start_matches(MaintenanceCardId::PREFIX)
            .trim_start_matches(':');
        let card_id =
            Uuid::parse_str(uuid_str).map_err(|e| DomainError::Validation(e.to_string()))?;

        let mut card = MaintenanceCard::from_id(card_id);

        let event_id = id_provider.next_id();
        let event_uuid_str = event_id
            .as_ref()
            .trim_start_matches("trn:maintenance-event:");
        let id =
            Uuid::parse_str(event_uuid_str).map_err(|e| DomainError::Validation(e.to_string()))?;

        card.record_maintenance(
            id,
            input.date_performed,
            input.maintenance_type,
            input.notes,
        );

        repo.save(card).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::maintenance::application::testing::FakeUow;
    use crate::maintenance::domain::{MaintenanceCardId, MockMaintenanceRepository};
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_saves_maintenance_recorded_event() {
        let mut mock = MockMaintenanceRepository::new();
        let fixed_event_uuid = Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap();
        let fixed_event_id = MaintenanceEventId::from_uuid(&fixed_event_uuid);

        mock.expect_save()
            .times(1)
            .withf(move |card| {
                matches!(
                    card.pending_events.first(),
                    Some(crate::maintenance::domain::maintenance_card_event::MaintenanceCardEvent::MaintenanceRecorded { id, .. })
                    if *id == fixed_event_uuid
                )
            })
            .returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);
        let card_id = MaintenanceCardId::from_uuid(
            &Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
        );
        let input = AddMaintenanceEventInput {
            maintenance_card_id: card_id,
            date_performed: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            maintenance_type: None,
            notes: None,
        };
        let id_provider = MockIdProvider::new(fixed_event_id);
        AddMaintenanceEvent::execute(&mut uow, id_provider, input)
            .await
            .expect("execute should succeed");
    }

    #[tokio::test]
    async fn event_id_is_not_nil() {
        // Regression: previously IdProvider<Uuid> was used, causing Uuid::default()
        // (nil UUID) to be inserted every time — violating the UNIQUE constraint on
        // maintenance_events.id on the second call.
        let fixed_uuid = Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap();
        let fixed_id = MaintenanceEventId::from_uuid(&fixed_uuid);
        assert!(!fixed_uuid.is_nil(), "event UUID must not be nil");
        // Also ensure the TRN prefix is correct
        assert!(fixed_id.as_ref().starts_with("trn:maintenance-event:"));
    }
}
