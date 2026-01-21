use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use uuid::Uuid;

pub const TRN_INVENTORY_PREFIX: &str = "trn:track-inventory:";

/// Strongly-typed identifier for a track inventory record.
///
/// `TrackInventoryId` is a transparent newtype wrapping a `String` that stores
/// the canonical TRN for inventory aggregates. The expected form is:
///
/// `trn:track-inventory:{UUID}`
///
/// Construct instances via `From<Uuid>`, `Default` (generates a new UUID) or
/// the fallible `TryFrom<&str>`/`TryFrom<String>` implementations to validate
/// external input. The type serializes as a plain string and is `sqlx::transparent`
/// for convenient persistence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct TrackInventoryId(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TrackInventoryIdError {
    #[error("invalid inventory trn: {0}")]
    InvalidTrn(String),
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

impl TrackInventoryId {
    pub fn new_from_uuid(u: Uuid) -> Self {
        TrackInventoryId(format!("{}{}", TRN_INVENTORY_PREFIX, u))
    }
}

impl Deref for TrackInventoryId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for TrackInventoryId {
    type Error = TrackInventoryIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(TRN_INVENTORY_PREFIX) {
            return Err(TrackInventoryIdError::InvalidTrn(value.to_string()));
        }
        let suffix = &value[TRN_INVENTORY_PREFIX.len()..];
        match Uuid::parse_str(suffix) {
            Ok(u) => Ok(TrackInventoryId(format!("{}{}", TRN_INVENTORY_PREFIX, u))),
            Err(_) => Err(TrackInventoryIdError::InvalidUuid(suffix.to_string())),
        }
    }
}

impl TryFrom<String> for TrackInventoryId {
    type Error = TrackInventoryIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TrackInventoryId::try_from(value.as_str())
    }
}

impl From<Uuid> for TrackInventoryId {
    fn from(u: Uuid) -> Self {
        TrackInventoryId::new_from_uuid(u)
    }
}

impl Default for TrackInventoryId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        TrackInventoryId::from(u)
    }
}

impl fmt::Display for TrackInventoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = TrackInventoryId::from(u);
        assert_eq!(id.to_string(), format!("{}{}", TRN_INVENTORY_PREFIX, u));
    }

    #[test]
    fn it_should_try_from_invalid_trn_fails() {
        let bad = "not-a-trn";
        let err = TrackInventoryId::try_from(bad).expect_err("invalid trn should fail");
        assert_eq!(err, TrackInventoryIdError::InvalidTrn(bad.to_string()));
    }

    #[test]
    fn it_should_try_from_trn_with_invalid_uuid_suffix_fails() {
        let bad = format!("{}{}", TRN_INVENTORY_PREFIX, "not-a-uuid");
        let err = TrackInventoryId::try_from(bad.as_str()).expect_err("invalid uuid should fail");
        assert_eq!(
            err,
            TrackInventoryIdError::InvalidUuid("not-a-uuid".to_string())
        );
    }
}
