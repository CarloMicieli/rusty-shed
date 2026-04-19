use crate::budget::domain::BudgetConfiguration;
use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::dashboard::{
    BudgetDashboardSummary, BudgetQuarter, MonthlySpendingPoint, QuarterlyActivityPoint,
    SpendingLevel,
};
use crate::budget::domain::monthly_budget_record::{MonthStatus, MonthlyBudgetRecord};
use crate::core::domain::calendar::Year;
use crate::core::domain::domain_error::DomainError;
use chrono::Datelike;

async fn get_budget_config<U>(uow: &mut U) -> Result<Option<BudgetConfiguration>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let mut repo = uow.budget_repo();
    repo.get_config().await
}

async fn get_monthly_spending_totals<U>(
    uow: &mut U,
    year: i32,
    currency_code: &str,
) -> Result<[i64; 12], DomainError>
where
    U: BudgetUowExt + Send,
{
    let monthly_spending = {
        let mut repo = uow.budget_repo();
        repo.get_monthly_spending(year, currency_code)
            .await
            .map_err(DomainError::Infrastructure)?
    };

    let mut spending_totals = [0i64; 12];
    for (month, amount) in monthly_spending {
        if let Some(total) = spending_totals.get_mut((month.saturating_sub(1)) as usize) {
            *total = amount;
        }
    }

    Ok(spending_totals)
}

async fn get_extra_budget_totals<U>(uow: &mut U, year: i32) -> Result<[i64; 12], DomainError>
where
    U: BudgetUowExt + Send,
{
    let extra_budgets = {
        let mut repo = uow.budget_repo();
        repo.get_extra_budgets(year).await?
    };

    let mut extra_totals = [0i64; 12];
    for extra in extra_budgets {
        let month_index = (extra.month.value().saturating_sub(1)) as usize;
        if let Some(total) = extra_totals.get_mut(month_index) {
            *total += extra.amount.amount;
        }
    }

    Ok(extra_totals)
}

fn build_monthly_budget_records(
    year: Year,
    config: &BudgetConfiguration,
    monthly_spending: [i64; 12],
    extra_totals: [i64; 12],
) -> Vec<MonthlyBudgetRecord> {
    let current_year = chrono::Utc::now().year();
    let current_month = chrono::Utc::now().month() as u8;
    let base_monthly = config.monthly_amount();
    let mut records = Vec::with_capacity(12);
    let mut rollover = 0;

    for month in 1..=12 {
        let index = (month - 1) as usize;
        let actual_spend = monthly_spending[index];
        let extra = extra_totals[index];

        let status = if year.value() > current_year
            || (year.value() == current_year && month > current_month)
        {
            MonthStatus::Projected
        } else if year.value() == current_year && month == current_month {
            MonthStatus::InProgress
        } else {
            MonthStatus::Completed
        };

        let available = base_monthly + extra + rollover;
        let remaining = available - actual_spend;
        let rollover_out = if remaining > 0 { remaining } else { 0 };

        records.push(MonthlyBudgetRecord {
            year: year.value(),
            month,
            base_budget: base_monthly,
            extra_budget: extra,
            actual_spend,
            rollover_in: rollover,
            rollover_out,
            status,
            currency: config.base_amount.currency,
        });

        rollover = rollover_out;
    }

    records
}

async fn get_monthly_budget_records_for_config<U>(
    uow: &mut U,
    year: Year,
    config: &BudgetConfiguration,
) -> Result<Vec<MonthlyBudgetRecord>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let monthly_spending =
        get_monthly_spending_totals(uow, year.value(), config.base_amount.currency.to_code())
            .await?;
    let extra_totals = get_extra_budget_totals(uow, year.value()).await?;

    Ok(build_monthly_budget_records(
        year,
        config,
        monthly_spending,
        extra_totals,
    ))
}

fn resolve_dashboard_currency<'a>(
    config: Option<&BudgetConfiguration>,
    user_currency: &'a str,
) -> (crate::core::domain::currency::Currency, &'a str) {
    let currency_code = config
        .map(|c| c.base_amount.currency.to_code())
        .unwrap_or(user_currency);

    let currency = config.map(|c| c.base_amount.currency).unwrap_or_else(|| {
        use crate::core::domain::currency::Currency;
        Currency::from_code(user_currency).unwrap_or(Currency::EUR)
    });

    (currency, currency_code)
}

