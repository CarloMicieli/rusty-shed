//! `Prototype` domain struct — a real-world rolling stock class or series.

use serde::{Deserialize, Serialize};

/// A real-world rolling stock class or series that serves as the master
/// catalog entry for formation elements.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Prototype {
    /// Unique identifier.  Format: `trn:prototype:<railway>-<series-slug>`
    pub id: String,

    /// The railway company that operated this prototype.
    pub railway_company_id: String,

    /// Human-readable series or class designation (e.g. `"E.444 Tartaruga"`).
    pub series_code: String,

    /// Rolling-stock type (e.g. `"Locomotive"`, `"Coach"`).
    pub car_type: String,

    /// Optional service-level string (e.g. `"1st Class"`).
    pub service_level: Option<String>,

    /// High-level category (e.g. `"Passenger"`, `"Freight"`).
    pub category: String,

    /// Whether a traction motor is built in.
    pub is_motorized: bool,

    /// Whether this is a display-only model with no working motor.
    pub default_is_dummy: bool,

    /// `true` for user-created custom prototypes.
    pub is_custom: bool,

    /// Optional free-form notes.
    pub notes: Option<String>,
}
