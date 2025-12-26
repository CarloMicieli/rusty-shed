use crate::core::domain::MonetaryAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Purchase information associated with a `CollectionItem`.
///
/// A collection item may have acquisition-related metadata that describes how
/// the user obtained (or intends to obtain) the item. This enum models the
/// three supported acquisition states:
///
/// - `Purchased`: the item was purchased and its value is counted in the
///   collection total (unless explicitly excluded elsewhere).
/// - `Sold`: the item was sold; we keep a record of the original purchase
///   and the sale, but the item is excluded from the collection's monetary
///   total value.
/// - `PreOrdered`: the item is pre-ordered from a seller; the collector may
///   have paid a deposit and the full total price is known as well.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", rename_all = "lowercase")]
#[specta(tag = "kind", content = "data")]
pub enum PurchaseInfo {
    /// A standard purchase record.
    Purchased(PurchasedInfo),

    /// A sold item record that preserves original purchase data and the
    /// subsequent sale information.
    Sold(SoldInfo),

    /// A preorder record with deposit and total price information.
    PreOrdered(PreOrderInfo),
}

impl PurchaseInfo {
    /// Return the canonical identifier for this purchase record.
    ///
    /// Always present for all variants.
    pub fn id(&self) -> &str {
        match self {
            PurchaseInfo::Purchased(p) => &p.id,
            PurchaseInfo::Sold(s) => &s.id,
            PurchaseInfo::PreOrdered(po) => &po.id,
        }
    }

    /// Return an optional seller identifier or name for this purchase record.
    ///
    /// Returns `Some(&str)` when a seller is present, otherwise `None`.
    pub fn seller(&self) -> Option<&str> {
        match self {
            PurchaseInfo::Purchased(p) => p.seller.as_deref(),
            PurchaseInfo::Sold(s) => s.seller.as_deref(),
            PurchaseInfo::PreOrdered(po) => po.seller.as_deref(),
        }
    }
}

/// Details for a purchased item.
///
/// This struct holds the canonical purchase identifier and optional price
/// information. `price` is optional to support legacy records where the
/// monetary amount or currency was not stored.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PurchasedInfo {
    /// Unique identifier for this purchase record (for example a UUID).
    pub id: String,

    /// Date when the item was purchased (ISO `YYYY-MM-DD`).
    pub purchase_date: NaiveDate,

    /// The price paid when purchasing the item, if known.
    ///
    /// Represented as a domain `MonetaryAmount` (amount in smallest unit + currency).
    pub price: Option<MonetaryAmount>,

    /// Optional seller identifier or human-friendly name.
    pub seller: Option<String>,
}

/// Details for an item that was sold.
///
/// We keep both the original purchase information and the sale data so the
/// application can show provenance (what was paid originally) together with
/// the sale outcome. The item should be excluded from collection value
/// aggregations once sold.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct SoldInfo {
    /// Unique identifier for the original purchase record (or the sale record,
    /// depending on how you model identifiers). This is the canonical id for
    /// the persisted purchase_info row.
    pub id: String,

    /// Date when the item was originally purchased (ISO `YYYY-MM-DD`).
    pub purchase_date: NaiveDate,

    /// Original purchase price, if available.
    ///
    /// Use `None` to indicate the original price is unknown or not stored.
    pub purchase_price: Option<MonetaryAmount>,

    /// Date when the item was sold (ISO `YYYY-MM-DD`).
    pub sale_date: NaiveDate,

    /// Price obtained when the item was sold. This value is required for
    /// `Sold` records because a sale without a price is not meaningful for
    /// financial reporting.
    pub sale_price: MonetaryAmount,

    /// Optional buyer identifier (when the buyer is a tracked entity).
    pub buyer: Option<String>,

    /// Optional seller identifier for completeness (may be the shop that
    /// originally sold the item or the intermediary that handled the sale).
    pub seller: Option<String>,
}

/// Details for a pre-order entry.
///
/// Preorders record at least the deposit paid and the total price expected
/// for the item. Both monetary amounts must use the same currency; use the
/// `validate_currencies_match` helper to assert that condition.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PreOrderInfo {
    /// Unique identifier for this preorder record.
    pub id: String,

    /// Date when the preorder was placed (ISO `YYYY-MM-DD`).
    pub order_date: NaiveDate,

    /// Amount paid as deposit (in smallest unit + currency).
    pub deposit: MonetaryAmount,

    /// Total price for the pre-ordered item (in smallest unit + currency).
    pub total_price: MonetaryAmount,

    /// Optional seller identifier or shop name.
    pub seller: Option<String>,

    /// Optional expected delivery date (ETA) for the preorder.
    pub expected_date: Option<NaiveDate>,
}

impl PreOrderInfo {
    /// Validate that the preorder `deposit` and `total_price` share the same
    /// currency.
    ///
    /// Returns `Ok(())` when currencies match, otherwise returns
    /// `crate::core::domain::error::Error::CurrencyMismatch`.
    pub fn validate_currencies_match(&self) -> Result<(), crate::core::domain::error::Error> {
        if self.deposit.currency != self.total_price.currency {
            return Err(crate::core::domain::error::Error::CurrencyMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::MonetaryAmount;
    use crate::core::domain::currency::Currency;
    use chrono::NaiveDate;

    #[test]
    fn purchased_id_and_seller_accessor() {
        let p = PurchasedInfo {
            id: "p1".to_string(),
            purchase_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            price: Some(MonetaryAmount::new(1500, Currency::EUR)),
            seller: Some("shop-1".to_string()),
        };
        let pi = PurchaseInfo::Purchased(p.clone());
        assert_eq!(pi.id(), "p1");
        assert_eq!(pi.seller(), Some("shop-1"));
    }

    #[test]
    fn sold_id_and_seller_accessor() {
        let s = SoldInfo {
            id: "s1".to_string(),
            purchase_date: NaiveDate::from_ymd_opt(2020, 5, 10).unwrap(),
            purchase_price: Some(MonetaryAmount::new(2000, Currency::USD)),
            sale_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            sale_price: MonetaryAmount::new(2500, Currency::USD),
            buyer: Some("buyer-1".to_string()),
            seller: Some("seller-shop".to_string()),
        };
        let pi = PurchaseInfo::Sold(s.clone());
        assert_eq!(pi.id(), "s1");
        assert_eq!(pi.seller(), Some("seller-shop"));
    }

    #[test]
    fn preorder_seller_none_and_validate_currency_mismatch() {
        let preorder = PreOrderInfo {
            id: "pre1".to_string(),
            order_date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            deposit: MonetaryAmount::new(500, Currency::EUR),
            total_price: MonetaryAmount::new(1000, Currency::USD), // mismatched currency
            seller: None,
            expected_date: None,
        };

        let pi = PurchaseInfo::PreOrdered(preorder.clone());
        // seller is None
        assert_eq!(pi.seller(), None);
        // validate currencies should fail due to mismatch
        assert!(preorder.validate_currencies_match().is_err());
    }
}