fn build_monthly_spending_points(
    currency: crate::core::domain::currency::Currency,
    current_year: i32,
    all_spending: &[(i32, i32, i64)],
    current_year_records: Option<&[MonthlyBudgetRecord]>,
) -> Vec<MonthlySpendingPoint> {
    let mut monthly_totals = [0i64; 12];

    if let Some(records) = current_year_records {
        for record in records {
            monthly_totals[(record.month - 1) as usize] = record.actual_spend;
        }
    } else {
        for &(year, month, amount) in all_spending {
            if year != current_year {
                continue;
            }

            if let Some(total) = monthly_totals.get_mut((month.saturating_sub(1)) as usize) {
                *total = amount;
            }
        }
    }

    monthly_totals
        .into_iter()
        .enumerate()
        .map(|(index, amount)| MonthlySpendingPoint {
            month: (index + 1) as u8,
            amount,
            currency,
        })
        .collect()
}

fn build_quarterly_activity_points(
    start_year: i32,
    current_year: i32,
    all_spending: &[(i32, i32, i64)],
) -> Vec<QuarterlyActivityPoint> {
    let year_count = (current_year - start_year + 1) as usize;
    let mut quarter_totals = vec![[0i64; 4]; year_count];

    for &(year, month, amount) in all_spending {
        if !(start_year..=current_year).contains(&year) {
            continue;
        }

        let year_index = (year - start_year) as usize;
        let quarter_index = (BudgetQuarter::from_month(month as u8).number() - 1) as usize;
        quarter_totals[year_index][quarter_index] += amount;
    }

    let mut quarterly_activity = Vec::with_capacity(year_count * 4);
    for (year_index, totals) in quarter_totals.into_iter().enumerate() {
        let year = start_year + year_index as i32;
        let max_quarter = totals.iter().copied().max().unwrap_or(0);

        for (quarter_index, total) in totals.into_iter().enumerate() {
            let percentage = if max_quarter > 0 {
                (total as f64 / max_quarter as f64) * 100.0
            } else {
                0.0
            };

            quarterly_activity.push(QuarterlyActivityPoint {
                year,
                quarter: match quarter_index {
                    0 => BudgetQuarter::Q1,
                    1 => BudgetQuarter::Q2,
                    2 => BudgetQuarter::Q3,
                    _ => BudgetQuarter::Q4,
                },
                spending_level: SpendingLevel::from_percentage(percentage),
                amount: total,
            });
        }
    }

    quarterly_activity
}

async fn get_budget_dashboard_with_context<U>(
    uow: &mut U,
    user_currency: &str,
    config: Option<&BudgetConfiguration>,
    current_year_records: Option<&[MonthlyBudgetRecord]>,
) -> Result<BudgetDashboardSummary, DomainError>
where
    U: BudgetUowExt + Send,
{
    let now = chrono::Utc::now();
    let current_year = now.year();
    let current_month = now.month() as u8;
    let (currency, currency_code) = resolve_dashboard_currency(config, user_currency);

    let start_year = current_year - 4;
    let all_spending = {
        let mut repo = uow.budget_repo();
        repo.get_multi_year_monthly_spending(start_year, current_year, currency_code)
            .await
            .map_err(DomainError::Infrastructure)?
    };

    let owned_current_year_records = if current_year_records.is_none() {
        match config {
            Some(config) => Some(
                get_monthly_budget_records_for_config(
                    uow,
                    Year::try_from(current_year)
                        .map_err(|e| DomainError::Validation(e.to_string()))?,
                    config,
                )
                .await?,
            ),
            None => None,
        }
    } else {
        None
    };

    let current_year_records = current_year_records.or(owned_current_year_records.as_deref());

    let (remaining_amount, remaining_percentage, total_available, monthly_goal) =
        if let (Some(config), Some(records)) = (config, current_year_records) {
            let current_record = records
                .iter()
                .find(|record| record.month == current_month)
                .ok_or_else(|| {
                    DomainError::BusinessRule("Current month record not found".to_string())
                })?;

            (
                Some(current_record.remaining()),
                Some(current_record.remaining_percentage()),
                Some(current_record.available()),
                Some(config.monthly_amount()),
            )
        } else {
            (None, None, None, None)
        };

    Ok(BudgetDashboardSummary {
        remaining_amount,
        remaining_percentage,
        total_available,
        currency,
        monthly_spending: build_monthly_spending_points(
            currency,
            current_year,
            &all_spending,
            current_year_records,
        ),
        monthly_goal,
        quarterly_activity: build_quarterly_activity_points(
            start_year,
            current_year,
            &all_spending,
        ),
    })
}

