use crate::budget::BudgetEvent;
use crate::budget::domain::{
    BudgetConfiguration, BudgetRepository, BudgetUowExt, ExtraBudgetEntry, ExtraBudgetId,
};
use crate::budget::infrastructure::database;
use crate::budget::infrastructure::mappers::{row_to_budget_config, row_to_extra_budget};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use async_trait::async_trait;
use sqlx::SqliteConnection;

/// SQLite-specific implementation of the BudgetRepository.
pub struct SqliteBudgetRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

enum BudgetEventRoute<'a> {
    BudgetConfigured(&'a BudgetEvent),
    ExtraBudgetAdded {
        extra_budget_id: &'a ExtraBudgetId,
        year: i32,
        month: u8,
        amount: &'a crate::core::domain::MonetaryAmount,
        reason: &'a Option<String>,
        timestamp: &'a chrono::DateTime<chrono::Utc>,
    },
    ExtraBudgetRemoved {
        extra_budget_id: &'a ExtraBudgetId,
    },
    AnnualResetPerformed {
        year: i32,
        timestamp: &'a chrono::DateTime<chrono::Utc>,
    },
}

impl<'conn> SqliteBudgetRepository<'conn> {
    /// Create a new SqliteBudgetRepository.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    async fn handle_budget_configured_event(
        &mut self,
        event: &BudgetEvent,
    ) -> Result<(), DomainError> {
        let BudgetEvent::BudgetConfigured {
            config_id,
            mode,
            base_amount,
            last_reset_year,
            created_at,
            version,
            timestamp,
        } = event
        else {
            return Ok(());
        };

        let mode_str = match mode {
            crate::budget::domain::BudgetMode::Yearly => "YEARLY",
            crate::budget::domain::BudgetMode::Monthly => "MONTHLY",
        };

        database::save_budget_config(
            self.executor,
            config_id.value(),
            mode_str,
            base_amount.amount,
            base_amount.currency.to_code(),
            last_reset_year.value(),
            &created_at.to_rfc3339(),
            &timestamp.to_rfc3339(),
            i32::from(*version),
        )
        .await
    }

    async fn handle_extra_budget_added(
        &mut self,
        extra_budget_id: &ExtraBudgetId,
        year: i32,
        month: u8,
        amount: &crate::core::domain::MonetaryAmount,
        reason: &Option<String>,
        timestamp: &chrono::DateTime<chrono::Utc>,
    ) -> Result<(), DomainError> {
        database::add_extra_budget(
            self.executor,
            extra_budget_id.as_ref(),
            year,
            month as i32,
            amount.amount,
            amount.currency.to_code(),
            reason.as_deref(),
            &timestamp.to_rfc3339(),
            0,
        )
        .await
    }

    async fn handle_extra_budget_removed(
        &mut self,
        extra_budget_id: &ExtraBudgetId,
    ) -> Result<(), DomainError> {
        database::remove_extra_budget(self.executor, extra_budget_id.as_ref()).await
    }

    async fn handle_annual_reset(
        &mut self,
        year: i32,
        timestamp: &chrono::DateTime<chrono::Utc>,
    ) -> Result<(), DomainError> {
        let sql = r#"
            UPDATE budget_config
            SET last_reset_year = ?1, updated_at = ?2
            WHERE id = 1
        "#;
        sqlx::query(sql)
            .bind(year)
            .bind(timestamp.to_rfc3339())
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;
        Ok(())
    }

    fn classify_event(event: &BudgetEvent) -> BudgetEventRoute<'_> {
        match event {
            BudgetEvent::BudgetConfigured { .. } => BudgetEventRoute::BudgetConfigured(event),
            BudgetEvent::ExtraBudgetAdded {
                extra_budget_id,
                year,
                month,
                amount,
                reason,
                timestamp,
            } => BudgetEventRoute::ExtraBudgetAdded {
                extra_budget_id,
                year: *year,
                month: *month,
                amount,
                reason,
                timestamp,
            },
            BudgetEvent::ExtraBudgetRemoved {
                extra_budget_id, ..
            } => BudgetEventRoute::ExtraBudgetRemoved { extra_budget_id },
            BudgetEvent::AnnualResetPerformed { year, timestamp } => {
                BudgetEventRoute::AnnualResetPerformed {
                    year: *year,
                    timestamp,
                }
            }
        }
    }

    async fn apply_event_route(&mut self, route: BudgetEventRoute<'_>) -> Result<(), DomainError> {
        match route {
            BudgetEventRoute::BudgetConfigured(event) => {
                self.handle_budget_configured_event(event).await
            }
            BudgetEventRoute::ExtraBudgetAdded {
                extra_budget_id,
                year,
                month,
                amount,
                reason,
                timestamp,
            } => {
                self.handle_extra_budget_added(
                    extra_budget_id,
                    year,
                    month,
                    amount,
                    reason,
                    timestamp,
                )
                .await
            }
            BudgetEventRoute::ExtraBudgetRemoved { extra_budget_id } => {
                self.handle_extra_budget_removed(extra_budget_id).await
            }
            BudgetEventRoute::AnnualResetPerformed { year, timestamp } => {
                self.handle_annual_reset(year, timestamp).await
            }
        }
    }

    /// Route a single domain event to the correct SQL operation.
    ///
    /// The match is exhaustive: adding a new `BudgetEvent` variant forces the
    /// compiler to require a corresponding persistence branch here.
    ///
    /// Note: This method handles conversion of domain types to persistence format.
    /// For example, BudgetMode enum is converted to DB-compatible string ("YEARLY"/"MONTHLY")
    /// here, not in the domain layer.
    async fn handle_event(&mut self, event: &BudgetEvent) -> Result<(), DomainError> {
        self.apply_event_route(Self::classify_event(event)).await
    }
}

