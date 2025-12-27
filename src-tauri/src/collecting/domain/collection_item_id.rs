use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Identifier for a single item in a collection.
///
/// This newtype wraps a `Uuid` to provide a distinct domain type for collection
/// item identifiers. Construction from strings is fallible — the string must be
/// a valid UUID representation (for example `"550e8400-e29b-41d4-a716-446655440000"`).
///
/// # Requirements
/// - `TryFrom<&str>` / `TryFrom<String>` will return an error if the provided
///   string is not a valid UUID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct CollectionItemId(pub Uuid);

/// Errors that can occur when creating a `CollectionItemId` from a string.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CollectionItemIdError {
    /// The provided string was not a valid UUID.
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

impl TryFrom<&str> for CollectionItemId {
    type Error = CollectionItemIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value)
            .map(CollectionItemId)
            .map_err(|_| CollectionItemIdError::InvalidUuid(value.to_string()))
    }
}

impl TryFrom<&String> for CollectionItemId {
    type Error = CollectionItemIdError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Uuid::parse_str(value)
            .map(CollectionItemId)
            .map_err(|_| CollectionItemIdError::InvalidUuid(value.to_string()))
    }
}

impl TryFrom<String> for CollectionItemId {
    type Error = CollectionItemIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map(CollectionItemId)
            .map_err(|_| CollectionItemIdError::InvalidUuid(value))
    }
}

impl From<Uuid> for CollectionItemId {
    fn from(u: Uuid) -> Self {
        CollectionItemId(u)
    }
}

impl Default for CollectionItemId {
    /// Create a new `CollectionItemId` with a freshly generated UUID (v4).
    ///
    /// This implements `Default` so callers can use `CollectionItemId::default()`
    /// when creating a new identifier.
    fn default() -> Self {
        CollectionItemId(Uuid::new_v4())
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
    fn parse_valid_uuid() {
        let u = Uuid::new_v4();
        let s = u.to_string();
        let id = CollectionItemId::try_from(s.as_str()).expect("should parse uuid");
        assert_eq!(id.0, u);
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn parse_invalid_uuid() {
        let err = CollectionItemId::try_from("not-a-uuid").expect_err("invalid uuid should fail");
        assert_eq!(
            err,
            CollectionItemIdError::InvalidUuid("not-a-uuid".to_string())
        );
    }

    #[test]
    fn from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        assert_eq!(id.to_string(), u.to_string());
    }

    #[test]
    fn serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        // serde(transparent) -> serialized as plain string
        assert_eq!(s, format!("\"{}\"", u));
        let de: CollectionItemId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn generates_unique_uuid() {
        let a = CollectionItemId::default();
        let b = CollectionItemId::default();
        assert_ne!(a, b, "Two generated UUIDs should not be equal");
    }
}
