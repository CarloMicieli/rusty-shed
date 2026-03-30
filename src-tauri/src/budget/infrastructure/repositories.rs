// Budget Repository Implementation
// Implements BudgetRepository trait for SQLite

use crate::budget::domain::{
    BudgetConfiguration, BudgetEvent, BudgetRepository, ExtraBudgetEntry, ExtraBudgetId,
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

    /// Route a single domain event to the correct SQL operation.
    ///
    /// The match is exhaustive: adding a new `BudgetEvent` variant forces the
    /// compiler to require a corresponding persistence branch here.
    async fn handle_event(&mut self, event: &BudgetEvent) -> Result<(), DomainError> {
        match event {
            BudgetEvent::BudgetConfigured {
                config_id,
                mode,
                base_amount,
                last_reset_year,
                created_at,
                version,
                timestamp,
            } => {
                database::save_budget_config(
                    self.executor,
                    config_id.value(),
                    mode,
                    base_amount.amount,
                    base_amount.currency.to_code(),
                    *last_reset_year,
                    &created_at.to_rfc3339(),
                    &timestamp.to_rfc3339(),
                    *version as i32,
                )
                .await?;
            }
            BudgetEvent::ExtraBudgetAdded {
                extra_budget_id,
                year,
                month,
                amount,
                reason,
                timestamp,
            } => {
                database::add_extra_budget(
                    self.executor,
                    extra_budget_id.as_ref(),
                    *year,
                    *month as i32,
                    amount.amount,
                    amount.currency.to_code(),
                    reason.as_deref(),
                    &timestamp.to_rfc3339(),
                    0, // initial version for new entries
                )
                .await?;
            }
            BudgetEvent::ExtraBudgetRemoved {
                extra_budget_id, ..
            } => {
                database::remove_extra_budget(self.executor, extra_budget_id.as_ref()).await?;
            }
            BudgetEvent::AnnualResetPerformed { year, timestamp } => {
                let sql = r#"
                    UPDATE budget_config
                    SET last_reset_year = ?1, updated_at = ?2
                    WHERE id = 1
                "#;
                sqlx::query(sql)
                    .bind(year)
                    .bind(timestamp.to_rfc3339())
                    .execute(&mut *self.executor)
                    .await?;
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
    ) -> Result<Option<ExtraBudgetEntry>, DomainError> {
        let row = database::get_extra_budget_by_id(self.executor, id.as_ref()).await?;

        match row {
            Some(r) => Ok(Some(
                row_to_extra_budget(r).map_err(|e| DomainError::Validation(e.to_string()))?,
            )),
            None => Ok(None),
        }
    }
}

/// Extension trait for SqliteUnitOfWork to provide budget repository access.
pub trait BudgetUowExt {
    fn budget_repo(&mut self) -> SqliteBudgetRepository<'_>;
}

impl<'conn> BudgetUowExt for SqliteUnitOfWork<'conn> {
    fn budget_repo(&mut self) -> SqliteBudgetRepository<'_> {
        SqliteBudgetRepository::new(&mut self.tx)
    }
}
