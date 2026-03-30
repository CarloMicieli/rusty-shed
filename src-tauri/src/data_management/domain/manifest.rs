use serde::{Deserialize, Serialize};
use specta::Type;

/// Manifest DTOs for deserialization from import packages.

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ManifestDto {
    #[serde(rename = "$schema", default)]
    pub schema: Option<String>,
    pub version: String,
    #[serde(default)]
    pub exported_at: Option<String>,
    #[serde(default)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub registered_company_name: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RailwayCompanyRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct RailwayModelRecord {
    pub id: String,
    pub manufacturer_id: String,
    pub product_code: String,
    pub description: String,
    pub scale: String,
    pub epoch: String,
    pub category: CategoryRecord,
    pub power_method: String,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub delivery_date: Option<String>,
    #[serde(default)]
    pub availability_status: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub rolling_stocks: Vec<RollingStockRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryRecord {
    pub r#type: String,
    #[serde(default)]
    pub sub_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RollingStockRecord {
    pub railway_company_id: String,
    pub series_code: String,
    #[serde(default)]
    pub road_number: Option<String>,
    #[serde(default)]
    pub livery: Option<String>,
    #[serde(default)]
    pub friendly_name: Option<String>,
    #[serde(default)]
    pub is_dummy: Option<bool>,
    #[serde(default)]
    pub length_over_buffers: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CollectionItemRecord {
    pub id: String,
    pub railway_model_id: String,
    pub added_date: String,
    #[serde(default)]
    pub removed_date: Option<String>,
    #[serde(default)]
    pub purchase_condition: Option<String>,
    #[serde(default)]
    pub model_condition: Option<String>,
    #[serde(default)]
    pub box_condition: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub purchase: Option<PurchaseRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseRecord {
    pub r#type: String,
    #[serde(default)]
    pub purchase_date: Option<String>,
    #[serde(default)]
    pub price: Option<MoneyRecord>,
    #[serde(default)]
    pub seller_id: Option<String>,
    #[serde(default)]
    pub sale_date: Option<String>,
    #[serde(default)]
    pub sale_price: Option<MoneyRecord>,
    #[serde(default)]
    pub deposit_amount: Option<MoneyRecord>,
    #[serde(default)]
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
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
    #[serde(default)]
    pub address: Option<AddressRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AddressRecord {
    #[serde(default)]
    pub street: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceCardRecord {
    pub id: String,
    pub collection_item_id: String,
    #[serde(default)]
    pub last_maintenance_date: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub radius: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackInventoryRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
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
    #[serde(default)]
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
    pub car_type: String,
    #[serde(default)]
    pub service_level: Option<String>,
    pub category: String,
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
    #[serde(default)]
    pub category_id: Option<String>,
    #[serde(default)]
    pub start_year: Option<i64>,
    #[serde(default)]
    pub end_year: Option<i64>,
    #[serde(default)]
    pub epoch: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub owned_rolling_stock_id: Option<String>,
    pub position_order: i64,
    /// 1 = traction required, -1 = traction excluded, 0 = default
    pub traction_override: i64,
}
