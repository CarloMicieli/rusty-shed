use crate::budget::domain::BudgetConfiguration;
use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::dashboard::{
    BudgetDashboardSummary, BudgetQuarter, MonthlySpendingPoint, QuarterlyActivityPoint,
    SpendingLevel,
};
use crate::budget::domain::monthly_budget_record::{MonthStatus, MonthlyBudgetRecord};
use crate::core::domain::calendar::Year;
use crate::core::domain::domain_error::DomainError;
use chrono::{DateTime, Datelike, Utc};

/// Aggregate result for the Finance page bootstrap request.
#[derive(Debug, Clone)]
pub struct BudgetBootstrapData {
    /// The budget configuration, if set.
    /// This is needed to interpret the monthly records and dashboard summary,
    /// but may be `None` if no budget has been configured yet.
    pub config: Option<BudgetConfiguration>,
    /// The budget dashboard summary, which includes:
    /// - Remaining budget for the current month (if budget configured)
    /// - 12-month monthly spending points (always populated, using user currency if no budget configured)
    /// - 5-year quarterly activity points (always populated, using user currency if no budget configured)
    pub dashboard_summary: BudgetDashboardSummary,
    /// The monthly budget records
    pub monthly_records: Option<Vec<MonthlyBudgetRecord>>,
}

/// Calculate monthly budget records for a given year with rollover chain.
///
/// This function derives the rollover chain from spending data and extra budgets,
/// computing each month's available budget, spending, and rollover to the next month.
///
/// # Arguments
/// * `unit_of_work` - Unit of work for database access
/// * `year` - The year to calculate records for
///
/// # Returns
/// A vector of 12 `MonthlyBudgetRecord` objects, one for each month of the year.
pub async fn get_monthly_budget_records<U>(
    unit_of_work: &mut U,
    year: Year,
) -> Result<Vec<MonthlyBudgetRecord>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let config = get_budget_config(unit_of_work)
        .await?
        .ok_or_else(|| DomainError::Validation("Budget configuration not set".to_string()))?;

    get_monthly_budget_records_for_config(unit_of_work, year, &config).await
}

/// Get budget dashboard summary for widgets.
///
/// This function aggregates:
/// - Current month remaining budget (donut chart)
/// - 12-month spending data (bar chart)
/// - 5-year quarterly activity (heatmap)
///
/// # Arguments
/// * `unit_of_work` - Unit of work for database access
/// * `user_currency` - User's preferred currency code from settings (used when no budget configured)
///
/// # Returns
/// A `BudgetDashboardSummary` with spending data always populated. Budget-specific fields
/// (remaining_amount, remaining_percentage, total_available, monthly_goal) are Some() if
/// budget is configured, None otherwise.
pub async fn get_budget_dashboard<U>(
    unit_of_work: &mut U,
    user_currency: &str,
) -> Result<BudgetDashboardSummary, DomainError>
where
    U: BudgetUowExt + Send,
{
    let config = get_budget_config(unit_of_work).await?;
    let now = Utc::now();
    get_budget_dashboard_with_context(unit_of_work, now, user_currency, config.as_ref(), None).await
}

