use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// A product identifier (manufacturer model/code) used to uniquely identify
/// a rolling stock model or catalogue item.
///
/// This is a thin newtype wrapper around `String` to provide domain-level
/// type-safety and to allow attaching trait impls specific to product codes.
///
/// It derives `Serialize`/`Deserialize` for easy (de)serialization with Serde.
///
/// Requirements
/// - The product code MUST be a non-empty, non-blank string. Constructions via
///   `TryFrom<&str>` / `TryFrom<String>` will return an error if the input is
///   empty or contains only whitespace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct ProductCode(pub String);

impl Deref for ProductCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for ProductCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("product code must not be empty"));
        }
        Ok(ProductCode(value.to_owned()))
    }
}

impl TryFrom<String> for ProductCode {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("product code must not be empty"));
        }
        Ok(ProductCode(value))
    }
}

impl std::fmt::Display for ProductCode {
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
        let product_code = ProductCode::try_from("ACME-123").expect("expected valid product code");
        assert_eq!(product_code.0, "ACME-123");
    }

    #[test]
    fn try_from_str_empty_fails() {
        let err = ProductCode::try_from("").expect_err("empty product code should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn try_from_string_blank_fails() {
        let err =
            ProductCode::try_from("   ".to_string()).expect_err("blank product code should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must not be empty"));
    }

    #[test]
    fn display_outputs_inner_string() {
        let product_code = ProductCode::try_from("X-1").unwrap();
        assert_eq!(product_code.to_string(), "X-1");
    }

    #[test]
    fn serde_roundtrip_as_string() {
        let product_code = ProductCode::try_from("SER-9").unwrap();
        let s = serde_json::to_string(&product_code).expect("serialize");
        // since ProductCode is serde(transparent) it should serialize as a plain JSON string
        assert_eq!(s, "\"SER-9\"");
        let de: ProductCode = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, product_code);
    }
}
