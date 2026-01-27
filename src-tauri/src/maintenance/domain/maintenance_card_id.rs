use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a maintenance card.
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
pub struct MaintenanceCardId(String);

impl_identifier_traits!(MaintenanceCardId);

impl MaintenanceCardId {
    /// Creates a new `MaintenanceCardId` from a UUID.
    pub fn from_uuid(id: &Uuid) -> Self {
        MaintenanceCardId::new_from_parts(&[&id.to_string()])
    }
}

impl AsRef<str> for MaintenanceCardId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for MaintenanceCardId {
    const PREFIX: &'static str = "trn:maintenance-card";

    fn from_string_unchecked(s: String) -> Self {
        MaintenanceCardId(s)
    }
}

impl From<Uuid> for MaintenanceCardId {
    fn from(id: Uuid) -> Self {
        MaintenanceCardId::from_uuid(&id)
    }
}

impl Default for MaintenanceCardId {
    fn default() -> Self {
        MaintenanceCardId::from_uuid(&Uuid::new_v4())
    }
}