/// Get the combined Finance page bootstrap payload.
///
/// This query shares one unit-of-work and computes the current dashboard summary and the
/// requested year's monthly records from the same budget configuration lookup.
pub async fn get_budget_bootstrap<U>(
    unit_of_work: &mut U,
    year: Year,
    user_currency: &str,
) -> Result<BudgetBootstrapData, DomainError>
where
    U: BudgetUowExt + Send,
{
    let config = get_budget_config(unit_of_work).await?;
    let now = Utc::now();
    let current_year = now.year();

    let monthly_records = if let Some(config) = config.as_ref() {
        Some(get_monthly_budget_records_for_config(unit_of_work, year, config).await?)
    } else {
        None
    };

    let dashboard_summary = get_budget_dashboard_with_context(
        unit_of_work,
        now,
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

/// Returns the budget configuration (eg, the budget mode and base amount) if set, or None if no budget configured.
async fn get_budget_config<U>(
    unit_of_work: &mut U,
) -> Result<Option<BudgetConfiguration>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let mut repo = unit_of_work.budget_repo();
    repo.get_config().await
}

async fn get_monthly_budget_records_for_config<U>(
    unit_of_work: &mut U,
    year: Year,
    config: &BudgetConfiguration,
) -> Result<Vec<MonthlyBudgetRecord>, DomainError>
where
    U: BudgetUowExt + Send,
{
    let monthly_spending = get_monthly_spending_totals(
        unit_of_work,
        year.value(),
        config.base_amount.currency.to_code(),
    )
    .await?;
    let extra_totals = get_extra_budget_totals(unit_of_work, year.value()).await?;

    Ok(build_monthly_budget_records(
        year,
        config,
        monthly_spending,
        extra_totals,
    ))
}

/// Returns the monthly spending by month for the given `year`
async fn get_monthly_spending_totals<U>(
    unit_of_work: &mut U,
    year: i32,
    currency_code: &str,
) -> Result<[i64; 12], DomainError>
where
    U: BudgetUowExt + Send,
{
    let monthly_spending = {
        let mut repo = unit_of_work.budget_repo();
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

/// Returns the extra budgets by month for the given `year`, aggregated into totals per month.
async fn get_extra_budget_totals<U>(
    unit_of_work: &mut U,
    year: i32,
) -> Result<[i64; 12], DomainError>
where
    U: BudgetUowExt + Send,
{
    let extra_budgets = {
        let mut repo = unit_of_work.budget_repo();
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
    let now = Utc::now();
    let current_year = now.year();
    let current_month = now.month() as u8;
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
    unit_of_work: &mut U,
    now: DateTime<Utc>,
    user_currency: &str,
    config: Option<&BudgetConfiguration>,
    current_year_records: Option<&[MonthlyBudgetRecord]>,
) -> Result<BudgetDashboardSummary, DomainError>
where
    U: BudgetUowExt + Send,
{
    let current_year = now.year();
    let current_month = now.month() as u8;
    let (currency, currency_code) = resolve_dashboard_currency(config, user_currency);

    let start_year = current_year - 4;
    let all_spending = {
        let mut repo = unit_of_work.budget_repo();
        repo.get_multi_year_monthly_spending(start_year, current_year, currency_code)
            .await
            .map_err(DomainError::Infrastructure)?
    };

    let owned_current_year_records = if current_year_records.is_none() {
        match config {
            Some(config) => Some(
                get_monthly_budget_records_for_config(
                    unit_of_work,
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

        let result = get_monthly_budget_records(&mut uow, year).await;

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
        let mut mock_get = MockBudgetRepository::new();
        mock_get.expect_get_config().once().returning(|| Ok(None));

        let mut uow = FakeBudgetUow::new().with_repo(mock_get);
        let year = Year::try_from(2025).unwrap();

        let result = get_monthly_budget_records(&mut uow, year).await;

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

        let year = Year::try_from(Utc::now().year()).unwrap();

        let result = get_budget_bootstrap(&mut uow, year, "EUR")
            .await
            .expect("expected bootstrap query to succeed");

        assert!(result.config.is_some());
        assert!(result.monthly_records.is_some());
        assert_eq!(result.dashboard_summary.monthly_spending.len(), 12);
    }

    mod get_budget_dashboard_with_context_tests {
        use super::*;
        use crate::budget::BudgetMode;
        use crate::core::domain::{Currency, MonetaryAmount};
        use chrono::{TimeZone, Utc};
        use mockall::predicate::*;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn test_get_dashboard_calculates_correct_remaining_values() {
            let mut mock_repo = MockBudgetRepository::new();
            mock_repo
                .expect_get_multi_year_monthly_spending()
                .with(eq(2022), eq(2026), eq("USD"))
                .times(1)
                .returning(|_, _, _| Ok(vec![]));

            let mut uow = FakeBudgetUow::new().with_repo(mock_repo);

            let now = Utc.with_ymd_and_hms(2026, 4, 15, 0, 0, 0).unwrap();
            let user_currency = "USD";

            let amount = MonetaryAmount::new(1000, Currency::USD);
            let config = BudgetConfiguration::new(BudgetMode::Monthly, amount);

            let records = vec![MonthlyBudgetRecord {
                month: 4,
                base_budget: 1000,
                actual_spend: 400,
                rollover_in: 100,
                extra_budget: 0,
                year: 2026,
                rollover_out: 0,
                currency: Currency::USD,
                status: MonthStatus::InProgress,
            }];

            let result = get_budget_dashboard_with_context(
                &mut uow,
                now,
                user_currency,
                Some(&config),
                Some(&records),
            )
            .await
            .unwrap();

            assert_eq!(result.remaining_amount, Some(700));
            assert_eq!(result.total_available, Some(1100));
            assert_eq!(result.monthly_goal, Some(1000));
            assert_eq!(result.currency, Currency::USD);
        }

        #[tokio::test]
        async fn test_fails_when_current_month_missing() {
            let mut mock_repo = MockBudgetRepository::new();
            mock_repo
                .expect_get_multi_year_monthly_spending()
                .returning(|_, _, _| Ok(vec![]));

            let mut uow = FakeBudgetUow::new().with_repo(mock_repo);

            let now = Utc.with_ymd_and_hms(2026, 12, 1, 0, 0, 0).unwrap(); // December

            let amount = MonetaryAmount::new(1000, Currency::USD);
            let config = BudgetConfiguration::new(BudgetMode::Monthly, amount);
            let records = vec![]; // Empty records will fail the find() for month 12

            let result = get_budget_dashboard_with_context(
                &mut uow,
                now,
                "USD",
                Some(&config),
                Some(&records),
            )
            .await;

            match result {
                Err(DomainError::BusinessRule(msg)) => {
                    assert!(msg.contains("month record not found"))
                }
                _ => panic!("Expected BusinessRule error"),
            }
        }
    }

    mod build_quarterly_activity_points_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_quarterly_mapping_and_relative_spending() {
            let start_year = 2024;
            let current_year = 2025;

            // Data:
            // 2024 Q1: 1000 (Max) -> 100%
            // 2024 Q2: 500        -> 50%
            // 2025 Q1: 2000 (Max) -> 100% (Independent of 2024)
            let spending = vec![
                (2024, 1, 1000), // Q1
                (2024, 4, 500),  // Q2
                (2025, 2, 2000), // Q1
            ];

            let result = build_quarterly_activity_points(start_year, current_year, &spending);

            // Result size should be (2025 - 2024 + 1) * 4 = 8
            assert_eq!(result.len(), 8);

            // Verify 2024 Q1 (Max for 2024)
            let q1_2024 = &result[0];
            assert_eq!(q1_2024.year, 2024);
            assert_eq!(q1_2024.amount, 1000);
            assert_eq!(q1_2024.spending_level, SpendingLevel::High); // 100%

            // Verify 2024 Q2 (50% of 2024 Max)
            let q2_2024 = &result[1];
            assert_eq!(q2_2024.amount, 500);
            assert_eq!(q2_2024.spending_level, SpendingLevel::Medium); // 50%

            // Verify 2025 Q1 (Max for 2025 - check isolation)
            let q1_2025 = &result[4];
            assert_eq!(q1_2025.amount, 2000);
            assert_eq!(q1_2025.spending_level, SpendingLevel::High); // 100%
        }

        #[test]
        fn test_ignores_out_of_range_years() {
            let start_year = 2025;
            let current_year = 2025;

            // Spending in 2024 should be ignored
            let spending = vec![(2024, 1, 1000), (2025, 1, 500)];

            let result = build_quarterly_activity_points(start_year, current_year, &spending);

            assert_eq!(result[0].year, 2025);
            assert_eq!(result[0].amount, 500);
            // If 2024 wasn't ignored, Q1 2025 wouldn't be the max
            assert_eq!(result[0].spending_level, SpendingLevel::High);
        }

        #[test]
        fn test_handles_zero_spending_gracefully() {
            let start_year = 2026;
            let current_year = 2026;
            let spending = vec![];

            let result = build_quarterly_activity_points(start_year, current_year, &spending);

            assert_eq!(result.len(), 4);
            for point in result {
                assert_eq!(point.amount, 0);
                assert_eq!(point.spending_level, SpendingLevel::None); // Assuming 0% = None
            }
        }

        #[test]
        fn test_aggregates_months_into_quarters() {
            let start_year = 2026;
            let current_year = 2026;
            let spending = vec![(2026, 1, 100), (2026, 2, 200), (2026, 3, 300)];

            let result = build_quarterly_activity_points(start_year, current_year, &spending);

            // Q1 total should be 100+200+300 = 600
            assert_eq!(result[0].amount, 600);
            assert_eq!(result[0].quarter, BudgetQuarter::Q1);
        }
    }

    mod build_monthly_spending_points_tests {
        use super::*;
        use crate::core::domain::Currency;
        use pretty_assertions::assert_eq;

        fn create_monthly_budget_record(month: u8, actual_spend: i64) -> MonthlyBudgetRecord {
            MonthlyBudgetRecord {
                year: 2026,
                month,
                base_budget: 1000,
                extra_budget: 500,
                rollover_in: 200,
                actual_spend,
                rollover_out: 1300, // (1000 + 500 + 200) - 400
                status: MonthStatus::InProgress,
                currency: Currency::USD,
            }
        }

        #[test]
        fn test_uses_records_when_provided() {
            let currency = Currency::USD;
            let current_year = 2026;

            // Data in all_spending that SHOULD BE IGNORED
            let all_spending = vec![(2026, 1, 5000)];

            // Data in records that SHOULD BE USED
            let records = vec![
                create_monthly_budget_record(1, 1000),
                create_monthly_budget_record(5, 2500),
            ];

            let result = build_monthly_spending_points(
                currency,
                current_year,
                &all_spending,
                Some(&records),
            );

            assert_eq!(result.len(), 12);
            // Jan (Month 1) should be from records
            assert_eq!(result[0].amount, 1000);
            // May (Month 5) should be from records
            assert_eq!(result[4].amount, 2500);
            // Other months should be 0
            assert_eq!(result[1].amount, 0);
        }

        #[test]
        fn test_falls_back_to_all_spending() {
            let currency = Currency::USD;
            let current_year = 2026;

            let all_spending = vec![
                (2025, 1, 9999),  // Wrong year, should be ignored
                (2026, 2, 1200),  // Correct year, Feb
                (2026, 12, 3000), // Correct year, Dec
            ];

            let result = build_monthly_spending_points(
                currency,
                current_year,
                &all_spending,
                None, // Fallback triggered
            );

            assert_eq!(result[0].amount, 0); // Jan
            assert_eq!(result[1].amount, 1200); // Feb
            assert_eq!(result[11].amount, 3000); // Dec
        }

        #[test]
        fn test_month_index_safety() {
            let currency = Currency::USD;
            let current_year = 2026;

            let all_spending = vec![
                (2026, 0, 100),  // Invalid
                (2026, 13, 200), // Invalid
                (2026, 1, 500),  // Valid January
            ];

            let result = build_monthly_spending_points(currency, current_year, &all_spending, None);

            // Jan (Index 0) should be 500, not affected by the 100 from month 0
            assert_eq!(result[0].amount, 500);
            assert_eq!(result.len(), 12);

            // Ensure no other months were accidentally populated
            assert!(result.iter().skip(1).all(|r| r.amount == 0));
        }

        #[test]
        fn test_empty_data_returns_twelve_zeros() {
            let result = build_monthly_spending_points(Currency::USD, 2026, &[], None);

            assert_eq!(result.len(), 12);
            assert!(result.iter().all(|p| p.amount == 0));
        }
    }
}
