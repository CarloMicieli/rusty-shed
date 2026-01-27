use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for PurchaseInfo records.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct PurchaseInfoId(String);

impl_identifier_traits!(PurchaseInfoId);

impl PurchaseInfoId {
    /// Create a `PurchaseInfoId` from a UUID.
    pub fn from_uuid(u: &Uuid) -> Self {
        PurchaseInfoId::new_from_parts(&[&u.to_string()])
    }
}

impl AsRef<str> for PurchaseInfoId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for PurchaseInfoId {
    const PREFIX: &'static str = "trn:purchase";

    fn from_string_unchecked(s: String) -> Self {
        PurchaseInfoId(s)
    }
}

impl From<Uuid> for PurchaseInfoId {
    fn from(u: Uuid) -> Self {
        PurchaseInfoId::from_uuid(&u)
    }
}

impl Default for PurchaseInfoId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        PurchaseInfoId::from_uuid(&u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_valid_trn_with_uuid_suffix() {
        let u = Uuid::new_v4();
        let trn = format!("trn:purchase:{}", u);
        let id = PurchaseInfoId::try_from(trn.as_str()).expect("should parse trn");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_parse_invalid_trn() {
        let bad = "not-a-trn";
        let err = PurchaseInfoId::try_from(bad).expect_err("invalid trn should fail");
        assert!(format!("{}", err).contains("Invalid prefix"));
    }

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = PurchaseInfoId::from(u);
        let expected = format!("trn:purchase:{}", u);
        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let u = Uuid::new_v4();
        let id = PurchaseInfoId::from(u);
        let s = serde_json::to_string(&id).expect("serialize");
        let expected = format!("\"trn:purchase:{}\"", u);
        assert_eq!(s, expected);
        let de: PurchaseInfoId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
