use crate::core::domain::address::{Address, AddressFields};
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use chrono::{DateTime, Utc};

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

impl From<SellerRow> for Seller {
    fn from(row: SellerRow) -> Self {
        let address_fields = AddressFields {
            street: row.street_address,
            extended: row.extended_address,
            city: row.city,
            region: row.state_region,
            postal: row.postal_code,
            country: row.country_code,
        };
        let address = Address::try_from(address_fields).ok();

        // Parse timestamps from DB (stored as RFC3339 strings)
        let created_at_dt = DateTime::parse_from_rfc3339(&row.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let updated_at_dt = DateTime::parse_from_rfc3339(&row.updated_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Seller {
            id: SellerId(row.id),
            name: row.name,
            seller_type: row.seller_type,
            email: row.email,
            phone: row.phone,
            website_url: row.website_url,
            address,
            created_at: created_at_dt,
            updated_at: updated_at_dt,
            pending_events: Vec::new(),
        }
    }
}
