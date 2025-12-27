//! Domain type representing a railway company.
//!
//! This module defines the `RailwayCompany` struct used across the catalog
//! domain to represent operating or owning railway companies. Fields are kept
//! minimal and optional where the underlying database allows null values.

use crate::catalog::domain::period_of_activity::PeriodOfActivity;
use serde::{Deserialize, Serialize};

/// A railway company (operator or owner).
///
/// This struct models a real-world railway company. Some fields are optional
/// because the corresponding database columns may be nullable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RailwayCompany {
    /// The common name of the railway company (not null).
    pub name: String,

    /// The legally registered company name (nullable).
    pub registered_company_name: Option<String>,

    /// The ISO 3166-1 alpha-2 country code where the company is registered
    /// (nullable). Example: `"IT"` for Italy.
    pub country_code: Option<String>,

    /// The period of activity of the railway company (nullable).
    pub period_of_activity: Option<PeriodOfActivity>,
}
