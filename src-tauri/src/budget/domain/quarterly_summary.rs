use super::dashboard::BudgetQuarter;
use crate::catalog::domain::railway_model::Category;
use crate::core::domain::monetary_amount::MonetaryAmount;
use serde::{Deserialize, Serialize};
use specta::Type;

/// Spending breakdown for a single category in a quarter.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CategorySpending {
    pub category: Category,
    pub amount: MonetaryAmount,
    pub percentage: f64,
}

/// Summary of spending for a quarter with category breakdown.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlySummary {
    pub year: i32,
    pub quarter: BudgetQuarter,
    pub total_spending: MonetaryAmount,
    pub category_breakdown: Vec<CategorySpending>,
}

impl QuarterlySummary {
    /// Create a new quarterly summary.
    pub fn new(
        year: i32,
        quarter: BudgetQuarter,
        category_breakdown: Vec<CategorySpending>,
    ) -> Self {
        let total_spending = if category_breakdown.is_empty() {
            MonetaryAmount::new(0, crate::core::domain::Currency::EUR) // Default currency
        } else {
            let first_currency = category_breakdown[0].amount.currency;
            category_breakdown
                .iter()
                .fold(MonetaryAmount::new(0, first_currency), |acc, cs| {
                    MonetaryAmount::new(acc.amount + cs.amount.amount, acc.currency)
                })
        };

        Self {
            year,
            quarter,
            total_spending,
            category_breakdown,
        }
    }
}
