// Budget Repository Implementation
// Implements BudgetRepository trait for SQLite

use crate::budget::domain::{
    BudgetConfiguration, BudgetRepository, ExtraBudgetEntry, ExtraBudgetId,
};
use crate::budget::infrastructure::database;
use crate::budget::infrastructure::mappers::{row_to_budget_config, row_to_extra_budget};
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
}

#[async_trait]
impl<'conn> BudgetRepository for SqliteBudgetRepository<'conn> {
    async fn get_config(&mut self) -> Result<Option<BudgetConfiguration>, String> {
        let row = database::get_budget_config(self.executor)
            .await
            .map_err(|e| format!("Failed to get budget config: {}", e))?;

        match row {
            Some(r) => Ok(Some(
                row_to_budget_config(r).map_err(|e| format!("Failed to map config: {}", e))?,
            )),
            None => Ok(None),
        }
    }

    async fn save_config(&mut self, config: &BudgetConfiguration) -> Result<(), String> {
        let mode_str = match config.mode {
            crate::budget::domain::BudgetMode::Yearly => "YEARLY",
            crate::budget::domain::BudgetMode::Monthly => "MONTHLY",
        };

        database::save_budget_config(
            self.executor,
            config.id.value(),
            mode_str,
            config.base_amount.amount,
            config.base_amount.currency.to_code(),
            config.last_reset_year,
            &config.created_at.to_rfc3339(),
            &config.updated_at.to_rfc3339(),
            config.version as i32,
        )
        .await
        .map_err(|e| format!("Failed to save budget config: {}", e))?;

        Ok(())
    }

    async fn get_extra_budgets(&mut self, year: i32) -> Result<Vec<ExtraBudgetEntry>, String> {
        let rows = database::get_extra_budgets(self.executor, year)
            .await
            .map_err(|e| format!("Failed to get extra budgets: {}", e))?;

        rows.into_iter()
            .map(|r| {
                row_to_extra_budget(r).map_err(|e| format!("Failed to map extra budget: {}", e))
            })
            .collect()
    }

    async fn get_extra_budget_by_id(
        &mut self,
        id: &ExtraBudgetId,
    ) -> Result<Option<ExtraBudgetEntry>, String> {
        let row = database::get_extra_budget_by_id(self.executor, id.as_ref())
            .await
            .map_err(|e| format!("Failed to get extra budget: {}", e))?;

        match row {
            Some(r) => {
                Ok(Some(row_to_extra_budget(r).map_err(|e| {
                    format!("Failed to map extra budget: {}", e)
                })?))
            }
            None => Ok(None),
        }
    }

    async fn add_extra_budget(&mut self, entry: &ExtraBudgetEntry) -> Result<(), String> {
        database::add_extra_budget(
            self.executor,
            entry.id.as_ref(),
            entry.year,
            entry.month as i32,
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
