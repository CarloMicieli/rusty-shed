use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a track product.
///
/// `TrackId` is a transparent newtype wrapping a `String` that stores a TRN
/// (Train) identifier for track products. The canonical form is:
///
/// trn:track:{manufacturer_slug}:{product_code_slug}
///
/// where both parts are slugified (lowercased, hyphen-separated).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct TrackId(pub String);

impl_identifier_traits!(TrackId);

impl AsRef<str> for TrackId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for TrackId {
    const PREFIX: &'static str = "trn:track";

    fn from_string_unchecked(s: String) -> Self {
        TrackId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_new_from_parts_generates_trn() {
        let id = TrackId::new_from_parts(&["ACME", "P-100"]);
        let expected = "trn:track:acme:p-100";
        assert_eq!(id.as_ref(), expected);
    }

    #[test]
    fn it_should_try_from_valid_trn_ok() {
        let s = "trn:track:mn-acme:p100";
        let id = TrackId::try_from(s).unwrap();
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn it_should_try_from_invalid_trn_fails() {
        let err = TrackId::try_from("not-a-trn").expect_err("should fail");
        assert!(format!("{}", err).contains("Invalid prefix"));
    }
}
