use serde::{Deserialize, Serialize};
use slug::slugify;
use std::fmt;
use std::ops::Deref;

/// TRN prefix for track identifiers.
pub const TRN_PREFIX: &str = "trn:track:";

/// Strongly-typed identifier for a track product.
///
/// `TrackId` is a transparent newtype wrapping a `String` that stores a TRN
/// (Train) identifier for track products. The canonical form produced by
/// `TrackId::new_from_parts` is:
///
/// trn:track:{manufacturer_slug}:{product_code_slug}
///
/// where `{manufacturer_slug}` and `{product_code_slug}` are the slugified
/// namespace-specific parts (lowercased, hyphen-separated). Prefer using the
/// provided constructors and `TryFrom` implementations to validate external
/// input. The type serializes as a plain string and is `sqlx::transparent` for
/// convenient persistence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct TrackId(pub String);

impl TrackId {
    pub fn new_from_parts(manufacturer: &str, product_code: &str) -> Self {
        let m = slugify(manufacturer);
        let p = slugify(product_code);
        TrackId(format!("{}{}:{}", TRN_PREFIX, m, p))
    }
}

impl Deref for TrackId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TrackIdError {
    #[error("invalid track trn: {0}")]
    InvalidTrn(String),
}

impl TryFrom<&str> for TrackId {
    type Error = TrackIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(TRN_PREFIX) {
            return Err(TrackIdError::InvalidTrn(value.to_string()));
        }
        // basic validation: ensure suffix contains ':' separating manufacturer and product
        let suffix = &value[TRN_PREFIX.len()..];
        if !suffix.contains(":") {
            return Err(TrackIdError::InvalidTrn(value.to_string()));
        }
        Ok(TrackId(value.to_owned()))
    }
}

impl TryFrom<String> for TrackId {
    type Error = TrackIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TrackId::try_from(value.as_str())
    }
}

impl fmt::Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_from_parts_generates_trn() {
        let id = TrackId::new_from_parts("ACME", "P-100");
        let expected = format!("{}{}:{}", TRN_PREFIX, slugify("ACME"), slugify("P-100"));
        assert_eq!(id.0, expected);
    }

    #[test]
    fn try_from_valid_trn_ok() {
        let s = format!("{}{}:{}", TRN_PREFIX, "mn-acme", "p100");
        let id = TrackId::try_from(s.as_str()).unwrap();
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn try_from_invalid_trn_fails() {
        let err = TrackId::try_from("not-a-trn").expect_err("should fail");
        assert_eq!(format!("{}", err), "invalid track trn: not-a-trn");
    }
}
