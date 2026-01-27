use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Identifier for a collection.
///
/// This newtype wraps a `String` containing a TRN of the form
/// `trn:collection:{id}`. The identifier follows standard TRN conventions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct CollectionId(String);

impl_identifier_traits!(CollectionId);

impl CollectionId {
    /// Default collection ID suffix.
    pub const DEFAULT_COLLECTION_ID: &str = "1";
}

impl AsRef<str> for CollectionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for CollectionId {
    const PREFIX: &'static str = "trn:collection";

    fn from_string_unchecked(s: String) -> Self {
        CollectionId(s)
    }
}

impl Default for CollectionId {
    /// Return the constant TRN-based collection id `trn:collection:1`.
    fn default() -> Self {
        CollectionId::new_from_parts(&[CollectionId::DEFAULT_COLLECTION_ID])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn() {
        let s = "trn:collection:1";
        let id = CollectionId::try_from(s).expect("should parse trn");
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = CollectionId::try_from(bad).expect_err("invalid trn should fail");
        assert!(format!("{}", err).contains("Invalid prefix"));
    }

    #[test]
    fn it_should_from_str_and_display() {
        let s = "trn:collection:1";
        let id = CollectionId::try_from(s).expect("should parse");
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let s = "trn:collection:1";
        let id = CollectionId::try_from(s).expect("should parse");
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
        assert_eq!(a.to_string(), "trn:collection:1");
    }
}
