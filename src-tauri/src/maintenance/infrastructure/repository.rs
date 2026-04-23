use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::identifiers::Identifier;
use crate::core::infrastructure::WithDomainContext;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceEvent;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::domain::maintenance_card_event::MaintenanceCardEvent;
use crate::maintenance::domain::read_models::{
    MaintenanceCardEventView, MaintenanceCardView, RollingStockDisplayInfo,
};
use crate::maintenance::domain::{MaintenanceCard, MaintenanceEventId};
use crate::maintenance::domain::{MaintenanceRepository, MaintenanceUowExt};
use crate::maintenance::infrastructure::database;
use async_trait::async_trait;
use sqlx::SqliteConnection;

/// SQLite implementation of [`MaintenanceRepository`].
pub struct SqliteMaintenanceRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteMaintenanceRepository<'conn> {
    /// Create a new repository bound to the given connection / transaction executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

/// Load all persisted [`MaintenanceEvent`] domain objects for a given card TRN.
///
/// Delegates the query to [`database::load_events_for_card`] and converts each row
/// to a domain event via `TryFrom`.
async fn load_domain_events_for_card(
    executor: &mut SqliteConnection,
    card_trn: &str,
) -> Result<Vec<MaintenanceEvent>, DomainError> {
    let rows = database::load_events_for_card(executor, card_trn)
        .await
        .with_domain_context("Error listing maintenance events for card")?;

    rows.into_iter()
        .map(|er| MaintenanceEvent::try_from(er).map_err(DomainError::Validation))
        .collect()
}