/// Aggregate result for the Finance page bootstrap request.
#[derive(Debug, Clone)]
pub struct BudgetBootstrapData {
    pub config: Option<BudgetConfiguration>,
    pub dashboard_summary: BudgetDashboardSummary,
    pub monthly_records: Option<Vec<MonthlyBudgetRecord>>,
}

/// Calculate monthly budget records for a given year with rollover chain.
///
/// This function derives the rollover chain from spending data and extra budgets,
/// computing each month's available budget, spending, and rollover to the next month.
///
/// # Arguments
/// * `uow` - Unit of work for database access
/// * `year` - The year to calculate records for
///
/// # Returns
/// A vector of 12 `MonthlyBudgetRecord` objects, one for each month of the year.
pub async fn get_monthly_budget_records<U>(
    uow: &mut U,
    year: Year,
) -> Result<Vec<MonthlyBudgetRecord>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let config = get_budget_config(uow)
        .await?
        .ok_or_else(|| DomainError::Validation("Budget configuration not set".to_string()))?;

    get_monthly_budget_records_for_config(uow, year, &config).await
}

/// Get budget dashboard summary for widgets.
///
/// This function aggregates:
/// - Current month remaining budget (donut chart)
/// - 12-month spending data (bar chart)
/// - 5-year quarterly activity (heatmap)
///
/// # Arguments
/// * `uow` - Unit of work for database access
/// * `user_currency` - User's preferred currency code from settings (used when no budget configured)
///
/// # Returns
/// A `BudgetDashboardSummary` with spending data always populated. Budget-specific fields
/// (remaining_amount, remaining_percentage, total_available, monthly_goal) are Some() if
/// budget is configured, None otherwise.
pub async fn get_budget_dashboard<U>(
    uow: &mut U,
    user_currency: &str,
) -> Result<BudgetDashboardSummary, DomainError>
where
    U: BudgetUowExt + Send,
{
    let config = get_budget_config(uow).await?;
    get_budget_dashboard_with_context(uow, user_currency, config.as_ref(), None).await
}

