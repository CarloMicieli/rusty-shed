use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifier for a single item in a collection.
///
/// This newtype wraps a `String` containing a TRN of the form
/// `trn:collection-item:{uuid}`. The UUID suffix is validated on construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct CollectionItemId(String);

impl_identifier_traits!(CollectionItemId);

impl AsRef<str> for CollectionItemId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for CollectionItemId {
    const PREFIX: &'static str = "trn:collection-item";

    fn from_string_unchecked(s: String) -> Self {
        CollectionItemId(s)
    }
}

impl CollectionItemId {
    /// Create a new `CollectionItemId` from a given `Uuid`.
    pub fn from_uuid(id: &Uuid) -> Self {
        CollectionItemId::new_from_parts(&[&id.to_string()])
    }
}

impl From<Uuid> for CollectionItemId {
    fn from(u: Uuid) -> Self {
        CollectionItemId::from_uuid(&u)
    }
}

impl Default for CollectionItemId {
    /// Create a new `CollectionItemId` with a freshly generated UUID (v4),
    /// wrapped in the TRN prefix.
    fn default() -> Self {
        let u = Uuid::new_v4();
        CollectionItemId::from_uuid(&u)
    }
}

/// Garde validator: rejects a `&str` that cannot be parsed as a `CollectionItemId`.
pub fn validate_collection_item_id(value: &str, _: &()) -> garde::Result {
    CollectionItemId::try_from(value)
        .map(|_| ())
        .map_err(|_| garde::Error::new("error_invalid_collection_item_id"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("trn:collection-item:{}", u);
        let id = CollectionItemId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = CollectionItemId::try_from(bad).expect_err("invalid trn should fail");
        assert!(format!("{}", err).contains("Invalid prefix"));
    }

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let expected = format!("trn:collection-item:{}", u);
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = CollectionItemId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        // serde(transparent) -> serialized as plain string
        let expected = format!("\"trn:collection-item:{}\"", u);
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
