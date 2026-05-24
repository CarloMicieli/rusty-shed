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

    /// Route a single domain event to the correct SQL operation.
    ///
    /// The match is exhaustive: adding a new `BudgetEvent` variant forces the
    /// compiler to require a corresponding persistence branch here.
    ///
    /// Note: This method handles conversion of domain types to persistence format.
    /// For example, BudgetMode enum is converted to DB-compatible string ("YEARLY"/"MONTHLY")
    /// here, not in the domain layer.
    async fn handle_event(&mut self, event: &BudgetEvent) -> Result<(), DomainError> {
        match event {
            BudgetEvent::BudgetConfigured { .. } => {
                self.handle_budget_configured_event(event).await?;
            }
            BudgetEvent::ExtraBudgetAdded {
                extra_budget_id,
                year,
                month,
                amount,
                reason,
                timestamp,
            } => {
                self.handle_extra_budget_added(
                    extra_budget_id,
                    *year,
                    *month,
                    amount,
                    reason,
                    timestamp,
                )
                .await?;
            }
            BudgetEvent::ExtraBudgetRemoved {
                extra_budget_id, ..
            } => {
                self.handle_extra_budget_removed(extra_budget_id).await?;
            }
            BudgetEvent::AnnualResetPerformed { year, timestamp } => {
                self.handle_annual_reset(*year, timestamp).await?;
            }
        }
        Ok(())
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
}
