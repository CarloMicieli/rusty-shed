use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::identifiers::Identifier;
use crate::core::infrastructure::WithDomainContext;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceEvent;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::domain::maintenance_card_event::MaintenanceCardEvent;
use crate::maintenance::domain::{MaintenanceCard, MaintenanceEventId};
use crate::maintenance::domain::{MaintenanceRepository, MaintenanceUowExt};
use crate::maintenance::infrastructure::entities::{MaintenanceCardRow, MaintenanceEventRow};
use crate::maintenance::interface::{MaintenanceCardEventView, MaintenanceCardView};
use async_trait::async_trait;
use sqlx::SqliteConnection;

/// SQLite implementation of the MaintenanceRepository.
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
    async fn find_by_rolling_stock_id(
        &mut self,
        owned_rolling_stock_id: &OwnedRollingStockId,
    ) -> Result<Option<MaintenanceCard>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at,
            version
        FROM maintenance_cards
        WHERE owned_rolling_stock_id = ?"#;
        // OwnedRollingStockId is stored as a TRN string in the database
        let trn = owned_rolling_stock_id.to_string();

        let row = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .bind(trn)
            .fetch_optional(&mut *self.executor)
            .await
            .with_domain_context("Error fetching maintenance card by stock id")?;

        // Map infra row into domain model when present and load persisted events.
        let maybe_card = match row {
            Some(r) => {
                let card_trn = r.id.clone(); // Clone the TRN for later use
                let mut card = MaintenanceCard::try_from(r).map_err(DomainError::Validation)?;

                // Query persisted events for this card id (TRN)
                let events_q = r#"SELECT
                    id,
                    maintenance_card_id,
                    date_performed,
                    notes,
                    maintenance_type
                FROM maintenance_events
                WHERE maintenance_card_id = ?
                ORDER BY date_performed DESC"#;

                let rows = sqlx::query_as::<_, MaintenanceEventRow>(events_q)
                    .bind(card_trn)
                    .fetch_all(&mut *self.executor)
                    .await
                    .with_domain_context("Error listing maintenance events for card")?;

                // Map infra event rows into domain events
                let mut domain_events = Vec::with_capacity(rows.len());
                for er in rows.into_iter() {
                    domain_events
                        .push(MaintenanceEvent::try_from(er).map_err(DomainError::Validation)?);
                }

                card.events = domain_events;
                Some(card)
            }
            None => None,
        };

        Ok(maybe_card)
    }

    async fn find_by_id(
        &mut self,
        id: &MaintenanceCardId,
    ) -> Result<Option<MaintenanceCard>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at,
            version
        FROM maintenance_cards
        WHERE id = ?"#;

        // MaintenanceCardId is stored as a TRN string in the database
        let trn = id.to_string();

        let row = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .bind(trn)
            .fetch_optional(&mut *self.executor)
            .await
            .with_domain_context("Error fetching maintenance card by id")?;

        let maybe_card = match row {
            Some(r) => {
                let card_trn = r.id.clone(); // Clone the TRN for later use
                let mut card = MaintenanceCard::try_from(r).map_err(DomainError::Validation)?;

                // Load persisted events for this card.
                let events_q = r#"SELECT
                    id,
                    maintenance_card_id,
                    date_performed,
                    notes,
                    maintenance_type
                FROM maintenance_events
                WHERE maintenance_card_id = ?
                ORDER BY date_performed DESC"#;

                let rows = sqlx::query_as::<_, MaintenanceEventRow>(events_q)
                    .bind(card_trn)
                    .fetch_all(&mut *self.executor)
                    .await
                    .with_domain_context("Error listing maintenance events for card")?;

                let mut domain_events = Vec::with_capacity(rows.len());
                for er in rows.into_iter() {
                    domain_events
                        .push(MaintenanceEvent::try_from(er).map_err(DomainError::Validation)?);
                }

                card.events = domain_events;
                Some(card)
            }
            None => None,
        };

        Ok(maybe_card)
    }

    // Persist changes for a maintenance card by consuming its pending events.

    async fn find_view_by_id(
        &mut self,
        id: &MaintenanceCardId,
    ) -> Result<Option<MaintenanceCardView>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at,
            version
        FROM maintenance_cards
        WHERE id = ?"#;

        let trn = id.to_string();

        let row = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .bind(trn)
            .fetch_optional(&mut *self.executor)
            .await
            .with_domain_context("Error fetching maintenance card by id for view")?;

        let maybe = match row {
            Some(r) => {
                // load events for view
                let events_q = r#"SELECT
                    id,
                    maintenance_card_id,
                    date_performed,
                    notes,
                    maintenance_type
                FROM maintenance_events
                WHERE maintenance_card_id = ?
                ORDER BY date_performed DESC"#;

                let rows = sqlx::query_as::<_, MaintenanceEventRow>(events_q)
                    .bind(r.id.to_string())
                    .fetch_all(&mut *self.executor)
                    .await
                    .with_domain_context("Error listing maintenance events for view")?;

                let mut events = Vec::with_capacity(rows.len());
                for er in rows.into_iter() {
                    // parse event id uuid from TRN
                    let id_trn = er.id.to_string();
                    let uuid_str = id_trn.trim_start_matches(MaintenanceEventId::PREFIX);
                    let uuid_str = uuid_str.trim_start_matches(':');
                    let evt_uuid = uuid::Uuid::parse_str(uuid_str)
                        .map_err(|_| DomainError::Validation("invalid event id".to_string()))?;

                    let maintenance_type = er
                        .maintenance_type
                        .as_ref()
                        .and_then(|s| s.parse::<MaintenanceType>().ok());

                    events.push(MaintenanceCardEventView {
                        id: evt_uuid,
                        date_performed: er.date_performed,
                        maintenance_type,
                        notes: er.notes,
                    });
                }

                Some(MaintenanceCardView {
                    id: r.id,
                    owned_rolling_stock_id: r.owned_rolling_stock_id,
                    last_maintenance_date: r.last_maintenance_date,
                    next_maintenance_date: r.next_maintenance_date,
                    events,
                })
            }
            None => None,
        };

        Ok(maybe)
    }

    async fn save(&mut self, maintenance_card: MaintenanceCard) -> Result<(), DomainError> {
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

        let owned_rolling_stock_id = maintenance_card.owned_rolling_stock_id.to_string();

        for ev in maintenance_card.pending_events.into_iter() {
            match ev {
                MaintenanceCardEvent::MaintenanceRecorded {
                    id,
                    maintenance_card_id,
                    date_performed,
                    maintenance_type,
                    notes,
                } => {
                    // Build TRN strings for database storage
                    let event_trn = format!("trn:maintenance-event:{}", id);
                    let card_trn = format!("trn:maintenance-card:{}", maintenance_card_id);

                    sqlx::query(insert_sql)
                        .bind(event_trn)
                        .bind(&card_trn)
                        .bind(date_performed.format("%Y-%m-%d").to_string())
                        .bind(maintenance_type.as_ref().map(|t| t.to_string()))
                        .bind(notes.clone())
                        .execute(&mut *self.executor)
                        .await
                        .with_domain_context("Error inserting new maintenance event")?;

                    sqlx::query(update_sql)
                        .bind(date_performed.format("%Y-%m-%d").to_string())
                        .bind(&card_trn)
                        .execute(&mut *self.executor)
                        .await
                        .with_domain_context(
                            "Error updating maintenance card last_maintenance_date",
                        )?;
                }
                MaintenanceCardEvent::Created {
                    id,
                    maintenance_card_id,
                    created_at,
                } => {
                    let event_trn = format!("trn:maintenance-event:{}", id);
                    let card_trn = format!("trn:maintenance-card:{}", maintenance_card_id);

                    // Insert the maintenance card row first (required by the FK constraint
                    // on maintenance_events.maintenance_card_id → maintenance_cards.id)
                    let insert_card_sql = r#"INSERT INTO maintenance_cards (
                        id,
                        owned_rolling_stock_id,
                        created_at,
                        updated_at,
                        version
                    ) VALUES (?, ?, ?, ?, 0)"#;

                    let now_dt = chrono::Local::now()
                        .naive_local()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();

                    sqlx::query(insert_card_sql)
                        .bind(&card_trn)
                        .bind(&owned_rolling_stock_id)
                        .bind(&now_dt)
                        .bind(&now_dt)
                        .execute(&mut *self.executor)
                        .await
                        .with_domain_context("Error inserting maintenance card row")?;

                    sqlx::query(insert_sql)
                        .bind(event_trn)
                        .bind(&card_trn)
                        .bind(created_at.format("%Y-%m-%d").to_string())
                        .bind(None::<String>)
                        .bind(None::<String>)
                        .execute(&mut *self.executor)
                        .await
                        .with_domain_context("Error inserting maintenance created event")?;
                }
            }
        }

        Ok(())
    }

    async fn list_due_cards(&mut self) -> Result<Vec<MaintenanceCard>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at,
            version
        FROM maintenance_cards
        WHERE next_maintenance_date <= date('now')
           OR (
               next_maintenance_date IS NULL
               AND last_maintenance_date IS NOT NULL
               AND last_maintenance_date <= date('now')
           )
           OR (
               next_maintenance_date IS NULL
               AND last_maintenance_date IS NULL
           )"#;

        let rows = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .fetch_all(&mut *self.executor)
            .await
            .with_domain_context("Error querying due maintenance cards")?;

        // Map infra rows into domain `MaintenanceCard`
        let mut cards: Vec<MaintenanceCard> = Vec::with_capacity(rows.len());
        for r in rows.into_iter() {
            let card = MaintenanceCard::try_from(r).map_err(DomainError::Validation)?;
            cards.push(card);
        }

        Ok(cards)
    }

    async fn list_due_card_views(
        &mut self,
    ) -> Result<Vec<crate::maintenance::interface::MaintenanceCardView>, DomainError> {
        let q = r#"SELECT
            id,
            owned_rolling_stock_id,
            last_maintenance_date,
            next_maintenance_date,
            created_at,
            updated_at,
            version
        FROM maintenance_cards
        WHERE next_maintenance_date <= date('now')
           OR (
               next_maintenance_date IS NULL
               AND last_maintenance_date IS NOT NULL
               AND last_maintenance_date <= date('now')
           )
           OR (
               next_maintenance_date IS NULL
               AND last_maintenance_date IS NULL
           )"#;

        let rows = sqlx::query_as::<_, MaintenanceCardRow>(q)
            .fetch_all(&mut *self.executor)
            .await
            .with_domain_context("Error querying due maintenance cards for view")?;

        let mut views = Vec::with_capacity(rows.len());
        for r in rows.into_iter() {
            // load events for each card
            let events_q = r#"SELECT
                id,
                maintenance_card_id,
                date_performed,
                notes,
                maintenance_type
            FROM maintenance_events
            WHERE maintenance_card_id = ?
            ORDER BY date_performed DESC"#;

            let rows_ev = sqlx::query_as::<_, MaintenanceEventRow>(events_q)
                .bind(r.id.to_string())
                .fetch_all(&mut *self.executor)
                .await
                .with_domain_context("Error listing maintenance events for view")?;

            let mut events = Vec::with_capacity(rows_ev.len());
            for er in rows_ev.into_iter() {
                let id_trn = er.id.to_string();
                let uuid_str = id_trn
                    .trim_start_matches(crate::maintenance::domain::MaintenanceEventId::PREFIX);
                let uuid_str = uuid_str.trim_start_matches(':');
                let evt_uuid = uuid::Uuid::parse_str(uuid_str)
                    .map_err(|_| DomainError::Validation("invalid event id".to_string()))?;

                let maintenance_type = er
                    .maintenance_type
                    .as_ref()
                    .and_then(|s| s.parse::<MaintenanceType>().ok());

                events.push(MaintenanceCardEventView {
                    id: evt_uuid,
                    date_performed: er.date_performed,
                    maintenance_type,
                    notes: er.notes,
                });
            }

            views.push(MaintenanceCardView {
                id: r.id,
                owned_rolling_stock_id: r.owned_rolling_stock_id,
                last_maintenance_date: r.last_maintenance_date,
                next_maintenance_date: r.next_maintenance_date,
                events,
            });
        }

        Ok(views)
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

    use crate::collecting::domain::OwnedRollingStockId;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::maintenance::domain::MaintenanceCard;
    use crate::maintenance::domain::MaintenanceCardId;
    use crate::maintenance::domain::MaintenanceUowExt;

    use crate::maintenance::domain::maintenance_card_event::MaintenanceCardEvent;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_get_card_by_stock_id_found(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        let id = OwnedRollingStockId::try_from(
            "trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-02c7ce47bc70",
        )
        .expect("should parse owned rolling stock id");

        let maybe = repo.find_by_rolling_stock_id(&id).await.expect("repo get");

        assert!(maybe.is_some());

        let maybe = maybe.expect("card exists");

        assert_eq!(
            maybe.id.to_string(),
            "trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2"
        );
        assert_eq!(
            maybe.last_maintenance_date,
            Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_list_events_order(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        let card = repo
            .find_by_rolling_stock_id(
                &OwnedRollingStockId::try_from(
                    "trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-02c7ce47bc70",
                )
                .expect("should parse owned rolling stock id"),
            )
            .await
            .expect("get card")
            .expect("card exists");

        assert!(card.events.len() >= 2);
        // First event should be the most recent (2025-03-01)
        assert_eq!(
            card.events[0].date_performed,
            NaiveDate::from_ymd_opt(2025, 3, 1).unwrap()
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_list_due_cards(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        let due = repo.list_due_cards().await.expect("list due");
        // Fixture has next_maintenance_date = 2025-07-01, which is overdue as of 2026-01-26
        assert!(due.iter().any(|c: &MaintenanceCard| {
            c.id.to_string()
                .contains("3284cc76-1472-4b12-a7d4-62043416adc2")
        }));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_record_event_transaction_via_repo(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        // Use the existing maintenance card from fixtures
        let existing_card_uuid = Uuid::parse_str("3284cc76-1472-4b12-a7d4-62043416adc2").unwrap();

        let new_event = MaintenanceCardEvent::MaintenanceRecorded {
            id: Uuid::parse_str("66666666-6666-6666-6666-666666666666").unwrap(),
            maintenance_card_id: existing_card_uuid,
            date_performed: NaiveDate::from_ymd_opt(2025, 12, 20).unwrap(),
            maintenance_type: Some("INSPECTION".parse().unwrap_or_default()),
            notes: Some("Repo-level transaction test".to_string()),
        };

        // perform the transactional operation via the repository
        let card_id = match &new_event {
            MaintenanceCardEvent::MaintenanceRecorded {
                maintenance_card_id,
                ..
            } => *maintenance_card_id,
            MaintenanceCardEvent::Created {
                maintenance_card_id,
                ..
            } => *maintenance_card_id,
        };
        let mut card = MaintenanceCard::from_id(card_id);
        card.pending_events = vec![new_event.clone()];
        repo.save(card).await.expect("record event");

        // Extract inner fields from the enum variant for assertions
        let (evt_id, evt_card_id, evt_date) = match &new_event {
            MaintenanceCardEvent::MaintenanceRecorded {
                id,
                maintenance_card_id,
                date_performed,
                ..
            } => (*id, *maintenance_card_id, *date_performed),
            MaintenanceCardEvent::Created {
                id,
                maintenance_card_id,
                created_at,
            } => (*id, *maintenance_card_id, *created_at),
        };

        // Query the card by ID to verify the event was recorded
        let card_with_events = repo
            .find_by_id(&MaintenanceCardId::from_uuid(&evt_card_id))
            .await
            .expect("get card")
            .expect("card exists");

        assert!(card_with_events.events.iter().any(|e| e.id == evt_id));

        // card last_maintenance_date updated in the same transaction
        assert_eq!(
            card_with_events.last_maintenance_date.expect("date"),
            evt_date
        );
    }
}
