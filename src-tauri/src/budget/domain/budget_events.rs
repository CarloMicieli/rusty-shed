use crate::budget::domain::{BudgetConfigId, ExtraBudgetId};
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Domain events for budget tracking.
///
/// These events capture changes to budget configuration and extra budgets.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BudgetEvent {
    /// Budget configuration was created or updated.
    BudgetConfigured {
        config_id: BudgetConfigId,
        mode: String,
        base_amount: MonetaryAmount,
        timestamp: DateTime<Utc>,
    },
    /// An extra budget was added to a specific month.
    ExtraBudgetAdded {
        extra_budget_id: ExtraBudgetId,
        year: i32,
        month: u8,
        amount: MonetaryAmount,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// An extra budget was removed.
    ExtraBudgetRemoved {
        extra_budget_id: ExtraBudgetId,
        timestamp: DateTime<Utc>,
    },
    /// Annual reset was performed (rollover reset at year boundary).
    AnnualResetPerformed { year: i32, timestamp: DateTime<Utc> },
}

impl BudgetEvent {
    /// Get the event name for logging/debugging.
    pub fn event_name(&self) -> &'static str {
        match self {
            BudgetEvent::BudgetConfigured { .. } => "BUDGET_CONFIGURED",
            BudgetEvent::ExtraBudgetAdded { .. } => "EXTRA_BUDGET_ADDED",
            BudgetEvent::ExtraBudgetRemoved { .. } => "EXTRA_BUDGET_REMOVED",
            BudgetEvent::AnnualResetPerformed { .. } => "ANNUAL_RESET_PERFORMED",
        }
    }

    /// Get the timestamp when the event occurred.
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            BudgetEvent::BudgetConfigured { timestamp, .. } => timestamp,
            BudgetEvent::ExtraBudgetAdded { timestamp, .. } => timestamp,
            BudgetEvent::ExtraBudgetRemoved { timestamp, .. } => timestamp,
            BudgetEvent::AnnualResetPerformed { timestamp, .. } => timestamp,
        }
    }
}
