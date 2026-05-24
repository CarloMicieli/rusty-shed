use crate::budget::domain::{BudgetConfigId, BudgetMode, ExtraBudgetId};
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Domain events for budget tracking.
///
/// Every variant carries the domain concept (not DB-serialized strings).
/// Serialization to persistence format (e.g., "YEARLY" string) happens exclusively
/// in the infrastructure layer when persisting events.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BudgetEvent {
    /// Budget configuration was created or updated.
    BudgetConfigured {
        config_id: BudgetConfigId,
        mode: BudgetMode,
        base_amount: MonetaryAmount,
        last_reset_year: crate::core::domain::calendar::Year,
        created_at: DateTime<Utc>,
        version: u8,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;
    use crate::core::domain::calendar::Year;

    #[test]
    fn it_should_return_expected_event_names() {
        let timestamp = Utc::now();
        let year = Year::try_from(2026).unwrap();

        let configured = BudgetEvent::BudgetConfigured {
            config_id: BudgetConfigId::singleton(),
            mode: BudgetMode::Monthly,
            base_amount: MonetaryAmount::new(100_000, Currency::EUR),
            last_reset_year: year,
            created_at: timestamp,
            version: 1,
            timestamp,
        };
        assert_eq!(configured.event_name(), "BUDGET_CONFIGURED");

        let added = BudgetEvent::ExtraBudgetAdded {
            extra_budget_id: ExtraBudgetId::default(),
            year: 2026,
            month: 4,
            amount: MonetaryAmount::new(10_000, Currency::EUR),
            reason: Some("Gift".to_string()),
            timestamp,
        };
        assert_eq!(added.event_name(), "EXTRA_BUDGET_ADDED");

        let removed = BudgetEvent::ExtraBudgetRemoved {
            extra_budget_id: ExtraBudgetId::default(),
            timestamp,
        };
        assert_eq!(removed.event_name(), "EXTRA_BUDGET_REMOVED");

        let reset = BudgetEvent::AnnualResetPerformed {
            year: 2027,
            timestamp,
        };
        assert_eq!(reset.event_name(), "ANNUAL_RESET_PERFORMED");
    }

    #[test]
    fn it_should_expose_event_timestamp() {
        let timestamp = Utc::now();
        let event = BudgetEvent::AnnualResetPerformed {
            year: 2027,
            timestamp,
        };

        assert_eq!(event.timestamp(), &timestamp);
    }
}
