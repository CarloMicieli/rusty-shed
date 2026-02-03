// Budget Tracking Feature Module
// Feature: 001-budget-tracking

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;

// Re-exports for convenience
pub use domain::{
    BudgetConfiguration, BudgetEvent, BudgetMode, ExtraBudgetEntry, MonthStatus,
    MonthlyBudgetRecord,
};
