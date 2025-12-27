use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Strongly-typed identifier for a railway in the catalog domain.
///
/// `RailwayId` wraps a string value and provides a distinct type instead of
/// using plain strings everywhere. This improves type safety and makes intent
/// explicit in function signatures and data structures.
///
/// The inner string is kept private to allow the crate to enforce invariants
/// or to provide controlled constructors/parsers elsewhere. `RailwayId` also
/// derives Serde traits so it serializes/deserializes as a plain string.
///
/// Requirements
/// - The railway id MUST be a non-empty, non-blank string. Constructions via
///   `TryFrom<&str>` / `TryFrom<String>` will return an error if the input is
///   empty or contains only whitespace.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct RailwayId(String);

impl RailwayId {
    /// Creates a new `RailwayId` from the given string.
    ///
    /// # Parameters
    ///
    /// - `id`: the string identifier for the railway
    ///
    /// # Returns
    ///
    /// A new `RailwayId` instance wrapping the provided string.
    pub fn new<S: Into<String>>(id: S) -> Self {
        RailwayId(id.into())
    }
}

impl Deref for RailwayId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for RailwayId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("railway id must not be empty"));
        }
        Ok(RailwayId(value.to_owned()))
    }
}

impl TryFrom<String> for RailwayId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("railway id must not be empty"));
        }
        Ok(RailwayId(value))
    }
}

impl std::fmt::Display for RailwayId {
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
        let id = RailwayId::try_from("RY-ACME").expect("expected valid railway id");
        assert_eq!(id.0, "RY-ACME");
    }

    #[test]
    fn try_from_str_empty_fails() {
        let err = RailwayId::try_from("").expect_err("empty railway id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn try_from_string_blank_fails() {
        let err = RailwayId::try_from("   ".to_string()).expect_err("blank railway id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn deref_to_str() {
        let id = RailwayId::try_from("R-1").unwrap();
        let s: &str = &*id;
        assert_eq!(s, "R-1");
    }

    #[test]
    fn display_outputs_inner_string() {
        let id = RailwayId::try_from("RAIL-7").unwrap();
        assert_eq!(id.to_string(), "RAIL-7");
    }

    #[test]
    fn serde_roundtrip_as_string() {
        let id = RailwayId::try_from("RR-100").unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"RR-100\"");
        let de: RailwayId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
