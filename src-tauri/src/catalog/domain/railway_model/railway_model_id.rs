use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::Trn;
use crate::core::domain::identifiers::Identifier;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Strongly-typed railway model identifier.
///
/// `RailwayModelId` follows a TRN-like pattern with manufacturer namespace
/// and product code:
///
/// trn:railway-model:{manufacturer_slug}:{product_code}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct RailwayModelId(String);

impl AsRef<str> for RailwayModelId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for RailwayModelId {
    const PREFIX: &'static str = "trn:railway-model";

    fn from_string_unchecked(s: String) -> Self {
        RailwayModelId(s)
    }
}

impl RailwayModelId {
    /// Create a new `RailwayModelId` from a `ManufacturerId` and a product code.
    ///
    /// Parses the provided `manufacturer_id` as a `Trn` and uses its NSS
    /// component as the manufacturer namespace included in the resulting ID.
    /// Note: The product code is lowercased but NOT slugified to preserve its original format.
    pub fn new(
        manufacturer_id: &ManufacturerId,
        product_code: &str,
    ) -> Result<Self, RailwayModelIdError> {
        let manufacturer_trn = Trn::from_str(manufacturer_id.as_ref())
            .map_err(|_| RailwayModelIdError::InvalidManufacturerId)?;

        // Trim and replace whitespace with hyphens before lowercasing
        let sanitized = product_code
            .trim()
            .replace(char::is_whitespace, "-")
            .to_lowercase();
        let id = format!("{}:{}:{}", Self::PREFIX, manufacturer_trn.nss(), sanitized);
        Ok(RailwayModelId::from_string_unchecked(id))
    }
}

impl std::fmt::Display for RailwayModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum RailwayModelIdError {
    #[error("railway model id must not be empty")]
    EmptyId,
    #[error("invalid manufacturer id in railway model id")]
    InvalidManufacturerId,
    #[error("invalid manufacturer namespace in railway model id")]
    InvalidManufacturerNamespace,
    #[error("invalid product code in railway model id")]
    InvalidProductCode,
    #[error("railway model id must be a TRN: trn:railway-model:<manufacturer>:<product>")]
    InvalidFormat,
}

impl TryFrom<&str> for RailwayModelId {
    type Error = RailwayModelIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.trim();
        if v.is_empty() {
            return Err(RailwayModelIdError::EmptyId);
        }

        // Expect the TRN prefix
        if !v.starts_with(&format!("{}:", Self::PREFIX)) {
            return Err(RailwayModelIdError::InvalidFormat);
        }

        let rest = &v[Self::PREFIX.len() + 1..];

        // Split into exactly two components: manufacturer_nss and product_code
        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() != 2 {
            return Err(RailwayModelIdError::InvalidFormat);
        }

        let manufacturer_nss = parts[0].trim();
        let product_code = parts[1].trim();

        if manufacturer_nss.is_empty() {
            return Err(RailwayModelIdError::InvalidManufacturerNamespace);
        }
        if product_code.is_empty() {
            return Err(RailwayModelIdError::InvalidProductCode);
        }
        if product_code.contains(char::is_whitespace) {
            return Err(RailwayModelIdError::InvalidProductCode);
        }

        // Validate manufacturer_nss is a valid slug (lowercase alphanumeric + hyphens, no spaces)
        if manufacturer_nss.contains(char::is_whitespace) {
            return Err(RailwayModelIdError::InvalidManufacturerNamespace);
        }

        // Validate the manufacturer NSS by attempting to create a ManufacturerId from a TRN
        let manufacturer_trn = format!("trn:manufacturer:{}", manufacturer_nss);
        ManufacturerId::try_from(manufacturer_trn.as_str())
            .map_err(|_| RailwayModelIdError::InvalidManufacturerNamespace)?;

        Ok(RailwayModelId(v.to_owned()))
    }
}

