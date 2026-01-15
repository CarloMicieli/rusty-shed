use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::MonetaryAmount;
use crate::core::domain::metadata::Metadata;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use crate::wishlist::infrastructure::entities::{WishlistItemRow, WishlistRow};
use anyhow::Context;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;

impl TryFrom<WishlistRow> for Wishlist {
    type Error = anyhow::Error;

    fn try_from(row: WishlistRow) -> Result<Self, Self::Error> {
        let id = WishlistId::try_from(row.id.as_str()).context("invalid wishlist id")?;

        // Map DB NaiveDateTime into DateTime<Utc> for Metadata
        let created_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(row.created_at, Utc);
        let updated_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(row.updated_at, Utc);

        Ok(Wishlist {
            id,
            name: row.name,
            notes: row.notes,
            is_default: row.is_default != 0,
            metadata: Metadata {
                version: row.version as u8,
                created_at,
                updated_at,
            },
            items: vec![],
        })
    }
}

impl TryFrom<WishlistItemRow> for WishlistItem {
    type Error = anyhow::Error;

    fn try_from(row: WishlistItemRow) -> Result<Self, Self::Error> {
        let priority = WishlistPriority::from_str(&row.priority)
            .with_context(|| format!("Invalid priority: {}", row.priority))?;

        let status = WishlistStatus::from_str(&row.status)
            .with_context(|| format!("Invalid status: {}", row.status))?;

        let desired_price =
            map_monetary_amount(row.desired_price_amount, row.desired_price_currency)?;

        let purchased_price =
            map_monetary_amount(row.purchased_price_amount, row.purchased_price_currency)?;

        let railway_model_id = RailwayModelId::try_from(row.railway_model_id)?;

        let id = WishlistItemId::try_from(row.id.as_str()).context("invalid wishlist item id")?;

        Ok(WishlistItem {
            id,
            priority,
            status,
            added_date: row.added_date,
            removed_date: row.removed_date,
            notes: row.notes,
            desired_price,
            purchased_price,
            railway_model_id,
        })
    }
}

/// Helper to handle the common pattern of dual-optional price fields
fn map_monetary_amount(
    amount: Option<i64>,
    currency: Option<String>,
) -> anyhow::Result<Option<MonetaryAmount>> {
    match (amount, currency) {
        (Some(a), Some(c)) => MonetaryAmount::from_db(a, Some(&c))
            .map_err(|e| anyhow::anyhow!("Price mapping failed: {}", e)),
        // Return None if either is missing, or add a check if (Some, None) is an error state
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;
    use crate::wishlist::infrastructure::entities::WishlistItemRow;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[test]
    fn try_from_item_row_with_prices() {
        let row = WishlistItemRow {
            id: "trn:wishlist-item:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            wishlist_id: "trn:wishlist:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            railway_model_id: "trn:railway-model:acme:123456".to_string(),
            priority: "HIGH".to_string(),
            status: "WANTED".to_string(),
            desired_price_amount: Some(1234),
            desired_price_currency: Some("EUR".to_string()),
            added_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
            removed_date: None,
            notes: Some("note".to_string()),
            purchased_at: None,
            purchased_price_amount: Some(500),
            purchased_price_currency: Some("USD".to_string()),
        };

        let item = WishlistItem::try_from(row).expect("mapping should succeed");
        assert_eq!(
            item.id.to_string(),
            "trn:wishlist-item:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840"
        );
        assert_eq!(item.priority, WishlistPriority::High);
        assert_eq!(item.status, WishlistStatus::Wanted);
        assert!(item.desired_price.is_some());
        assert!(item.purchased_price.is_some());
        let dp = item.desired_price.unwrap();
        assert_eq!(dp.amount, 1234i64);
        assert_eq!(dp.currency, Currency::EUR);
    }

    #[test]
    fn try_from_item_row_missing_price_parts_results_none() {
        let row = WishlistItemRow {
            id: "trn:wishlist-item:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            wishlist_id: "trn:wishlist:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            railway_model_id: "trn:railway-model:mn-1:ACME-100".to_string(),
            priority: "NORMAL".to_string(),
            status: "ON_ORDER".to_string(),
            desired_price_amount: Some(0),
            desired_price_currency: None,
            added_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
            removed_date: None,
            notes: None,
            purchased_at: None,
            purchased_price_amount: None,
            purchased_price_currency: None,
        };

        let item = WishlistItem::try_from(row).expect("mapping should succeed");
        assert!(item.desired_price.is_none());
        assert!(item.purchased_price.is_none());
    }

    #[test]
    fn try_from_item_row_negative_amount_errors() {
        let row = WishlistItemRow {
            id: "trn:wishlist-item:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            wishlist_id: "trn:wishlist:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            railway_model_id: "trn:railway-model:mn-1:ACME-100".to_string(),
            priority: "LOW".to_string(),
            status: "WANTED".to_string(),
            desired_price_amount: Some(-100),
            desired_price_currency: Some("EUR".to_string()),
            added_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
            removed_date: None,
            notes: None,
            purchased_at: None,
            purchased_price_amount: None,
            purchased_price_currency: None,
        };

        let res = WishlistItem::try_from(row);
        assert!(res.is_err(), "negative amount should result in error");
    }

    #[test]
    fn try_from_item_row_invalid_priority_or_status_errors() {
        let mut row = WishlistItemRow {
            id: "trn:wishlist-item:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            wishlist_id: "trn:wishlist:f25e2ff1-0cfb-4fb6-a3b9-1e5c95e06840".to_string(),
            railway_model_id: "trn:railway-model:mn-1:ACME-100".to_string(),
            priority: "NORMAL".to_string(),
            status: "WANTED".to_string(),
            desired_price_amount: None,
            desired_price_currency: None,
            added_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
            removed_date: None,
            notes: None,
            purchased_at: None,
            purchased_price_amount: None,
            purchased_price_currency: None,
        };

        row.priority = "UNKNOWN".to_string();
        assert!(WishlistItem::try_from(row.clone()).is_err());

        row.priority = "NORMAL".to_string();
        row.status = "BAD".to_string();
        assert!(WishlistItem::try_from(row).is_err());
    }
}
