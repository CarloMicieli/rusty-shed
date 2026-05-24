use crate::sellers::domain::seller_id::SellerId;
use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackInventoryId, TrackPurchaseId};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;

/// Row representation for a track product.
///
/// This struct mirrors the columns selected for a product/track metadata
/// query and is used to map SQL results into domain-friendly types.
#[derive(Debug, Clone, FromRow)]
pub struct TrackProductRow {
    /// Internal track identifier.
    pub track_id: TrackId,
    /// Manufacturer product code (string identifier supplied by manufacturer).
    pub product_code: String,
    /// Manufacturer identifier.
    pub manufacturer_id: String,
    /// Stored as `i64` in the DB: non-zero indicates presence of a roadbed.
    pub with_roadbed: i64,
    /// Optional length in millimetres.
    pub length_mm: Option<i32>,
    /// Optional radius in millimetres.
    pub radius_mm: Option<i32>,
    /// Optional domain `TrackCode` (parsed/validated at the domain layer).
    pub track_code: Option<TrackCode>,
    /// Optional track type (e.g. "STRAIGHT", "CURVE").
    pub track_type: Option<String>,
}

/// Row representation for inventory items.
///
/// Connects a `TrackId` with the available `quantity` in an inventory.
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventoryItemRow {
    /// Internal track identifier.
    pub track_id: TrackId,
    /// Quantity available for the given track.
    pub quantity: i64,
    /// Required quantity for planning (target stock level).
    /// This field is only used for read-side queries (views), not for aggregate reconstruction.
    #[allow(dead_code)]
    pub required: i64,
}

/// Row representation for track purchases.
///
/// Represents a single purchase record for a track, including price and
/// optional seller information.
#[derive(Debug, Clone, FromRow)]
pub struct TrackPurchaseRow {
    /// Purchase identifier.
    pub id: TrackPurchaseId,
    /// Purchased track identifier.
    pub track_id: TrackId,
    /// Purchased quantity.
    pub quantity: i64,
    /// Price amount stored as smallest currency unit (e.g. cents).
    pub price_amount: i64,
    /// Currency code for the price (e.g. "USD").
    pub price_currency: String,
    /// Optional seller that supplied the purchase.
    pub seller_id: Option<SellerId>,
    /// Date of purchase (no timezone information required).
    pub purchase_date: NaiveDate,
}

/// Row representation for a track inventory header.
///
/// Contains metadata for an inventory (name, description and timestamps).
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventoryRow {
    /// Inventory identifier.
    pub id: TrackInventoryId,
    /// Optional name for the inventory.
    pub name: Option<String>,
    /// Optional description for the inventory.
    pub description: Option<String>,
    /// Timestamp when the inventory was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the inventory was last updated.
    pub updated_at: DateTime<Utc>,
    /// Version number for optimistic concurrency control.
    pub version: i64,
}

/// Row for the inventory list summary query (COUNT + SUM aggregates).
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventorySummaryRow {
    /// Inventory identifier.
    pub id: TrackInventoryId,
    /// Optional inventory name.
    pub name: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Number of distinct track types in the inventory.
    pub total_items: i64,
    /// Sum of all quantities across track types.
    pub total_quantity: i64,
}

/// Lightweight header row used when fetching a single inventory view.
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventoryHeaderViewRow {
    /// Inventory identifier.
    pub id: TrackInventoryId,
    /// Optional inventory name.
    pub name: Option<String>,
    /// Optional description.
    pub description: Option<String>,
}

/// Row for the per-item detail inside an inventory view (joined with product + manufacturer).
#[derive(Debug, Clone, FromRow)]
pub struct TrackInventoryItemViewRow {
    /// Track product identifier.
    pub track_id: TrackId,
    /// Current stock quantity.
    pub quantity: i64,
    /// Required (target) quantity.
    pub required: i64,
    /// Manufacturer product code.
    pub product_code: String,
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Optional track type string.
    pub track_type: Option<String>,
    /// Optional rail profile code.
    pub track_code: Option<TrackCode>,
    /// 1 if the piece has an integrated roadbed, 0 otherwise.
    pub with_roadbed: i64,
    /// Optional length in millimetres.
    pub length_mm: Option<i32>,
    /// Optional radius in millimetres.
    pub radius_mm: Option<i32>,
    /// Manufacturer name (denormalised from `manufacturers` table).
    pub manufacturer_name: String,
}

/// Row for the purchase history inside an inventory view (joined with product, manufacturer, seller).
#[derive(Debug, Clone, FromRow)]
pub struct TrackPurchaseViewRow {
    /// Purchase identifier.
    pub id: TrackPurchaseId,
    /// Purchased track identifier.
    pub track_id: TrackId,
    /// Purchased quantity.
    pub quantity: i64,
    /// Price amount in smallest currency unit.
    pub price_amount: i64,
    /// Currency code (e.g. "EUR").
    pub price_currency: String,
    /// Date of purchase.
    pub purchase_date: NaiveDate,
    /// Optional seller name (denormalised).
    pub seller_name: Option<String>,
    /// Manufacturer product code.
    pub product_code: String,
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Optional track type string.
    pub track_type: Option<String>,
    /// Optional rail profile code.
    pub track_code: Option<TrackCode>,
    /// 1 if the piece has an integrated roadbed, 0 otherwise.
    pub with_roadbed: i64,
    /// Optional length in millimetres.
    pub length_mm: Option<i32>,
    /// Optional radius in millimetres.
    pub radius_mm: Option<i32>,
    /// Manufacturer name (denormalised from `manufacturers` table).
    pub manufacturer_name: String,
}

/// Row for the all-products view query (joined with manufacturer name).
#[derive(Debug, Clone, FromRow)]
pub struct TrackProductViewRow {
    /// Track product identifier.
    pub track_id: TrackId,
    /// Manufacturer product code.
    pub product_code: String,
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Optional track type string.
    pub track_type: Option<String>,
    /// Optional rail profile code.
    pub track_code: Option<TrackCode>,
    /// 1 if the piece has an integrated roadbed, 0 otherwise.
    pub with_roadbed: i64,
    /// Optional length in millimetres.
    pub length_mm: Option<i32>,
    /// Optional radius in millimetres.
    pub radius_mm: Option<i32>,
    /// Manufacturer name (denormalised from `manufacturers` table).
    pub manufacturer_name: String,
}

/// Bundles the common product columns shared by item-view and purchase-view rows.
///
/// Used as an intermediate type by `From` implementations in the `mappers` module.
pub struct TrackProductFields {
    /// Canonical track identifier.
    pub track_id: TrackId,
    /// Human-readable manufacturer name (denormalised).
    pub manufacturer_name: String,
    /// Manufacturer product code.
    pub product_code: String,
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Optional track type string (parsed to [`TrackType`]).
    pub track_type: Option<String>,
    /// Optional rail profile code (parsed to [`TrackCode`]).
    pub track_code: Option<TrackCode>,
    /// Non-zero indicates integrated roadbed.
    pub with_roadbed: i64,
    /// Optional length in millimetres.
    pub length_mm: Option<i32>,
    /// Optional radius in millimetres.
    pub radius_mm: Option<i32>,
}
