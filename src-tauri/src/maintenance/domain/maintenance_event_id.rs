use serde::{Deserialize, Serialize};
use std::fmt;

/// Strongly-typed identifier for a maintenance event.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, specta::Type, sqlx::Type,
)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct MaintenanceEventId(String);

impl MaintenanceEventId {
    /// TRN prefix expected for maintenance event identifiers.
    pub const TRN_PREFIX: &'static str = "trn:maintenance-event:";

    /// Creates a new `MaintenanceEventId` from a UUID.
    ///
    /// # Parameters
    /// - `id`: the UUID to create the MaintenanceEventId from
    ///
    /// # Returns
    /// A new `MaintenanceEventId` instance with a TRN.
    pub fn new(id: &uuid::Uuid) -> Self {
        MaintenanceEventId(format!("{}{}", MaintenanceEventId::TRN_PREFIX, id))
    }
}

impl fmt::Display for MaintenanceEventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
