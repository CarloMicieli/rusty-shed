use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::WithDomainContext;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::maintenance::domain::events::MaintenanceEvent;
use crate::maintenance::domain::{MaintenanceRepository, MaintenanceUowExt};
use crate::maintenance::infrastructure::entities::{MaintenanceCardRow, MaintenanceEventRow};
use async_trait::async_trait;
use sqlx::SqliteConnection;

// Note: Repository now persists domain events (event-driven). The
// application layer should produce `MaintenanceEvent` values which are
// persisted here and then applied to the maintenance_cards projection.

/// SQLite-specific repository implementation.
pub struct SqliteMaintenanceRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteMaintenanceRepository<'conn> {
    /// Create a new repository bound to the given executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl<'conn> MaintenanceRepository for SqliteMaintenanceRepository<'conn> {
    async fn get_card_by_stock_id(
        &mut self,
        owned_rolling_stock_id: &str,
    ) -> Result<Option<MaintenanceCardRow>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at
        FROM maintenance_cards
        WHERE owned_rolling_stock_id = ?"#;
        let row = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .bind(owned_rolling_stock_id)
            .fetch_optional(&mut *self.executor)
            .await
            .with_domain_context("Error fetching maintenance card by stock id")?;
        Ok(row)
    }

    // Legacy single-event shim removed. Use `record_events_transaction`.

    async fn record_events_transaction(
        &mut self,
        events: Vec<MaintenanceEvent>,
    ) -> Result<(), DomainError> {
        let insert_sql = r#"INSERT INTO maintenance_events (
            id,
            maintenance_card_id,
            date_performed,
            maintenance_type,
            notes
        ) VALUES (?, ?, ?, ?, ?)"#;

        let update_sql = r#"UPDATE maintenance_cards
            SET
                last_maintenance_date = ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?"#;

        for ev in events.iter() {
            match ev {
                MaintenanceEvent::MaintenanceRecorded {
                    id,
                    maintenance_card_id,
                    date_performed,
                    maintenance_type,
                    notes,
                } => {
                    sqlx::query(insert_sql)
                        .bind(id.to_string())
                        .bind(maintenance_card_id.to_string())
                        .bind(date_performed.format("%Y-%m-%d").to_string())
                        .bind(maintenance_type.as_ref().map(|t| t.to_string()))
                        .bind(notes.clone())
                        .execute(&mut *self.executor)
                        .await
                        .with_domain_context("Error inserting new maintenance event")?;

                    sqlx::query(update_sql)
                        .bind(date_performed.format("%Y-%m-%d").to_string())
                        .bind(maintenance_card_id.to_string())
                        .execute(&mut *self.executor)
                        .await
                        .with_domain_context(
                            "Error updating maintenance card last_maintenance_date",
                        )?;
                }
            }
        }

        Ok(())
    }

    async fn list_events_for_card(
        &mut self,
        maintenance_card_id: &str,
    ) -> Result<Vec<MaintenanceEventRow>, DomainError> {
        let q = r#"SELECT
            id,
            maintenance_card_id,
            date_performed,
            notes,
            maintenance_type
        FROM maintenance_events
        WHERE maintenance_card_id = ?
        ORDER BY date_performed DESC"#;

        let rows = sqlx::query_as::<_, MaintenanceEventRow>(q)
            .bind(maintenance_card_id)
            .fetch_all(&mut *self.executor)
            .await
            .with_domain_context("Error listing maintenance events for card")?;

        Ok(rows)
    }

    async fn list_due_cards(&mut self) -> Result<Vec<MaintenanceCardRow>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at
        FROM maintenance_cards
        WHERE next_maintenance_date <= date('now')
           OR (
               next_maintenance_date IS NULL
               AND last_maintenance_date IS NOT NULL
               AND last_maintenance_date <= date('now')
           )"#;

        let rows = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .fetch_all(&mut *self.executor)
            .await
            .with_domain_context("Error querying due maintenance cards")?;

        Ok(rows)
    }
}

impl<'conn> MaintenanceUowExt for SqliteUnitOfWork<'conn> {
    fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_> {
        Box::new(SqliteMaintenanceRepository::new(&mut self.tx))
            as Box<dyn MaintenanceRepository + Send + '_>
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::maintenance::infrastructure::entities::MaintenanceCardRow;
    use crate::maintenance::infrastructure::repository::MaintenanceUowExt;

    use crate::maintenance::domain::events::MaintenanceEvent;

    #[ignore]
    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_get_card_by_stock_id_found(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();

        let maybe = repo
            .get_card_by_stock_id("d3606635-4c4e-462b-ae9f-02c7ce47bc770")
            .await
            .expect("repo get")
            .expect("expected card");

        assert_eq!(maybe.id.to_string(), "11111111-1111-1111-1111-111111111111");
        assert_eq!(
            maybe.last_maintenance_date,
            Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        );
    }

    #[ignore]
    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_list_events_order(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();

        let events = repo
            .list_events_for_card("11111111-1111-1111-1111-111111111111")
            .await
            .expect("list events");

        assert!(events.len() >= 2);
        // First event should be the most recent (2025-03-01)
        assert_eq!(
            events[0].date_performed,
            NaiveDate::from_ymd_opt(2025, 3, 1).unwrap()
        );
    }

    #[ignore]
    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_list_due_cards(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();

        let due = repo.list_due_cards().await.expect("list due");
        // Given current date in test environment (2025-12-28), the fixture with next_maintenance_date 2025-07-01 should be due
        assert!(due.iter().any(
            |c: &MaintenanceCardRow| c.id.to_string() == "11111111-1111-1111-1111-111111111111"
        ));
    }

    #[ignore]
    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_record_event_transaction_via_repo(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();

        let new_event = MaintenanceEvent::MaintenanceRecorded {
            id: Uuid::parse_str("66666666-6666-6666-6666-666666666666").unwrap(),
            maintenance_card_id: Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
            date_performed: NaiveDate::from_ymd_opt(2025, 12, 20).unwrap(),
            maintenance_type: Some("INSPECTION".parse().unwrap_or_default()),
            notes: Some("Repo-level transaction test".to_string()),
        };

        // perform the transactional operation via the repository
        repo.record_events_transaction(vec![new_event.clone()])
            .await
            .expect("record event");

        // Extract inner fields from the enum variant for assertions
        let (evt_id, evt_card_id, evt_date) = match &new_event {
            MaintenanceEvent::MaintenanceRecorded { id, maintenance_card_id, date_performed, .. } => (
                *id,
                maintenance_card_id.to_string(),
                *date_performed,
            ),
        };

        // events visible on same transactional repo
        let events = repo
            .list_events_for_card(&evt_card_id)
            .await
            .expect("list events");

        assert!(events.iter().any(|e| e.id == evt_id));

        // card last_maintenance_date updated in the same transaction
        let card = repo
            .get_card_by_stock_id(&evt_card_id)
            .await
            .expect("get card")
            .expect("card exists");

        assert_eq!(card.last_maintenance_date.expect("date"), evt_date);
    }
}
