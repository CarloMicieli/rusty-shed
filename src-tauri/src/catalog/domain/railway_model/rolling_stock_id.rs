use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use std::str;
use uuid::Uuid;

/// A unique identifier for a rolling stock represented as a TRN string.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct RollingStockId(String);

impl_identifier_traits!(RollingStockId);

impl RollingStockId {
    /// Create a new `RollingStockId` from a UUID.
    pub fn from_uuid(id: &Uuid) -> Self {
        RollingStockId::new_from_parts(&[&id.to_string()])
    }

    /// Return the underlying `Uuid` value parsed from the TRN suffix.
    pub fn value(&self) -> Uuid {
        let s = &self.0[Self::PREFIX.len() + 1..];
        Uuid::parse_str(s).expect("invalid uuid stored in RollingStockId")
    }
}

impl AsRef<str> for RollingStockId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for RollingStockId {
    const PREFIX: &'static str = "trn:rolling-stock";

    fn from_string_unchecked(s: String) -> Self {
        RollingStockId(s)
    }
}

impl From<Uuid> for RollingStockId {
    fn from(id: Uuid) -> Self {
        RollingStockId::from_uuid(&id)
    }
}

impl Default for RollingStockId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        RollingStockId::from_uuid(&id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod rolling_stock_ids {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_rolling_stock_id_from_str() {
            let uuid_str = "3302b9a7-252c-4b41-8de2-eb71efb1888e";
            let trn = format!("trn:rolling-stock:{}", uuid_str);
            let id = RollingStockId::try_from(trn.as_str()).unwrap();
            let uuid = Uuid::parse_str(uuid_str).unwrap();
            assert_eq!(uuid, id.value());
        }

        #[test]
        fn it_should_create_new_rolling_stock_id_from_uuid() {
            let uuid = Uuid::new_v4();
            let rolling_stock_id: RollingStockId = uuid.into();
            assert_eq!(uuid, rolling_stock_id.value());
        }

        #[test]
        fn it_should_fail_to_parse_invalid_values_as_rolling_stocks() {
            let result = RollingStockId::try_from("invalid value");
            assert!(result.is_err());
        }
    }
}
