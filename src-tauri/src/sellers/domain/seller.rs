use crate::core::domain::address::Address;
use crate::core::domain::metadata::Metadata;
use crate::sellers::domain::seller_event::SellerEvent;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use serde::{Deserialize, Serialize};

/// Represents a seller (a shop, private seller or distributor) in the system.
///
/// This domain-level struct is used by application use-cases and persisted via
/// the sellers repository.
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

    /// Metadata about the seller (creation date, last modified, version, etc.).
    pub metadata: Metadata,

    /// Events produced by operations on the aggregate that have not yet been
    /// persisted/handled by a repository or unit of work.
    pub pending_events: Vec<SellerEvent>,
}

impl Seller {
    /// Pull and clear pending events for persistence.
    pub fn pull_events(&mut self) -> Vec<SellerEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
