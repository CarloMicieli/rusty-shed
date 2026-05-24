use crate::core::domain::currency::Currency;
use serde::{Deserialize, Serialize};

/// Monthly budget record - read model/projection.
///
/// This is a computed view of the budget status for a single month,
/// derived from the budget configuration, extra budgets, and spending records.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyBudgetRecord {
    /// The year this record represents.
    pub year: i32,
    /// The month this record represents.
    pub month: u8,
    /// Monthly allocation
    pub base_budget: i64,
    /// Sum of extra budgets for this month
    pub extra_budget: i64,
    /// Sum of purchases this month
    pub actual_spend: i64,
    /// Rollover from previous month
    pub rollover_in: i64,
    /// Rollover to next month (calculated)
    pub rollover_out: i64,
    pub status: MonthStatus,
    pub currency: Currency,
}

impl MonthlyBudgetRecord {
    const MONTH_NAMES: [&'static str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

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
        if !(1..=12).contains(&self.month) {
            return "Invalid";
        }

        Self::MONTH_NAMES
            .get((self.month - 1) as usize)
            .copied()
            .unwrap_or("Invalid")
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
            base_budget: 10000,
            extra_budget: 2000,
            actual_spend: 0,
            rollover_in: 1000,
            rollover_out: 0,
            status: MonthStatus::InProgress,
            currency: Currency::USD,
        };
        assert_eq!(record.available(), 13000);
    }

    #[test]
    fn test_remaining_calculation() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10000,
            extra_budget: 0,
            actual_spend: 3000,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::InProgress,
            currency: Currency::USD,
        };
        assert_eq!(record.remaining(), 7000);
    }

    #[test]
    fn test_remaining_percentage() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10000,
            extra_budget: 0,
            actual_spend: 2500,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::InProgress,
            currency: Currency::USD,
        };
        assert_eq!(record.remaining_percentage(), 75.0);
    }

    #[test]
    fn test_over_budget() {
        let record = MonthlyBudgetRecord {
            year: 2026,
            month: 1,
            base_budget: 10000,
            extra_budget: 0,
            actual_spend: 12000,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::Completed,
            currency: Currency::USD,
        };
        assert!(record.is_over_budget());
        assert_eq!(record.remaining(), -2000);
    }

    #[test]
    fn test_month_name_exhaustive_mapping() {
        let expected = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        for (idx, month_name) in expected.iter().enumerate() {
            let record = MonthlyBudgetRecord {
                year: 2026,
                month: (idx + 1) as u8,
                base_budget: 0,
                extra_budget: 0,
                actual_spend: 0,
                rollover_in: 0,
                rollover_out: 0,
                status: MonthStatus::Projected,
                currency: Currency::USD,
            };
            assert_eq!(record.month_name(), *month_name);
        }
    }

    #[test]
    fn test_month_name_invalid_values() {
        let zero_month = MonthlyBudgetRecord {
            year: 2026,
            month: 0,
            base_budget: 0,
            extra_budget: 0,
            actual_spend: 0,
            rollover_in: 0,
            rollover_out: 0,
            status: MonthStatus::Projected,
            currency: Currency::USD,
        };

        let thirteenth_month = MonthlyBudgetRecord {
            month: 13,
            ..zero_month.clone()
        };

        assert_eq!(zero_month.month_name(), "Invalid");
        assert_eq!(thirteenth_month.month_name(), "Invalid");
    }
}