/// Convert a slice of [`MaintenanceEventRow`]s into [`MaintenanceCardEventView`]s.
///
/// Extracted to de-duplicate the identical mapping block used by both
/// [`find_view_by_id`](SqliteMaintenanceRepository::find_view_by_id) and
/// [`list_due_card_views`](SqliteMaintenanceRepository::list_due_card_views).
fn map_event_rows_to_views(
    rows: Vec<crate::maintenance::infrastructure::entities::MaintenanceEventRow>,
) -> Result<Vec<MaintenanceCardEventView>, DomainError> {
    let mut events = Vec::with_capacity(rows.len());

    for er in rows {
        let event_id = MaintenanceEventId::try_from(er.id.as_ref())?;
        let uuid_str = &event_id.as_ref()[MaintenanceEventId::PREFIX.len() + 1..];
        let evt_uuid = uuid::Uuid::parse_str(uuid_str)
            .map_err(|_| DomainError::Infrastructure("invalid event id uuid".to_string()))?;

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

    Ok(events)
}

#[async_trait]
impl<'conn> MaintenanceRepository for SqliteMaintenanceRepository<'conn> {
    /// Find the maintenance card for the given owned rolling stock.
    ///
    /// Loads all persisted events onto the returned card.
    async fn find_by_rolling_stock_id(
        &mut self,
        owned_rolling_stock_id: &OwnedRollingStockId,
    ) -> Result<Option<MaintenanceCard>, DomainError> {
        // OwnedRollingStockId is stored as a TRN string in the database.
        let trn = owned_rolling_stock_id.to_string();

        let row = database::find_maintenance_card_by_stock_id(&mut *self.executor, &trn)
            .await
            .with_domain_context("Error fetching maintenance card by stock id")?;

        let maybe_card = match row {
            Some(r) => {
                let card_trn = r.id.clone();
                let mut card = MaintenanceCard::try_from(r).map_err(DomainError::Validation)?;
                card.events =
                    load_domain_events_for_card(&mut *self.executor, card_trn.as_ref()).await?;
                Some(card)
            }
            None => None,
        };

        Ok(maybe_card)
    }

    /// Find a maintenance card by its own TRN id.
    ///
    /// Loads all persisted events onto the returned card.
    async fn find_by_id(
        &mut self,
        id: &MaintenanceCardId,
    ) -> Result<Option<MaintenanceCard>, DomainError> {
        // MaintenanceCardId is stored as a TRN string in the database.
        let trn = id.to_string();

        let row = database::find_maintenance_card_by_id(&mut *self.executor, &trn)
            .await
            .with_domain_context("Error fetching maintenance card by id")?;

        let maybe_card = match row {
            Some(r) => {
                let card_trn = r.id.clone();
                let mut card = MaintenanceCard::try_from(r).map_err(DomainError::Validation)?;
                card.events =
                    load_domain_events_for_card(&mut *self.executor, card_trn.as_ref()).await?;
                Some(card)
            }
            None => None,
        };

        Ok(maybe_card)
    }

    /// Fetch a rich view of a maintenance card, including catalog display info and event history.
    async fn find_view_by_id(
        &mut self,
        id: &MaintenanceCardId,
    ) -> Result<Option<MaintenanceCardView>, DomainError> {
        let trn = id.to_string();

        let row = database::find_maintenance_card_with_display_by_id(&mut *self.executor, &trn)
            .await
            .with_domain_context("Error fetching maintenance card by id for view")?;

        let maybe = match row {
            Some(r) => {
                let event_rows = database::load_events_for_card(&mut *self.executor, r.id.as_ref())
                    .await
                    .with_domain_context("Error listing maintenance events for view")?;

                let events = map_event_rows_to_views(event_rows)?;

                let display_info = if r.manufacturer_name.is_some()
                    || r.product_code.is_some()
                    || r.series_code.is_some()
                    || r.road_number.is_some()
                {
                    Some(RollingStockDisplayInfo {
                        manufacturer_name: r.manufacturer_name,
                        product_code: r.product_code,
                        series_code: r.series_code,
                        road_number: r.road_number,
                        rolling_stock_category: r.rolling_stock_category,
                    })
                } else {
                    None
                };

                Some(MaintenanceCardView {
                    id: r.id,
                    owned_rolling_stock_id: r.owned_rolling_stock_id,
                    last_maintenance_date: r.last_maintenance_date,
                    next_maintenance_date: r.next_maintenance_date,
                    events,
                    display_info,
                })
            }
            None => None,
        };

        Ok(maybe)
    }

    /// Persist changes for a maintenance card by consuming its pending domain events.
    ///
    /// # Errors
    /// Returns [`DomainError::Conflict`] when a `Created` event attempts to insert a card
    /// for a rolling stock that already has one.
    async fn save(&mut self, maintenance_card: MaintenanceCard) -> Result<(), DomainError> {
        let owned_rolling_stock_trn = maintenance_card.owned_rolling_stock_id.to_string();

        for ev in maintenance_card.pending_events.into_iter() {
            match ev {
                MaintenanceCardEvent::MaintenanceRecorded {
                    id,
                    maintenance_card_id,
                    date_performed,
                    maintenance_type,
                    notes,
                } => {
                    let event_trn = format!("trn:maintenance-event:{}", id);
                    let card_trn = format!("trn:maintenance-card:{}", maintenance_card_id);
                    let date_str = date_performed.format("%Y-%m-%d").to_string();
                    let type_str = maintenance_type.as_ref().map(|t| t.to_string());

                    database::insert_maintenance_event(
                        &mut *self.executor,
                        &event_trn,
                        &card_trn,
                        &date_str,
                        type_str.as_deref(),
                        notes.as_deref(),
                    )
                    .await
                    .with_domain_context("Error inserting new maintenance event")?;

                    database::update_maintenance_card_last_date(
                        &mut *self.executor,
                        &date_str,
                        &card_trn,
                    )
                    .await
                    .with_domain_context("Error updating maintenance card last_maintenance_date")?;

                    let next_date_str = maintenance_card
                        .next_maintenance_date
                        .map(|date| date.format("%Y-%m-%d").to_string());

                    database::update_maintenance_card_next_date(
                        &mut *self.executor,
                        next_date_str.as_deref(),
                        &card_trn,
                    )
                    .await
                    .with_domain_context("Error updating maintenance card next_maintenance_date")?;
                }

                MaintenanceCardEvent::Created {
                    id,
                    maintenance_card_id,
                    created_at,
                } => {
                    let event_trn = format!("trn:maintenance-event:{}", id);
                    let card_trn = format!("trn:maintenance-card:{}", maintenance_card_id);
                    let now_dt = chrono::Local::now()
                        .naive_local()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();

                    // Insert the maintenance card row first (required by the FK constraint
                    // on maintenance_events.maintenance_card_id → maintenance_cards.id).
                    let insert_result = database::insert_maintenance_card(
                        &mut *self.executor,
                        &card_trn,
                        &owned_rolling_stock_trn,
                        &now_dt,
                    )
                    .await;

                    if let Err(e) = insert_result {
                        if e.to_string().contains("UNIQUE constraint failed") {
                            return Err(DomainError::Conflict(
                                "A maintenance card already exists for this rolling stock."
                                    .to_string(),
                            ));
                        }
                        return Err(DomainError::Infrastructure(e.to_string()));
                    }

                    database::insert_maintenance_event(
                        &mut *self.executor,
                        &event_trn,
                        &card_trn,
                        &created_at.format("%Y-%m-%d").to_string(),
                        None,
                        None,
                    )
                    .await
                    .with_domain_context("Error inserting maintenance created event")?;
                }
            }
        }

        Ok(())
    }

    /// Return all maintenance cards that are currently due.
    async fn list_due_cards(&mut self) -> Result<Vec<MaintenanceCard>, DomainError> {
        let rows = database::find_due_maintenance_cards(&mut *self.executor)
            .await
            .with_domain_context("Error querying due maintenance cards")?;

        let mut cards = Vec::with_capacity(rows.len());
        for r in rows {
            let card = MaintenanceCard::try_from(r).map_err(DomainError::Validation)?;
            cards.push(card);
        }

        Ok(cards)
    }

    /// Delete a single maintenance event and recalculate `last_maintenance_date` for its card.
    ///
    /// # Errors
    /// Returns [`DomainError::NotFound`] when no event with `event_id` exists.
    async fn delete_event(&mut self, event_id: &MaintenanceEventId) -> Result<(), DomainError> {
        let event_trn = event_id.to_string();

        // Retrieve the owning card's TRN before deleting so we can update its projection.
        let card_trn = database::find_event_card_id(&mut *self.executor, &event_trn)
            .await
            .with_domain_context("Error finding owning card for maintenance event")?
            .ok_or_else(|| DomainError::NotFound {
                resource: "MaintenanceEvent".to_string(),
                identifier: event_trn.clone(),
            })?;

        database::delete_maintenance_event(&mut *self.executor, &event_trn)
            .await
            .with_domain_context("Error deleting maintenance event")?;

        database::recalculate_last_maintenance_date(&mut *self.executor, &card_trn)
            .await
            .with_domain_context("Error updating maintenance card after event deletion")?;

        Ok(())
    }

    /// Return rich view models for all currently due maintenance cards.
    async fn list_due_card_views(&mut self) -> Result<Vec<MaintenanceCardView>, DomainError> {
        let rows = database::find_due_maintenance_card_views(&mut *self.executor)
            .await
            .with_domain_context("Error querying due maintenance cards for view")?;

        let mut views = Vec::with_capacity(rows.len());

        for r in rows {
            let event_rows = database::load_events_for_card(&mut *self.executor, r.id.as_ref())
                .await
                .with_domain_context("Error listing maintenance events for view")?;

            let events = map_event_rows_to_views(event_rows)?;

            let display_info = if r.manufacturer_name.is_some()
                || r.product_code.is_some()
                || r.series_code.is_some()
                || r.road_number.is_some()
            {
                Some(RollingStockDisplayInfo {
                    manufacturer_name: r.manufacturer_name,
                    product_code: r.product_code,
                    series_code: r.series_code,
                    road_number: r.road_number,
                    rolling_stock_category: r.rolling_stock_category,
                })
            } else {
                None
            };

            views.push(MaintenanceCardView {
                id: r.id,
                owned_rolling_stock_id: r.owned_rolling_stock_id,
                last_maintenance_date: r.last_maintenance_date,
                next_maintenance_date: r.next_maintenance_date,
                events,
                display_info,
            });
        }

        Ok(views)
    }
}

impl MaintenanceUowExt for SqliteUnitOfWork {
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

        // Perform the transactional operation via the repository.
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
        card.schedule_next_maintenance(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap());
        card.pending_events = vec![new_event.clone()];
        repo.save(card).await.expect("record event");

        // Extract inner fields from the enum variant for assertions.
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

        // Query the card by ID to verify the event was recorded.
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
        assert_eq!(
            card_with_events.next_maintenance_date,
            Some(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap())
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_prevents_duplicate_card_for_same_stock(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        // The fixture already has a maintenance card for this owned_rolling_stock_id.
        // Attempt to create a second card for the same stock.
        let duplicate_card_uuid = Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap();
        let event_uuid = Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap();
        let created_at = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();

        let mut card = MaintenanceCard::from_id(duplicate_card_uuid);
        card.owned_rolling_stock_id = OwnedRollingStockId::try_from(
            "trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-02c7ce47bc70",
        )
        .expect("should parse owned rolling stock id");
        card.pending_events = vec![MaintenanceCardEvent::Created {
            id: event_uuid,
            maintenance_card_id: duplicate_card_uuid,
            created_at,
        }];

        let result = repo.save(card).await;
        assert!(
            matches!(
                result,
                Err(crate::core::domain::domain_error::DomainError::Conflict(_))
            ),
            "Expected DomainError::Conflict for duplicate card, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_list_due_card_views_includes_display_info(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        let views = repo
            .list_due_card_views()
            .await
            .expect("list due card views");

        // The fixture card is linked to ACME manufacturer / product code 60100
        let card_view = views
            .iter()
            .find(|v| {
                v.id.to_string()
                    .contains("3284cc76-1472-4b12-a7d4-62043416adc2")
            })
            .expect("fixture card should be in due views");

        let display_info = card_view
            .display_info
            .as_ref()
            .expect("display_info should be Some");

        assert_eq!(
            display_info.manufacturer_name.as_deref(),
            Some("ACME"),
            "manufacturer_name should match fixture"
        );
        assert_eq!(
            display_info.product_code.as_deref(),
            Some("60100"),
            "product_code should match fixture"
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_find_view_by_id_includes_display_info(pool: SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool)
            .await
            .expect("should create unit of work");
        let mut repo = unit_of_work.maintenance_repository();

        let card_id = MaintenanceCardId::try_from(
            "trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2",
        )
        .expect("should parse card id");

        let view = repo
            .find_view_by_id(&card_id)
            .await
            .expect("find_view_by_id should not error")
            .expect("card should exist");

        let display_info = view
            .display_info
            .as_ref()
            .expect("display_info should be Some");

        assert_eq!(
            display_info.manufacturer_name.as_deref(),
            Some("ACME"),
            "manufacturer_name should match fixture"
        );
        assert_eq!(
            display_info.product_code.as_deref(),
            Some("60100"),
            "product_code should match fixture"
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_delete_event_removes_row(pool: SqlitePool) {
        // Uses event: trn:maintenance-event:ad4f1aa7-1142-43eb-afb4-cb56871ac29d
        let event_uuid = Uuid::parse_str("ad4f1aa7-1142-43eb-afb4-cb56871ac29d").unwrap();
        let event_id = crate::maintenance::domain::MaintenanceEventId::from_uuid(&event_uuid);

        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();
        repo.delete_event(&event_id).await.expect("delete");

        // Load the card and confirm event is gone.
        let card_id = MaintenanceCardId::try_from(
            "trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2",
        )
        .expect("parse");
        let card = repo
            .find_by_id(&card_id)
            .await
            .expect("find")
            .expect("exists");
        assert!(
            !card.events.iter().any(|e| e.id == event_uuid),
            "deleted event should not be present"
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_delete_event_updates_last_maintenance_date(pool: SqlitePool) {
        // Delete the most recent event (2025-03-01). last_maintenance_date should fall back to 2025-01-01.
        let event_uuid = Uuid::parse_str("ad4f1aa7-1142-43eb-afb4-cb56871ac29d").unwrap();
        let event_id = crate::maintenance::domain::MaintenanceEventId::from_uuid(&event_uuid);

        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();
        repo.delete_event(&event_id).await.expect("delete");

        let card_id = MaintenanceCardId::try_from(
            "trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2",
        )
        .expect("parse");
        let card = repo
            .find_by_id(&card_id)
            .await
            .expect("find")
            .expect("exists");
        assert_eq!(
            card.last_maintenance_date,
            NaiveDate::from_ymd_opt(2025, 1, 1),
            "last_maintenance_date should roll back to the previous event"
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_maintenance.sql")
    )]
    async fn repo_delete_event_not_found_returns_error(pool: SqlitePool) {
        let nonexistent_uuid = Uuid::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        let event_id = crate::maintenance::domain::MaintenanceEventId::from_uuid(&nonexistent_uuid);

        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.expect("uow");
        let mut repo = unit_of_work.maintenance_repository();
        let result = repo.delete_event(&event_id).await;
        assert!(
            matches!(
                result,
                Err(crate::core::domain::domain_error::DomainError::NotFound { .. })
            ),
            "Expected NotFound error, got: {:?}",
            result
        );
    }
}
