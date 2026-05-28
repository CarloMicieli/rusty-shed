use crate::core::domain::address::Address;
use crate::core::domain::metadata::Metadata;
use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::update_seller::UpdateSellerInput;
use crate::sellers::domain::seller::Seller as DomainSeller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Seller {
    pub id: SellerId,
    pub name: String,
    pub seller_type: SellerType,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub address: Option<Address>,
    pub metadata: Metadata,
}

impl From<DomainSeller> for Seller {
    fn from(value: DomainSeller) -> Self {
        Self {
            id: value.id,
            name: value.name,
            seller_type: value.seller_type,
            email: value.email,
            phone: value.phone,
            website_url: value.website_url,
            address: value.address,
            metadata: value.metadata,
        }
    }
}

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

#[cfg(test)]
mod update_seller_payload_conversion_tests {
    use super::*;
    use crate::core::infrastructure::error::CommandError;
    use crate::sellers::domain::seller_type::SellerType;

    fn valid_update() -> UpdateSellerPayload {
        UpdateSellerPayload {
            id: "trn:seller:test-shop".to_string(),
            name: "Test Shop".to_string(),
            seller_type: SellerType::Shop,
            email: Some("shop@example.com".to_string()),
            phone: Some("12345".to_string()),
            website_url: Some("https://example.com".to_string()),
            street_address: Some("Main St".to_string()),
            extended_address: Some("Building A".to_string()),
            city: Some("Turin".to_string()),
            state_region: Some("TO".to_string()),
            postal_code: Some("10100".to_string()),
            country_code: Some("IT".to_string()),
            created_at: None,
        }
    }

    #[test]
    fn update_payload_try_from_success_with_created_at_none() {
        let payload = valid_update();

        let converted = UpdateSellerInput::try_from(payload).expect("conversion should succeed");

        assert_eq!(converted.id.as_ref(), "trn:seller:test-shop");
        assert_eq!(converted.name, "Test Shop");
        assert_eq!(converted.seller_type, SellerType::Shop);
        assert_eq!(converted.email.as_deref(), Some("shop@example.com"));
        assert_eq!(converted.phone.as_deref(), Some("12345"));
        assert_eq!(
            converted.website_url.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(converted.street_address.as_deref(), Some("Main St"));
        assert_eq!(converted.extended_address.as_deref(), Some("Building A"));
        assert_eq!(converted.city.as_deref(), Some("Turin"));
        assert_eq!(converted.state_region.as_deref(), Some("TO"));
        assert_eq!(converted.postal_code.as_deref(), Some("10100"));
        assert_eq!(converted.country_code.as_deref(), Some("IT"));
        assert!(converted.created_at.is_none());
    }

    #[test]
    fn update_payload_try_from_success_with_valid_rfc3339_created_at() {
        let mut payload = valid_update();
        payload.created_at = Some("2024-05-28T14:30:00+00:00".to_string());

        let converted = UpdateSellerInput::try_from(payload).expect("conversion should succeed");

        assert_eq!(
            converted.created_at.map(|dt| dt.to_rfc3339()),
            Some("2024-05-28T14:30:00+00:00".to_string())
        );
    }

    #[test]
    fn update_payload_try_from_fails_on_invalid_id() {
        let mut payload = valid_update();
        payload.id = String::new();

        let converted = UpdateSellerInput::try_from(payload);

        match converted {
            Err(CommandError::ValidationError(map)) => {
                assert!(map.contains_key("id"), "{map:?}");
            }
            other => panic!("expected ValidationError for id, got {other:?}"),
        }
    }

    #[test]
    fn update_payload_try_from_fails_on_invalid_created_at() {
        let mut payload = valid_update();
        payload.created_at = Some("invalid-datetime".to_string());

        let converted = UpdateSellerInput::try_from(payload);

        match converted {
            Err(CommandError::ValidationError(map)) => {
                assert!(map.contains_key("createdAt"), "{map:?}");
            }
            other => panic!("expected ValidationError for createdAt, got {other:?}"),
        }
    }
}
