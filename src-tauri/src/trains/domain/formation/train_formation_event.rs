//! Domain events for the `TrainFormation` aggregate.
//!
//! Each variant represents a discrete, past-tense fact that occurred to
//! a [`TrainFormation`]. Events are recorded in `pending_events` and
//! drained by the repository after every successful persistence cycle.

use crate::trains::domain::formation::formation_element::FormationElement;
use serde::{Deserialize, Serialize};

/// Discrete events emitted by the `TrainFormation` aggregate.
///
/// The enum uses a `tag`/`content` externally-tagged serde representation
/// so that the TypeScript bindings produced by `tauri-specta` can
/// discriminate variants at runtime.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum TrainFormationEvent {
    /// A new formation was created with the given `id` and initial `name`.
    Created { id: String, name: String },

    /// The formation was renamed.
    Renamed { name: String },

    /// Optional metadata fields were updated.
    MetadataUpdated {
        category_id: Option<String>,
        start_year: Option<i32>,
        end_year: Option<i32>,
        epoch: Option<String>,
        notes: Option<String>,
    },

    /// A new element was appended to the formation.
    ElementAdded { element: FormationElement },

    /// An element was removed from the formation.
    ElementRemoved { element_id: String },

    /// All element positions were reordered.
    ElementsReordered { ordered_element_ids: Vec<String> },

    /// An owned rolling-stock model was assigned to a slot.
    RollingStockAssigned {
        element_id: String,
        owned_rolling_stock_id: String,
    },

    /// The rolling-stock assignment was cleared from a slot.
    RollingStockUnassigned { element_id: String },

    /// The per-slot traction override value was changed.
    TractionOverrideSet {
        element_id: String,
        traction_override: i32,
    },

    /// The formation was deleted.
    Deleted { id: String },
}
