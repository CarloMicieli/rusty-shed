// Budget Query Service
// Feature: 001-budget-tracking - Phase 4 (US2) & Phase 5 (US3)

use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::dashboard::{
    BudgetDashboardSummary, BudgetQuarter, MonthlySpendingPoint, QuarterlyActivityPoint,
    SpendingLevel,
};
use crate::budget::domain::monthly_budget_record::{MonthStatus, MonthlyBudgetRecord};
use crate::core::domain::domain_error::DomainError;
use chrono::Datelike;

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
    year: i32,
) -> Result<Vec<MonthlyBudgetRecord>, DomainError>
where
    U: BudgetUowExt + Send,
{
    // Get budget configuration
    let config_option = {
        let mut repo = uow.budget_repo();
        repo.get_config().await?
    };

    let config = config_option
        .ok_or_else(|| DomainError::Validation("Budget configuration not set".to_string()))?;

    // Check if annual reset is needed
    let current_year = chrono::Utc::now().year();
    let _needs_reset = current_year > config.last_reset_year;

    // Get spending data for the year
    let monthly_spending = {
        let mut repo = uow.budget_repo();
        repo.get_monthly_spending(year, config.base_amount.currency.to_code())
            .await
            .map_err(DomainError::Infrastructure)?
    };

    // Convert to a map for easier lookup
    let mut spending_map: std::collections::HashMap<u8, i64> = std::collections::HashMap::new();
    for (month, amount) in monthly_spending {
        spending_map.insert(month as u8, amount);
    }

    // Get extra budgets for the year
    let extra_budgets = {
        let mut repo = uow.budget_repo();
        repo.get_extra_budgets(year).await?
    };

    // Convert to a map for easier lookup
    let mut extra_map: std::collections::HashMap<u8, i64> = std::collections::HashMap::new();
    for extra in extra_budgets {
        *extra_map.entry(extra.month).or_insert(0) += extra.amount.amount;
    }

    // Calculate monthly records with rollover chain
    let base_monthly = config.monthly_amount();
    let mut records = Vec::with_capacity(12);
    // For now, always start with 0 rollover
    // TODO: In the future, can carry over from previous year for historical data
    let mut rollover = 0;

    let now = chrono::Utc::now();
    let current_month = now.month() as u8;

    for month in 1..=12 {
        let actual_spend = spending_map.get(&month).copied().unwrap_or(0);
        let extra = extra_map.get(&month).copied().unwrap_or(0);

        // Determine month status
        let status = if year > current_year || (year == current_year && month > current_month) {
            MonthStatus::Projected
        } else if year == current_year && month == current_month {
            MonthStatus::InProgress
        } else {
            MonthStatus::Completed
        };

        let available = base_monthly + extra + rollover;
        let remaining = available - actual_spend;

        // Rollover only positive amounts to next month
        let rollover_out = if remaining > 0 { remaining } else { 0 };

        records.push(MonthlyBudgetRecord {
            year,
            month,
            base_budget: base_monthly,
            extra_budget: extra,
            actual_spend,
            rollover_in: rollover,
            rollover_out,
            status,
            currency: config.base_amount.currency,
        });

        // Set rollover for next month
        rollover = rollover_out;
    }

    Ok(records)
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
    // Get budget configuration (optional)
    let config_option = {
        let mut repo = uow.budget_repo();
        repo.get_config().await?
    };

    let now = chrono::Utc::now();
    let current_year = now.year();
    let current_month = now.month() as u8;

    // Determine currency to use
    let currency_code = config_option
        .as_ref()
        .map(|c| c.base_amount.currency.to_code())
        .unwrap_or(user_currency);

    // Parse currency for the response
    let currency = config_option
        .as_ref()
        .map(|c| c.base_amount.currency)
        .unwrap_or_else(|| {
            // Try to parse user currency, fallback to EUR if invalid
            use crate::core::domain::currency::Currency;
            Currency::from_code(user_currency).unwrap_or(Currency::EUR)
        });

    // Fetch all multi-year spending data (covers bar chart + heatmap).
    // Build as Vec<(year, month, amount)> using per-year trait calls.
    let start_year = current_year - 4;
    let mut all_spending: Vec<(i32, i32, i64)> = Vec::new();
    for y in start_year..=current_year {
        let mut repo = uow.budget_repo();
        let year_data = repo
            .get_monthly_spending(y, currency_code)
            .await
            .map_err(DomainError::Infrastructure)?;
        for (month, amount) in year_data {
            all_spending.push((y, month, amount));
        }
    }

    // Get budget-specific data if budget is configured.
    // monthly_records already contain actual_spend per month, so use them for the bar chart.
    let (
        remaining_amount,
        remaining_percentage,
        total_available,
        monthly_goal,
        monthly_records_opt,
    ) = if let Some(ref config) = config_option {
        // Get monthly records for the current year (contains rollover chain + actual spend)
        let monthly_records = get_monthly_budget_records(uow, current_year).await?;

        // Find current month's record
        let current_record = monthly_records
            .iter()
            .find(|r| r.month == current_month)
            .ok_or_else(|| {
                DomainError::BusinessRule("Current month record not found".to_string())
            })?;

        (
            Some(current_record.remaining()),
            Some(current_record.remaining_percentage()),
            Some(current_record.available()),
            Some(config.monthly_amount()),
            Some(monthly_records),
        )
    } else {
        (None, None, None, None, None)
    };

    // Build monthly spending points for bar chart (current year).
    // When budget is configured, use the already-computed monthly records to avoid re-reading
    // current-year data from all_spending (the rollover chain already has actual_spend).
    let monthly_spending: Vec<MonthlySpendingPoint> = (1i32..=12)
        .map(|month| {
            let amount = if let Some(ref records) = monthly_records_opt {
                records
                    .iter()
                    .find(|r| r.month as i32 == month)
                    .map(|r| r.actual_spend)
                    .unwrap_or(0)
            } else {
                all_spending
                    .iter()
                    .find(|(y, m, _)| *y == current_year && *m == month)
                    .map(|(_, _, amt)| *amt)
                    .unwrap_or(0)
            };

            MonthlySpendingPoint {
                month: month as u8,
                amount,
                currency,
            }
        })
        .collect();

    // Build quarterly activity for heatmap (last 5 years) from in-memory data — no extra queries.
    let mut quarterly_activity = Vec::new();

    for year in start_year..=current_year {
        let mut q1_total = 0i64;
        let mut q2_total = 0i64;
        let mut q3_total = 0i64;
        let mut q4_total = 0i64;

        for &(y, month, amount) in &all_spending {
            if y != year {
                continue;
            }
            match BudgetQuarter::from_month(month as u8) {
                BudgetQuarter::Q1 => q1_total += amount,
                BudgetQuarter::Q2 => q2_total += amount,
                BudgetQuarter::Q3 => q3_total += amount,
                BudgetQuarter::Q4 => q4_total += amount,
            }
        }

        // Calculate max spending for the year to determine levels
        let max_quarter = [q1_total, q2_total, q3_total, q4_total]
            .iter()
            .max()
            .copied()
            .unwrap_or(0);

        for (quarter, total) in [
            (BudgetQuarter::Q1, q1_total),
            (BudgetQuarter::Q2, q2_total),
            (BudgetQuarter::Q3, q3_total),
            (BudgetQuarter::Q4, q4_total),
        ] {
            let percentage = if max_quarter > 0 {
                (total as f64 / max_quarter as f64) * 100.0
            } else {
                0.0
            };

            quarterly_activity.push(QuarterlyActivityPoint {
                year,
                quarter,
                spending_level: SpendingLevel::from_percentage(percentage),
                amount: total,
            });
        }
    }

    Ok(BudgetDashboardSummary {
        remaining_amount,
        remaining_percentage,
        total_available,
        currency,
        monthly_spending,
        monthly_goal,
        quarterly_activity,
    })
}
