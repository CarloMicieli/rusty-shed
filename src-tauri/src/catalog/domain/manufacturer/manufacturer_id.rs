use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a manufacturer in the catalog domain.
///
/// `ManufacturerId` wraps a `String` and enforces that values constructed via
/// the fallible `TryFrom` implementations are TRNs of the form
/// `trn:manufacturer:{slug}` where `{slug}` is a lowercase ASCII slug made of
/// letters, digits and hyphens (no leading/trailing hyphens, no uppercase).
///
/// The type serializes and deserializes as a plain string (`serde(transparent)`)
/// and implementing this invariant at construction time avoids accidental use
/// of invalid identifiers across the codebase.
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
pub struct ManufacturerId(String);

impl_identifier_traits!(ManufacturerId);

impl AsRef<str> for ManufacturerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for ManufacturerId {
    const PREFIX: &'static str = "trn:manufacturer";

    fn from_string_unchecked(s: String) -> Self {
        ManufacturerId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::identifiers::IdParseError;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_try_from_str_success() {
        let id = ManufacturerId::try_from("trn:manufacturer:mn-acme")
            .expect("expected valid manufacturer id");
        assert_eq!(id.as_ref(), "trn:manufacturer:mn-acme");
    }

    #[test]
    fn it_should_try_from_str_empty_fails() {
        let result = ManufacturerId::try_from("");
        assert!(result.is_err());
        match result.unwrap_err() {
            IdParseError::InvalidPrefix { .. } => {}
            _ => panic!("Expected InvalidPrefix error"),
        }
    }

    #[test]
    fn it_should_try_from_string_blank_fails() {
        let result = ManufacturerId::try_from("   ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn it_should_try_from_ref_string_success() {
        let s = "trn:manufacturer:m-1".to_string();
        let id = ManufacturerId::try_from(&s).expect("expected valid manufacturer id from &String");
        assert_eq!(id.as_ref(), "trn:manufacturer:m-1");
    }

    #[test]
    fn it_should_display_outputs_inner_string() {
        let id = ManufacturerId::try_from("trn:manufacturer:man-100").unwrap();
        assert_eq!(id.to_string(), "trn:manufacturer:man-100");
    }

    #[test]
    fn it_should_serde_roundtrip_as_string() {
        let id = ManufacturerId::try_from("trn:manufacturer:mn-200").unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"trn:manufacturer:mn-200\"");
        let de: ManufacturerId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn it_should_invalid_prefix_fails() {
        let result = ManufacturerId::try_from("trn:other:acme");
        assert!(result.is_err());
        match result.unwrap_err() {
            IdParseError::InvalidPrefix { expected, .. } => {
                assert_eq!(expected, "trn:manufacturer");
            }
            _ => panic!("Expected InvalidPrefix error"),
        }
    }

    #[test]
    fn it_should_create_from_name() {
        let id = ManufacturerId::new_from_parts(&["ACME Corporation"]);
        assert_eq!(id.as_ref(), "trn:manufacturer:acme-corporation");
    }

    #[test]
    fn it_should_validate_correct_format() {
        assert!(ManufacturerId::is_valid("trn:manufacturer:acme"));
        assert!(ManufacturerId::is_valid("trn:manufacturer:marklin"));
    }

    #[test]
    fn it_should_reject_invalid_format() {
        assert!(!ManufacturerId::is_valid("trn:other:acme"));
        assert!(!ManufacturerId::is_valid("trn:manufacturer:"));
        assert!(!ManufacturerId::is_valid(""));
    }
}
