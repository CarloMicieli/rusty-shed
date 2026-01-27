use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a decoder (master record).
///
/// This newtype is `#[repr(transparent)]` and serializes as a plain string.
/// URNs are of the form `trn:decoder:{manufacturer}:{product_code}`.
#[repr(transparent)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[sqlx(transparent)]
#[specta(transparent)]
pub struct DecoderId(String);

impl_identifier_traits!(DecoderId);

impl AsRef<str> for DecoderId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for DecoderId {
    const PREFIX: &'static str = "trn:decoder";

    fn from_string_unchecked(s: String) -> Self {
        DecoderId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_decoder_id_from_parts_normalises() {
        let id = DecoderId::new_from_parts(&["ACME Ltd", "P 1000"]);
        assert_eq!(id.as_ref(), "trn:decoder:acme-ltd:p-1000");
    }

    #[test]
    fn it_should_decoder_id_display_and_try_from() {
        let id = DecoderId::new_from_parts(&["A", "B"]);
        let s = id.to_string();
        let parsed = DecoderId::try_from(s.as_str()).expect("parse ok");
        assert_eq!(parsed, id);
    }

    #[test]
    fn it_should_decoder_id_try_from_empty_fails() {
        let parsed = DecoderId::try_from("");
        assert!(
            parsed.is_err(),
            "empty string should not parse to DecoderId"
        );
    }
}
