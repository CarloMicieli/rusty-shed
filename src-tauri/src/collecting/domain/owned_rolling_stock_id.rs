use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use uuid::Uuid;

/// Strongly-typed identifier for an `OwnedRollingStock`.
///
/// A small strongly-typed newtype for owned rolling stock identifiers used by the
/// collecting domain. These identifiers are represented as TRNs (token resource
/// names) and have the form `trn:owned-rolling-stock:{uuid}` where the suffix is
/// a standard UUID (v4) string. The type is serde-transparent so it serializes
/// as a plain string in JSON and database mappings.
///
/// The inner string is private; construction should use `TryFrom<&str>` to
/// validate the TRN format or `From<Uuid>` / `Default` to generate a fresh id.
/// The type derives `Serialize`/`Deserialize` and is `#[serde(transparent)]` so
/// it appears as a simple string in serialized forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct OwnedRollingStockId(String);

impl OwnedRollingStockId {
    /// The TRN prefix for owned rolling stock identifiers.
    pub const TRN_PREFIX: &str = "trn:owned-rolling-stock:";

    /// Construct an `OwnedRollingStockId` without validating the format.
    ///
    /// This is intended as a lightweight convenience (e.g. for tests or when
    /// working with legacy values). Use `TryFrom<&str>` to validate user or
    /// external input.
    pub fn new<S: Into<String>>(s: S) -> Self {
        OwnedRollingStockId(s.into())
    }

    /// Get the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for OwnedRollingStockId {
    type Error = OwnedRollingStockIdError;

    /// Try to parse a TRN of the form `trn:owned-rolling-stock:{uuid}`.
    ///
    /// Returns `Err(InvalidTrn(..))` when the prefix is missing and
    /// `Err(InvalidUuid(..))` when the suffix is not a valid UUID.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(Self::TRN_PREFIX) {
            return Err(OwnedRollingStockIdError::InvalidTrn(value.to_string()));
        }
        let suffix = &value[Self::TRN_PREFIX.len()..];
        match Uuid::parse_str(suffix) {
            Ok(u) => Ok(OwnedRollingStockId(format!("{}{}", Self::TRN_PREFIX, u))),
            Err(_) => Err(OwnedRollingStockIdError::InvalidUuid(suffix.to_string())),
        }
    }
}

impl TryFrom<&String> for OwnedRollingStockId {
    type Error = OwnedRollingStockIdError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        OwnedRollingStockId::try_from(value.as_str())
    }
}

impl TryFrom<String> for OwnedRollingStockId {
    type Error = OwnedRollingStockIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        OwnedRollingStockId::try_from(value.as_str())
    }
}

impl From<Uuid> for OwnedRollingStockId {
    /// Create a TRN from a `Uuid` value.
    fn from(u: Uuid) -> Self {
        OwnedRollingStockId(format!("{}{}", Self::TRN_PREFIX, u))
    }
}

impl Default for OwnedRollingStockId {
    /// Generate a fresh TRN using a newly generated UUID (v4).
    fn default() -> Self {
        let u = Uuid::new_v4();
        OwnedRollingStockId(format!("{}{}", Self::TRN_PREFIX, u))
    }
}

impl fmt::Display for OwnedRollingStockId {
    /// Display the inner TRN string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors that may occur while parsing an `OwnedRollingStockId` from a string.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OwnedRollingStockIdError {
    /// The provided string does not start with the expected TRN prefix.
    #[error("invalid owned rolling stock trn: {0}")]
    InvalidTrn(String),

    /// The suffix after the TRN prefix is not a valid UUID.
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("{}{}", OwnedRollingStockId::TRN_PREFIX, u);
        let id = OwnedRollingStockId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = OwnedRollingStockId::try_from(bad).expect_err("invalid trn should fail");
        assert_eq!(err, OwnedRollingStockIdError::InvalidTrn(bad.to_string()));
    }

    #[test]
    fn it_should_parse_trn_with_invalid_uuid_suffix() {
        let bad = format!("{}{}", OwnedRollingStockId::TRN_PREFIX, "not-a-uuid");
        let err =
            OwnedRollingStockId::try_from(bad.as_str()).expect_err("invalid uuid should fail");
        assert_eq!(
            err,
            OwnedRollingStockIdError::InvalidUuid("not-a-uuid".to_string())
        );
    }

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = OwnedRollingStockId::from(u);
        let expected = format!("{}{}", OwnedRollingStockId::TRN_PREFIX, u);
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = OwnedRollingStockId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        let expected = format!("\"{}{}\"", OwnedRollingStockId::TRN_PREFIX, u);
        assert_eq!(s, expected);
        let de: OwnedRollingStockId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
