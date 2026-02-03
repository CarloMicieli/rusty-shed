use crate::budget::domain::{BudgetConfigId, BudgetEvent, BudgetMode};
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

        // Record the configuration event
        config.pending_events.push(BudgetEvent::BudgetConfigured {
            config_id: config.id,
            mode: format!("{:?}", mode),
            base_amount,
            timestamp: now,
        });

        config
    }

    /// Update the budget configuration.
    pub fn update(&mut self, mode: BudgetMode, base_amount: MonetaryAmount) {
        self.mode = mode;
        self.base_amount = base_amount.clone();
        self.updated_at = Utc::now();

        // Record the update event
        self.pending_events.push(BudgetEvent::BudgetConfigured {
            config_id: self.id,
            mode: format!("{:?}", mode),
            base_amount,
            timestamp: self.updated_at,
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

    /// Get and clear pending events.
    pub fn drain_events(&mut self) -> Vec<BudgetEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Get pending events without clearing them.
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
