use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use uuid::Uuid;

/// Identifier for a single item in a collection.
///
/// This newtype wraps a `String` containing a TRN of the form
/// `trn:collection-item:{uuid}`. Construction from strings is fallible — the
/// string must start with the TRN prefix and the suffix must be a valid UUID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct CollectionItemId(String);

impl CollectionItemId {
    /// TRN prefix expected for collection item identifiers.
    pub const TRN_PREFIX: &str = "trn:collection-item:";

    /// Create a new `CollectionItemId` from a given `Uuid`.
    pub fn from_id(id: &Uuid) -> Self {
        CollectionItemId(format!("{}{}", Self::TRN_PREFIX, id))
    }
}

/// Errors that can occur when creating a `CollectionItemId` from a string.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CollectionItemIdError {
    /// The provided string did not contain a valid TRN with a UUID suffix.
    #[error("invalid collection item trn: {0}")]
    InvalidTrn(String),
    /// The UUID suffix was not a valid UUID.
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

impl TryFrom<&str> for CollectionItemId {
    type Error = CollectionItemIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(Self::TRN_PREFIX) {
            return Err(CollectionItemIdError::InvalidTrn(value.to_string()));
        }
        let suffix = &value[Self::TRN_PREFIX.len()..];
        match Uuid::parse_str(suffix) {
            Ok(u) => Ok(CollectionItemId(format!("{}{}", Self::TRN_PREFIX, u))),
            Err(_) => Err(CollectionItemIdError::InvalidUuid(suffix.to_string())),
        }
    }
}

impl TryFrom<&String> for CollectionItemId {
    type Error = CollectionItemIdError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        CollectionItemId::try_from(value.as_str())
    }
}

impl TryFrom<String> for CollectionItemId {
    type Error = CollectionItemIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        CollectionItemId::try_from(value.as_str())
    }
}

impl From<Uuid> for CollectionItemId {
    fn from(u: Uuid) -> Self {
        CollectionItemId(format!("{}{}", Self::TRN_PREFIX, u))
    }
}

impl Default for CollectionItemId {
    /// Create a new `CollectionItemId` with a freshly generated UUID (v4),
    /// wrapped in the TRN prefix.
    fn default() -> Self {
        let u = Uuid::new_v4();
        CollectionItemId(format!("{}{}", Self::TRN_PREFIX, u))
    }
}

impl fmt::Display for CollectionItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("{}{}", CollectionItemId::TRN_PREFIX, u);
        let id = CollectionItemId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = CollectionItemId::try_from(bad).expect_err("invalid trn should fail");
        assert_eq!(err, CollectionItemIdError::InvalidTrn(bad.to_string()));
    }

    #[test]
    fn it_should_parse_trn_with_invalid_uuid_suffix() {
        let bad = format!("{}{}", CollectionItemId::TRN_PREFIX, "not-a-uuid");
        let err = CollectionItemId::try_from(bad.as_str()).expect_err("invalid uuid should fail");
        assert_eq!(
            err,
            CollectionItemIdError::InvalidUuid("not-a-uuid".to_string())
        );
    }

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let expected = format!("{}{}", CollectionItemId::TRN_PREFIX, u);
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        // serde(transparent) -> serialized as plain string
        let expected = format!("\"{}{}\"", CollectionItemId::TRN_PREFIX, u);
        assert_eq!(s, expected);
        let de: CollectionItemId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn it_should_generates_unique_trn_default() {
        let a = CollectionItemId::default();
        let b = CollectionItemId::default();
        assert_ne!(a, b, "Two generated ids should not be equal");
    }
}
