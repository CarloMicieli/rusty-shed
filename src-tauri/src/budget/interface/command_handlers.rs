use crate::budget::application::add_extra_budget::{AddExtraBudgetInput, AddExtraBudgetUseCase};
use crate::budget::application::budget_query;
use crate::budget::application::historical_query;
use crate::budget::application::remove_extra_budget::RemoveExtraBudgetUseCase;
use crate::budget::application::set_budget::{SetBudgetInput, SetBudgetUseCase};
use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::ExtraBudgetId;
use crate::budget::domain::monthly_budget_record::MonthStatus;
use crate::budget::domain::monthly_budget_record::MonthlyBudgetRecord;
use crate::budget::interface::command_args::{
    AddExtraBudgetArgs, BudgetBootstrapDto, BudgetConfigDto, BudgetDashboardSummary,
    CategorySpending, ExtraBudgetDto, GetBudgetBootstrapArgs, GetExtraBudgetsArgs,
    GetMonthlyBudgetRecordsArgs, GetQuarterlySummariesArgs, MonetaryAmountDto,
    MonthlyBudgetRecordDto, MonthlySpendingPoint, QuarterlyActivityPoint, QuarterlySummary,
    RemoveExtraBudgetArgs, SetBudgetConfigArgs,
};
use crate::core::domain::Currency;
use crate::core::domain::calendar::{Month, Year};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use chrono::Datelike;
use tracing::info;

fn map_budget_config_dto(config: crate::budget::domain::BudgetConfiguration) -> BudgetConfigDto {
    BudgetConfigDto {
        id: config.id.value(),
        mode: config.mode,
        base_amount: config.base_amount.amount,
        monthly_amount: config.monthly_amount(),
        yearly_amount: config.yearly_amount(),
        currency: config.base_amount.currency,
        last_reset_year: config.last_reset_year.value(),
        created_at: config.metadata.created_at.to_rfc3339(),
        updated_at: config.metadata.updated_at.to_rfc3339(),
        version: config.metadata.version as u32,
    }
}

fn map_monthly_budget_record_dto(
    record: MonthlyBudgetRecord,
) -> Result<MonthlyBudgetRecordDto, CommandError> {
    let status_str = match record.status {
        MonthStatus::Projected => "PROJECTED".to_string(),
        MonthStatus::InProgress => "IN_PROGRESS".to_string(),
        MonthStatus::Completed => "COMPLETED".to_string(),
    };

    let year = Year::try_from(record.year)
        .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?;
    let month = Month::try_from(record.month)
        .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?;

    Ok(MonthlyBudgetRecordDto {
        year,
        month,
        base_budget: record.base_budget,
        extra_budget: record.extra_budget,
        actual_spend: record.actual_spend,
        rollover_in: record.rollover_in,
        rollover_out: record.rollover_out,
        available: record.available(),
        remaining: record.remaining(),
        remaining_percentage: record.remaining_percentage(),
        status: status_str,
        currency: record.currency,
    })
}

fn map_budget_dashboard_summary_dto(
    summary: crate::budget::domain::dashboard::BudgetDashboardSummary,
) -> BudgetDashboardSummary {
    BudgetDashboardSummary {
        remaining_amount: summary.remaining_amount,
        remaining_percentage: summary.remaining_percentage,
        total_available: summary.total_available,
        currency: summary.currency,
        monthly_spending: summary
            .monthly_spending
            .into_iter()
            .map(|point| MonthlySpendingPoint {
                month: point.month,
                amount: point.amount,
                currency: point.currency,
            })
            .collect(),
        monthly_goal: summary.monthly_goal,
        quarterly_activity: summary
            .quarterly_activity
            .into_iter()
            .map(|point| QuarterlyActivityPoint {
                year: point.year,
                quarter: point.quarter.into(),
                spending_level: point.spending_level.into(),
                amount: point.amount,
            })
            .collect(),
    }
}

fn map_quarterly_summary_dto(
    summary: crate::budget::domain::quarterly_summary::QuarterlySummary,
) -> QuarterlySummary {
    QuarterlySummary {
        year: summary.year,
        quarter: summary.quarter.into(),
        total_spending: MonetaryAmountDto {
            amount: summary.total_spending.amount,
            currency: summary.total_spending.currency,
        },
        category_breakdown: summary
            .category_breakdown
            .into_iter()
            .map(|entry| CategorySpending {
                category: entry.category,
                amount: MonetaryAmountDto {
                    amount: entry.amount.amount,
                    currency: entry.amount.currency,
                },
                percentage: entry.percentage,
            })
            .collect(),
    }
}

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

pub async fn get_budget_config_inner(
    state: &AppState,
) -> Result<Option<BudgetConfigDto>, CommandError> {
    info!("Fetching budget configuration");

    let mut unit_of_work = state.unit_of_work().await?;
    let config = {
        let mut repo = unit_of_work.budget_repo();
        repo.get_config().await.map_err(CommandError::from)?
    };

    unit_of_work.commit().await?;

    Ok(config.map(map_budget_config_dto))
}