/// Get the combined Finance page bootstrap payload.
///
/// This query shares one unit-of-work and computes the current dashboard summary and the
/// requested year's monthly records from the same budget configuration lookup.
pub async fn get_budget_bootstrap<U>(
    uow: &mut U,
    year: Year,
    user_currency: &str,
) -> Result<BudgetBootstrapData, DomainError>
where
    U: BudgetUowExt + Send,
{
    let config = get_budget_config(uow).await?;
    let current_year = chrono::Utc::now().year();

    let monthly_records = if let Some(config) = config.as_ref() {
        Some(get_monthly_budget_records_for_config(uow, year, config).await?)
    } else {
        None
    };

    let dashboard_summary = get_budget_dashboard_with_context(
        uow,
        user_currency,
        config.as_ref(),
        if year.value() == current_year {
            monthly_records.as_deref()
        } else {
            None
        },
    )
    .await?;

    Ok(BudgetBootstrapData {
        config,
        dashboard_summary,
        monthly_records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::application::testing::{FakeBudgetUow, sample_budget_config};
    use crate::budget::domain::repository::MockBudgetRepository;
    use crate::core::domain::calendar::Year;

    /// Helper: a mock that returns empty spending and empty extra budget.
    fn mock_empty_spending() -> MockBudgetRepository {
        let mut m = MockBudgetRepository::new();
        m.expect_get_monthly_spending()
            .once()
            .returning(|_, _| Ok(vec![]));
        m
    }

    fn mock_empty_extra_budgets() -> MockBudgetRepository {
        let mut m = MockBudgetRepository::new();
        m.expect_get_extra_budgets()
            .once()
            .returning(|_| Ok(vec![]));
        m
    }

    #[tokio::test]
    async fn it_should_return_12_monthly_records() {
        // Arrange – three budget_repo() calls: get_config, get_monthly_spending, get_extra_budgets
        let config = sample_budget_config();

        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(config.clone())));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_empty_spending())
            .with_repo(mock_empty_extra_budgets());

        let year = Year::try_from(2025).unwrap();

        // Act
        let result = get_monthly_budget_records(&mut uow, year).await;

        // Assert
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        let records = result.unwrap();
        assert_eq!(records.len(), 12, "Expected 12 monthly records");
    }

    #[tokio::test]
    async fn it_should_return_all_12_months_numbered_correctly() {
        let config = sample_budget_config();

        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(config.clone())));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_empty_spending())
            .with_repo(mock_empty_extra_budgets());

        let year = Year::try_from(2025).unwrap();
        let records = get_monthly_budget_records(&mut uow, year)
            .await
            .expect("expected Ok");

        for (i, record) in records.iter().enumerate() {
            assert_eq!(
                record.month,
                (i + 1) as u8,
                "Month at index {i} should be {}",
                i + 1
            );
            assert_eq!(record.year, 2025);
        }
    }

    #[tokio::test]
    async fn it_should_fail_when_no_budget_config_set() {
        // Arrange – only get_config is called; returns None → Validation error
        let mut mock_get = MockBudgetRepository::new();
        mock_get.expect_get_config().once().returning(|| Ok(None));

        let mut uow = FakeBudgetUow::new().with_repo(mock_get);
        let year = Year::try_from(2025).unwrap();

        // Act
        let result = get_monthly_budget_records(&mut uow, year).await;

        // Assert
        assert!(
            matches!(result, Err(DomainError::Validation(_))),
            "Expected Validation error, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn it_should_calculate_rollover_chain() {
        // Arrange – base budget = 100_000 per month, no extra, spending in Jan = 60_000
        // → rollover Jan→Feb = 40_000
        use crate::budget::domain::BudgetMode;
        use crate::core::domain::Currency;
        use crate::core::domain::monetary_amount::MonetaryAmount;

        let config = crate::budget::domain::BudgetConfiguration::new(
            BudgetMode::Monthly,
            MonetaryAmount::new(100_000, Currency::EUR),
        );

        let mut mock_get = MockBudgetRepository::new();
        mock_get
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(config.clone())));

        // January spending = 60_000 (month 1)
        let mut mock_spending = MockBudgetRepository::new();
        mock_spending
            .expect_get_monthly_spending()
            .once()
            .returning(|_, _| Ok(vec![(1, 60_000)]));

        let mut mock_extra = MockBudgetRepository::new();
        mock_extra
            .expect_get_extra_budgets()
            .once()
            .returning(|_| Ok(vec![]));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get)
            .with_repo(mock_spending)
            .with_repo(mock_extra);

        let year = Year::try_from(2020).unwrap(); // past year → all Completed

        // Act
        let records = get_monthly_budget_records(&mut uow, year)
            .await
            .expect("expected Ok");

        // January: available 100_000, spent 60_000, rollover_out 40_000
        let jan = &records[0];
        assert_eq!(jan.month, 1);
        assert_eq!(jan.actual_spend, 60_000);
        assert_eq!(jan.rollover_out, 40_000);

        // February: rollover_in should be the Jan rollover_out
        let feb = &records[1];
        assert_eq!(feb.month, 2);
        assert_eq!(feb.rollover_in, 40_000);
        assert_eq!(feb.base_budget, 100_000);
    }

    #[tokio::test]
    async fn it_should_return_budget_bootstrap_without_duplicate_year_records_query() {
        let config = sample_budget_config();

        let mut mock_get_config = MockBudgetRepository::new();
        mock_get_config
            .expect_get_config()
            .once()
            .returning(move || Ok(Some(config.clone())));

        let mut mock_dashboard_spending = MockBudgetRepository::new();
        mock_dashboard_spending
            .expect_get_multi_year_monthly_spending()
            .once()
            .returning(|_, _, _| Ok(vec![]));

        let mut mock_current_year_spending = MockBudgetRepository::new();
        mock_current_year_spending
            .expect_get_monthly_spending()
            .once()
            .returning(|_, _| Ok(vec![]));

        let mut mock_current_year_extra = MockBudgetRepository::new();
        mock_current_year_extra
            .expect_get_extra_budgets()
            .once()
            .returning(|_| Ok(vec![]));

        let mut uow = FakeBudgetUow::new()
            .with_repo(mock_get_config)
            .with_repo(mock_current_year_spending)
            .with_repo(mock_current_year_extra)
            .with_repo(mock_dashboard_spending);

        let year = Year::try_from(chrono::Utc::now().year()).unwrap();

        let result = get_budget_bootstrap(&mut uow, year, "EUR")
            .await
            .expect("expected bootstrap query to succeed");

        assert!(result.config.is_some());
        assert!(result.monthly_records.is_some());
        assert_eq!(result.dashboard_summary.monthly_spending.len(), 12);
    }
}
