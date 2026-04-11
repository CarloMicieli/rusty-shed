// Tauri Command Handlers for Budget Feature
// Feature: 001-budget-tracking

use crate::budget::application::add_extra_budget::{AddExtraBudgetInput, AddExtraBudgetUseCase};
use crate::budget::application::budget_query;
use crate::budget::application::historical_query;
use crate::budget::application::remove_extra_budget::RemoveExtraBudgetUseCase;
use crate::budget::application::set_budget::{SetBudgetInput, SetBudgetUseCase};
use crate::budget::domain::BudgetUowExt;
use crate::budget::domain::ExtraBudgetId;
use crate::budget::domain::dashboard::BudgetDashboardSummary;
use crate::budget::domain::monthly_budget_record::MonthStatus;
use crate::budget::domain::quarterly_summary::QuarterlySummary;
use crate::budget::interface::command_args::{
    AddExtraBudgetArgs, BudgetConfigDto, ExtraBudgetDto, GetExtraBudgetsArgs,
    GetMonthlyBudgetRecordsArgs, GetQuarterlySummariesArgs, MonthlyBudgetRecordDto,
    RemoveExtraBudgetArgs, SetBudgetConfigArgs,
};
use crate::core::domain::Currency;
use crate::core::domain::calendar::{Month, Year};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use chrono::Datelike;
use log::info;

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

    Ok(config.map(|c| BudgetConfigDto {
        id: c.id.value(),
        mode: c.mode,
        base_amount: c.base_amount.amount,
        monthly_amount: c.monthly_amount(),
        yearly_amount: c.yearly_amount(),
        currency: c.base_amount.currency,
        last_reset_year: c.last_reset_year,
        created_at: c.created_at.to_rfc3339(),
        updated_at: c.updated_at.to_rfc3339(),
        version: c.version,
    }))
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

    Ok(BudgetConfigDto {
        id: config.id.value(),
        mode: config.mode,
        base_amount: config.base_amount.amount,
        monthly_amount: config.monthly_amount(),
        yearly_amount: config.yearly_amount(),
        currency: config.base_amount.currency,
        last_reset_year: config.last_reset_year,
        created_at: config.created_at.to_rfc3339(),
        updated_at: config.updated_at.to_rfc3339(),
        version: config.version,
    })
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
        .map(|record| {
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
        })
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

    Ok(summary)
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

    Ok(summaries)
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
    let year = match args.year {
        Some(y) => y,
        None => Year::try_from(chrono::Utc::now().year())
            .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?,
    };
    let currency_code = match args.currency {
        Some(ref code) => code.clone(),
        None => {
            let settings = crate::settings::get_settings(app)
                .await
                .map_err(|e| CommandError::validation_field("currency", e))?;
            settings.currency
        }
    };
    get_quarterly_summaries_inner(&state, year, currency_code).await
}
