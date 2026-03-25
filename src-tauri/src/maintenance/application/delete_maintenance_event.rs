use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceEventId;
use crate::maintenance::domain::MaintenanceUowExt;

/// Use-case responsible for deleting a single maintenance event.
pub struct DeleteMaintenanceEvent;

impl DeleteMaintenanceEvent {
    /// Remove the maintenance event identified by `event_id` and keep the
    /// owning card's projection consistent.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        event_id: MaintenanceEventId,
    ) -> Result<(), DomainError>
    where
        U: MaintenanceUowExt + Send,
    {
        let mut repo = unit_of_work.maintenance_repository();
        repo.delete_event(&event_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maintenance::application::testing::FakeUow;
    use crate::maintenance::domain::{MaintenanceEventId, MockMaintenanceRepository};
    use uuid::Uuid;

    #[tokio::test]
    async fn it_calls_delete_event_with_correct_id() {
        let mut mock = MockMaintenanceRepository::new();
        let event_uuid = Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap();
        let event_id = MaintenanceEventId::from_uuid(&event_uuid);
        let expected_id = event_id.clone();

        mock.expect_delete_event()
            .times(1)
            .withf(move |id| id == &expected_id)
            .returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);
        DeleteMaintenanceEvent::execute(&mut uow, event_id)
            .await
            .expect("execute should succeed");
    }
}
