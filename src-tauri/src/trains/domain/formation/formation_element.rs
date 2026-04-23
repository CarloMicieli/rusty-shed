use serde::{Deserialize, Serialize};

/// An ordered slot in a `TrainFormation`'s composition.
///
/// Each element anchors on a mandatory `Prototype` (the real-world rolling
/// stock class) and optionally assigns a specific owned physical model via
/// `owned_rolling_stock_id`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct FormationElement {
    /// Unique identifier for this element.
    /// Format: `trn:element:<uuid>`
    pub id: String,

    /// The `Prototype` this slot is based on.
    pub prototype_id: String,

    /// Optional reference to a specific owned physical model.
    pub owned_rolling_stock_id: Option<String>,

    /// 0-based sequential position within the formation.
    pub position_order: i32,

    /// Per-slot traction override.
    /// - `0` = use `Prototype.is_motorized` / `default_is_dummy` defaults
    /// - `1` = count as traction regardless of prototype classification
    /// - `-1` = exclude from traction count regardless of prototype
    pub traction_override: i32,
}
