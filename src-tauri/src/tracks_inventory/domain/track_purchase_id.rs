use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use uuid::Uuid;

pub const TRN_PURCHASE_PREFIX: &str = "trn:track-purchase:";

/// Strongly-typed identifier for a track purchase record.
///
/// `TrackPurchaseId` is a transparent newtype wrapping a `String` that stores
/// the canonical TRN for purchase events. The expected form is:
///
/// `trn:track-purchase:{UUID}`
///
/// Construct instances via `From<Uuid>`, `Default` (generates a new UUID) or
/// the fallible `TryFrom<&str>`/`TryFrom<String>` implementations to validate
/// external input. The type serializes as a plain string and is `sqlx::transparent`
/// for convenient persistence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct TrackPurchaseId(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TrackPurchaseIdError {
    #[error("invalid purchase trn: {0}")]
    InvalidTrn(String),
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

impl TrackPurchaseId {
    pub fn new_from_uuid(u: Uuid) -> Self {
        TrackPurchaseId(format!("{}{}", TRN_PURCHASE_PREFIX, u))
    }
}

impl Deref for TrackPurchaseId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for TrackPurchaseId {
    type Error = TrackPurchaseIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(TRN_PURCHASE_PREFIX) {
            return Err(TrackPurchaseIdError::InvalidTrn(value.to_string()));
        }
        let suffix = &value[TRN_PURCHASE_PREFIX.len()..];
        match Uuid::parse_str(suffix) {
            Ok(u) => Ok(TrackPurchaseId(format!("{}{}", TRN_PURCHASE_PREFIX, u))),
            Err(_) => Err(TrackPurchaseIdError::InvalidUuid(suffix.to_string())),
        }
    }
}

impl TryFrom<String> for TrackPurchaseId {
    type Error = TrackPurchaseIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TrackPurchaseId::try_from(value.as_str())
    }
}

impl From<Uuid> for TrackPurchaseId {
    fn from(u: Uuid) -> Self {
        TrackPurchaseId::new_from_uuid(u)
    }
}

impl Default for TrackPurchaseId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        TrackPurchaseId::from(u)
    }
}

impl fmt::Display for TrackPurchaseId {
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
    fn from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = TrackPurchaseId::from(u);
        assert_eq!(id.to_string(), format!("{}{}", TRN_PURCHASE_PREFIX, u));
    }

    #[test]
    fn try_from_invalid_trn() {
        let err = TrackPurchaseId::try_from("bad").unwrap_err();
        assert!(matches!(err, TrackPurchaseIdError::InvalidTrn(_)));
    }
}
