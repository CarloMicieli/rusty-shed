// Historical Query - Get quarterly summaries with category breakdown
// Feature: 001-budget-tracking

use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::dashboard::BudgetQuarter;
use crate::budget::domain::quarterly_summary::{CategorySpending, QuarterlySummary};
use crate::catalog::domain::railway_model::Category;
use crate::core::domain::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::monetary_amount::MonetaryAmount;
use std::collections::HashMap;
use std::str::FromStr;

/// Get quarterly summaries with category breakdown for a specific year.
///
/// # Arguments
/// * `uow` - Unit of work for database access
/// * `year` - The year to get quarterly summaries for
/// * `currency_code` - The currency code to filter by
///
/// # Returns
/// A vector of quarterly summaries, one for each quarter that has data.
pub async fn get_quarterly_summaries<U>(
    uow: &mut U,
    year: i32,
    currency_code: &str,
) -> Result<Vec<QuarterlySummary>, DomainError>
where
    U: BudgetUowExt + Send,
{
    // Fetch quarterly spending by category from database
    let rows = {
        let mut repo = uow.budget_repo();
        repo.get_quarterly_spending_by_category(year, currency_code)
            .await
            .map_err(DomainError::Infrastructure)?
    };

    // Group by quarter
    let mut quarterly_data: HashMap<i32, Vec<(Category, i64)>> = HashMap::new();

    for (quarter_num, category_str, amount) in rows {
        let category = Category::from_str(&category_str)
            .map_err(|e| DomainError::Validation(format!("Invalid category: {}", e)))?;

        quarterly_data
            .entry(quarter_num)
            .or_default()
            .push((category, amount));
    }

    // Convert to QuarterlySummary objects
    let mut summaries = Vec::new();

    for (quarter_num, categories) in quarterly_data {
        let quarter = quarter_from_number(quarter_num)?;

        // Calculate total for percentage calculations
        let total: i64 = categories.iter().map(|(_, amount)| amount).sum();

        // Create category breakdown
        let category_breakdown: Vec<CategorySpending> = categories
            .into_iter()
            .map(|(category, amount)| {
                let percentage = if total > 0 {
                    (amount as f64 / total as f64) * 100.0
                } else {
                    0.0
                };

                CategorySpending {
                    category,
                    amount: MonetaryAmount::new(
                        amount,
                        Currency::from_code(currency_code).expect("Invalid currency"),
                    ),
                    percentage,
                }
            })
            .collect();

        summaries.push(QuarterlySummary::new(year, quarter, category_breakdown));
    }

    // Sort by quarter
    summaries.sort_by_key(|s| match s.quarter {
        BudgetQuarter::Q1 => 1,
        BudgetQuarter::Q2 => 2,
        BudgetQuarter::Q3 => 3,
        BudgetQuarter::Q4 => 4,
    });

    Ok(summaries)
}

/// Convert quarter number (1-4) to BudgetQuarter enum.
fn quarter_from_number(num: i32) -> Result<BudgetQuarter, DomainError> {
    match num {
        1 => Ok(BudgetQuarter::Q1),
        2 => Ok(BudgetQuarter::Q2),
        3 => Ok(BudgetQuarter::Q3),
        4 => Ok(BudgetQuarter::Q4),
        _ => Err(DomainError::Validation(format!(
            "Invalid quarter number: {}",
            num
        ))),
    }
}
