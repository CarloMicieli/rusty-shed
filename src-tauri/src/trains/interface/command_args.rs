//! Input argument structs for the train-formations Tauri commands.
//!
//! All structs:
//! - derive `garde::Validate` for boundary validation
//! - derive `specta::Type` so tauri-specta generates TypeScript bindings
//! - derive `serde::Deserialize` for Tauri deserialization

use crate::core::domain::calendar::Year;
use garde::Validate;

// ── Shared validation helpers ─────────────────────────────────────────────────

/// Valid `specification_type` discriminator values.
const VALID_SPECIFICATION_TYPES: &[&str] = &[
    "LOCOMOTIVE",
    "PASSENGER_CAR",
    "FREIGHT_CAR",
    "RAILCAR",
    "ELECTRIC_MULTIPLE_UNIT",
];

fn validate_specification_type(value: &str, _ctx: &()) -> garde::Result {
    if VALID_SPECIFICATION_TYPES.contains(&value) {
        Ok(())
    } else {
        Err(garde::Error::new(format!(
            "specification_type '{value}' is not valid; expected one of: {}",
            VALID_SPECIFICATION_TYPES.join(", ")
        )))
    }
}

// ── Formation CRUD ────────────────────────────────────────────────────────────

/// Arguments for `create_train_formation`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct CreateTrainFormationArgs {
    /// Formation name — required, 1–100 characters.
    #[garde(length(min = 1, max = 100))]
    pub name: String,

    #[garde(skip)]
    pub category_id: Option<String>,

    #[garde(dive)]
    pub start_year: Option<Year>,

    #[garde(dive)]
    pub end_year: Option<Year>,

    #[garde(skip)]
    pub epoch: Option<String>,

    #[garde(skip)]
    pub notes: Option<String>,
}

/// Arguments for `update_train_formation`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct UpdateTrainFormationArgs {
    /// New name for the formation. `None` = keep existing name.
    #[garde(inner(length(min = 1, max = 100)))]
    pub name: Option<String>,

    #[garde(skip)]
    pub category_id: Option<String>,

    #[garde(dive)]
    pub start_year: Option<Year>,

    #[garde(dive)]
    pub end_year: Option<Year>,

    #[garde(skip)]
    pub epoch: Option<String>,

    #[garde(skip)]
    pub notes: Option<String>,
}

// ── Composition CRUD ──────────────────────────────────────────────────────────

/// Arguments for `add_formation_element`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct AddFormationElementArgs {
    #[garde(skip)]
    pub prototype_id: String,

    #[garde(skip)]
    pub owned_rolling_stock_id: Option<String>,
}

/// Arguments for `reorder_formation_elements`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct ReorderFormationElementsArgs {
    /// Complete ordered list of all element IDs.
    #[garde(length(min = 1))]
    pub element_ids: Vec<String>,
}

/// Arguments for `assign_rolling_stock_to_element`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct AssignRollingStockToElementArgs {
    /// `None` = unassign.
    #[garde(skip)]
    pub owned_rolling_stock_id: Option<String>,
}

/// Arguments for `set_traction_override`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct SetTractionOverrideArgs {
    /// 0 = use prototype default, 1 = force count, -1 = force exclude.
    #[garde(range(min = -1, max = 1))]
    pub traction_override: i32,
}

// ── Prototype Library ─────────────────────────────────────────────────────────

/// Arguments for `create_custom_prototype`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct CreateCustomPrototypeArgs {
    #[garde(skip)]
    pub railway_company_id: String,

    #[garde(length(min = 1, max = 50))]
    pub series_code: String,

    #[garde(skip)]
    pub friendly_name: Option<String>,

    #[garde(skip)]
    pub is_motorized: bool,

    #[garde(skip)]
    pub default_is_dummy: bool,

    #[garde(skip)]
    pub notes: Option<String>,

    /// Specification discriminator: `LOCOMOTIVE` | `PASSENGER_CAR` | `FREIGHT_CAR` |
    /// `RAILCAR` | `ELECTRIC_MULTIPLE_UNIT`
    #[garde(custom(validate_specification_type))]
    pub specification_type: String,

    // Locomotive-specific
    #[garde(skip)]
    pub locomotive_type: Option<String>,
    #[garde(skip)]
    pub locomotive_series: Option<String>,

    // PassengerCar-specific
    #[garde(skip)]
    pub service_level: Option<String>,
    #[garde(skip)]
    pub passenger_car_type: Option<String>,

    // FreightCar-specific
    #[garde(skip)]
    pub freight_car_type: Option<String>,

    // Railcar-specific
    #[garde(skip)]
    pub railcar_type: Option<String>,

    // ElectricMultipleUnit-specific
    #[garde(skip)]
    pub electric_multiple_unit_type: Option<String>,
    #[garde(skip)]
    pub elements_count: Option<i64>,
    #[garde(skip)]
    pub is_permanently_coupled: Option<bool>,
}

// ── Formation Categories ──────────────────────────────────────────────────────

/// Arguments for `create_formation_category`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct CreateFormationCategoryArgs {
    #[garde(length(min = 1, max = 80))]
    pub name: String,
}
