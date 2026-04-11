use crate::sellers::domain::seller_type::SellerType;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SellerRow {
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
    pub created_at: String,
    pub updated_at: String,
}
