// Transport DTOs (Args) for Tauri commands
// Following ADR 8: Standardize Tauri Command and Use Case Input Handling

use crate::budget::domain::BudgetMode;
use crate::core::domain::Currency;
use garde::Validate;
use serde::{Deserialize, Serialize};

/// Arguments for setting/updating the budget configuration.
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct SetBudgetConfigArgs {
    /// Budget mode: YEARLY or MONTHLY
    pub mode: BudgetMode,
    /// Base amount in minor currency units (cents)
    #[garde(range(min = 0))]
    pub base_amount: i64,
    /// Optional currency code (inherits from settings if not provided)
    pub currency: Option<String>,
}

/// Arguments for adding an extra budget to a specific month.
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddExtraBudgetArgs {
    /// Target year (2000-2100)
    #[garde(range(min = 2000, max = 2100))]
    pub year: i32,
    /// Target month (1-12)
    #[garde(range(min = 1, max = 12))]
    pub month: u8,
    /// Amount in minor currency units (must be positive)
    #[garde(range(min = 1))]
    pub amount: i64,
    /// Optional currency code (inherits from settings if not provided)
    pub currency: Option<String>,
    /// Optional reason for the extra budget
    pub reason: Option<String>,
}

/// Arguments for removing an extra budget entry.
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct RemoveExtraBudgetArgs {
    /// ID of the extra budget entry to remove
    pub id: String,
}

/// Arguments for querying monthly budget records.
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct GetMonthlyBudgetRecordsArgs {
    /// Year to query (defaults to current year if not provided)
    pub year: Option<i32>,
}

/// Arguments for querying extra budgets for a year.
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct GetExtraBudgetsArgs {
    /// Year to query
    #[garde(range(min = 2000, max = 2100))]
    pub year: i32,
}

/// Arguments for querying quarterly summaries.
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct GetQuarterlySummariesArgs {
    /// Year to query (defaults to current year if not provided)
    pub year: Option<i32>,
    /// Currency code (defaults to settings currency if not provided)
    pub currency: Option<String>,
}

// DTO Response types

/// Budget configuration DTO for transport layer.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfigDto {
    pub id: i32,
    pub mode: BudgetMode,
    pub base_amount: i64,
    pub monthly_amount: i64,
    pub yearly_amount: i64,
    pub currency: Currency,
    pub last_reset_year: i32,
    pub created_at: String, // ISO 8601
    pub updated_at: String, // ISO 8601
    pub version: u32,
}

/// Monthly budget record DTO.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyBudgetRecordDto {
    pub year: i32,
    pub month: u8,
    pub base_budget: i64,
    pub extra_budget: i64,
    pub actual_spend: i64,
    pub rollover_in: i64,
    pub rollover_out: i64,
    pub available: i64,
    pub remaining: i64,
    pub remaining_percentage: f64,
    pub status: String, // "PROJECTED", "IN_PROGRESS", "COMPLETED"
    pub currency: Currency,
}

/// Extra budget entry DTO.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ExtraBudgetDto {
    pub id: String,
    pub year: i32,
    pub month: u8,
    pub amount: i64,
    pub currency: Currency,
    pub reason: Option<String>,
    pub created_at: String, // ISO 8601
    pub version: u32,
}