impl TryFrom<String> for RailwayModelId {
    type Error = RailwayModelIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        RailwayModelId::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const MANUFACTURER_ID_TRN: &str = "trn:manufacturer:acme";
    const RAILWAY_MODEL_TRN: &str = "trn:railway-model:acme:123456";

    #[test]
    fn it_should_try_from_str_success() {
        let id = RailwayModelId::try_from(RAILWAY_MODEL_TRN).expect("expected valid id");
        assert_eq!(id.to_string(), RAILWAY_MODEL_TRN);
    }

    #[rstest]
    #[case("", RailwayModelIdError::EmptyId)]
    #[case("   ", RailwayModelIdError::EmptyId)]
    #[case("invalid-format", RailwayModelIdError::InvalidFormat)]
    #[case(
        "trn:railway-model::123456",
        RailwayModelIdError::InvalidManufacturerNamespace
    )]
    #[case("trn:railway-model:acme:", RailwayModelIdError::InvalidProductCode)]
    #[case(
        "trn:railway-model:unknown manufacturer:123456",
        RailwayModelIdError::InvalidManufacturerNamespace
    )]
    #[case(
        "trn:railway-model:acme:1252 116",
        RailwayModelIdError::InvalidProductCode
    )]
    fn try_from_str_empty_fails(#[case] input: &str, #[case] expected_error: RailwayModelIdError) {
        let err = RailwayModelId::try_from(input).expect_err("empty id should fail");
        assert_eq!(err, expected_error);
    }

    #[test]
    fn it_should_deref_to_str() {
        let id = RailwayModelId::try_from(RAILWAY_MODEL_TRN).unwrap();
        let s: &str = id.as_ref();
        assert_eq!(s, RAILWAY_MODEL_TRN);
    }

    #[test]
    fn serde_roundtrip_as_string() {
        let id = RailwayModelId::try_from(RAILWAY_MODEL_TRN).unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        assert_eq!(s, "\"trn:railway-model:acme:123456\"");
        let de: RailwayModelId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn it_should_new_with_trn_manufacturer_success() {
        let manufacturer_id =
            ManufacturerId::try_from(MANUFACTURER_ID_TRN).expect("valid manufacturer trn");
        let railway_model_id =
            RailwayModelId::new(&manufacturer_id, "P123").expect("valid Railway model ID");

        assert_eq!(railway_model_id.to_string(), "trn:railway-model:acme:p123");
    }

    #[test]
    fn it_should_new_replace_spaces_with_hyphens() {
        let manufacturer_id =
            ManufacturerId::try_from(MANUFACTURER_ID_TRN).expect("valid manufacturer trn");
        let id = RailwayModelId::new(&manufacturer_id, "1252 116").expect("valid id");
        assert_eq!(id.to_string(), "trn:railway-model:acme:1252-116");
    }

    #[test]
    fn it_should_new_trim_leading_trailing_spaces() {
        let manufacturer_id =
            ManufacturerId::try_from(MANUFACTURER_ID_TRN).expect("valid manufacturer trn");
        let id = RailwayModelId::new(&manufacturer_id, " P100 ").expect("valid id");
        assert_eq!(id.to_string(), "trn:railway-model:acme:p100");
    }

    #[test]
    fn it_should_new_replace_multiple_spaces_with_hyphens() {
        let manufacturer_id =
            ManufacturerId::try_from(MANUFACTURER_ID_TRN).expect("valid manufacturer trn");
        let id = RailwayModelId::new(&manufacturer_id, "1252  116").expect("valid id");
        assert_eq!(id.to_string(), "trn:railway-model:acme:1252--116");
    }

    #[test]
    fn it_should_new_with_non_trn_manufacturer() {
        // Pass a ManufacturerId that doesn't follow TRN format
        let m = ManufacturerId::from_string_unchecked("invalid-format".to_string());
        let result = RailwayModelId::new(&m, "P1");

        assert!(result.is_err());
        let err = result.expect_err("expected error for invalid manufacturer id");
        let msg = format!("{}", err);
        assert!(msg.contains("invalid manufacturer id"));
    }
}
