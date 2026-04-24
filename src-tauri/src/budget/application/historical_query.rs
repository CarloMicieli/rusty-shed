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
/// * `unit_of_work` - Unit of work for database access
/// * `year` - The year to get quarterly summaries for
/// * `currency_code` - The currency code to filter by
///
/// # Returns
/// A vector of quarterly summaries, one for each quarter that has data.
pub async fn get_quarterly_summaries<U>(
    unit_of_work: &mut U,
    year: Year,
    currency_code: &str,
) -> Result<Vec<QuarterlySummary>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let rows = {
        let mut repo = unit_of_work.budget_repo();
        repo.get_quarterly_spending_by_category(year.value(), currency_code)
            .await
            .map_err(DomainError::Infrastructure)?
    };

    let mut quarterly_data: HashMap<i32, Vec<(Category, i64)>> = HashMap::new();

    for (quarter_num, category_str, amount) in rows {
        let category = Category::from_str(&category_str)
            .map_err(|e| DomainError::Validation(format!("Invalid category: {}", e)))?;

        quarterly_data
            .entry(quarter_num)
            .or_default()
            .push((category, amount));
    }

    let mut summaries = Vec::new();

    for (quarter_num, categories) in quarterly_data {
        let quarter = quarter_from_number(quarter_num)?;

        let total: i64 = categories.iter().map(|(_, amount)| amount).sum();

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
    use rstest::rstest;

    #[tokio::test]
    async fn it_should_return_empty_when_no_data() {
        let mut mock = MockBudgetRepository::new();
        mock.expect_get_quarterly_spending_by_category()
            .once()
            .returning(|_, _| Ok(vec![]));

        let mut uow = FakeBudgetUow::new().with_repo(mock);
        let year = Year::try_from(2025).unwrap();

        let result = get_quarterly_summaries(&mut uow, year, "EUR").await;

        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn it_should_group_spending_by_quarter() {
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

        let result = get_quarterly_summaries(&mut uow, year, "EUR")
            .await
            .expect("expected Ok");

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].quarter, BudgetQuarter::Q1);
        assert_eq!(result[1].quarter, BudgetQuarter::Q2);

        let q1_total: i64 = result[0]
            .category_breakdown
            .iter()
            .map(|c| c.amount.amount)
            .sum();
        assert_eq!(q1_total, 50_000);
    }

    #[tokio::test]
    async fn it_should_fail_on_invalid_category_string() {
        let rows = vec![(1i32, "INVALID_CATEGORY_XYZ".to_string(), 10_000i64)];

        let mut mock = MockBudgetRepository::new();
        mock.expect_get_quarterly_spending_by_category()
            .once()
            .returning(move |_, _| Ok(rows.clone()));

        let mut uow = FakeBudgetUow::new().with_repo(mock);
        let year = Year::try_from(2025).unwrap();

        let result = get_quarterly_summaries(&mut uow, year, "EUR").await;

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
    #[rstest]
    #[case(1, BudgetQuarter::Q1)]
    #[case(2, BudgetQuarter::Q2)]
    #[case(3, BudgetQuarter::Q3)]
    #[case(4, BudgetQuarter::Q4)]
    fn quarter_from_number_returns_expected_quarter(
        #[case] input: i32,
        #[case] expected: BudgetQuarter,
    ) {
        let res = quarter_from_number(input).expect("expected Ok");
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(0)]
    #[case(5)]
    #[case(-1)]
    #[case(100)]
    fn quarter_from_number_returns_validation_error_for_invalid_numbers(#[case] input: i32) {
        match quarter_from_number(input) {
            Err(DomainError::Validation(msg)) => {
                // ensure error message contains the invalid number (keeps checks specific)
                assert!(
                    msg.contains(&input.to_string()),
                    "error message should mention the invalid number; got: {}",
                    msg
                );
            }
            other => panic!("expected DomainError::Validation, got: {:?}", other),
        }
    }
}
