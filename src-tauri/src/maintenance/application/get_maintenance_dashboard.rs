use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::MaintenanceCard;
use crate::maintenance::domain::MaintenanceUowExt;

/// Use-case to retrieve maintenance cards that are due or overdue.
pub struct GetMaintenanceDashboard;

impl GetMaintenanceDashboard {
    /// Execute the use-case using the provided Unit of Work.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the maintenance repository.
    ///
    /// # Returns
    /// - `Ok(Vec<MaintenanceCard>)` containing due or overdue maintenance cards.
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `MaintenanceUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<MaintenanceCard>, DomainError>
    where
        U: MaintenanceUowExt + Send,
    {
        let mut repo = unit_of_work.maintenance_repository();

        // Repository now returns domain `MaintenanceCard` values directly.
        let maintenance_cards = repo.list_due_cards().await?;
        Ok(maintenance_cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::maintenance::application::testing::FakeUow;
    use crate::maintenance::domain::MaintenanceCard;
    use crate::maintenance::domain::MaintenanceCardId;
    use crate::maintenance::domain::MockMaintenanceRepository;
    use crate::maintenance::infrastructure::entities::MaintenanceCardRow;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_returns_empty_when_no_due_cards() {
        let mut mock = MockMaintenanceRepository::new();
        mock.expect_list_due_cards()
            .times(1)
            .returning(|| Ok(Vec::<MaintenanceCard>::new()));

        let mut unit_of_work = FakeUow::new(mock);

        let cards = GetMaintenanceDashboard::execute(&mut unit_of_work)
            .await
            .expect("execute use-case");
        assert!(cards.is_empty());
    }

    #[tokio::test]
    async fn it_maps_rows_to_domain_cards() {
        let test_card_uuid = Uuid::new_v4();
        let test_stock_uuid = Uuid::new_v4();

        let row = MaintenanceCardRow {
            id: MaintenanceCardId::from_uuid(&test_card_uuid),
            owned_rolling_stock_id: OwnedRollingStockId::from(test_stock_uuid),
            last_maintenance_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            next_maintenance_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            created_at: None,
            updated_at: None,
            version: 0,
        };

        let mut mock = MockMaintenanceRepository::new();
        let card_expected = MaintenanceCard::try_from(row.clone()).expect("map row");
        mock.expect_list_due_cards()
            .times(1)
            .returning(move || Ok(vec![card_expected.clone()]));

        let mut unit_of_work = FakeUow::new(mock);

        let cards = GetMaintenanceDashboard::execute(&mut unit_of_work)
            .await
            .expect("execute use-case");
        assert_eq!(cards.len(), 1);
        let expected_id = MaintenanceCardId::from_uuid(&test_card_uuid);
        assert_eq!(cards[0].id.to_string(), expected_id.to_string());
        let expected_owned = OwnedRollingStockId::from(test_stock_uuid);
        assert_eq!(
            cards[0].owned_rolling_stock_id.to_string(),
            expected_owned.to_string()
        );
    }
}