#[async_trait]
impl<'conn> BudgetRepository for SqliteBudgetRepository<'conn> {
    /// Persist all pending events from the budget configuration aggregate.
    ///
    /// Events are applied in order within the caller's transaction. No aggregate
    /// fields are read directly — all SQL data comes from the event payload.
    async fn save(&mut self, mut config: BudgetConfiguration) -> Result<(), DomainError> {
        for event in config.drain_events() {
            self.handle_event(&event).await?;
        }
        Ok(())
    }

    async fn get_config(&mut self) -> Result<Option<BudgetConfiguration>, DomainError> {
        let row = database::get_budget_config(self.executor).await?;

        match row {
            Some(r) => Ok(Some(
                row_to_budget_config(r).map_err(|e| DomainError::Validation(e.to_string()))?,
            )),
            None => Ok(None),
        }
    }

    async fn get_extra_budgets(&mut self, year: i32) -> Result<Vec<ExtraBudgetEntry>, DomainError> {
        let rows = database::get_extra_budgets(self.executor, year).await?;

        rows.into_iter()
            .map(|r| row_to_extra_budget(r).map_err(|e| DomainError::Validation(e.to_string())))
            .collect()
    }

    async fn get_extra_budget_by_id(
        &mut self,
        id: &ExtraBudgetId,
    ) -> Result<Option<ExtraBudgetEntry>, String> {
        let row = database::get_extra_budget_by_id(self.executor, id.as_ref())
            .await
            .map_err(|e| format!("Failed to get extra budget by id: {}", e))?;

        match row {
            Some(r) => Ok(Some(row_to_extra_budget(r).map_err(|e| e.to_string())?)),
            None => Ok(None),
        }
    }
    async fn add_extra_budget(&mut self, entry: &ExtraBudgetEntry) -> Result<(), String> {
        database::add_extra_budget(
            self.executor,
            entry.id.as_ref(),
            entry.year.value(),
            entry.month.value() as i32,
            entry.amount.amount,
            entry.amount.currency.to_code(),
            entry.reason.as_deref(),
            &entry.created_at.to_rfc3339(),
            entry.version as i32,
        )
        .await
        .map_err(|e| format!("Failed to add extra budget: {}", e))?;

        Ok(())
    }

    async fn remove_extra_budget(&mut self, id: &ExtraBudgetId) -> Result<(), String> {
        database::remove_extra_budget(self.executor, id.as_ref())
            .await
            .map_err(|e| format!("Failed to remove extra budget: {}", e))?;

        Ok(())
    }

    async fn get_monthly_spending(
        &mut self,
        year: i32,
        currency: &str,
    ) -> Result<Vec<(i32, i64)>, String> {
        database::get_monthly_spending(self.executor, year, currency)
            .await
            .map_err(|e| format!("Failed to get monthly spending: {}", e))
    }

    async fn get_multi_year_monthly_spending(
        &mut self,
        start_year: i32,
        end_year: i32,
        currency: &str,
    ) -> Result<Vec<(i32, i32, i64)>, String> {
        database::get_multi_year_monthly_spending(self.executor, start_year, end_year, currency)
            .await
            .map_err(|e| format!("Failed to get multi-year monthly spending: {}", e))
    }

    async fn get_quarterly_spending_by_category(
        &mut self,
        year: i32,
        currency: &str,
    ) -> Result<Vec<(i32, String, i64)>, String> {
        database::get_quarterly_spending_by_category(self.executor, year, currency)
            .await
            .map_err(|e| format!("Failed to get quarterly spending: {}", e))
    }
}

