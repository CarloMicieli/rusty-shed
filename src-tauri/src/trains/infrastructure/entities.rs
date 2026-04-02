//! SQLx row structs for the train-formations tables.
//!
//! These are thin wrappers around raw SQL columns, annotated with
//! `sqlx::FromRow` so they can be populated directly from query results.
//! They are converted to domain types via the functions in [`super::mappers`].

use sqlx::FromRow;

/// Row representation for `train_formations`.
#[derive(Debug, Clone, FromRow)]
pub struct TrainFormationRow {
    pub id: String,
    pub name: String,
    pub category_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: i64,
}

/// Row representation for `formation_elements`.
#[derive(Debug, Clone, FromRow)]
pub struct FormationElementRow {
    pub id: String,
    pub formation_id: String,
    pub prototype_id: String,
    pub owned_rolling_stock_id: Option<String>,
    pub snapshot_series_code: Option<String>,
    pub snapshot_company_name: Option<String>,
    pub position_order: i32,
    pub traction_override: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// Row representation for `prototypes`.
#[derive(Debug, Clone, FromRow)]
pub struct PrototypeRow {
    pub id: String,
    pub railway_company_id: String,
    pub series_code: String,
    pub car_type: String,
    pub service_level: Option<String>,
    pub category: String,
    pub is_motorized: i64,
    pub default_is_dummy: i64,
    pub is_custom: i64,
    pub notes: Option<String>,
    pub version: i64,
}

/// Row representation for `formation_categories`.
#[derive(Debug, Clone, FromRow)]
pub struct FormationCategoryRow {
    pub id: String,
    pub name: String,
    pub is_custom: i64,
}

/// Extended element row that joins `prototypes` and `railway_companies`
/// to produce the full view model.
#[derive(Debug, Clone, FromRow)]
pub struct FormationElementDetailRow {
    // element columns
    pub id: String,
    pub formation_id: String,
    pub prototype_id: String,
    pub owned_rolling_stock_id: Option<String>,
    pub snapshot_series_code: Option<String>,
    pub snapshot_company_name: Option<String>,
    pub position_order: i32,
    pub traction_override: i32,

    // prototype columns (aliased)
    pub proto_railway_company_id: String,
    pub proto_company_name: String,
    pub proto_series_code: String,
    pub proto_car_type: String,
    pub proto_service_level: Option<String>,
    pub proto_category: String,
    pub proto_is_motorized: i64,
    pub proto_default_is_dummy: i64,
    pub proto_is_custom: i64,

    /// COUNT of `owned_rolling_stocks` rows that reference the same `prototype_id`.
    pub owned_count_for_prototype: i64,
}

/// Summary row for the formation list page.
#[derive(Debug, Clone, FromRow)]
pub struct TrainFormationSummaryRow {
    pub id: String,
    pub name: String,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub element_count: i64,
    pub owned_count: i64,
    pub version: i64,
    /// Aggregated traction flag (1 = has traction, 0 = none). Computed via SQL SUM/CASE.
    pub has_traction: i64,
}

/// Prototype row joined with its railway company name, used by `search_prototypes()`.
#[derive(Debug, Clone, FromRow)]
pub struct PrototypeWithCompanyRow {
    pub id: String,
    pub railway_company_id: String,
    pub company_name: String,
    pub series_code: String,
    pub car_type: String,
    pub service_level: Option<String>,
    pub category: String,
    pub is_motorized: i64,
    pub default_is_dummy: i64,
    pub is_custom: i64,
    pub notes: Option<String>,
    pub version: i64,
}

impl From<PrototypeWithCompanyRow> for PrototypeRow {
    fn from(r: PrototypeWithCompanyRow) -> Self {
        PrototypeRow {
            id: r.id,
            railway_company_id: r.railway_company_id,
            series_code: r.series_code,
            car_type: r.car_type,
            service_level: r.service_level,
            category: r.category,
            is_motorized: r.is_motorized,
            default_is_dummy: r.default_is_dummy,
            is_custom: r.is_custom,
            notes: r.notes,
            version: r.version,
        }
    }
}
