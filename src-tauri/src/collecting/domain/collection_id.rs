use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Identifier for a collection.
///
/// This newtype wraps a `Uuid` to provide a distinct domain type for collection
/// identifiers. Construction from strings is fallible — the string must be
/// a valid UUID representation (for example `"550e8400-e29b-41d4-a716-446655440000"`).
///
/// # Requirements
/// - `TryFrom<&str>` / `TryFrom<String>` will return an error if the provided
///   string is not a valid UUID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct CollectionId(pub Uuid);

/// Errors that can occur when creating a `CollectionId` from a string.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CollectionIdError {
    /// The provided string was not a valid UUID.
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

impl TryFrom<&str> for CollectionId {
    type Error = CollectionIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value)
            .map(CollectionId)
            .map_err(|_| CollectionIdError::InvalidUuid(value.to_string()))
    }
}

impl TryFrom<String> for CollectionId {
    type Error = CollectionIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map(CollectionId)
            .map_err(|_| CollectionIdError::InvalidUuid(value))
    }
}

impl From<Uuid> for CollectionId {
    fn from(u: Uuid) -> Self {
        CollectionId(u)
    }
}

impl Default for CollectionId {
    /// Generate a new `CollectionId` with a random v4 UUID.
    fn default() -> Self {
        CollectionId(Uuid::new_v4())
    }
}

impl fmt::Display for CollectionId {
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
        let id = CollectionId::try_from(s.as_str()).expect("should parse uuid");
        assert_eq!(id.0, u);
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn parse_invalid_uuid() {
        let err = CollectionId::try_from("not-a-uuid").expect_err("invalid uuid should fail");
        assert_eq!(err, CollectionIdError::InvalidUuid("not-a-uuid".to_string()));
    }

    #[test]
    fn from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = CollectionId::from(u);
        assert_eq!(id.to_string(), u.to_string());
    }

    #[test]
    fn serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = CollectionId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        // serde(transparent) -> serialized as plain string
        assert_eq!(s, format!("\"{}\"", u));
        let de: CollectionId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn default_generates_unique_uuid() {
        let a = CollectionId::default();
        let b = CollectionId::default();
        assert_ne!(a, b, "Two generated UUIDs should not be equal");
    }
}

