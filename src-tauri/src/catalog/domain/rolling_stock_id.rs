//! Domain-level identifier type for rolling stock (railway vehicles).
//!
//! `RollingStockId` is a lightweight, strongly-typed wrapper around a
//! `Uuid` used throughout the domain and persistence layers to identify a
//! particular rolling stock instance. The type is marked `#[sqlx(transparent)]` so
//! it stores as a plain UUID in the database, and derives `Serialize` /
//! `Deserialize` for convenient (de)serialization in APIs.
//!
//! Semantics and usage:
//! - Create a new random id with `RollingStockId::new()`.
//! - Convert from a `Uuid` using `From<Uuid>`.
//! - Parse from a textual UUID representation via `str::FromStr`.
//! - Obtain the underlying `Uuid` with `value()` (it is returned by value
//!   since `Uuid` is `Copy`).
//!
//! Example (non-doctest):
//! ```text
//! let id = RollingStockId::new();
//! let uuid = id.value();
//! let parsed: RollingStockId = uuid.to_string().parse().expect("valid uuid");
//! ```

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::str;
use uuid::Uuid;

/// A unique identifier for a rolling stock.
///
/// This is a thin, domain-specific wrapper around `Uuid` that provides
/// stronger typing in the codebase so rolling stock IDs are not confused
/// with other UUIDs. It is `Copy` and `Clone` which makes it convenient
/// to pass by value.
///
/// Persistence and serialization:
/// - `#[sqlx(transparent)]` ensures the value is stored as a regular UUID
///   column in SQLite/Postgres when using `sqlx`.
/// - `Serialize`/`Deserialize` derive implementations allow easy JSON
///   encoding for APIs or fixtures.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct RollingStockId(Uuid);

impl RollingStockId {
    /// Create a new random rolling stock id.
    ///
    /// This generates a new v4 UUID under the hood. Prefer this when
    /// creating new domain objects which require a unique identifier.
    pub fn new() -> Self {
        RollingStockId::default()
    }

    /// Return the underlying `Uuid` value.
    ///
    /// `Uuid` is `Copy`, so this returns the UUID by value (cheap).
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for RollingStockId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        RollingStockId(id)
    }
}

impl fmt::Display for RollingStockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for RollingStockId {
    type Err = anyhow::Error;

    /// Parse a `RollingStockId` from its string representation.
    ///
    /// Returns an error if the input is not a valid UUID string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::try_parse(s).map_err(|_| anyhow!("invalid rolling stock id"))?;
        Ok(RollingStockId(id))
    }
}

impl From<Uuid> for RollingStockId {
    /// Convert a `Uuid` into a `RollingStockId`.
    fn from(id: Uuid) -> Self {
        RollingStockId(id)
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
            assert_eq!(
                RollingStockId(Uuid::from_str("3302b9a7-252c-4b41-8de2-eb71efb1888e").unwrap()),
                id
            );
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
