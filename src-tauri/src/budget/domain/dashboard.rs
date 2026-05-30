use crate::core::domain::currency::Currency;
use serde::{Deserialize, Serialize};

/// Quarter enum for quarterly summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetQuarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl BudgetQuarter {
    /// Get the quarter number (1-4).
    pub fn number(&self) -> u8 {
        match self {
            BudgetQuarter::Q1 => 1,
            BudgetQuarter::Q2 => 2,
            BudgetQuarter::Q3 => 3,
            BudgetQuarter::Q4 => 4,
        }
    }

    /// Get quarter from month (1-12).
    pub fn from_month(month: u8) -> Self {
        match month {
            1..=3 => BudgetQuarter::Q1,
            4..=6 => BudgetQuarter::Q2,
            7..=9 => BudgetQuarter::Q3,
            10..=12 => BudgetQuarter::Q4,
            _ => BudgetQuarter::Q1, // Default to Q1 for invalid months
        }
    }
}

/// Spending level for heatmap visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpendingLevel {
    None,   // 0
    Low,    // 1-33% of max
    Medium, // 34-66% of max
    High,   // 67-100% of max
}

impl SpendingLevel {
    /// Calculate spending level from amount and max amount.
    pub fn from_percentage(percentage: f64) -> Self {
        if percentage == 0.0 {
            SpendingLevel::None
        } else if percentage <= 33.0 {
            SpendingLevel::Low
        } else if percentage <= 66.0 {
            SpendingLevel::Medium
        } else {
            SpendingLevel::High
        }
    }
}

/// Monthly spending point for bar chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlySpendingPoint {
    pub month: u8, // 1-12
    pub amount: i64,
    pub currency: Currency,
}

/// Quarterly activity point for heatmap.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlyActivityPoint {
    pub year: i32,
    pub quarter: BudgetQuarter,
    pub spending_level: SpendingLevel,
    pub amount: i64,
}

/// Budget dashboard summary - aggregates all dashboard widgets data.
///
/// When no budget is configured, only spending data (monthly_spending and quarterly_activity)
/// will be populated. Budget-specific fields (remaining_amount, remaining_percentage,
/// total_available, monthly_goal) will be None.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetDashboardSummary {
    /// Current month's remaining budget amount (None if no budget configured)
    pub remaining_amount: Option<i64>,
    /// Remaining as percentage (0.0 to 100.0+) (None if no budget configured)
    pub remaining_percentage: Option<f64>,
    /// Total available this month (base + extra + rollover) (None if no budget configured)
    pub total_available: Option<i64>,
    /// Currency for all amounts
    pub currency: Currency,
    /// Monthly spending for bar chart (12 data points)
    pub monthly_spending: Vec<MonthlySpendingPoint>,
    /// Monthly budget goal line amount (None if no budget configured)
    pub monthly_goal: Option<i64>,
    /// Quarterly activity for heatmap (up to 20 data points: 5 years × 4 quarters)
    pub quarterly_activity: Vec<QuarterlyActivityPoint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_from_month() {
        assert_eq!(BudgetQuarter::from_month(1), BudgetQuarter::Q1);
        assert_eq!(BudgetQuarter::from_month(3), BudgetQuarter::Q1);
        assert_eq!(BudgetQuarter::from_month(4), BudgetQuarter::Q2);
        assert_eq!(BudgetQuarter::from_month(6), BudgetQuarter::Q2);
        assert_eq!(BudgetQuarter::from_month(7), BudgetQuarter::Q3);
        assert_eq!(BudgetQuarter::from_month(9), BudgetQuarter::Q3);
        assert_eq!(BudgetQuarter::from_month(10), BudgetQuarter::Q4);
        assert_eq!(BudgetQuarter::from_month(12), BudgetQuarter::Q4);
    }

    #[test]
    fn test_quarter_from_month_falls_back_to_q1_for_invalid_values() {
        assert_eq!(BudgetQuarter::from_month(0), BudgetQuarter::Q1);
        assert_eq!(BudgetQuarter::from_month(13), BudgetQuarter::Q1);
    }

    #[test]
    fn test_quarter_number_maps_all_variants() {
        assert_eq!(BudgetQuarter::Q1.number(), 1);
        assert_eq!(BudgetQuarter::Q2.number(), 2);
        assert_eq!(BudgetQuarter::Q3.number(), 3);
        assert_eq!(BudgetQuarter::Q4.number(), 4);
    }

    #[test]
    fn test_spending_level_from_percentage() {
        assert_eq!(SpendingLevel::from_percentage(0.0), SpendingLevel::None);
        assert_eq!(SpendingLevel::from_percentage(20.0), SpendingLevel::Low);
        assert_eq!(SpendingLevel::from_percentage(50.0), SpendingLevel::Medium);
        assert_eq!(SpendingLevel::from_percentage(90.0), SpendingLevel::High);
    }

    #[test]
    fn test_spending_level_from_percentage_boundaries() {
        assert_eq!(SpendingLevel::from_percentage(33.0), SpendingLevel::Low);
        assert_eq!(SpendingLevel::from_percentage(33.1), SpendingLevel::Medium);
        assert_eq!(SpendingLevel::from_percentage(66.0), SpendingLevel::Medium);
        assert_eq!(SpendingLevel::from_percentage(66.1), SpendingLevel::High);
    }
}