impl BudgetUowExt for SqliteUnitOfWork {
    fn budget_repo(&mut self) -> Box<dyn BudgetRepository + '_> {
        Box::new(SqliteBudgetRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::domain::{BudgetMode, ExtraBudgetEntry};
    use crate::core::domain::calendar::{Month, Year};
    use crate::core::domain::{Currency, MonetaryAmount};

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_none_when_budget_configuration_is_missing(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let mut repo = uow.budget_repo();
        let loaded = repo.get_config().await.expect("get_config should succeed");

        assert!(loaded.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_save_and_load_budget_configuration(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        {
            let mut repo = uow.budget_repo();
            let config = BudgetConfiguration::new(
                BudgetMode::Monthly,
                MonetaryAmount::new(120_000, Currency::EUR),
            )
            .expect("test: valid config");

            repo.save(config).await.expect("save should succeed");
        }

        {
            let mut repo = uow.budget_repo();
            let loaded = repo
                .get_config()
                .await
                .expect("get_config should succeed")
                .expect("config should exist");

            assert_eq!(loaded.mode, BudgetMode::Monthly);
            assert_eq!(loaded.base_amount.amount, 120_000);
            assert_eq!(loaded.base_amount.currency, Currency::EUR);
        }

        uow.commit().await.expect("commit should succeed");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_fail_on_invalid_budget_configuration_row(conn: sqlx::SqlitePool) {
        let mut setup = conn.acquire().await.expect("acquire connection");
        sqlx::query(
            "INSERT INTO budget_config (id, mode, base_amount, currency, last_reset_year, created_at, updated_at, version)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind(1_i32)
        .bind("YEARLY")
        .bind(120_000_i64)
        .bind("EUR")
        .bind(99_999_i32)
        .bind("2026-01-01T00:00:00Z")
        .bind("2026-01-01T00:00:00Z")
        .bind(0_i32)
        .execute(&mut *setup)
        .await
        .expect("insert invalid config row");
        drop(setup);

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");
        let mut repo = uow.budget_repo();

        let error = repo
            .get_config()
            .await
            .expect_err("invalid config row should fail");

        assert!(
            matches!(error, DomainError::Validation(message) if message.contains("Invalid year"))
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_add_get_and_remove_extra_budget(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let entry = ExtraBudgetEntry::new(
            Year::try_from(2026).expect("valid year"),
            Month::try_from(4).expect("valid month"),
            MonetaryAmount::new(5_000, Currency::EUR),
            Some("gift".to_string()),
        )
        .expect("valid extra budget entry");

        {
            let mut repo = uow.budget_repo();
            repo.add_extra_budget(&entry)
                .await
                .expect("add_extra_budget should succeed");

            let found = repo
                .get_extra_budget_by_id(&entry.id)
                .await
                .expect("get_extra_budget_by_id should succeed")
                .expect("entry should be present");
            assert_eq!(found.id, entry.id);
            assert_eq!(found.amount.amount, 5_000);

            repo.remove_extra_budget(&entry.id)
                .await
                .expect("remove_extra_budget should succeed");

            let missing = repo
                .get_extra_budget_by_id(&entry.id)
                .await
                .expect("get_extra_budget_by_id should succeed after delete");
            assert!(missing.is_none());
        }

        uow.commit().await.expect("commit should succeed");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_empty_extra_budgets_when_year_has_no_rows(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");
        let mut repo = uow.budget_repo();

        let entries = repo
            .get_extra_budgets(2026)
            .await
            .expect("get_extra_budgets should succeed");

        assert!(entries.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_fail_on_invalid_extra_budget_row(conn: sqlx::SqlitePool) {
        let mut setup = conn.acquire().await.expect("acquire connection");
        sqlx::query(
            "INSERT INTO extra_budgets (id, year, month, amount, currency, reason, created_at, version)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind("trn:extra-budget:11111111-1111-1111-1111-111111111111")
        .bind(2026_i32)
        .bind(4_i32)
        .bind(5_000_i64)
        .bind("INVALID")
        .bind(Some("gift"))
        .bind("2026-01-01T00:00:00Z")
        .bind(0_i32)
        .execute(&mut *setup)
        .await
        .expect("insert invalid extra budget row");
        drop(setup);

        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");
        let mut repo = uow.budget_repo();

        let error = repo
            .get_extra_budgets(2026)
            .await
            .expect_err("invalid extra budget row should fail");

        assert!(
            matches!(error, DomainError::Validation(message) if message.contains("Invalid currency"))
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_filter_monthly_spending_by_currency(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        {
            let mut repo = uow.budget_repo();

            let eur = repo
                .get_monthly_spending(2025, "EUR")
                .await
                .expect("EUR spending query should succeed");
            assert_eq!(eur, vec![(12, 17_500)]);

            let usd = repo
                .get_monthly_spending(2025, "USD")
                .await
                .expect("USD spending query should succeed");
            assert!(usd.is_empty());
        }

        uow.commit().await.expect("commit should succeed");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_return_multi_year_and_quarterly_aggregates(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        {
            let mut repo = uow.budget_repo();

            let multi_year = repo
                .get_multi_year_monthly_spending(2024, 2025, "EUR")
                .await
                .expect("multi year query should succeed");
            assert_eq!(multi_year, vec![(2025, 12, 17_500)]);

            let quarterly = repo
                .get_quarterly_spending_by_category(2025, "EUR")
                .await
                .expect("quarterly query should succeed");
            assert_eq!(quarterly, vec![(4, "LOCOMOTIVES".to_string(), 17_500)]);
        }

        uow.commit().await.expect("commit should succeed");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_persist_all_budget_event_variants_via_save(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        let mut config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(240_000, Currency::EUR),
        )
        .expect("valid budget config");

        let extra = ExtraBudgetEntry::new(
            Year::try_from(2026).expect("valid year"),
            Month::try_from(5).expect("valid month"),
            MonetaryAmount::new(15_000, Currency::EUR),
            Some("bonus".to_string()),
        )
        .expect("valid extra budget entry");

        config.add_extra_budget(&extra);
        config.remove_extra_budget(extra.id.clone());
        config.last_reset_year = Year::try_from(2020).expect("valid reset year");
        config
            .perform_annual_reset()
            .expect("annual reset should succeed");

        {
            let mut repo = uow.budget_repo();
            repo.save(config).await.expect("save should succeed");

            let loaded_config = repo
                .get_config()
                .await
                .expect("get_config should succeed")
                .expect("config should exist");
            assert_eq!(loaded_config.mode, BudgetMode::Monthly);
            assert_eq!(loaded_config.base_amount.amount, 240_000);

            let removed_extra = repo
                .get_extra_budget_by_id(&extra.id)
                .await
                .expect("get_extra_budget_by_id should succeed");
            assert!(removed_extra.is_none());
        }

        uow.commit().await.expect("commit should succeed");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_map_add_extra_budget_database_failures(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        sqlx::query("DROP TABLE extra_budgets")
            .execute(&conn)
            .await
            .expect("drop should succeed");

        let entry = ExtraBudgetEntry::new(
            Year::try_from(2026).expect("valid year"),
            Month::try_from(4).expect("valid month"),
            MonetaryAmount::new(5_000, Currency::EUR),
            Some("gift".to_string()),
        )
        .expect("valid extra budget entry");

        let err = {
            let mut repo = uow.budget_repo();
            repo.add_extra_budget(&entry)
                .await
                .expect_err("add should fail")
        };

        assert!(err.starts_with("Failed to add extra budget:"), "{err}");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_map_get_extra_budget_by_id_database_failures(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        sqlx::query("DROP TABLE extra_budgets")
            .execute(&conn)
            .await
            .expect("drop should succeed");

        let id = ExtraBudgetId::default();

        let err = {
            let mut repo = uow.budget_repo();
            repo.get_extra_budget_by_id(&id)
                .await
                .expect_err("lookup should fail")
        };

        assert!(
            err.starts_with("Failed to get extra budget by id:"),
            "{err}"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_map_remove_extra_budget_database_failures(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        sqlx::query("DROP TABLE extra_budgets")
            .execute(&conn)
            .await
            .expect("drop should succeed");

        let id = ExtraBudgetId::default();

        let err = {
            let mut repo = uow.budget_repo();
            repo.remove_extra_budget(&id)
                .await
                .expect_err("remove should fail")
        };

        assert!(err.starts_with("Failed to remove extra budget:"), "{err}");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_map_monthly_spending_query_failures(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        sqlx::query("DROP TABLE purchase_infos")
            .execute(&conn)
            .await
            .expect("drop should succeed");

        let err = {
            let mut repo = uow.budget_repo();
            repo.get_monthly_spending(2025, "EUR")
                .await
                .expect_err("query should fail")
        };

        assert!(err.starts_with("Failed to get monthly spending:"), "{err}");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn it_should_map_multi_year_and_quarterly_query_failures(conn: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&conn)
            .await
            .expect("should create unit of work");

        sqlx::query("DROP TABLE purchase_infos")
            .execute(&conn)
            .await
            .expect("drop should succeed");

        let (multi_year_err, quarterly_err) = {
            let mut repo = uow.budget_repo();
            let multi_year_err = repo
                .get_multi_year_monthly_spending(2024, 2025, "EUR")
                .await
                .expect_err("query should fail");
            let quarterly_err = repo
                .get_quarterly_spending_by_category(2025, "EUR")
                .await
                .expect_err("query should fail");
            (multi_year_err, quarterly_err)
        };

        assert!(
            multi_year_err.starts_with("Failed to get multi-year monthly spending:"),
            "{multi_year_err}"
        );
        assert!(
            quarterly_err.starts_with("Failed to get quarterly spending:"),
            "{quarterly_err}"
        );
    }
}
