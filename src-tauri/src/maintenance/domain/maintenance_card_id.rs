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

/// Garde validator: rejects a `&str` that cannot be parsed as a `MaintenanceCardId`.
pub fn validate_maintenance_card_id(value: &str, _: &()) -> garde::Result {
    MaintenanceCardId::try_from(value)
        .map(|_| ())
        .map_err(|_| garde::Error::new("error_invalid_maintenance_card_id"))
}

impl Default for MaintenanceCardId {
    fn default() -> Self {
        MaintenanceCardId::from_uuid(&Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_accepts_valid_trn() {
        let id = MaintenanceCardId::from_uuid(&Uuid::new_v4());
        assert!(validate_maintenance_card_id(&id.to_string(), &()).is_ok());
    }

    #[test]
    fn validate_rejects_empty_string() {
        let err = validate_maintenance_card_id("", &()).unwrap_err();
        assert_eq!(err.to_string(), "error_invalid_maintenance_card_id");
    }

    #[test]
    fn validate_rejects_wrong_prefix() {
        let err = validate_maintenance_card_id("trn:other:some-id", &()).unwrap_err();
        assert_eq!(err.to_string(), "error_invalid_maintenance_card_id");
    }

    #[test]
    fn validate_rejects_plain_string() {
        let err = validate_maintenance_card_id("not-a-trn", &()).unwrap_err();
        assert_eq!(err.to_string(), "error_invalid_maintenance_card_id");
    }
}
