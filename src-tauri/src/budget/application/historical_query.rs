use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::dashboard::BudgetQuarter;
use crate::budget::domain::quarterly_summary::{CategorySpending, QuarterlySummary};
use crate::catalog::domain::railway_model::Category;
use crate::core::domain::Currency;
use crate::core::domain::calendar::Year;
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
    year: Year,
    currency_code: &str,
) -> Result<Vec<QuarterlySummary>, DomainError>
where
    U: BudgetUowExt + Send,
{
    // Fetch quarterly spending by category from database
    let rows = {
        let mut repo = uow.budget_repo();
        repo.get_quarterly_spending_by_category(year.value(), currency_code)
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

        summaries.push(QuarterlySummary::new(
            year.value(),
            quarter,
            category_breakdown,
        ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::application::testing::FakeBudgetUow;
    use crate::budget::domain::repository::MockBudgetRepository;
    use crate::core::domain::calendar::Year;

    #[tokio::test]
    async fn it_should_return_empty_when_no_data() {
        // Arrange – one budget_repo() call; returns empty rows
        let mut mock = MockBudgetRepository::new();
        mock.expect_get_quarterly_spending_by_category()
            .once()
            .returning(|_, _| Ok(vec![]));

        let mut uow = FakeBudgetUow::new().with_repo(mock);
        let year = Year::try_from(2025).unwrap();

        // Act
        let result = get_quarterly_summaries(&mut uow, year, "EUR").await;

        // Assert
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn it_should_group_spending_by_quarter() {
        // Arrange – Q1 has Locomotives spending, Q2 has FreightCars spending
        let rows = vec![
            (1i32, "LOCOMOTIVES".to_string(), 50_000i64),
            (2i32, "FREIGHT_CARS".to_string(), 30_000i64),
        ];

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_quarterly_spending_by_category()
            .once()
            .returning(move |_, _| Ok(rows.clone()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);
        let year = Year::try_from(2025).unwrap();

        // Act
        let result = get_quarterly_summaries(&mut uow, year, "EUR")
            .await
            .expect("expected Ok");

        // Assert – two quarters returned, sorted Q1 before Q2
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].quarter, BudgetQuarter::Q1);
        assert_eq!(result[1].quarter, BudgetQuarter::Q2);

        // Q1 total
        let q1_total: i64 = result[0]
            .category_breakdown
            .iter()
            .map(|c| c.amount.amount)
            .sum();
        assert_eq!(q1_total, 50_000);
    }

    #[tokio::test]
    async fn it_should_fail_on_invalid_category_string() {
        // Arrange – mock returns a category string not in the Category enum
        let rows = vec![(1i32, "INVALID_CATEGORY_XYZ".to_string(), 10_000i64)];

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_quarterly_spending_by_category()
            .once()
            .returning(move |_, _| Ok(rows.clone()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);
        let year = Year::try_from(2025).unwrap();

        // Act
        let result = get_quarterly_summaries(&mut uow, year, "EUR").await;

        // Assert
        assert!(
            matches!(result, Err(DomainError::Validation(_))),
            "Expected Validation error for invalid category, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_propagate_repository_error() {
        let mut mock = MockBudgetRepository::new();
        mock.expect_get_quarterly_spending_by_category()
            .once()
            .returning(|_, _| Err("db read error".to_string()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);
        let year = Year::try_from(2025).unwrap();

        let result = get_quarterly_summaries(&mut uow, year, "EUR").await;

        assert!(
            matches!(result, Err(DomainError::Infrastructure(_))),
            "Expected Infrastructure error, got: {:?}",
            result
        );
    }
}
