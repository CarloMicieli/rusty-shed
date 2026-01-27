use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a maintenance event.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    specta::Type,
    sqlx::Type,
)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct MaintenanceEventId(String);

impl_identifier_traits!(MaintenanceEventId);

impl MaintenanceEventId {
    /// Creates a new `MaintenanceEventId` from a UUID.
    pub fn from_uuid(id: &uuid::Uuid) -> Self {
        MaintenanceEventId::new_from_parts(&[&id.to_string()])
    }
}

impl AsRef<str> for MaintenanceEventId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for MaintenanceEventId {
    const PREFIX: &'static str = "trn:maintenance-event";

    fn from_string_unchecked(s: String) -> Self {
        MaintenanceEventId(s)
    }
}

impl From<uuid::Uuid> for MaintenanceEventId {
    fn from(id: uuid::Uuid) -> Self {
        MaintenanceEventId::from_uuid(&id)
    }
}

impl Default for MaintenanceEventId {
    fn default() -> Self {
        MaintenanceEventId::from_uuid(&uuid::Uuid::new_v4())
    }
}
