use crate::core::domain::address::Address;
use crate::core::domain::metadata::Metadata;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use serde::{Deserialize, Serialize};

/// Domain events produced by the `Seller` aggregate.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SellerEvent {
    Created {
        aggregate_id: SellerId,
        name: String,
        seller_type: SellerType,
        email: Option<String>,
        phone: Option<String>,
        website_url: Option<String>,
        address: Option<Address>,
        metadata: Metadata,
    },
    Updated {
        aggregate_id: SellerId,
        name: String,
        seller_type: SellerType,
        email: Option<String>,
        phone: Option<String>,
        website_url: Option<String>,
        address: Option<Address>,
        metadata: Metadata,
    },
    Deleted {
        aggregate_id: SellerId,
    },
}
