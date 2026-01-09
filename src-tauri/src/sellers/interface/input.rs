use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::update_seller::UpdateSellerInput;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateSellerPayload {
    pub name: String,
    pub seller_type: SellerType,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSellerPayload {
    pub id: String,
    pub name: String,
    pub seller_type: SellerType,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
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
