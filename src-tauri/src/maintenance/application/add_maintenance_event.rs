use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceEventId;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::domain::MaintenanceUowExt;
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

    /// Optional scheduled date for the next maintenance event.
    pub next_maintenance_date: Option<chrono::NaiveDate>,
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
        let mut card = repo
            .find_by_id(&input.maintenance_card_id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "MaintenanceCard".to_string(),
                identifier: input.maintenance_card_id.to_string(),
            })?;

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

        if let Some(next_maintenance_date) = input.next_maintenance_date {
            card.schedule_next_maintenance(next_maintenance_date);
        }

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
        let card_uuid = Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap();
        let card_id = MaintenanceCardId::from_uuid(&card_uuid);

        mock.expect_find_by_id()
            .times(1)
            .withf({
                let card_id = card_id.clone();
                move |id| id == &card_id
            })
            .returning(move |_| {
                Ok(Some(crate::maintenance::domain::MaintenanceCard::from_id(
                    card_uuid,
                )))
            });

        mock.expect_save()
            .times(1)
            .withf(move |card| {
                matches!(
                    card.pending_events.first(),
                    Some(crate::maintenance::domain::maintenance_card_event::MaintenanceCardEvent::MaintenanceRecorded { id, .. })
                    if *id == fixed_event_uuid
                )
                    && card.next_maintenance_date
                        == Some(NaiveDate::from_ymd_opt(2025, 7, 1).unwrap())
            })
            .returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);
        let input = AddMaintenanceEventInput {
            maintenance_card_id: card_id,
            date_performed: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            maintenance_type: None,
            notes: None,
            next_maintenance_date: Some(NaiveDate::from_ymd_opt(2025, 7, 1).unwrap()),
        };
        let id_provider = MockIdProvider::new(fixed_event_id);
        AddMaintenanceEvent::execute(&mut uow, id_provider, input)
            .await
            .expect("execute should succeed");
    }

    #[tokio::test]
    async fn it_returns_not_found_when_card_is_missing() {
        let mut mock = MockMaintenanceRepository::new();
        let card_id = MaintenanceCardId::from_uuid(
            &Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
        );

        mock.expect_find_by_id().times(1).returning(|_| Ok(None));

        let mut uow = FakeUow::new(mock);
        let input = AddMaintenanceEventInput {
            maintenance_card_id: card_id.clone(),
            date_performed: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            maintenance_type: None,
            notes: None,
            next_maintenance_date: None,
        };
        let id_provider = MockIdProvider::new(MaintenanceEventId::from_uuid(
            &Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        ));

        let result = AddMaintenanceEvent::execute(&mut uow, id_provider, input).await;

        assert!(matches!(
            result,
            Err(DomainError::NotFound { resource, identifier })
                if resource == "MaintenanceCard" && identifier == card_id.to_string()
        ));
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
