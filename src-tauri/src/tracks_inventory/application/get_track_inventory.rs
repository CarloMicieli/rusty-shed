//! Query to fetch a single track inventory with full details.

use crate::core::domain::currency::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::application::views::{
    TrackInventoryItemView, TrackInventoryView, TrackProductView, TrackPurchaseView,
};
use crate::tracks_inventory::domain::{
    TrackCode, TrackId, TrackInventoryId, TrackPurchaseId, TrackType,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

/// Query to fetch a single track inventory with items and purchases.
pub struct GetTrackInventoryQuery;

impl GetTrackInventoryQuery {
    /// Execute the query to get a specific track inventory.
    ///
    /// # Arguments
    /// - `unit_of_work`: Transactional unit providing repository access.
    /// - `id`: The inventory ID to fetch.
    ///
    /// # Returns
    /// * `TrackInventoryView` - Complete inventory view with items and purchases.
    /// * `DomainError::NotFound` - If inventory doesn't exist.
    /// * `DomainError` - On other errors.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork,
        id: &TrackInventoryId,
    ) -> Result<TrackInventoryView, DomainError> {
        // Fetch inventory header
        let header_sql = r#"
            SELECT id, name, description
            FROM track_inventories
            WHERE id = ?1
        "#;

        let header: Option<InventoryHeaderRow> = sqlx::query_as(header_sql)
            .bind(id)
            .fetch_optional(&mut *unit_of_work.tx)
            .await
            .map_err(DomainError::from)?;

        let header = header.ok_or_else(|| DomainError::NotFound {
            resource: "TrackInventory".to_string(),
            identifier: id.to_string(),
        })?;

        // Fetch inventory items with product details
        let items_sql = r#"
            SELECT 
                tii.track_id,
                tii.quantity,
                tii.required,
                tp.product_code,
                tp.description,
                tp.track_type,
                tp.track_code,
                tp.with_roadbed,
                tp.length_mm,
                tp.radius_mm,
                m.name as manufacturer_name
            FROM track_inventory_items tii
            INNER JOIN track_products tp ON tii.track_id = tp.track_id
            INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
            WHERE tii.inventory_id = ?1
            ORDER BY tp.product_code
        "#;

        let item_rows: Vec<InventoryItemRow> = sqlx::query_as(items_sql)
            .bind(id)
            .fetch_all(&mut *unit_of_work.tx)
            .await
            .map_err(DomainError::from)?;

        let items: Vec<TrackInventoryItemView> = item_rows
            .into_iter()
            .map(|row| TrackInventoryItemView {
                track_id: row.track_id.clone(),
                track_product: TrackProductView {
                    track_id: row.track_id,
                    manufacturer_name: row.manufacturer_name,
                    product_code: row.product_code,
                    description: row.description.unwrap_or_default(),
                    track_type: row
                        .track_type
                        .and_then(|t| t.parse::<TrackType>().ok())
                        .unwrap_or(TrackType::Straight),
                    track_code: row.track_code.unwrap_or(TrackCode::Code83),
                    with_roadbed: row.with_roadbed == 1,
                    length: row.length_mm.and_then(|mm| {
                        Length::try_new(Decimal::from_i32(mm).unwrap(), MeasureUnit::Millimeters)
                            .ok()
                    }),
                    radius: row.radius_mm.and_then(|mm| {
                        Length::try_new(Decimal::from_i32(mm).unwrap(), MeasureUnit::Millimeters)
                            .ok()
                    }),
                },
                quantity: row.quantity,
                required: row.required,
            })
            .collect();

        // Fetch purchases with product details
        let purchases_sql = r#"
            SELECT 
                tp_hist.id,
                tp_hist.track_id,
                tp_hist.quantity,
                tp_hist.price_amount,
                tp_hist.price_currency,
                tp_hist.purchase_date,
                s.name as seller_name,
                tp.product_code,
                tp.description,
                tp.track_type,
                tp.track_code,
                tp.with_roadbed,
                tp.length_mm,
                tp.radius_mm,
                m.name as manufacturer_name
            FROM track_purchases tp_hist
            INNER JOIN track_products tp ON tp_hist.track_id = tp.track_id
            INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
            LEFT JOIN sellers s ON tp_hist.seller_id = s.id
            WHERE tp_hist.inventory_id = ?1
            ORDER BY tp_hist.purchase_date DESC
        "#;

        let purchase_rows: Vec<PurchaseRow> = sqlx::query_as(purchases_sql)
            .bind(id)
            .fetch_all(&mut *unit_of_work.tx)
            .await
            .map_err(DomainError::from)?;

        let purchases: Vec<TrackPurchaseView> = purchase_rows
            .into_iter()
            .map(|row| {
                let currency = Currency::from_code(&row.price_currency).unwrap_or(Currency::USD);
                TrackPurchaseView {
                    id: row.id,
                    track_product: TrackProductView {
                        track_id: row.track_id.clone(),
                        manufacturer_name: row.manufacturer_name,
                        product_code: row.product_code,
                        description: row.description.unwrap_or_default(),
                        track_type: row
                            .track_type
                            .and_then(|t| t.parse::<TrackType>().ok())
                            .unwrap_or(TrackType::Straight),
                        track_code: row.track_code.unwrap_or(TrackCode::Code83),
                        with_roadbed: row.with_roadbed == 1,
                        length: row.length_mm.and_then(|mm| {
                            Length::try_new(
                                Decimal::from_i32(mm).unwrap(),
                                MeasureUnit::Millimeters,
                            )
                            .ok()
                        }),
                        radius: row.radius_mm.and_then(|mm| {
                            Length::try_new(
                                Decimal::from_i32(mm).unwrap(),
                                MeasureUnit::Millimeters,
                            )
                            .ok()
                        }),
                    },
                    quantity: row.quantity,
                    price: MonetaryAmount::new(row.price_amount, currency),
                    seller_name: row.seller_name,
                    purchase_date: row.purchase_date,
                }
            })
            .collect();

        Ok(TrackInventoryView {
            id: header.id,
            name: header.name.unwrap_or_default(),
            description: header.description,
            items,
            purchases,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InventoryHeaderRow {
    id: TrackInventoryId,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct InventoryItemRow {
    track_id: TrackId,
    quantity: i64,
    required: i64,
    product_code: String,
    description: Option<String>,
    track_type: Option<String>,
    track_code: Option<TrackCode>,
    with_roadbed: i64,
    length_mm: Option<i32>,
    radius_mm: Option<i32>,
    manufacturer_name: String,
}

#[derive(Debug, sqlx::FromRow)]
struct PurchaseRow {
    id: TrackPurchaseId,
    track_id: TrackId,
    quantity: i64,
    price_amount: i64,
    price_currency: String,
    purchase_date: NaiveDate,
    seller_name: Option<String>,
    product_code: String,
    description: Option<String>,
    track_type: Option<String>,
    track_code: Option<TrackCode>,
    with_roadbed: i64,
    length_mm: Option<i32>,
    radius_mm: Option<i32>,
    manufacturer_name: String,
}
