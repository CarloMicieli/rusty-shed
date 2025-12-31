use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Seller {
    pub id: SellerId,
    pub name: String,
    pub r#type: SellerType,
    pub url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
