//! `Prototype` domain aggregate — a real-world rolling stock class or series.

use crate::catalog::domain::prototype::prototype_id::PrototypeId;
use crate::catalog::domain::prototype::specification::Specification;
use crate::catalog::domain::railway_company::RailwayCompanyId;
use serde::{Deserialize, Serialize};

/// A real-world rolling stock class or series that serves as the master
/// catalog entry for train formation elements.
///
/// `Prototype` captures the physical and operational characteristics of a
/// particular rolling-stock class (e.g. the FS E.656 locomotive group or the
/// UIC-Z1 Gran Comfort coaches). Formation builder slots reference a
/// `Prototype` to describe the vehicle that the slot represents.
///
/// The optional `friendly_name` carries a well-known nickname for the class
/// (e.g. `"Caimano"` for the E.646 or `"Tartaruga"` for the E.444), separate
/// from the official `series_code`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Prototype {
    /// Unique TRN identifier.  Format: `trn:prototype:<railway>-<series-slug>`.
    pub id: PrototypeId,

    /// The railway company that operated this prototype.
    pub railway_company_id: RailwayCompanyId,

    /// Official series or class designation (e.g. `"E.656"`, `"nB"`, `"E.404"`).
    pub series_code: String,

    /// Well-known popular nickname for the class (e.g. `"Caimano"`, `"Tartaruga"`).
    pub friendly_name: Option<String>,

    /// Whether this is a display-only model with no working motor.
    pub default_is_dummy: bool,

    /// Whether a traction motor is built in.
    pub is_motorized: bool,

    /// `true` for user-created custom prototypes.
    pub is_custom: bool,

    /// Type-specific technical attributes.
    pub specification: Specification,

    /// Optional free-form notes.
    pub notes: Option<String>,
}
