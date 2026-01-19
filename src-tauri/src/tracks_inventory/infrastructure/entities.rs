use crate::sellers::domain::seller_id::SellerId;
use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackInventoryId, TrackPurchaseId};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;

/// Row representation for a track product
#[derive(Debug, Clone, FromRow)]
pub struct TrackProductRow {
    pub track_id: TrackId,
    pub product_code: String,
    pub manufacturer_id: String,
    pub with_roadbed: i64,
    pub length_mm: Option<i32>,
    pub radius_mm: Option<i32>,
    pub track_code: Option<TrackCode>,
    pub description: Option<String>,
}

/// Row representation for inventory items
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventoryItemRow {
    pub track_id: TrackId,
    pub quantity: i64,
}

/// Row representation for purchases
#[derive(Debug, Clone, FromRow)]
pub struct TrackPurchaseRow {
    pub id: TrackPurchaseId,
    pub track_id: TrackId,
    pub quantity: i64,
    pub price_amount: i64,
    pub price_currency: String,
    pub seller_id: Option<SellerId>,
    pub purchase_date: NaiveDate,
}

/// Row representation for a track inventory header
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventoryRow {
    pub id: TrackInventoryId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}
