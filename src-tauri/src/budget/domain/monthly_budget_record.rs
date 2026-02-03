use crate::core::domain::currency::Currency;
use serde::{Deserialize, Serialize};

/// Monthly budget record - read model/projection.
///
/// This is a computed view of the budget status for a single month,
/// derived from the budget configuration, extra budgets, and spending records.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyBudgetRecord {
    pub year: i32,
    pub month: u8,         // 1-12
    pub base_budget: i64,  // Monthly allocation
    pub extra_budget: i64, // Sum of extra budgets for this month
    pub actual_spend: i64, // Sum of purchases this month
    pub rollover_in: i64,  // Rollover from previous month
    pub rollover_out: i64, // Rollover to next month (calculated)
    pub status: MonthStatus,
    pub currency: Currency,
}

impl MonthlyBudgetRecord {
    /// Total available budget for this month (before spending).
    pub fn available(&self) -> i64 {
        self.base_budget + self.extra_budget + self.rollover_in
    }

    /// Remaining budget after spending.
    pub fn remaining(&self) -> i64 {
        self.available() - self.actual_spend
    }

    /// Percentage of budget remaining (0-100).
    pub fn remaining_percentage(&self) -> f64 {
        let available = self.available();
        if available == 0 {
            return 0.0;
        }
        ((self.remaining() as f64) / (available as f64)) * 100.0
    }

    /// Percentage of budget spent (0-100).
    pub fn spent_percentage(&self) -> f64 {
        100.0 - self.remaining_percentage()
    }

    /// Check if over budget (spent more than available).
    pub fn is_over_budget(&self) -> bool {
        self.remaining() < 0
    }

    /// Format the month name for display.
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Invalid",
        }
    }
}

/// Month status - indicates whether a month is projected, in progress, or completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MonthStatus {
    /// Future month, not yet reached.
    Projected,
    /// Current month, in progress.
    InProgress,
    /// Past month, completed.
    Completed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_calculation() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10_000,
            extra_budget: 2_000,
            actual_spend: 0,
            rollover_in: 1_000,
            rollover_out: 0,
            status: MonthStatus::InProgress,
            currency: Currency::USD,
        };
        assert_eq!(record.available(), 13_000); // 10k + 2k + 1k
    }

    #[test]
    fn test_remaining_calculation() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10_000,
            extra_budget: 0,
            actual_spend: 3_000,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::InProgress,
            currency: Currency::USD,
        };
        assert_eq!(record.remaining(), 7_000); // 10k - 3k
    }

    #[test]
    fn test_remaining_percentage() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10_000,
            extra_budget: 0,
            actual_spend: 2_500,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::InProgress,
            currency: Currency::USD,
        };
        assert_eq!(record.remaining_percentage(), 75.0); // 7.5k / 10k = 75%
    }

    #[test]
    fn test_over_budget() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10_000,
            extra_budget: 0,
            actual_spend: 12_000,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::Completed,
            currency: Currency::USD,
        };
        assert!(record.is_over_budget());
        assert_eq!(record.remaining(), -2_000);
    }
}
