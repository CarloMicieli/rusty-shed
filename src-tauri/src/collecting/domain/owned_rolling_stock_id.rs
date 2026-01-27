use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for an `OwnedRollingStock`.
///
/// A small strongly-typed newtype for owned rolling stock identifiers used by the
/// collecting domain. These identifiers are represented as TRNs (token resource
/// names) and have the form `trn:owned-rolling-stock:{uuid}` where the suffix is
/// a standard UUID (v4) string. The type is serde-transparent so it serializes
/// as a plain string in JSON and database mappings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct OwnedRollingStockId(String);

impl_identifier_traits!(OwnedRollingStockId);

impl OwnedRollingStockId {
    /// Create an `OwnedRollingStockId` from a UUID.
    pub fn from_uuid(u: &Uuid) -> Self {
        OwnedRollingStockId::new_from_parts(&[&u.to_string()])
    }
}

impl AsRef<str> for OwnedRollingStockId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for OwnedRollingStockId {
    const PREFIX: &'static str = "trn:owned-rolling-stock";

    fn from_string_unchecked(s: String) -> Self {
        OwnedRollingStockId(s)
    }
}

impl From<Uuid> for OwnedRollingStockId {
    fn from(u: Uuid) -> Self {
        OwnedRollingStockId::from_uuid(&u)
    }
}

impl Default for OwnedRollingStockId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        OwnedRollingStockId::from_uuid(&u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("trn:owned-rolling-stock:{}", u);
        let id = OwnedRollingStockId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = OwnedRollingStockId::try_from(bad).expect_err("invalid trn should fail");
        assert!(format!("{}", err).contains("Invalid prefix"));
    }

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = OwnedRollingStockId::from(u);
        let expected = format!("trn:owned-rolling-stock:{}", u);
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = OwnedRollingStockId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        let expected = format!("\"trn:owned-rolling-stock:{}\"", u);
        assert_eq!(s, expected);
        let de: OwnedRollingStockId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
