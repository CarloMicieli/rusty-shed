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

    /// Price amount in the smallest currency unit (e.g. cents).
    pub price_amount: i64, // in cents

    /// ISO currency code for the price (e.g. "EUR", "USD").
    pub price_currency: String,

    /// Seller name or vendor identifier.
    pub seller: String,
}
