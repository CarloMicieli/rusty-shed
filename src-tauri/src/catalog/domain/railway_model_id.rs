use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// A strongly-typed identifier for a railway model.
///
/// This newtype wraps a `String` so that code dealing with railway model
/// identifiers can use a distinct type instead of raw `String`s. It derives
/// `Serialize` and `Deserialize` so it can be used directly with `serde`.
///
/// Requirements
/// - The railway model id MUST be a non-empty, non-blank string. Constructions
///   via `TryFrom<&str>` / `TryFrom<String>` will return an error if the input
///   is empty or contains only whitespace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct RailwayModelId(String);

impl Deref for RailwayModelId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for RailwayModelId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("railway model id must not be empty"));
        }
        Ok(RailwayModelId(value.to_owned()))
    }
}

impl TryFrom<String> for RailwayModelId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("railway model id must not be empty"));
        }
        Ok(RailwayModelId(value))
    }
}

impl std::fmt::Display for RailwayModelId {
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
        let id = RailwayModelId::try_from("RM-2025").expect("expected valid id");
        assert_eq!(id.0, "RM-2025");
    }

    #[test]
    fn try_from_str_empty_fails() {
        let err = RailwayModelId::try_from("").expect_err("empty id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn try_from_string_blank_fails() {
        let err = RailwayModelId::try_from("   ".to_string()).expect_err("blank id should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn deref_to_str() {
        let id = RailwayModelId::try_from("R-1").unwrap();
        // Deref should allow &*id to be &str
        let s: &str = &*id;
        assert_eq!(s, "R-1");
    }

    #[test]
    fn serde_roundtrip_as_string() {
        let id = RailwayModelId::try_from("RM-42").unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"RM-42\"");
        let de: RailwayModelId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }
}
