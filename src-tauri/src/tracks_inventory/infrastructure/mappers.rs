//! Pure conversion implementations for tracks inventory infrastructure.
//!
//! This module contains no async code and makes no database calls.
//! It provides [`From`] and [`TryFrom`] implementations to convert raw SQL
//! row structs (from [`super::entities`]) into domain types.

use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use crate::core::domain::metadata::Metadata;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::tracks_inventory::domain::views::{
    TrackInventoryItemView, TrackInventoryListItem, TrackInventoryView, TrackProductView,
    TrackPurchaseView,
};
use crate::tracks_inventory::domain::{
    TrackCode, TrackInventory, TrackProduct, TrackPurchase, TrackQuantity, TrackType,
};
use crate::tracks_inventory::infrastructure::entities::{
    TrackInventoryHeaderViewRow, TrackInventoryItemRow, TrackInventoryItemViewRow,
    TrackInventoryRow, TrackInventorySummaryRow, TrackProductFields, TrackProductRow,
    TrackProductViewRow, TrackPurchaseRow, TrackPurchaseViewRow,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/// Converts a millimetre integer value to a [`Length`].
///
/// Returns `None` if the integer cannot be represented as a `Decimal` or if
/// [`Length::try_new`] rejects the value.
pub fn mm_to_length(mm: i32) -> Option<Length> {
    Decimal::from_i32(mm).and_then(|d| Length::try_new(d, MeasureUnit::Millimeters).ok())
}

// ---------------------------------------------------------------------------
// From / TryFrom implementations
// ---------------------------------------------------------------------------

impl From<TrackProductFields> for TrackProductView {
    fn from(f: TrackProductFields) -> Self {
        TrackProductView {
            track_id: f.track_id,
            manufacturer_name: f.manufacturer_name,
            product_code: f.product_code,
            description: f.description.unwrap_or_default(),
            track_type: f
                .track_type
                .and_then(|t| t.parse::<TrackType>().ok())
                .unwrap_or(TrackType::Straight),
            track_code: f.track_code.unwrap_or(TrackCode::Code83),
            with_roadbed: f.with_roadbed == 1,
            length: f.length_mm.and_then(mm_to_length),
            radius: f.radius_mm.and_then(mm_to_length),
        }
    }
}

impl TryFrom<TrackProductRow> for TrackProduct {
    type Error = DomainError;

    fn try_from(row: TrackProductRow) -> Result<Self, Self::Error> {
        let manufacturer_id = ManufacturerId::try_from(row.manufacturer_id)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let length = row.length_mm.and_then(|mm| {
            Decimal::from_i64(mm as i64)
                .and_then(|d| Length::try_new(d, MeasureUnit::Millimeters).ok())
        });

        let radius = row.radius_mm.and_then(|mm| {
            Decimal::from_i64(mm as i64)
                .and_then(|d| Length::try_new(d, MeasureUnit::Millimeters).ok())
        });

        Ok(TrackProduct {
            track_id: row.track_id,
            product_code: row.product_code,
            manufacturer_id,
            description: row.description.unwrap_or_default(),
            with_roadbed: row.with_roadbed == 1,
            length,
            radius,
            track_code: row.track_code.unwrap_or(TrackCode::Code83),
            track_type: row
                .track_type
                .and_then(|t| t.parse::<TrackType>().ok())
                .unwrap_or(TrackType::Straight),
            metadata: Default::default(),
        })
    }
}

impl From<TrackProductViewRow> for TrackProductView {
    fn from(row: TrackProductViewRow) -> Self {
        TrackProductView::from(TrackProductFields {
            track_id: row.track_id,
            manufacturer_name: row.manufacturer_name,
            product_code: row.product_code,
            description: row.description,
            track_type: row.track_type,
            track_code: row.track_code,
            with_roadbed: row.with_roadbed,
            length_mm: row.length_mm,
            radius_mm: row.radius_mm,
        })
    }
}

impl TryFrom<TrackPurchaseRow> for TrackPurchase {
    type Error = DomainError;

    fn try_from(row: TrackPurchaseRow) -> Result<Self, Self::Error> {
        let currency = Currency::from_code(&row.price_currency)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        Ok(TrackPurchase {
            track_purchase_id: row.id,
            track_id: row.track_id,
            quantity: row.quantity,
            price: MonetaryAmount::new(row.price_amount, currency),
            seller_id: row.seller_id,
            purchase_date: row.purchase_date,
        })
    }
}

impl From<TrackPurchaseViewRow> for TrackPurchaseView {
    fn from(row: TrackPurchaseViewRow) -> Self {
        // Fallback to USD is defensive; invalid currency codes are prevented at write time.
        let currency = Currency::from_code(&row.price_currency).unwrap_or(Currency::USD);
        let track_product = TrackProductView::from(TrackProductFields {
            track_id: row.track_id.clone(), // Clone required: track_id is consumed by From<TrackProductFields> but must also be stored in TrackPurchaseView
            manufacturer_name: row.manufacturer_name,
            product_code: row.product_code,
            description: row.description,
            track_type: row.track_type,
            track_code: row.track_code,
            with_roadbed: row.with_roadbed,
            length_mm: row.length_mm,
            radius_mm: row.radius_mm,
        });
        TrackPurchaseView {
            id: row.id,
            track_product,
            quantity: row.quantity,
            price: MonetaryAmount::new(row.price_amount, currency),
            seller_name: row.seller_name,
            purchase_date: row.purchase_date,
        }
    }
}

impl From<TrackInventorySummaryRow> for TrackInventoryListItem {
    fn from(row: TrackInventorySummaryRow) -> Self {
        TrackInventoryListItem {
            id: row.id,
            name: row.name.unwrap_or_default(),
            description: row.description,
            total_items: row.total_items,
            total_quantity: row.total_quantity,
        }
    }
}

impl From<TrackInventoryItemViewRow> for TrackInventoryItemView {
    fn from(row: TrackInventoryItemViewRow) -> Self {
        let track_product = TrackProductView::from(TrackProductFields {
            track_id: row.track_id.clone(), // Clone required: track_id is consumed by From<TrackProductFields> but must also be stored in TrackInventoryItemView
            manufacturer_name: row.manufacturer_name,
            product_code: row.product_code,
            description: row.description,
            track_type: row.track_type,
            track_code: row.track_code,
            with_roadbed: row.with_roadbed,
            length_mm: row.length_mm,
            radius_mm: row.radius_mm,
        });
        TrackInventoryItemView {
            track_id: row.track_id,
            track_product,
            quantity: row.quantity,
            required: row.required,
        }
    }
}

// ---------------------------------------------------------------------------
// Aggregate assembly (multi-argument – not suitable for From/TryFrom)
// ---------------------------------------------------------------------------

/// Assembles a complete [`TrackInventory`] aggregate from its component rows.
///
/// # Errors
///
/// Returns [`DomainError`] if any purchase row contains an invalid currency
/// code or other un-parseable field.
pub fn assemble_track_inventory(
    header: TrackInventoryRow,
    item_rows: Vec<TrackInventoryItemRow>,
    purchase_rows: Vec<TrackPurchaseRow>,
) -> Result<TrackInventory, DomainError> {
    let mut inventory_map = HashMap::with_capacity(item_rows.len());
    for item in item_rows {
        inventory_map.insert(
            item.track_id.clone(), // Clone required: track_id is moved into the HashMap key, but must also be stored in the TrackQuantity value
            TrackQuantity {
                track_id: item.track_id,
                quantity: item.quantity,
            },
        );
    }

    let mut purchases = Vec::with_capacity(purchase_rows.len());
    for row in purchase_rows {
        purchases.push(TrackPurchase::try_from(row)?);
    }

    let created_at: DateTime<Utc> = header.created_at;
    let updated_at: DateTime<Utc> = header.updated_at;
    let version_u8: u8 = if header.version < 0 {
        0
    } else if header.version > (u8::MAX as i64) {
        u8::MAX
    } else {
        header.version as u8
    };

    let metadata = Metadata {
        version: version_u8,
        created_at,
        updated_at,
    };

    Ok(TrackInventory {
        id: header.id,
        inventory: inventory_map,
        purchase_history: purchases,
        metadata,
        name: header.name.unwrap_or_default(),
        description: header.description,
        pending_events: Vec::new(),
    })
}

/// Assembles a [`TrackInventoryView`] read-model from its component rows.
pub fn assemble_inventory_view(
    header: TrackInventoryHeaderViewRow,
    item_rows: Vec<TrackInventoryItemViewRow>,
    purchase_rows: Vec<TrackPurchaseViewRow>,
) -> TrackInventoryView {
    let items: Vec<TrackInventoryItemView> = item_rows
        .into_iter()
        .map(TrackInventoryItemView::from)
        .collect();
    let purchases: Vec<TrackPurchaseView> = purchase_rows
        .into_iter()
        .map(TrackPurchaseView::from)
        .collect();

    TrackInventoryView {
        id: header.id,
        name: header.name.unwrap_or_default(),
        description: header.description,
        items,
        purchases,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackInventoryId, TrackPurchaseId};
    use crate::tracks_inventory::infrastructure::entities::{
        TrackInventoryItemRow, TrackInventoryRow, TrackProductFields, TrackPurchaseRow,
    };
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    fn make_header() -> TrackInventoryRow {
        TrackInventoryRow {
            id: TrackInventoryId::try_from(
                "trn:track-inventory:00000000-0000-0000-0000-000000000001",
            )
            .unwrap(),
            name: Some("Test".to_string()),
            description: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 0,
        }
    }

    #[test]
    fn map_inventory_summary_uses_default_name_when_none() {
        let row = TrackInventorySummaryRow {
            id: TrackInventoryId::try_from(
                "trn:track-inventory:00000000-0000-0000-0000-000000000001",
            )
            .unwrap(),
            name: None,
            description: None,
            total_items: 3,
            total_quantity: 10,
        };
        let item = TrackInventoryListItem::from(row);
        assert_eq!(item.name, "");
        assert_eq!(item.total_items, 3);
        assert_eq!(item.total_quantity, 10);
    }

    #[test]
    fn assemble_track_inventory_empty_rows() {
        let header = make_header();
        let result = assemble_track_inventory(header, vec![], vec![]);
        assert!(result.is_ok());
        let inv = result.unwrap();
        assert!(inv.inventory.is_empty());
        assert!(inv.purchase_history.is_empty());
    }

    #[test]
    fn assemble_track_inventory_invalid_currency_returns_error() {
        let header = make_header();
        let bad_purchase = TrackPurchaseRow {
            id: TrackPurchaseId::try_from(
                "trn:track-purchase:00000000-0000-0000-0000-000000000001",
            )
            .unwrap(),
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 1,
            price_amount: 100,
            price_currency: "XYZ_BAD".to_string(),
            seller_id: None,
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };
        let result = assemble_track_inventory(header, vec![], vec![bad_purchase]);
        assert!(result.is_err());
    }

    #[test]
    fn map_track_product_view_defaults() {
        let fields = TrackProductFields {
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            manufacturer_name: "ACME".to_string(),
            product_code: "60100".to_string(),
            description: None,
            track_type: None,
            track_code: None,
            with_roadbed: 0,
            length_mm: None,
            radius_mm: None,
        };
        let view = TrackProductView::from(fields);
        assert_eq!(view.description, "");
        assert_eq!(view.track_code, TrackCode::Code83);
        assert_eq!(view.track_type, TrackType::Straight);
        assert!(!view.with_roadbed);
    }

    #[test]
    fn assemble_track_inventory_populates_inventory_map() {
        let header = make_header();
        let item = TrackInventoryItemRow {
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 5,
            required: 0,
        };
        let inv = assemble_track_inventory(header, vec![item], vec![]).unwrap();
        let qty = inv
            .inventory
            .get(&TrackId::try_from("trn:track:acme:60100").unwrap())
            .map(|q| q.quantity);
        assert_eq!(qty, Some(5));
    }
}
