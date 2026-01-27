use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a railway in the catalog domain.
///
/// `RailwayCompanyId` wraps a string value and provides a distinct type instead of
/// using plain strings everywhere. This improves type safety and makes intent
/// explicit in function signatures and data structures.
///
/// The identifier follows the TRN format: `trn:railway-company:{slug}`.
/// All identifiers are automatically slugified and validated.
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
pub struct RailwayCompanyId(String);

impl_identifier_traits!(RailwayCompanyId);

impl AsRef<str> for RailwayCompanyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for RailwayCompanyId {
    const PREFIX: &'static str = "trn:railway-company";

    fn from_string_unchecked(s: String) -> Self {
        RailwayCompanyId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_new_from_parts() {
        let id = RailwayCompanyId::new_from_parts(&["ACME Railways"]);
        assert_eq!(id.as_ref(), "trn:railway-company:acme-railways");
    }

    #[test]
    fn it_should_try_from_str_success() {
        let id = RailwayCompanyId::try_from("trn:railway-company:acme")
            .expect("expected valid railway id");
        assert_eq!(id.as_ref(), "trn:railway-company:acme");
    }

    #[test]
    fn it_should_try_from_str_empty_fails() {
        let err = RailwayCompanyId::try_from("").expect_err("empty railway id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid prefix"));
    }

    #[test]
    fn it_should_try_from_str_invalid_prefix_fails() {
        let err =
            RailwayCompanyId::try_from("trn:other:test").expect_err("wrong prefix should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid prefix"));
    }

    #[test]
    fn it_should_display_outputs_inner_string() {
        let id = RailwayCompanyId::new_from_parts(&["RAIL-7"]);
        assert_eq!(id.to_string(), "trn:railway-company:rail-7");
    }

    #[test]
    fn it_should_serde_roundtrip_as_string() {
        let id = RailwayCompanyId::new_from_parts(&["RR-100"]);
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"trn:railway-company:rr-100\"");
        let de: RailwayCompanyId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
