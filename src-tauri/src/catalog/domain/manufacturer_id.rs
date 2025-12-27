//! Strongly-typed identifier for a manufacturer in the catalog domain.
//!
//! `ManufacturerId` wraps a `String` and exists so that functions and
//! data structures use a distinct type instead of raw strings. This improves
//! type-safety and avoids accidental mixing of different id types.
//!
//! The type serializes and deserializes as a plain string (`serde(transparent)`)
//! and enforces a small invariant when constructed via the fallible
//! `TryFrom` implementations: the id must not be empty or whitespace-only.
//!
//! See the `TryFrom` impls and unit tests for examples of usage.

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, specta::Type, sqlx::Type,
)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct ManufacturerId(String);

impl ManufacturerId {
    /// Create a new ManufacturerId from any string-like value.
    ///
    /// This constructor does not validate the input; prefer the fallible
    /// `TryFrom` implementations when you need to ensure the id is non-empty
    /// and non-blank.
    pub fn new<S: Into<String>>(value: S) -> Self {
        ManufacturerId(value.into())
    }
}

impl Deref for ManufacturerId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for ManufacturerId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("manufacturer id must not be empty"));
        }
        Ok(ManufacturerId(value.to_owned()))
    }
}

impl TryFrom<String> for ManufacturerId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("manufacturer id must not be empty"));
        }
        Ok(ManufacturerId(value))
    }
}

impl TryFrom<&String> for ManufacturerId {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        ManufacturerId::try_from(value.as_str())
    }
}

impl std::fmt::Display for ManufacturerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn try_from_str_success() {
        let id = ManufacturerId::try_from("MN-ACME").expect("expected valid manufacturer id");
        assert_eq!(id.0, "MN-ACME");
    }

    #[test]
    fn try_from_str_empty_fails() {
        let err = ManufacturerId::try_from("").expect_err("empty manufacturer id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn try_from_string_blank_fails() {
        let err = ManufacturerId::try_from("   ".to_string())
            .expect_err("blank manufacturer id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn try_from_ref_string_success() {
        let s = "M-1".to_string();
        let id = ManufacturerId::try_from(&s).expect("expected valid manufacturer id from &String");
        assert_eq!(&*id, "M-1");
    }

    #[test]
    fn deref_to_str() {
        let id = ManufacturerId::try_from("MAN-7").unwrap();
        let s: &str = &id;
        assert_eq!(s, "MAN-7");
    }

    #[test]
    fn display_outputs_inner_string() {
        let id = ManufacturerId::try_from("MAN-100").unwrap();
        assert_eq!(id.to_string(), "MAN-100");
    }

    #[test]
    fn serde_roundtrip_as_string() {
        let id = ManufacturerId::try_from("MN-200").unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"MN-200\"");
        let de: ManufacturerId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
