use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

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

impl DecoderId {
    /// Construct a decoder URN from manufacturer and product code parts.
    ///
    /// Both inputs are trimmed, lowercased and internal whitespace is replaced
    /// with hyphens. Example: `("ACME Ltd", " P 1000") -> trn:decoder:acme-ltd:p-1000`.
    pub fn from_parts(manufacturer: &str, product_code: &str) -> Self {
        fn norm(s: &str) -> String {
            s.trim()
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join("-")
        }

        let id = format!("trn:decoder:{}:{}", norm(manufacturer), norm(product_code));
        DecoderId(id)
    }

    /// Create a `DecoderId` from a raw string without validation.
    pub fn new<S: Into<String>>(s: S) -> Self {
        DecoderId(s.into())
    }
}

impl Deref for DecoderId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for DecoderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DecoderId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(anyhow::anyhow!("decoder id must not be empty"));
        }
        Ok(DecoderId(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_decoder_id_from_parts_normalises() {
        let id = DecoderId::from_parts(" ACME Ltd ", " P 1000 ");
        assert_eq!(&*id, "trn:decoder:acme-ltd:p-1000");
    }

    #[test]
    fn it_should_decoder_id_display_and_fromstr() {
        let id = DecoderId::from_parts("A", "B");
        let s = id.to_string();
        let parsed = s.parse::<DecoderId>().expect("parse ok");
        assert_eq!(parsed, id);
    }

    #[test]
    fn it_should_decoder_id_fromstr_empty_fails() {
        let parsed = "".parse::<DecoderId>();
        assert!(
            parsed.is_err(),
            "empty string should not parse to DecoderId"
        );
    }
}
