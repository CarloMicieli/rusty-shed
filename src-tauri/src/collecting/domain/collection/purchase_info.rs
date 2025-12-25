use crate::core::domain::MonetaryAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Purchase information associated with a `CollectionItem`.
///
/// This structure stores details about the acquisition of a collection item,
/// including an identifier, purchase date, price and seller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseInfo {
    /// Unique identifier for the purchase record (e.g. UUID).
    pub id: String,

    /// The collection item this purchase info refers to.
    pub item_id: String,

    /// The purchase date (stored as a `NaiveDate`).
    ///
    /// Parsed from the database string (ISO 8601 `YYYY-MM-DD` expected).
    pub purchase_date: NaiveDate,

    /// Price represented as an optional `MonetaryAmount`. `None` means the currency
    /// was not present in the DB (NULL) or price is unknown.
    pub price: Option<MonetaryAmount>,

    /// Seller name or vendor identifier.
    pub seller: String,
}
