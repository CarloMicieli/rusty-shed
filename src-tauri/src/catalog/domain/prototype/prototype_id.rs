//! Strongly-typed identifier for the `Prototype` catalog domain entity.

use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a prototype in the catalog domain.
///
/// `PrototypeId` wraps a string value and provides a distinct type instead of
/// using plain strings everywhere. This improves type safety and makes intent
/// explicit in function signatures and data structures.
///
/// The identifier follows the TRN format: `trn:prototype:{railway}-{series-slug}`.
/// All identifiers are automatically slugified and validated.
///
/// # Examples
///
/// ```rust,ignore
/// use rusty_shed_lib::catalog::domain::prototype::prototype_id::PrototypeId;
/// use rusty_shed_lib::core::domain::identifiers::Identifier;
///
/// let id = PrototypeId::new_from_parts(&["fs-e444-tartaruga"]);
/// assert_eq!(id.as_ref(), "trn:prototype:fs-e444-tartaruga");
/// ```
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
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct PrototypeId(String);

impl_identifier_traits!(PrototypeId);

impl AsRef<str> for PrototypeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for PrototypeId {
    const PREFIX: &'static str = "trn:prototype";

    fn from_string_unchecked(s: String) -> Self {
        PrototypeId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_new_from_parts() {
        let id = PrototypeId::new_from_parts(&["fs-e444-tartaruga"]);
        assert_eq!(id.as_ref(), "trn:prototype:fs-e444-tartaruga");
    }

    #[test]
    fn it_should_try_from_str_success() {
        let id = PrototypeId::try_from("trn:prototype:fs-e444-tartaruga")
            .expect("expected valid prototype id");
        assert_eq!(id.as_ref(), "trn:prototype:fs-e444-tartaruga");
    }

    #[test]
    fn it_should_try_from_str_empty_fails() {
        let err = PrototypeId::try_from("").expect_err("empty prototype id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid prefix"));
    }

    #[test]
    fn it_should_try_from_str_invalid_prefix_fails() {
        let err = PrototypeId::try_from("trn:other:test").expect_err("wrong prefix should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid prefix"));
    }

    #[test]
    fn it_should_display_outputs_inner_string() {
        let id = PrototypeId::new_from_parts(&["db-e103"]);
        assert_eq!(id.to_string(), "trn:prototype:db-e103");
    }

    #[test]
    fn it_should_serde_roundtrip_as_string() {
        let id = PrototypeId::new_from_parts(&["sbb-re44-ii"]);
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"trn:prototype:sbb-re44-ii\"");
        let de: PrototypeId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
