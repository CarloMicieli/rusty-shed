use crate::core::domain::address::Address;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use serde::{Deserialize, Serialize};

/// Presentation model for a `Seller` aggregate returned to the client.
///
/// This view mirrors the persisted `Seller` fields but intentionally omits
/// domain-only fields such as `pending_events`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SellerView {
    /// Unique identifier for the seller.
    pub id: SellerId,
    /// Name of the seller.
    pub name: String,
    /// Type of the seller.
    pub seller_type: SellerType,
    /// Contact email of the seller.
    pub email: Option<String>,
    /// Contact phone number of the seller.
    pub phone: Option<String>,
    /// Website URL of the seller.
    pub website_url: Option<String>,
    /// Address of the seller.
    pub address: Option<Address>,
    /// Whether this row is system-seeded and protected.
    pub is_system_seeded: bool,
    /// Total usage count across buyer and seller references.
    pub usage_count: i64,
}

impl From<Seller> for SellerView {
    fn from(s: Seller) -> Self {
        SellerView {
            id: s.id,
            name: s.name,
            seller_type: s.seller_type,
            email: s.email,
            phone: s.phone,
            website_url: s.website_url,
            address: s.address,
            is_system_seeded: false,
            usage_count: 0,
        }
    }
}
