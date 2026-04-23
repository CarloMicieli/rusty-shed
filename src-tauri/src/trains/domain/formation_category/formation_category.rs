use serde::{Deserialize, Serialize};

/// A named classification for a type of train formation
/// (e.g. `"EuroCity"`, `"TEE"`, `"Regional"`).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct FormationCategory {
    /// Unique identifier.  Format: `trn:formation-category:<slug>`
    pub id: String,

    /// Display name (globally unique).
    pub name: String,

    /// `true` for user-created custom categories.
    pub is_custom: bool,
}
