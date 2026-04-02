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
    pub friendly_name: Option<String>,
    pub is_motorized: i64,
    pub default_is_dummy: i64,
    pub is_custom: i64,
    pub notes: Option<String>,
    /// Discriminator: `LOCOMOTIVE` | `PASSENGER_CAR` | `FREIGHT_CAR` | `RAILCAR` | `ELECTRIC_MULTIPLE_UNIT`
    pub specification_type: String,
    // Locomotive-specific
    pub locomotive_type: Option<String>,
    pub locomotive_series: Option<String>,
    // PassengerCar-specific
    pub service_level: Option<String>,
    pub passenger_car_type: Option<String>,
    // FreightCar-specific
    pub freight_car_type: Option<String>,
    // Railcar-specific
    pub railcar_type: Option<String>,
    // ElectricMultipleUnit-specific
    pub electric_multiple_unit_type: Option<String>,
    pub elements_count: Option<i64>,
    pub is_permanently_coupled: Option<i64>,
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

    // prototype columns (aliased with `proto_` prefix)
    pub proto_railway_company_id: String,
    pub proto_company_name: String,
    pub proto_series_code: String,
    pub proto_friendly_name: Option<String>,
    pub proto_is_motorized: i64,
    pub proto_default_is_dummy: i64,
    pub proto_is_custom: i64,
    pub proto_specification_type: String,
    // Locomotive-specific
    pub proto_locomotive_type: Option<String>,
    pub proto_locomotive_series: Option<String>,
    // PassengerCar-specific
    pub proto_service_level: Option<String>,
    pub proto_passenger_car_type: Option<String>,
    // FreightCar-specific
    pub proto_freight_car_type: Option<String>,
    // Railcar-specific
    pub proto_railcar_type: Option<String>,
    // ElectricMultipleUnit-specific
    pub proto_electric_multiple_unit_type: Option<String>,
    pub proto_elements_count: Option<i64>,
    pub proto_is_permanently_coupled: Option<i64>,

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
    pub friendly_name: Option<String>,
    pub is_motorized: i64,
    pub default_is_dummy: i64,
    pub is_custom: i64,
    pub notes: Option<String>,
    pub specification_type: String,
    // Locomotive-specific
    pub locomotive_type: Option<String>,
    pub locomotive_series: Option<String>,
    // PassengerCar-specific
    pub service_level: Option<String>,
    pub passenger_car_type: Option<String>,
    // FreightCar-specific
    pub freight_car_type: Option<String>,
    // Railcar-specific
    pub railcar_type: Option<String>,
    // ElectricMultipleUnit-specific
    pub electric_multiple_unit_type: Option<String>,
    pub elements_count: Option<i64>,
    pub is_permanently_coupled: Option<i64>,
    pub version: i64,
}

impl From<PrototypeWithCompanyRow> for PrototypeRow {
    fn from(r: PrototypeWithCompanyRow) -> Self {
        PrototypeRow {
            id: r.id,
            railway_company_id: r.railway_company_id,
            series_code: r.series_code,
            friendly_name: r.friendly_name,
            is_motorized: r.is_motorized,
            default_is_dummy: r.default_is_dummy,
            is_custom: r.is_custom,
            notes: r.notes,
            specification_type: r.specification_type,
            locomotive_type: r.locomotive_type,
            locomotive_series: r.locomotive_series,
            service_level: r.service_level,
            passenger_car_type: r.passenger_car_type,
            freight_car_type: r.freight_car_type,
            railcar_type: r.railcar_type,
            electric_multiple_unit_type: r.electric_multiple_unit_type,
            elements_count: r.elements_count,
            is_permanently_coupled: r.is_permanently_coupled,
            version: r.version,
        }
    }
}
