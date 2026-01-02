use crate::core::domain::address::Address;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a seller (a shop, private seller or distributor) in the system.
///
/// This domain-level struct is used by application use-cases and persisted via
/// the sellers repository. Timestamps are represented using `chrono::DateTime<Utc>`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Seller {
    /// Strongly-typed seller identifier (format: `trn:seller:{slug}`).
    pub id: SellerId,

    /// Human-readable seller name.
    pub name: String,

    /// The category/type of the seller (Shop, Private, Marketplace, ...).
    pub seller_type: SellerType,

    /// Optional contact email.
    pub email: Option<String>,

    /// Optional contact phone number.
    pub phone: Option<String>,

    /// Optional website URL (if available).
    pub website_url: Option<String>,

    /// Optional postal address structured as an `Address` domain value.
    ///
    /// When present this contains street, (extended) house number, city, region,
    /// postal code and country as a single value object.
    pub address: Option<Address>,

    /// Creation timestamp (UTC).
    pub created_at: DateTime<Utc>,

    /// Last update timestamp (UTC).
    pub updated_at: DateTime<Utc>,
}
