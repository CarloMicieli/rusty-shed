//! View DTOs for the trains bounded context.
//!
//! These are read-optimised projections used by the application and
//! interface layers.  They live in the domain to break the dependency
//! on `infrastructure::mappers` and to enable pure application-layer
//! testing with mock repositories.

// ── View model structs (specta-typed read DTOs) ───────────────────────────────

/// Post-write response for `create_train_formation` and `update_train_formation`.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct TrainFormationView {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    pub element_count: i64,
    pub has_traction: bool,
}

/// Summary card for the formation list page.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct TrainFormationSummary {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub epoch: Option<String>,
    pub element_count: i64,
    /// Whether the formation has at least one effective traction slot.
    pub has_traction: bool,
    /// Elements that have an `owned_rolling_stock_id` assigned.
    pub owned_count: i64,
    /// Elements that do not have an `owned_rolling_stock_id` assigned.
    pub planned_count: i64,
}

/// Full detail for the formation builder screen.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct TrainFormationDetail {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    /// Ordered composition slots.
    pub elements: Vec<FormationElementView>,
    /// Whether the formation has at least one effective traction slot.
    pub has_traction: bool,
}

/// An individual element slot in a formation.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct FormationElementView {
    pub id: String,
    pub position_order: i32,
    pub prototype: PrototypeView,
    pub owned_rolling_stock_id: Option<String>,
    /// Snapshotted series code retained even after owned model deletion.
    pub snapshot_series_code: Option<String>,
    /// Snapshotted company name retained even after owned model deletion.
    pub snapshot_company_name: Option<String>,
    /// `true` when `snapshot_series_code` is set but `owned_rolling_stock_id` is `None`.
    pub stock_not_found: bool,
    /// Number of `owned_rolling_stocks` rows whose linked `rolling_stocks.prototype_id` matches.
    pub owned_count_for_prototype: i64,
    pub traction_override: i32,
    /// Derived: whether this slot counts as a traction source.
    pub is_traction_slot: bool,
}

/// Prototype in search results and formation element views.
///
/// Uses a flat layout with a `specification_type` discriminator and nullable
/// per-specification fields so that TypeScript consumers can switch on
/// `specification_type` at runtime without requiring discriminated-union
/// deserialization.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct PrototypeView {
    pub id: String,
    pub railway_company_id: String,
    pub company_name: String,
    pub series_code: String,
    pub friendly_name: Option<String>,
    pub is_motorized: bool,
    pub default_is_dummy: bool,
    pub is_custom: bool,
    /// Specification discriminator: `LOCOMOTIVE` | `PASSENGER_CAR` | `FREIGHT_CAR` |
    /// `RAILCAR` | `ELECTRIC_MULTIPLE_UNIT`
    pub specification_type: String,
    // Locomotive-specific (non-null when specification_type = "LOCOMOTIVE")
    pub locomotive_type: Option<String>,
    pub locomotive_series: Option<String>,
    // PassengerCar-specific (non-null when specification_type = "PASSENGER_CAR")
    pub service_level: Option<String>,
    pub passenger_car_type: Option<String>,
    // FreightCar-specific (non-null when specification_type = "FREIGHT_CAR")
    pub freight_car_type: Option<String>,
    // Railcar-specific (non-null when specification_type = "RAILCAR")
    pub railcar_type: Option<String>,
    // ElectricMultipleUnit-specific (non-null when specification_type = "ELECTRIC_MULTIPLE_UNIT")
    pub electric_multiple_unit_type: Option<String>,
    pub elements_count: Option<i64>,
    pub is_permanently_coupled: Option<bool>,
}

/// Prototypes grouped by railway company (for the search drawer).
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct PrototypeGroupView {
    pub railway_company_id: String,
    pub company_name: String,
    pub prototypes: Vec<PrototypeView>,
}

/// A formation category (seeded or custom).
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct FormationCategoryView {
    pub id: String,
    pub name: String,
    pub is_custom: bool,
}
