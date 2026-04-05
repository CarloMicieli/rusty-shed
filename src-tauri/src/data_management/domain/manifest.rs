use serde::{Deserialize, Serialize};
use specta::Type;

/// Manifest DTOs for deserialization from import packages.

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ManifestDto {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exported_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub data: DataContainerDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataContainerDto {
    #[serde(default)]
    pub manufacturers: Vec<ManufacturerRecord>,
    #[serde(rename = "railwayCompanies", default)]
    pub railway_companies: Vec<RailwayCompanyRecord>,
    #[serde(rename = "railwayModels", default)]
    pub railway_models: Vec<RailwayModelRecord>,
    #[serde(rename = "collectionItems", default)]
    pub collection_items: Vec<CollectionItemRecord>,
    #[serde(default)]
    pub sellers: Vec<SellerRecord>,
    #[serde(rename = "maintenanceCards", default)]
    pub maintenance_cards: Vec<MaintenanceCardRecord>,
    #[serde(rename = "trackProducts", default)]
    pub track_products: Vec<TrackProductRecord>,
    #[serde(rename = "trackInventories", default)]
    pub track_inventories: Vec<TrackInventoryRecord>,
    #[serde(default)]
    pub prototypes: Vec<PrototypeRecord>,
    #[serde(rename = "formationCategories", default)]
    pub formation_categories: Vec<FormationCategoryRecord>,
    #[serde(rename = "trainFormations", default)]
    pub train_formations: Vec<TrainFormationRecord>,
    #[serde(default)]
    pub wishlists: Vec<WishlistRecord>,
    #[serde(default)]
    pub decoders: Vec<DecoderRecord>,
    #[serde(rename = "digitalRollingStocks", default)]
    pub digital_rolling_stocks: Vec<DigitalRollingStockRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerRecord {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_company_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RailwayCompanyRecord {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_since: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_until: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct RailwayModelRecord {
    pub id: String,
    pub manufacturer_id: String,
    pub product_code: String,
    pub description: LocalizedTextRecord,
    pub scale: String,
    pub epoch: String,
    pub category: String,
    pub power_method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<LocalizedTextRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default)]
    pub rolling_stocks: Vec<RollingStockRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedTextRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub it: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RollingStockRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub railway_company_id: String,
    pub series_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub road_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub livery: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub electric_multiple_unit_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freight_car_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locomotive_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passenger_car_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub railcar_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_dummy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length_inches: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length_millimeters: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_minimum_radius_mm: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_coupling_socket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_coupling_close_couplers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_coupling_digital_shunting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_flywheel_fitted: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_body_shell: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_chassis: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_interior_lights: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_lights: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_sprung_buffers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dcc_interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CollectionItemRecord {
    pub id: String,
    pub railway_model_id: String,
    pub added_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub box_condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase: Option<PurchaseRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseRecord {
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<MoneyRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sale_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<MoneyRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_amount: Option<MoneyRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_delivery: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MoneyRecord {
    pub amount: u64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SellerRecord {
    pub id: String,
    pub name: String,
    pub seller_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AddressRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceCardRecord {
    pub id: String,
    pub collection_item_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_maintenance_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_maintenance_date: Option<String>,
    #[serde(default)]
    pub events: Vec<MaintenanceEventRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceEventRecord {
    pub id: String,
    pub date: String,
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<MoneyRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackProductRecord {
    pub track_id: String,
    pub manufacturer_id: String,
    pub product_code: String,
    pub description: String,
    pub track_type: String,
    pub track_code: String,
    pub with_roadbed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackInventoryRecord {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub items: Vec<TrackInventoryItemRecord>,
    #[serde(default)]
    pub purchases: Vec<TrackPurchaseRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackInventoryItemRecord {
    pub track_id: String,
    pub quantity: i64,
    #[serde(default)]
    pub required: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackPurchaseRecord {
    pub id: String,
    pub track_id: String,
    pub quantity: i64,
    pub price: MoneyRecord,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    pub purchase_date: String,
}

/// A rolling stock prototype from the train formations catalog.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrototypeRecord {
    pub id: String,
    pub railway_company_id: String,
    pub series_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Specification discriminator: `LOCOMOTIVE` | `PASSENGER_CAR` | `FREIGHT_CAR` |
    /// `RAILCAR` | `ELECTRIC_MULTIPLE_UNIT`
    pub specification_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locomotive_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locomotive_series: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passenger_car_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freight_car_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub railcar_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub electric_multiple_unit_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_permanently_coupled: Option<bool>,
    pub is_motorized: bool,
    pub is_custom: bool,
}

/// A user-defined or built-in train formation category.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct FormationCategoryRecord {
    pub id: String,
    pub name: String,
    pub is_custom: bool,
}

/// A named train formation with an ordered list of elements.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrainFormationRecord {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_year: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_year: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub elements: Vec<FormationElementRecord>,
}

/// A single slot in a train formation's composition.
///
/// `owned_rolling_stock_id` is exported for informational purposes only;
/// it is **not** restored on import (set to NULL) since the target database
/// may have different collection items.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct FormationElementRecord {
    pub id: String,
    pub prototype_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_rolling_stock_id: Option<String>,
    pub position_order: i64,
    /// 1 = traction required, -1 = traction excluded, 0 = default
    pub traction_override: i64,
}

/// A user wishlist with its items.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct WishlistRecord {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub is_default: bool,
    #[serde(default)]
    pub items: Vec<WishlistItemRecord>,
}

/// A decoder master record (hardware DCC decoder or similar).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DecoderRecord {
    pub id: String,
    pub manufacturer_id: String,
    pub product_code: String,
    pub decoder_type: String,
    pub protocol: String,
    pub decoder_interface: String,
}

/// A digital roster entry linking a collection item to its DCC address and decoder.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DigitalRollingStockRecord {
    pub id: String,
    /// References an owned rolling stock (collection item) in the source database.
    /// Preserved for informational purposes; not validated on import.
    pub owned_rolling_stock_id: String,
    pub dcc_address: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decoder_id: Option<String>,
}

/// A single item in a wishlist.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WishlistItemRecord {
    pub id: String,
    pub railway_model_id: String,
    pub priority: String,
    pub status: String,
    pub added_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_price: Option<MoneyRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchased_price: Option<MoneyRecord>,
}
