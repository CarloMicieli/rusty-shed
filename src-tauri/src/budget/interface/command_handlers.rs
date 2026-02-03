// Tauri Command Handlers for Budget Feature
// Feature: 001-budget-tracking

use crate::budget::application::add_extra_budget::{AddExtraBudgetInput, AddExtraBudgetUseCase};
use crate::budget::application::budget_query;
use crate::budget::application::historical_query;
use crate::budget::application::remove_extra_budget::RemoveExtraBudgetUseCase;
use crate::budget::application::set_budget::{SetBudgetInput, SetBudgetUseCase};
use crate::budget::domain::dashboard::BudgetDashboardSummary;
use crate::budget::domain::monthly_budget_record::MonthStatus;
use crate::budget::domain::quarterly_summary::QuarterlySummary;
use crate::budget::domain::{BudgetRepository, ExtraBudgetId};
use crate::budget::infrastructure::BudgetUowExt;
use crate::budget::interface::command_args::{
    AddExtraBudgetArgs, BudgetConfigDto, ExtraBudgetDto, GetExtraBudgetsArgs,
    GetMonthlyBudgetRecordsArgs, GetQuarterlySummariesArgs, MonthlyBudgetRecordDto,
    RemoveExtraBudgetArgs, SetBudgetConfigArgs,
};
use crate::core::domain::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use chrono::Datelike;
use log::info;

/// Tauri command to get the current budget configuration.
///
/// Returns `None` if no configuration has been set yet.
#[tauri::command]
#[specta::specta]
pub async fn get_budget_config(
    state: tauri::State<'_, AppState>,
) -> Result<Option<BudgetConfigDto>, CommandError> {
    info!("Fetching budget configuration");

    let mut unit_of_work = state.unit_of_work().await?;
    let config = {
        let mut repo = unit_of_work.budget_repo();
        repo.get_config()
            .await
            .map_err(CommandError::DatabaseError)?
    };

    unit_of_work.commit().await.map_err(CommandError::from)?;

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

/// Tauri command to set or update the budget configuration.
#[tauri::command]
#[specta::specta]
pub async fn set_budget_config(
    state: tauri::State<'_, AppState>,
    args: SetBudgetConfigArgs,
) -> Result<BudgetConfigDto, CommandError> {
    info!("Setting budget configuration: {:?}", args);

    // Get currency from args or fall back to settings
    let currency = match args.currency {
        Some(ref code) => {
            Currency::from_code(code).map_err(|e| DomainError::Validation(e.to_string()))?
        }
        None => {
            // Get currency from settings
            let settings = crate::settings::get_settings(state.clone()).await?;
            settings.currency
        }
    };

    let input = SetBudgetInput {
        mode: args.mode,
        base_amount: MonetaryAmount::new(args.base_amount, currency),
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let config = SetBudgetUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

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
    let year = args.year.unwrap_or_else(|| chrono::Utc::now().year());

    info!("Fetching monthly budget records for year {}", year);

    let mut unit_of_work = state.unit_of_work().await?;
    let records = budget_query::get_monthly_budget_records(&mut unit_of_work, year).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    // Convert to DTOs
    let dtos = records
        .into_iter()
        .map(|record| {
            let status_str = match record.status {
                MonthStatus::Projected => "PROJECTED".to_string(),
                MonthStatus::InProgress => "IN_PROGRESS".to_string(),
                MonthStatus::Completed => "COMPLETED".to_string(),
            };

            MonthlyBudgetRecordDto {
                year: record.year,
                month: record.month,
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
            }
        })
        .collect();

    Ok(dtos)
}

/// Tauri command to get budget dashboard summary.
///
/// Returns dashboard data for widgets (donut, bar chart, heatmap).
/// Returns `None` if budget is not configured.
#[tauri::command]
#[specta::specta]
pub async fn get_budget_dashboard(
    state: tauri::State<'_, AppState>,
) -> Result<Option<BudgetDashboardSummary>, CommandError> {
    info!("Fetching budget dashboard summary");

    let mut unit_of_work = state.unit_of_work().await?;
    let summary = budget_query::get_budget_dashboard(&mut unit_of_work).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(summary)
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
    state: tauri::State<'_, AppState>,
    args: AddExtraBudgetArgs,
) -> Result<ExtraBudgetDto, CommandError> {
    info!(
        "Adding extra budget for {}/{}: {}",
        args.year, args.month, args.amount
    );

    // Get currency from args or fall back to settings
    let currency = match args.currency {
        Some(ref code) => {
            Currency::from_code(code).map_err(|e| DomainError::Validation(e.to_string()))?
        }
        None => {
            let settings = crate::settings::get_settings(state.clone()).await?;
            settings.currency
        }
    };

    let input = AddExtraBudgetInput {
        year: args.year,
        month: args.month,
        amount: MonetaryAmount::new(args.amount, currency),
        reason: args.reason,
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let entry = AddExtraBudgetUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

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
    info!("Removing extra budget entry: {}", args.id);

    let id =
        ExtraBudgetId::try_from(args.id).map_err(|e| DomainError::Validation(e.to_string()))?;

    let mut unit_of_work = state.unit_of_work().await?;
    RemoveExtraBudgetUseCase::execute(&mut unit_of_work, id).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
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
    info!("Fetching extra budgets for year {}", args.year);

    let mut unit_of_work = state.unit_of_work().await?;
    let entries = {
        let mut repo = unit_of_work.budget_repo();
        repo.get_extra_budgets(args.year)
            .await
            .map_err(CommandError::DatabaseError)?
    };

    unit_of_work.commit().await.map_err(CommandError::from)?;

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
    state: tauri::State<'_, AppState>,
    args: GetQuarterlySummariesArgs,
) -> Result<Vec<QuarterlySummary>, CommandError> {
    let year = args.year.unwrap_or_else(|| chrono::Utc::now().year());

    info!("Fetching quarterly summaries for year {}", year);

    // Get currency from args or fall back to settings
    let currency_code = match args.currency {
        Some(ref code) => code.clone(),
        None => {
            let settings = crate::settings::get_settings(state.clone()).await?;
            settings.currency.to_code().to_string()
        }
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let summaries =
        historical_query::get_quarterly_summaries(&mut unit_of_work, year, &currency_code).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(summaries)
}
