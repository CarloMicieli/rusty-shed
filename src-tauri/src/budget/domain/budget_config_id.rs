use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for budget configuration.
/// Since budget_config is a singleton table (id=1), this is a simple wrapper.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type,
)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct BudgetConfigId(i32);

impl BudgetConfigId {
    /// The singleton ID for the budget configuration.
    pub const SINGLETON_ID: i32 = 1;

    /// Create a new BudgetConfigId.
    pub fn new(id: i32) -> Self {
        BudgetConfigId(id)
    }

    /// Get the singleton budget config ID.
    pub fn singleton() -> Self {
        BudgetConfigId(Self::SINGLETON_ID)
    }

    /// Get the inner value.
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl Default for BudgetConfigId {
    fn default() -> Self {
        BudgetConfigId::singleton()
    }
}

impl From<i32> for BudgetConfigId {
    fn from(id: i32) -> Self {
        BudgetConfigId(id)
    }
}

impl From<BudgetConfigId> for i32 {
    fn from(id: BudgetConfigId) -> Self {
        id.0
    }
}
