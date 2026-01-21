use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

pub const TRN_PREFIX: &str = "trn:collection:";
pub const DEFAULT_COLLECTION_ID: &str = "1";

/// Identifier for a collection.
///
/// This newtype wraps a `String` containing a TRN of the form
/// `trn:collection:1`. Construction from strings is fallible — the string must
/// exactly match the enforced TRN for the single-supported collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct CollectionId(String);

/// Errors that can occur when creating a `CollectionId` from a string.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CollectionIdError {
    /// The provided string was not a valid collection TRN.
    #[error("invalid collection trn: {0}")]
    InvalidTrn(String),
}

impl TryFrom<&str> for CollectionId {
    type Error = CollectionIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let expected = format!("{}{}", TRN_PREFIX, DEFAULT_COLLECTION_ID);
        if value == expected {
            Ok(CollectionId(value.to_string()))
        } else {
            Err(CollectionIdError::InvalidTrn(value.to_string()))
        }
    }
}

impl TryFrom<&String> for CollectionId {
    type Error = CollectionIdError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        CollectionId::try_from(value.as_str())
    }
}

impl TryFrom<String> for CollectionId {
    type Error = CollectionIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        CollectionId::try_from(value.as_str())
    }
}

impl Default for CollectionId {
    /// Return the constant TRN-based collection id `trn:collection:1`.
    fn default() -> Self {
        CollectionId(format!("{}{}", TRN_PREFIX, DEFAULT_COLLECTION_ID))
    }
}

impl fmt::Display for CollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the inner string for display; inner field is private but accessible here
        write!(f, "{}", self.0)
    }
}

impl Deref for CollectionId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn() {
        let s = format!("{}{}", TRN_PREFIX, DEFAULT_COLLECTION_ID);
        let id = CollectionId::try_from(s.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = CollectionId::try_from(bad).expect_err("invalid trn should fail");
        assert_eq!(err, CollectionIdError::InvalidTrn(bad.to_string()));
    }

    #[test]
    fn it_should_from_str_and_display() {
        let s = format!("{}{}", TRN_PREFIX, DEFAULT_COLLECTION_ID);
        let id = CollectionId::try_from(s.as_str()).expect("should parse");
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let s = format!("{}{}", TRN_PREFIX, DEFAULT_COLLECTION_ID);
        let id = CollectionId::try_from(s.as_str()).expect("should parse");
        let ser = serde_json::to_string(&id).expect("serialize");
        // serde(transparent) -> serialized as plain string
        assert_eq!(ser, format!("\"{}\"", s));
        let de: CollectionId = serde_json::from_str(&ser).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn it_should_default_is_constant_trn() {
        let a = CollectionId::default();
        let b = CollectionId::default();
        assert_eq!(a, b, "Default collection id should be constant");
    }
}
