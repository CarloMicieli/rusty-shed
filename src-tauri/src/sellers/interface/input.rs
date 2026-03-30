use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::update_seller::UpdateSellerInput;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CreateSellerPayload {
    /// Seller name (1-200 characters).
    #[garde(length(min = 1, max = 200))]
    pub name: String,
    pub seller_type: SellerType,
    /// Optional email address.
    pub email: Option<String>,
    /// Optional phone number (max 30 characters).
    #[garde(length(max = 30))]
    pub phone: Option<String>,
    /// Optional website URL.
    pub website_url: Option<String>,
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    /// ISO 3166-1 alpha-2 country code (exactly 2 characters).
    #[garde(length(min = 2, max = 2))]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSellerPayload {
    pub id: String,
    /// Seller name (1-200 characters).
    #[garde(length(min = 1, max = 200))]
    pub name: String,
    pub seller_type: SellerType,
    /// Optional email address.
    pub email: Option<String>,
    /// Optional phone number (max 30 characters).
    #[garde(length(max = 30))]
    pub phone: Option<String>,
    /// Optional website URL.
    pub website_url: Option<String>,
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    /// ISO 3166-1 alpha-2 country code (exactly 2 characters).
    #[garde(length(min = 2, max = 2))]
    pub country_code: Option<String>,
    pub created_at: Option<String>,
}

impl TryFrom<UpdateSellerPayload> for UpdateSellerInput {
    type Error = CommandError;

    fn try_from(payload: UpdateSellerPayload) -> Result<Self, Self::Error> {
        let UpdateSellerPayload {
            id,
            name,
            seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
        } = payload;

        let sid = SellerId::try_from(id.as_str())
            .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

        let created_at_dt = if let Some(created_at_str) = created_at {
            match chrono::DateTime::parse_from_rfc3339(created_at_str.as_str()) {
                Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
                Err(e) => return Err(CommandError::validation_field("createdAt", e.to_string())),
            }
        } else {
            None
        };

        Ok(UpdateSellerInput {
            id: sid,
            name,
            seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at: created_at_dt,
        })
    }
}

#[cfg(test)]
mod garde_tests {
    use super::*;
    use crate::sellers::domain::seller_type::SellerType;
    use garde::Validate;

    fn valid_create() -> CreateSellerPayload {
        CreateSellerPayload {
            name: "Model Train Shop".to_string(),
            seller_type: SellerType::Shop,
            email: None,
            phone: None,
            website_url: None,
            street_address: None,
            extended_address: None,
            city: None,
            state_region: None,
            postal_code: None,
            country_code: None,
        }
    }

    #[test]
    fn create_seller_valid_passes() {
        assert!(valid_create().validate().is_ok());
    }

    #[test]
    fn create_seller_empty_name_fails() {
        let payload = CreateSellerPayload {
            name: String::new(),
            ..valid_create()
        };
        let report = payload.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "name"),
            "{errors:?}"
        );
    }

    #[test]
    fn create_seller_name_too_long_fails() {
        let payload = CreateSellerPayload {
            name: "x".repeat(201),
            ..valid_create()
        };
        let report = payload.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "name"),
            "{errors:?}"
        );
    }

    #[test]
    fn create_seller_country_code_wrong_length_fails() {
        let payload = CreateSellerPayload {
            country_code: Some("DEU".to_string()),
            ..valid_create()
        };
        let report = payload.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "country_code"),
            "{errors:?}"
        );
    }

    #[test]
    fn create_seller_phone_too_long_fails() {
        let payload = CreateSellerPayload {
            phone: Some("1".repeat(31)),
            ..valid_create()
        };
        let report = payload.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "phone"),
            "{errors:?}"
        );
    }
}
