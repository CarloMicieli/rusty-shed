use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use uuid::Uuid;

pub const TRN_ITEM_PREFIX: &str = "trn:collection-item:";

/// Identifier for a single item in a collection.
///
/// This newtype wraps a `String` containing a TRN of the form
/// `trn:collection-item:{uuid}`. Construction from strings is fallible — the
/// string must start with the TRN prefix and the suffix must be a valid UUID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct CollectionItemId(String);

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
        if !value.starts_with(TRN_ITEM_PREFIX) {
            return Err(CollectionItemIdError::InvalidTrn(value.to_string()));
        }
        let suffix = &value[TRN_ITEM_PREFIX.len()..];
        match Uuid::parse_str(suffix) {
            Ok(u) => Ok(CollectionItemId(format!("{}{}", TRN_ITEM_PREFIX, u))),
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
        CollectionItemId(format!("{}{}", TRN_ITEM_PREFIX, u))
    }
}

impl Default for CollectionItemId {
    /// Create a new `CollectionItemId` with a freshly generated UUID (v4),
    /// wrapped in the TRN prefix.
    fn default() -> Self {
        let u = Uuid::new_v4();
        CollectionItemId(format!("{}{}", TRN_ITEM_PREFIX, u))
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
    fn parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("{}{}", TRN_ITEM_PREFIX, u);
        let id = CollectionItemId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = CollectionItemId::try_from(bad).expect_err("invalid trn should fail");
        assert_eq!(err, CollectionItemIdError::InvalidTrn(bad.to_string()));
    }

    #[test]
    fn parse_trn_with_invalid_uuid_suffix() {
        let bad = format!("{}{}", TRN_ITEM_PREFIX, "not-a-uuid");
        let err = CollectionItemId::try_from(bad.as_str()).expect_err("invalid uuid should fail");
        assert_eq!(
            err,
            CollectionItemIdError::InvalidUuid("not-a-uuid".to_string())
        );
    }

    #[test]
    fn from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let expected = format!("{}{}", TRN_ITEM_PREFIX, u.to_string());
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        // serde(transparent) -> serialized as plain string
        let expected = format!("\"{}{}\"", TRN_ITEM_PREFIX, u);
        assert_eq!(s, expected);
        let de: CollectionItemId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn generates_unique_trn_default() {
        let a = CollectionItemId::default();
        let b = CollectionItemId::default();
        assert_ne!(a, b, "Two generated ids should not be equal");
    }
}
