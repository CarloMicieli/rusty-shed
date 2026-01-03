use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::core::domain::Trn;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const TRN_PREFIX: &str = "trn:railway-model:";

/// Strongly-typed railway model identifier.
///
/// `RailwayModelId` is a thin newtype over `String` used to represent railway
/// model identifiers across the domain. Values created with the provided
/// constructor follow a TRN-like pattern and include the manufacturer namespace
/// and the product code. The canonical form produced by `RailwayModelId::new`
/// is:
///
/// trn:railway-model:{manufacturer_nss}:{product_code}
///
/// where `{manufacturer_nss}` is the namespace-specific part (NSS) of a
/// `ManufacturerId` TRN (for example the `mn-acme` part of
/// `trn:manufacturer:mn-acme`). The module-level constant `TRN_PREFIX` holds
/// the `trn:railway-model:` prefix used by the constructor.
///
/// Notes on construction:
/// - `TryFrom<&str>` and `TryFrom<String>` perform only a non-empty/blank
///   check and will accept any non-blank string (they do not parse or validate
///   TRN structure).
/// - Use `RailwayModelId::new(manufacturer_id, product_code)` to create an
///   instance from a `ManufacturerId`; this validates the manufacturer and
///   returns a `RailwayModelIdError` on failure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, specta::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct RailwayModelId(String);

impl RailwayModelId {
    /// Create a new `RailwayModelId` from a `ManufacturerId` and a product code.
    ///
    /// Behaviour:
    /// - Parses the provided `manufacturer_id` as a `Trn` and uses its NSS
    ///   component as the manufacturer namespace included in the resulting ID.
    /// - The `product_code` is appended verbatim after the manufacturer NSS.
    ///
    /// Returns:
    /// - `Ok(RailwayModelId)` when the manufacturer is valid and a value is
    ///   constructed successfully.
    /// - `Err(RailwayModelIdError::InvalidManufacturerId)` when the
    ///   `manufacturer_id` cannot be parsed as a `Trn`.
    pub fn new(
        manufacturer_id: &ManufacturerId,
        product_code: &str,
    ) -> Result<Self, RailwayModelIdError> {
        let manufacturer_trn = Trn::from_str(manufacturer_id)
            .map_err(|_| RailwayModelIdError::InvalidManufacturerId)?;
        let value = format!("{}{}:{}", TRN_PREFIX, manufacturer_trn.nss(), product_code);
        Ok(RailwayModelId(value))
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

impl Deref for RailwayModelId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for RailwayModelId {
    type Error = RailwayModelIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.trim();
        if v.is_empty() {
            return Err(RailwayModelIdError::EmptyId);
        }

        // Expect the TRN prefix: trn:railway-model:{manufacturer_nss}:{product_code}
        let rest = v
            .strip_prefix(TRN_PREFIX)
            .ok_or(RailwayModelIdError::InvalidFormat)?;

        // Split the remaining part into exactly two components: manufacturer_nss and product_code
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

        // Validate the manufacturer NSS by attempting to create a ManufacturerId from a TRN
        let manufacturer_trn = format!("{}{}", ManufacturerId::TRN_PREFIX, manufacturer_nss);
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

impl fmt::Display for RailwayModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
    fn try_from_str_success() {
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
    fn try_from_str_empty_fails(#[case] input: &str, #[case] expected_error: RailwayModelIdError) {
        let err = RailwayModelId::try_from(input).expect_err("empty id should fail");
        assert_eq!(err, expected_error);
    }

    #[test]
    fn deref_to_str() {
        let id = RailwayModelId::try_from(RAILWAY_MODEL_TRN).unwrap();
        let s: &str = &id;
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
    fn new_with_trn_manufacturer_success() {
        let manufacturer_id =
            ManufacturerId::try_from(MANUFACTURER_ID_TRN).expect("valid manufacturer trn");
        let railway_model_id =
            RailwayModelId::new(&manufacturer_id, "P123").expect("valid Railway model ID");

        assert_eq!(railway_model_id.to_string(), "trn:railway-model:acme:P123");
    }

    #[test]
    fn new_with_non_trn_manufacturer() {
        let m = ManufacturerId::new("not-a-trn");
        let result = RailwayModelId::new(&m, "P1");

        assert!(result.is_err());
        let err = result.expect_err("expected error for invalid manufacturer id");
        let msg = format!("{}", err);
        assert!(msg.contains("invalid manufacturer id"));
    }
}
