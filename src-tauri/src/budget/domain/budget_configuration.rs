use crate::budget::domain::{
    BudgetConfigId, BudgetEvent, BudgetMode, ExtraBudgetEntry, ExtraBudgetId,
};
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Budget Configuration aggregate root.
///
/// Singleton aggregate - only one configuration exists per user.
/// Controls the base budget amount and whether it's entered as yearly or monthly.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfiguration {
    pub id: BudgetConfigId,
    pub mode: BudgetMode,
    pub base_amount: MonetaryAmount,
    pub last_reset_year: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,

    #[serde(skip)]
    pub(crate) pending_events: Vec<BudgetEvent>, // Made pub(crate) for infrastructure layer access
}

impl BudgetConfiguration {
    /// Create a new budget configuration.
    pub fn new(mode: BudgetMode, base_amount: MonetaryAmount) -> Self {
        let now = Utc::now();
        let current_year = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);

        let mut config = Self {
            id: BudgetConfigId::singleton(),
            mode,
            base_amount: base_amount.clone(),
            last_reset_year: current_year,
            created_at: now,
            updated_at: now,
            version: 0,
            pending_events: Vec::new(),
        };

        // Record the configuration event with all SQL-needed fields
        config.pending_events.push(BudgetEvent::BudgetConfigured {
            config_id: config.id,
            mode: Self::mode_to_db_str(mode).to_string(),
            base_amount,
            last_reset_year: config.last_reset_year,
            created_at: config.created_at,
            version: config.version,
            timestamp: now,
        });

        config
    }

    /// Update the budget configuration.
    pub fn update(&mut self, mode: BudgetMode, base_amount: MonetaryAmount) {
        self.mode = mode;
        self.base_amount = base_amount.clone();
        self.updated_at = Utc::now();

        // Record the update event with all SQL-needed fields
        self.pending_events.push(BudgetEvent::BudgetConfigured {
            config_id: self.id,
            mode: Self::mode_to_db_str(mode).to_string(),
            base_amount,
            last_reset_year: self.last_reset_year,
            created_at: self.created_at,
            version: self.version,
            timestamp: self.updated_at,
        });
    }

    /// Emit an `ExtraBudgetAdded` event so the repository can INSERT the entry.
    ///
    /// The aggregate does not own extra budget entries as in-memory state;
    /// the event payload carries all fields the SQL operation requires.
    pub fn add_extra_budget(&mut self, entry: &ExtraBudgetEntry) {
        self.pending_events.push(BudgetEvent::ExtraBudgetAdded {
            extra_budget_id: entry.id.clone(),
            year: entry.year.value(),
            month: entry.month.value(),
            amount: entry.amount.clone(),
            reason: entry.reason.clone(),
            timestamp: entry.created_at,
        });
    }

    /// Emit an `ExtraBudgetRemoved` event so the repository can DELETE the entry.
    pub fn remove_extra_budget(&mut self, id: ExtraBudgetId) {
        self.pending_events.push(BudgetEvent::ExtraBudgetRemoved {
            extra_budget_id: id,
            timestamp: Utc::now(),
        });
    }

    /// Returns the monthly budget amount.
    /// For YEARLY mode, divides by 12.
    pub fn monthly_amount(&self) -> i64 {
        match self.mode {
            BudgetMode::Yearly => self.base_amount.amount / 12,
            BudgetMode::Monthly => self.base_amount.amount,
        }
    }

    /// Returns the yearly budget amount.
    /// For MONTHLY mode, multiplies by 12.
    pub fn yearly_amount(&self) -> i64 {
        match self.mode {
            BudgetMode::Yearly => self.base_amount.amount,
            BudgetMode::Monthly => self.base_amount.amount * 12,
        }
    }

    /// Convert a `BudgetMode` to the DB-compatible string representation.
    fn mode_to_db_str(mode: BudgetMode) -> &'static str {
        match mode {
            BudgetMode::Yearly => "YEARLY",
            BudgetMode::Monthly => "MONTHLY",
        }
    }

    /// Check if annual reset is needed (year changed since last reset).
    pub fn needs_annual_reset(&self) -> bool {
        let now = Utc::now();
        let current_year = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
        current_year > self.last_reset_year
    }

    /// Perform annual reset (update last_reset_year to current year).
    pub fn perform_annual_reset(&mut self) {
        let now = Utc::now();
        let current_year = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
        if current_year > self.last_reset_year {
            self.last_reset_year = current_year;
            self.updated_at = Utc::now();

            self.pending_events.push(BudgetEvent::AnnualResetPerformed {
                year: current_year,
                timestamp: self.updated_at,
            });
        }
    }

    /// Drain and return all pending domain events.
    ///
    /// Clears the internal buffer. Called once by the repository's `save()` to
    /// obtain the events to persist within the current transaction.
    pub fn drain_events(&mut self) -> Vec<BudgetEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Inspect pending events without clearing them (useful in tests).
    pub fn pending_events(&self) -> &[BudgetEvent] {
        &self.pending_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::currency::Currency;

    #[test]
    fn test_monthly_amount_calculation() {
        let config = BudgetConfiguration::new(
            BudgetMode::Yearly,
            MonetaryAmount::new(120_000, Currency::USD), // $1,200/year
        );
        assert_eq!(config.monthly_amount(), 10_000); // $100/month
    }

    #[test]
    fn test_yearly_amount_calculation() {
        let config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD), // $100/month
        );
        assert_eq!(config.yearly_amount(), 120_000); // $1,200/year
    }

    #[test]
    fn test_event_recording() {
        let config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        );
        assert_eq!(config.pending_events().len(), 1);
        assert_eq!(config.pending_events()[0].event_name(), "BUDGET_CONFIGURED");
    }
}
