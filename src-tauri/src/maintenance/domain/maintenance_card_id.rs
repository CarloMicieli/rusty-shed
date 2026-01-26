use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Strongly-typed identifier for a maintenance card.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, specta::Type, sqlx::Type,
)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct MaintenanceCardId(String);

impl MaintenanceCardId {
    /// TRN prefix expected for maintenance card identifiers.
    pub const TRN_PREFIX: &'static str = "trn:maintenance-card:";

    /// Creates a new `MaintenanceCardId` from a UUID.
    ///
    /// # Parameters
    /// - `id`: the UUID to create the MaintenanceCardId from
    ///
    /// # Returns
    /// A new `MaintenanceCardId` instance with a TRN.
    pub fn new(id: &Uuid) -> Self {
        MaintenanceCardId(format!("{}{}", MaintenanceCardId::TRN_PREFIX, id))
    }
}

impl fmt::Display for MaintenanceCardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
