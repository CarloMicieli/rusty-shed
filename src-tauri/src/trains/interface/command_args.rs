//! Input argument structs for the train-formations Tauri commands.
//!
//! All structs:
//! - derive `garde::Validate` for boundary validation
//! - derive `specta::Type` so tauri-specta generates TypeScript bindings
//! - derive `serde::Deserialize` for Tauri deserialization

use garde::Validate;

// ── Shared validation helpers ─────────────────────────────────────────────────

/// Allowed `car_type` values (mirrors `data-model.md` enum).
const VALID_CAR_TYPES: &[&str] = &[
    "Locomotive",
    "PowerCar",
    "Coach",
    "Couchette",
    "Dining",
    "Sleeping",
    "BaggageCar",
    "ControlCar",
    "FreightWagon",
];

fn validate_car_type_enum(car_type: &str, _ctx: &()) -> garde::Result {
    if VALID_CAR_TYPES.contains(&car_type) {
        Ok(())
    } else {
        Err(garde::Error::new(format!(
            "car_type '{car_type}' is not a valid car type; expected one of: {}",
            VALID_CAR_TYPES.join(", ")
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

    #[garde(skip)]
    pub start_year: Option<i32>,

    #[garde(skip)]
    pub end_year: Option<i32>,

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

    #[garde(skip)]
    pub start_year: Option<i32>,

    #[garde(skip)]
    pub end_year: Option<i32>,

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

    #[garde(custom(validate_car_type_enum))]
    pub car_type: String,

    #[garde(skip)]
    pub service_level: Option<String>,

    #[garde(skip)]
    pub category: String,

    #[garde(skip)]
    pub is_motorized: bool,

    #[garde(skip)]
    pub default_is_dummy: bool,

    #[garde(skip)]
    pub notes: Option<String>,
}

// ── Formation Categories ──────────────────────────────────────────────────────

/// Arguments for `create_formation_category`.
#[derive(Debug, Clone, serde::Deserialize, specta::Type, Validate)]
pub struct CreateFormationCategoryArgs {
    #[garde(length(min = 1, max = 80))]
    pub name: String,
}
