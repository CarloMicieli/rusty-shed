use crate::budget::domain::{
    BudgetConfigId, BudgetEvent, BudgetMode, ExtraBudgetEntry, ExtraBudgetId,
};
use crate::core::domain::calendar::Year;
use crate::core::domain::metadata::Metadata;
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Budget Configuration aggregate root.
///
/// Singleton aggregate - only one configuration exists per user.
/// Controls the base budget amount and whether it's entered as yearly or monthly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfiguration {
    pub id: BudgetConfigId,
    pub mode: BudgetMode,
    pub base_amount: MonetaryAmount,
    pub last_reset_year: Year,

    /// Resource metadata (created_at, updated_at, version).
    pub metadata: Metadata,

    #[serde(skip)]
    pub(crate) pending_events: Vec<BudgetEvent>,
}

impl BudgetConfiguration {
    /// Create a new budget configuration.
    pub fn new(mode: BudgetMode, base_amount: MonetaryAmount) -> Result<Self, String> {
        let now = Utc::now();
        let current_year_i32 = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
        let current_year =
            Year::try_from(current_year_i32).map_err(|e| format!("Invalid current year: {}", e))?;

        let metadata = Metadata {
            version: 0,
            created_at: now,
            updated_at: now,
        };

        let mut config = Self {
            id: BudgetConfigId::singleton(),
            mode,
            base_amount: base_amount.clone(),
            last_reset_year: current_year,
            metadata,
            pending_events: Vec::new(),
        };

        config.pending_events.push(BudgetEvent::BudgetConfigured {
            config_id: config.id,
            mode,
            base_amount,
            last_reset_year: config.last_reset_year,
            created_at: config.metadata.created_at,
            version: config.metadata.version,
            timestamp: now,
        });

        Ok(config)
    }

    /// Update the budget configuration.
    pub fn update(&mut self, mode: BudgetMode, base_amount: MonetaryAmount) {
        self.mode = mode;
        self.base_amount = base_amount.clone();
        self.metadata.updated_at = Utc::now();

        self.pending_events.push(BudgetEvent::BudgetConfigured {
            config_id: self.id,
            mode,
            base_amount,
            last_reset_year: self.last_reset_year,
            created_at: self.metadata.created_at,
            version: self.metadata.version,
            timestamp: self.metadata.updated_at,
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

    /// Check if annual reset is needed (year changed since last reset).
    pub fn needs_annual_reset(&self) -> bool {
        let now = Utc::now();
        let current_year_i32 = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
        Year::try_from(current_year_i32)
            .map(|current_year| current_year.value() > self.last_reset_year.value())
            .unwrap_or(false)
    }

    /// Perform annual reset (update last_reset_year to current year).
    pub fn perform_annual_reset(&mut self) -> Result<(), String> {
        let now = Utc::now();
        let current_year_i32 = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
        let current_year =
            Year::try_from(current_year_i32).map_err(|e| format!("Invalid current year: {}", e))?;

        if current_year.value() > self.last_reset_year.value() {
            self.last_reset_year = current_year;
            self.metadata.updated_at = Utc::now();

            self.pending_events.push(BudgetEvent::AnnualResetPerformed {
                year: current_year.value(),
                timestamp: self.metadata.updated_at,
            });
        }
        Ok(())
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
        )
        .unwrap();
        assert_eq!(config.monthly_amount(), 10_000); // $100/month
    }

    #[test]
    fn test_yearly_amount_calculation() {
        let config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD), // $100/month
        )
        .unwrap();
        assert_eq!(config.yearly_amount(), 120_000); // $1,200/year
    }

    #[test]
    fn test_event_recording() {
        let config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        )
        .unwrap();
        assert_eq!(config.pending_events().len(), 1);
        assert_eq!(config.pending_events()[0].event_name(), "BUDGET_CONFIGURED");
    }

    #[test]
    fn test_update_emits_configured_event() {
        let mut config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        )
        .unwrap();

        config.update(
            BudgetMode::Yearly,
            MonetaryAmount::new(120_000, Currency::USD),
        );

        assert_eq!(config.pending_events().len(), 2);
        assert_eq!(config.pending_events()[1].event_name(), "BUDGET_CONFIGURED");
        assert_eq!(config.mode, BudgetMode::Yearly);
    }

    #[test]
    fn test_drain_events_clears_pending_events() {
        let mut config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        )
        .unwrap();
        config.update(
            BudgetMode::Monthly,
            MonetaryAmount::new(15_000, Currency::USD),
        );

        let drained = config.drain_events();

        assert_eq!(drained.len(), 2);
        assert!(config.pending_events().is_empty());
    }

    #[test]
    fn test_perform_annual_reset_adds_event_when_year_is_older() {
        let mut config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        )
        .unwrap();

        let old_year = Year::try_from(2025).unwrap();
        config.last_reset_year = old_year;

        config.perform_annual_reset().unwrap();

        let current_year_i32 = Utc::now()
            .format("%Y")
            .to_string()
            .parse::<i32>()
            .unwrap_or(2026);
        assert_eq!(config.last_reset_year.value(), current_year_i32);
        assert!(
            config
                .pending_events()
                .iter()
                .any(|event| event.event_name() == "ANNUAL_RESET_PERFORMED")
        );
    }

    #[test]
    fn test_needs_annual_reset_returns_true_when_last_reset_is_previous_year() {
        let mut config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        )
        .unwrap();

        let current_year_i32 = Utc::now()
            .format("%Y")
            .to_string()
            .parse::<i32>()
            .unwrap_or(2026);
        config.last_reset_year = Year::try_from(current_year_i32 - 1).unwrap();

        assert!(config.needs_annual_reset());
    }

    #[test]
    fn test_needs_annual_reset_returns_false_when_last_reset_is_current_year() {
        let mut config = BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(10_000, Currency::USD),
        )
        .unwrap();

        let current_year_i32 = Utc::now()
            .format("%Y")
            .to_string()
            .parse::<i32>()
            .unwrap_or(2026);
        config.last_reset_year = Year::try_from(current_year_i32).unwrap();

        assert!(!config.needs_annual_reset());
    }
}
