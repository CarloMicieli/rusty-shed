use crate::budget::domain::{BudgetConfiguration, ExtraBudgetEntry, ExtraBudgetId};
use async_trait::async_trait;

/// Budget Repository trait.
///
/// Defines operations for persisting and retrieving budget-related aggregates.
#[async_trait]
pub trait BudgetRepository: Send + Sync {
    /// Get the budget configuration (singleton).
    async fn get_config(&mut self) -> Result<Option<BudgetConfiguration>, String>;

    /// Save or update the budget configuration.
    async fn save_config(&mut self, config: &BudgetConfiguration) -> Result<(), String>;

    /// Get all extra budget entries for a specific year.
    async fn get_extra_budgets(&mut self, year: i32) -> Result<Vec<ExtraBudgetEntry>, String>;

    /// Get a specific extra budget entry by ID.
    async fn get_extra_budget_by_id(
        &mut self,
        id: &ExtraBudgetId,
    ) -> Result<Option<ExtraBudgetEntry>, String>;

    /// Add a new extra budget entry.
    async fn add_extra_budget(&mut self, entry: &ExtraBudgetEntry) -> Result<(), String>;

    /// Remove an extra budget entry.
    async fn remove_extra_budget(&mut self, id: &ExtraBudgetId) -> Result<(), String>;
}
