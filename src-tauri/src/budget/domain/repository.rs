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
    ) -> Result<Option<ExtraBudgetEntry>, DomainError>;
}
