//! Dashboard module — aggregates statistics for the application home screen.
//!
//! Queries are read-only and span multiple bounded contexts (catalog,
//! collecting, budget, etc.) to produce summary figures such as total models,
//! recent acquisitions, and collection value. No domain state is mutated here.
//!
//! ## Layers
//! - **domain**: `DashboardSummary` view model and related value types
//! - **application**: query assembler that fans out to infrastructure repositories
//! - **infrastructure**: specialised read-model SQL queries (cross-table joins)
//! - **interface**: single `get_dashboard_summary` Tauri command

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
