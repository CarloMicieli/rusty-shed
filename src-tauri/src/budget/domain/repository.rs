use crate::budget::domain::{BudgetConfiguration, ExtraBudgetEntry, ExtraBudgetId};
use crate::core::domain::domain_error::DomainError;
use async_trait::async_trait;

/// Budget Repository trait.
///
/// All writes flow through `save()`, which drains the aggregate's pending events
/// and dispatches each to the correct SQL operation. Read methods remain
/// fine-grained because they do not touch aggregate state.
#[async_trait]
pub trait BudgetRepository: Send + Sync {
    /// Persist all pending events from the budget configuration aggregate.
    async fn save(&mut self, config: BudgetConfiguration) -> Result<(), DomainError>;

    /// Get the budget configuration (singleton), if one has been created.
    async fn get_config(&mut self) -> Result<Option<BudgetConfiguration>, DomainError>;

    /// Get all extra budget entries for a specific year.
    async fn get_extra_budgets(&mut self, year: i32) -> Result<Vec<ExtraBudgetEntry>, DomainError>;

    /// Get a specific extra budget entry by ID.
    async fn get_extra_budget_by_id(
        &mut self,
        id: &ExtraBudgetId,
    ) -> Result<Option<ExtraBudgetEntry>, String>;

    /// Add a new extra budget entry.
    async fn add_extra_budget(&mut self, entry: &ExtraBudgetEntry) -> Result<(), String>;

    /// Remove an extra budget entry.
    async fn remove_extra_budget(&mut self, id: &ExtraBudgetId) -> Result<(), String>;

    /// Get monthly spending totals aggregated from collection purchases.
    ///
    /// Returns a list of `(month, total_amount)` pairs for the given year and currency.
    async fn get_monthly_spending(
        &mut self,
        year: i32,
        currency: &str,
    ) -> Result<Vec<(i32, i64)>, String>;

    /// Get monthly spending totals for a range of years in a single query.
    ///
    /// Returns `(year, month, total_amount)` triples for all months in
    /// `[start_year, end_year]` that have at least one purchase in `currency`.
    /// Months with no spending are omitted.
    ///
    /// Prefer this over calling [`get_monthly_spending`] in a loop when multiple
    /// years of data are needed at once (e.g., the 5-year heatmap).
    async fn get_multi_year_monthly_spending(
        &mut self,
        start_year: i32,
        end_year: i32,
        currency: &str,
    ) -> Result<Vec<(i32, i32, i64)>, String>;

    /// Get quarterly spending broken down by rolling-stock category.
    ///
    /// Returns `(quarter_number, category_code, total_amount)` triples.
    async fn get_quarterly_spending_by_category(
        &mut self,
        year: i32,
        currency: &str,
    ) -> Result<Vec<(i32, String, i64)>, String>;
}

/// Extension trait for the Unit of Work to provide access to the budget repository.
pub trait BudgetUowExt: Send {
    fn budget_repo(&mut self) -> Box<dyn BudgetRepository + '_>;
}