pub async fn set_budget_config_inner(
    state: &AppState,
    args: SetBudgetConfigArgs,
    currency: Currency,
) -> Result<BudgetConfigDto, CommandError> {
    info!("Setting budget configuration: {:?}", args);

    let input = SetBudgetInput {
        mode: args.mode,
        base_amount: MonetaryAmount::new(args.base_amount, currency),
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let config = SetBudgetUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(map_budget_config_dto(config))
}

pub async fn get_monthly_budget_records_inner(
    state: &AppState,
    args: GetMonthlyBudgetRecordsArgs,
) -> Result<Vec<MonthlyBudgetRecordDto>, CommandError> {
    let year = match args.year {
        Some(y) => y,
        None => Year::try_from(chrono::Utc::now().year())
            .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?,
    };

    info!("Fetching monthly budget records for year {}", year.value());

    let mut unit_of_work = state.unit_of_work().await?;
    let records = budget_query::get_monthly_budget_records(&mut unit_of_work, year).await?;

    unit_of_work.commit().await?;

    let dtos: Result<Vec<_>, CommandError> = records
        .into_iter()
        .map(map_monthly_budget_record_dto)
        .collect();

    dtos
}

pub async fn get_budget_dashboard_inner(
    state: &AppState,
    currency_code: &str,
) -> Result<BudgetDashboardSummary, CommandError> {
    info!("Fetching budget dashboard summary");

    let mut unit_of_work = state.unit_of_work().await?;
    let summary = budget_query::get_budget_dashboard(&mut unit_of_work, currency_code).await?;

    unit_of_work.commit().await?;

    Ok(map_budget_dashboard_summary_dto(summary))
}

pub async fn get_budget_bootstrap_inner(
    state: &AppState,
    args: GetBudgetBootstrapArgs,
    currency_code: &str,
) -> Result<BudgetBootstrapDto, CommandError> {
    let year = match args.year {
        Some(year) => year,
        None => Year::try_from(chrono::Utc::now().year())
            .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?,
    };

    info!(
        "Fetching budget bootstrap payload for year {}",
        year.value()
    );

    let mut unit_of_work = state.unit_of_work().await?;
    let bootstrap =
        budget_query::get_budget_bootstrap(&mut unit_of_work, year, currency_code).await?;

    unit_of_work.commit().await?;

    let monthly_records = bootstrap
        .monthly_records
        .map(|records| {
            records
                .into_iter()
                .map(map_monthly_budget_record_dto)
                .collect()
        })
        .transpose()?;

    Ok(BudgetBootstrapDto {
        config: bootstrap.config.map(map_budget_config_dto),
        dashboard_summary: map_budget_dashboard_summary_dto(bootstrap.dashboard_summary),
        monthly_records,
    })
}

pub async fn add_extra_budget_inner(
    state: &AppState,
    args: AddExtraBudgetArgs,
    currency: Currency,
) -> Result<ExtraBudgetDto, CommandError> {
    info!(
        "Adding extra budget for {}/{}: {}",
        args.year.value(),
        args.month.value(),
        args.amount
    );

    let input = AddExtraBudgetInput {
        year: args.year,
        month: args.month,
        amount: MonetaryAmount::new(args.amount, currency),
        reason: args.reason,
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let entry = AddExtraBudgetUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(ExtraBudgetDto {
        id: entry.id.to_string(),
        year: entry.year,
        month: entry.month,
        amount: entry.amount.amount,
        currency: entry.amount.currency,
        reason: entry.reason,
        created_at: entry.created_at.to_rfc3339(),
        version: entry.version,
    })
}

pub async fn remove_extra_budget_inner(
    state: &AppState,
    args: RemoveExtraBudgetArgs,
) -> Result<(), CommandError> {
    info!("Removing extra budget entry: {}", args.id);

    let id =
        ExtraBudgetId::try_from(args.id).map_err(|e| DomainError::Validation(e.to_string()))?;

    let mut unit_of_work = state.unit_of_work().await?;
    RemoveExtraBudgetUseCase::execute(&mut unit_of_work, id).await?;

    unit_of_work.commit().await?;

    Ok(())
}

pub async fn get_extra_budgets_inner(
    state: &AppState,
    args: GetExtraBudgetsArgs,
) -> Result<Vec<ExtraBudgetDto>, CommandError> {
    info!("Fetching extra budgets for year {}", args.year.value());

    let mut unit_of_work = state.unit_of_work().await?;
    let entries = {
        let mut repo = unit_of_work.budget_repo();
        repo.get_extra_budgets(args.year.value())
            .await
            .map_err(CommandError::from)?
    };

    unit_of_work.commit().await?;

    let dtos = entries
        .into_iter()
        .map(|entry| ExtraBudgetDto {
            id: entry.id.to_string(),
            year: entry.year,
            month: entry.month,
            amount: entry.amount.amount,
            currency: entry.amount.currency,
            reason: entry.reason,
            created_at: entry.created_at.to_rfc3339(),
            version: entry.version,
        })
        .collect();

    Ok(dtos)
}

pub async fn get_quarterly_summaries_inner(
    state: &AppState,
    year: Year,
    currency_code: String,
) -> Result<Vec<QuarterlySummary>, CommandError> {
    info!("Fetching quarterly summaries for year {}", year.value());

    let mut unit_of_work = state.unit_of_work().await?;
    let summaries =
        historical_query::get_quarterly_summaries(&mut unit_of_work, year, &currency_code).await?;

    unit_of_work.commit().await?;

    Ok(summaries
        .into_iter()
        .map(map_quarterly_summary_dto)
        .collect())
}

fn resolve_quarterly_year(input: Option<Year>) -> Result<Year, CommandError> {
    match input {
        Some(year) => Ok(year),
        None => Year::try_from(chrono::Utc::now().year())
            .map_err(|e| CommandError::from(DomainError::Validation(e.to_string()))),
    }
}

async fn resolve_quarterly_currency(
    app: tauri::AppHandle,
    currency: Option<String>,
) -> Result<String, CommandError> {
    if let Some(code) = currency {
        return Ok(code);
    }

    let settings = crate::settings::get_settings(app)
        .await
        .map_err(|e| CommandError::validation_field("currency", e))?;
    Ok(settings.currency)
}

// ---------------------------------------------------------------------------
// Tauri command wrappers – thin shims that delegate to inner functions
// ---------------------------------------------------------------------------

/// Tauri command to get the current budget configuration.
///
/// Returns `None` if no configuration has been set yet.
#[tauri::command]
#[specta::specta]
pub async fn get_budget_config(
    state: tauri::State<'_, AppState>,
) -> Result<Option<BudgetConfigDto>, CommandError> {
    get_budget_config_inner(&state).await
}

/// Tauri command to set or update the budget configuration.
#[tauri::command]
#[specta::specta]
pub async fn set_budget_config(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    args: SetBudgetConfigArgs,
) -> Result<BudgetConfigDto, CommandError> {
    // Get currency from args or fall back to settings
    let currency = match args.currency {
        Some(ref code) => {
            Currency::from_code(code).map_err(|e| DomainError::Validation(e.to_string()))?
        }
        None => {
            let settings = crate::settings::get_settings(app)
                .await
                .map_err(|e| CommandError::validation_field("currency", e))?;
            Currency::from_code(&settings.currency)
                .map_err(|e| CommandError::validation_field("currency", e.to_string()))?
        }
    };
    set_budget_config_inner(&state, args, currency).await
}

// Additional command handlers will be implemented in later phases
// - get_budget_dashboard (Phase 5 - US3)
// - add_extra_budget (Phase 6 - US4)
// - remove_extra_budget (Phase 6 - US4)

/// Tauri command to get monthly budget records for a year.
///
/// Returns 12 monthly budget records with rollover calculations.
/// If year is not specified, uses the current year.
#[tauri::command]
#[specta::specta]
pub async fn get_monthly_budget_records(
    state: tauri::State<'_, AppState>,
    args: GetMonthlyBudgetRecordsArgs,
) -> Result<Vec<MonthlyBudgetRecordDto>, CommandError> {
    get_monthly_budget_records_inner(&state, args).await
}

/// Tauri command to get budget dashboard summary.
///
/// Returns dashboard data for widgets (donut, bar chart, heatmap).
/// Budget-specific fields will be None if budget is not configured, but spending data
/// will still be populated using the user's preferred currency from settings.
#[tauri::command]
#[specta::specta]
pub async fn get_budget_dashboard(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<BudgetDashboardSummary, CommandError> {
    let settings = crate::settings::get_settings(app)
        .await
        .map_err(|e| CommandError::validation_field("settings", e))?;
    get_budget_dashboard_inner(&state, &settings.currency).await
}

/// Tauri command to get the Finance page bootstrap payload.
///
/// Returns the config, dashboard summary, and selected-year monthly records in a single
/// response so the Finance page can hydrate without a request waterfall.
#[tauri::command]
#[specta::specta]
pub async fn get_budget_bootstrap(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    args: GetBudgetBootstrapArgs,
) -> Result<BudgetBootstrapDto, CommandError> {
    let settings = crate::settings::get_settings(app)
        .await
        .map_err(|e| CommandError::validation_field("settings", e))?;
    get_budget_bootstrap_inner(&state, args, &settings.currency).await
}

// Additional command handlers will be implemented in later phases:
// - get_quarterly_summaries (Phase 8 - US6)

/// Tauri command to add a one-time budget injection.
///
/// # Arguments
/// * `state` - App state for database access
/// * `args` - Arguments containing year, month, amount, and optional reason
///
/// # Returns
/// The created extra budget entry as a DTO.
#[tauri::command]
#[specta::specta]
pub async fn add_extra_budget(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    args: AddExtraBudgetArgs,
) -> Result<ExtraBudgetDto, CommandError> {
    // Get currency from args or fall back to settings
    let currency = match args.currency {
        Some(ref code) => {
            Currency::from_code(code).map_err(|e| DomainError::Validation(e.to_string()))?
        }
        None => {
            let settings = crate::settings::get_settings(app)
                .await
                .map_err(|e| CommandError::validation_field("currency", e))?;
            Currency::from_code(&settings.currency)
                .map_err(|e| CommandError::validation_field("currency", e.to_string()))?
        }
    };
    add_extra_budget_inner(&state, args, currency).await
}

/// Tauri command to remove an extra budget entry.
///
/// # Arguments
/// * `state` - App state for database access
/// * `args` - Arguments containing the extra budget ID
#[tauri::command]
#[specta::specta]
pub async fn remove_extra_budget(
    state: tauri::State<'_, AppState>,
    args: RemoveExtraBudgetArgs,
) -> Result<(), CommandError> {
    remove_extra_budget_inner(&state, args).await
}

/// Tauri command to get all extra budget entries for a specific year.
///
/// # Arguments
/// * `state` - App state for database access
/// * `args` - Arguments containing the year
///
/// # Returns
/// List of extra budget entries for the specified year.
#[tauri::command]
#[specta::specta]
pub async fn get_extra_budgets(
    state: tauri::State<'_, AppState>,
    args: GetExtraBudgetsArgs,
) -> Result<Vec<ExtraBudgetDto>, CommandError> {
    get_extra_budgets_inner(&state, args).await
}

/// Tauri command to get quarterly summaries with category breakdown.
///
/// # Arguments
/// * `state` - App state for database access
/// * `args` - Arguments containing year and optional currency
///
/// # Returns
/// List of quarterly summaries for the specified year, each with category breakdown.
#[tauri::command]
#[specta::specta]
pub async fn get_quarterly_summaries(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    args: GetQuarterlySummariesArgs,
) -> Result<Vec<QuarterlySummary>, CommandError> {
    let year = resolve_quarterly_year(args.year)?;
    let currency_code = resolve_quarterly_currency(app, args.currency).await?;
    get_quarterly_summaries_inner(&state, year, currency_code).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::domain::BudgetMode;
    use crate::core::domain::Currency;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    async fn seed_budget_config(state: &AppState) {
        let currency = Currency::from_code("EUR").expect("valid currency");
        let args = SetBudgetConfigArgs {
            mode: BudgetMode::Yearly,
            base_amount: 120_000,
            currency: None,
        };

        let _ = set_budget_config_inner(state, args, currency)
            .await
            .expect("budget config should be created");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_monthly_budget_records_inner_returns_empty_for_year_without_data(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let year = Year::try_from(2025).expect("valid year");
        seed_budget_config(&state).await;

        let records = get_monthly_budget_records_inner(
            &state,
            GetMonthlyBudgetRecordsArgs { year: Some(year) },
        )
        .await
        .expect("query should succeed");

        assert_eq!(records.len(), 12);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_budget_bootstrap_inner_returns_empty_monthly_records_without_data(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let year = Year::try_from(2025).expect("valid year");
        seed_budget_config(&state).await;

        let bootstrap =
            get_budget_bootstrap_inner(&state, GetBudgetBootstrapArgs { year: Some(year) }, "EUR")
                .await
                .expect("query should succeed");

        assert!(bootstrap.config.is_some());
        assert!(matches!(bootstrap.monthly_records, Some(records) if records.len() == 12));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_quarterly_summaries_inner_returns_empty_for_year_without_data(pool: SqlitePool) {
        let state = app_state(pool);
        let year = Year::try_from(2025).expect("valid year");
        seed_budget_config(&state).await;

        let summaries = get_quarterly_summaries_inner(&state, year, "EUR".to_string())
            .await
            .expect("query should succeed");

        assert!(summaries.is_empty());
    }

    #[test]
    fn resolve_quarterly_year_prefers_explicit_value() {
        let year = Year::try_from(2024).expect("valid year");
        let resolved = resolve_quarterly_year(Some(year)).expect("resolution should succeed");
        assert_eq!(resolved.value(), 2024);
    }

    #[test]
    fn resolve_quarterly_year_defaults_to_current_year() {
        let resolved = resolve_quarterly_year(None).expect("resolution should succeed");
        assert_eq!(resolved.value(), chrono::Utc::now().year());
    }
}
