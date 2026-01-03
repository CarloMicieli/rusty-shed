//! Domain-level identifier type for rolling stock (railway vehicles).
//!
//! `RollingStockId` is a lightweight, strongly-typed wrapper around a
//! TRN string of the form `trn:rolling-stock:{uuid}`. The type derives
//! `Serialize` / `Deserialize` for convenient (de)serialization in APIs.
//!
//! Semantics and usage:
//! - Create a new random id with `RollingStockId::new()`.
//! - Convert from a `Uuid` using `From<Uuid>` (produces a TRN string).
//! - Parse from a textual UUID representation via `str::FromStr` (accepts
//!   either a plain UUID or a full TRN). Use `value()` to obtain the
//!   underlying `Uuid`.

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::str;
use uuid::Uuid;

/// TRN prefix for rolling stock identifiers.
pub const TRN_RS_PREFIX: &str = "trn:rolling-stock:";

/// A unique identifier for a rolling stock represented as a TRN string.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct RollingStockId(String);

impl RollingStockId {
    /// Create a new random rolling stock id (TRN containing a v4 UUID).
    pub fn new() -> Self {
        RollingStockId::default()
    }

    /// Return the underlying `Uuid` value parsed from the TRN suffix.
    ///
    /// This will always succeed for ids produced by the constructors on this
    /// type. If the stored string is malformed this function will return a
    /// parsing error.
    pub fn value(&self) -> Uuid {
        // Stored form is always TRN, but accept plain uuid for legacy values.
        let s = if self.0.starts_with(TRN_RS_PREFIX) {
            &self.0[TRN_RS_PREFIX.len()..]
        } else {
            &self.0
        };
        Uuid::parse_str(s).expect("invalid uuid stored in RollingStockId")
    }

    /// Convenience: return the TRN string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RollingStockId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        RollingStockId(format!("{}{}", TRN_RS_PREFIX, id))
    }
}

impl fmt::Display for RollingStockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for RollingStockId {
    type Err = anyhow::Error;

    /// Parse a `RollingStockId` from either a plain UUID string or a full
    /// TRN `trn:rolling-stock:{uuid}`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If it's already a TRN with correct prefix, validate suffix is UUID
        if let Some(suffix) = s.strip_prefix(TRN_RS_PREFIX) {
            let _ = Uuid::try_parse(suffix).map_err(|_| anyhow!("invalid rolling stock id"))?;
            return Ok(RollingStockId(s.to_owned()));
        }

        // Otherwise try to parse as plain UUID and wrap into TRN form
        let id = Uuid::try_parse(s).map_err(|_| anyhow!("invalid rolling stock id"))?;
        Ok(RollingStockId(format!("{}{}", TRN_RS_PREFIX, id)))
    }
}

impl From<Uuid> for RollingStockId {
    /// Convert a `Uuid` into a `RollingStockId` TRN.
    fn from(id: Uuid) -> Self {
        RollingStockId(format!("{}{}", TRN_RS_PREFIX, id))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod rolling_stock_ids {
        use std::str::FromStr;

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_rolling_stock_id_from_str() {
            let id = "3302b9a7-252c-4b41-8de2-eb71efb1888e"
                .parse::<RollingStockId>()
                .unwrap();
            let uuid = Uuid::from_str("3302b9a7-252c-4b41-8de2-eb71efb1888e").unwrap();
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
            let result = "invalid value".parse::<RollingStockId>();
            assert!(result.is_err());
        }
    }
}
