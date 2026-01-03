use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Strongly-typed identifier for a manufacturer in the catalog domain.
///
/// `ManufacturerId` wraps a `String` and enforces that values constructed via
/// the fallible `TryFrom` implementations are TRNs of the form
/// `trn:manufacturer:{slug}` where `{slug}` is a lowercase ASCII slug made of
/// letters, digits and hyphens (no leading/trailing hyphens, no uppercase).
///
/// The type serializes and deserializes as a plain string (`serde(transparent)`)
/// and implementing this invariant at construction time avoids accidental use
/// of invalid identifiers across the codebase.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, specta::Type, sqlx::Type,
)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct ManufacturerId(String);

impl ManufacturerId {
    /// TRN prefix expected for manufacturer identifiers.
    pub const TRN_PREFIX: &'static str = "trn:manufacturer:";

    /// Create a new ManufacturerId from any string-like value without
    /// validation. Prefer the fallible `TryFrom` constructors when parsing
    /// external input.
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

fn is_valid_slug(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Must be all lowercase ASCII, digits or hyphen, no leading/trailing hyphen
    let bytes = s.as_bytes();
    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        return false;
    }
    let mut prev_hyphen = false;
    for &b in bytes {
        match b {
            b'a'..=b'z' | b'0'..=b'9' => prev_hyphen = false,
            b'-' => {
                if prev_hyphen {
                    return false; // no consecutive hyphens
                }
                prev_hyphen = true;
            }
            _ => return false,
        }
    }
    true
}

impl TryFrom<&str> for ManufacturerId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("manufacturer id must not be empty"));
        }

        // If the value is already a TRN, validate the slug suffix.
        if let Some(suffix) = value.strip_prefix(ManufacturerId::TRN_PREFIX) {
            if !is_valid_slug(suffix) {
                return Err(anyhow!("manufacturer id has invalid slug"));
            }
            return Ok(ManufacturerId(value.to_owned()));
        }

        // Accept a legacy plain identifier composed of letters/digits/hyphens
        // (for example `MN-1`) and normalise it into the TRN form by lowercasing
        // the suffix. This keeps backwards compatibility with existing DB
        // fixtures while ensuring the domain type always stores a TRN.
        let s = value.trim();
        if s.bytes()
            .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-'))
        {
            let slug = s.to_ascii_lowercase();
            if !is_valid_slug(&slug) {
                return Err(anyhow!("manufacturer id has invalid slug"));
            }
            return Ok(ManufacturerId(format!(
                "{}{}",
                ManufacturerId::TRN_PREFIX,
                slug
            )));
        }

        Err(anyhow!(
            "manufacturer id must be a TRN: trn:manufacturer:{{slug}}"
        ))
    }
}

impl TryFrom<String> for ManufacturerId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ManufacturerId::try_from(value.as_str())
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
        let id = ManufacturerId::try_from("trn:manufacturer:mn-acme")
            .expect("expected valid manufacturer id");
        assert_eq!(id.0, "trn:manufacturer:mn-acme");
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
        let s = "trn:manufacturer:m-1".to_string();
        let id = ManufacturerId::try_from(&s).expect("expected valid manufacturer id from &String");
        assert_eq!(&*id, "trn:manufacturer:m-1");
    }

    #[test]
    fn deref_to_str() {
        let id = ManufacturerId::try_from("trn:manufacturer:man-7").unwrap();
        let s: &str = &id;
        assert_eq!(s, "trn:manufacturer:man-7");
    }

    #[test]
    fn display_outputs_inner_string() {
        let id = ManufacturerId::try_from("trn:manufacturer:man-100").unwrap();
        assert_eq!(id.to_string(), "trn:manufacturer:man-100");
    }

    #[test]
    fn serde_roundtrip_as_string() {
        let id = ManufacturerId::try_from("trn:manufacturer:mn-200").unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"trn:manufacturer:mn-200\"");
        let de: ManufacturerId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn invalid_prefix_fails() {
        let err = ManufacturerId::try_from("MN ACME").expect_err("should fail non-trn");
        let msg = format!("{}", err);
        assert!(msg.contains("must be a TRN"));
    }

    #[test]
    fn invalid_slug_fails() {
        // uppercase and spaces are invalid in slug
        let bad = "trn:manufacturer:Bad Slug";
        let err = ManufacturerId::try_from(bad).expect_err("invalid slug should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("invalid slug"));
    }
}
