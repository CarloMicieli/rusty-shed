use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use uuid::Uuid;

/// TRN prefix used for purchase identifiers.
pub const TRN_PURCHASE_PREFIX: &str = "trn:purchase:";

/// Strongly-typed identifier for PurchaseInfo records.
///
/// The inner string is private. Use `TryFrom<&str>` to validate TRN inputs or
/// `From<Uuid>` / `Default` to generate new ids. The type derefs to `str` for
/// ergonomic access to the underlying string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct PurchaseInfoId(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PurchaseInfoIdError {
    #[error("invalid purchase trn: {0}")]
    InvalidTrn(String),
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}

impl PurchaseInfoId {
    /// Construct without validation (convenience for tests/legacy values).
    pub fn new<S: Into<String>>(s: S) -> Self {
        PurchaseInfoId(s.into())
    }
}

impl Deref for PurchaseInfoId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for PurchaseInfoId {
    type Error = PurchaseInfoIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(TRN_PURCHASE_PREFIX) {
            return Err(PurchaseInfoIdError::InvalidTrn(value.to_string()));
        }
        let suffix = &value[TRN_PURCHASE_PREFIX.len()..];
        match Uuid::parse_str(suffix) {
            Ok(u) => Ok(PurchaseInfoId(format!("{}{}", TRN_PURCHASE_PREFIX, u))),
            Err(_) => Err(PurchaseInfoIdError::InvalidUuid(suffix.to_string())),
        }
    }
}

impl TryFrom<&String> for PurchaseInfoId {
    type Error = PurchaseInfoIdError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        PurchaseInfoId::try_from(value.as_str())
    }
}

impl TryFrom<String> for PurchaseInfoId {
    type Error = PurchaseInfoIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PurchaseInfoId::try_from(value.as_str())
    }
}

impl From<Uuid> for PurchaseInfoId {
    fn from(u: Uuid) -> Self {
        PurchaseInfoId(format!("{}{}", TRN_PURCHASE_PREFIX, u))
    }
}

impl Default for PurchaseInfoId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        PurchaseInfoId(format!("{}{}", TRN_PURCHASE_PREFIX, u))
    }
}

impl fmt::Display for PurchaseInfoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PurchaseInfoId {
    type Err = PurchaseInfoIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PurchaseInfoId::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("{}{}", TRN_PURCHASE_PREFIX, u);
        let id = PurchaseInfoId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = PurchaseInfoId::try_from(bad).expect_err("invalid trn should fail");
        assert_eq!(err, PurchaseInfoIdError::InvalidTrn(bad.to_string()));
    }

    #[test]
    fn parse_trn_with_invalid_uuid_suffix() {
        let bad = format!("{}{}", TRN_PURCHASE_PREFIX, "not-a-uuid");
        let err = PurchaseInfoId::try_from(bad.as_str()).expect_err("invalid uuid should fail");
        assert_eq!(
            err,
            PurchaseInfoIdError::InvalidUuid("not-a-uuid".to_string())
        );
    }

    #[test]
    fn from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = PurchaseInfoId::from(u);
        let expected = format!("{}{}", TRN_PURCHASE_PREFIX, u);
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = PurchaseInfoId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        let expected = format!("\"{}{}\"", TRN_PURCHASE_PREFIX, u);
        assert_eq!(s, expected);
        let de: PurchaseInfoId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
