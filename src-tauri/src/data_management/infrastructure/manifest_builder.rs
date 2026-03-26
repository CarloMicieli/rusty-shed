use serde_json::{Map, Value, json};
use sqlx::{Row, SqlitePool};
/// Archive manifest builder
use std::path::Path;

use crate::data_management::domain::{ExportEntitySelection, ExportError};

// ─── DB-to-schema value converters ──────────────────────────────────────────

fn db_category_to_schema(db_value: &str) -> &'static str {
    match db_value {
        "LOCOMOTIVES" => "locomotive",
        "PASSENGER_CARS" => "passengerCar",
        "FREIGHT_CARS" => "freightCar",
        "ELECTRIC_MULTIPLE_UNITS" => "electricMultipleUnit",
        "RAILCARS" => "railcar",
        "TRAIN_SETS" | "STARTER_SETS" => "trainSet",
        _ => "locomotive",
    }
}

fn db_power_method_to_schema(db_value: &str) -> &'static str {
    match db_value {
        "AC" => "ac",
        "TRIX_EXPRESS" => "trixExpress",
        _ => "dc",
    }
}

fn db_manufacturer_status_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "ACTIVE" => Some("active"),
        "MERGED" => Some("merged"),
        "OUT_OF_BUSINESS" => Some("outOfBusiness"),
        _ => None,
    }
}

fn db_railway_company_status_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "ACTIVE" => Some("active"),
        "INACTIVE" => Some("inactive"),
        _ => None,
    }
}

fn db_availability_status_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "AVAILABLE" => Some("available"),
        "ANNOUNCED" => Some("announced"),
        "CANCELLED" => Some("cancelled"),
        "DISCONTINUED" => Some("discontinued"),
        _ => None,
    }
}

fn db_seller_type_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "SHOP" => Some("shop"),
        "PRIVATE" => Some("private"),
        "MARKETPLACE" => Some("marketplace"),
        "DISTRIBUTOR" => Some("distributor"),
        _ => None,
    }
}

fn db_purchase_type_to_schema(s: &str) -> Option<&'static str> {
    match s.to_ascii_uppercase().as_str() {
        "PURCHASED" => Some("purchased"),
        "SOLD" => Some("sold"),
        "PREORDERED" => Some("preordered"),
        _ => None,
    }
}

fn db_purchase_condition_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "NEW" => Some("new"),
        "PRE_OWNED" => Some("preowned"),
        "USED" => Some("used"),
        _ => None,
    }
}

fn db_model_condition_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "MINT" | "NEAR_MINT" => Some("mint"),
        "EXCELLENT" | "VERY_GOOD" => Some("excellent"),
        "GOOD" => Some("good"),
        "FAIR" => Some("fair"),
        "POOR" | "FOR_PARTS" => Some("poor"),
        _ => None,
    }
}

fn db_box_condition_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "ORIGINAL_MINT" => Some("mint"),
        "ORIGINAL_GOOD" | "REPLACEMENT_BOX" => Some("good"),
        "ORIGINAL_WORN" => Some("damaged"),
        "NO_BOX" => Some("missing"),
        _ => None,
    }
}

fn db_maintenance_type_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "WHEEL_CLEANING" | "TRACK_CLEANING" | "CONTACT_CLEANING" => Some("cleaning"),
        "LUBRICATION" | "GEAR_GREASE" => Some("lubrication"),
        "MOTOR_BRUSH_REPLACEMENT"
        | "TRACTION_TIRE_REPLACEMENT"
        | "SPEAKER_REPAIR"
        | "COUPLER_ADJUSTMENT"
        | "DETAIL_REPAIR"
        | "DECODER_INSTALL"
        | "FIRMWARE_UPDATE"
        | "STAY_ALIVE_INSTALL"
        | "OTHER" => Some("repair"),
        "WEATHERING" => Some("modification"),
        "GENERAL_INSPECTION" => Some("inspection"),
        _ => None,
    }
}

// ─── JSON helpers ────────────────────────────────────────────────────────────

/// Remove null-valued fields from JSON objects (recursive).
///
/// The import JSON schema uses `additionalProperties: false` and typed fields
/// without nullable support. When a DB column is NULL, `serde_json::json!`
/// produces `"field": null`, which fails schema validation. Stripping null
/// fields before writing the manifest ensures only present values are included.
fn strip_null_fields(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let filtered: Map<String, Value> = map
                .into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, strip_null_fields(v)))
                .collect();
            Value::Object(filtered)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(strip_null_fields).collect()),
        other => other,
    }
}

/// Convert an optional DB enum string to its schema representation.
fn enum_value<F>(raw: Option<String>, converter: F) -> Value
where
    F: Fn(&str) -> Option<&'static str>,
{
    raw.as_deref()
        .and_then(converter)
        .map(|s| Value::String(s.to_string()))
        .unwrap_or(Value::Null)
}

fn probe_model_image(media_dir: &Path, model_id: &str) -> Option<String> {
    let base = model_id.replace(':', "_");
    for ext in &["png", "jpg", "jpeg"] {
        let filename = format!("{}.{}", base, ext);
        if media_dir.join(&filename).exists() {
            return Some(filename);
        }
    }
    None
}

// ─── Main builder ────────────────────────────────────────────────────────────

/// Build export manifest from selected entities.
///
/// Produces a manifest.json compatible with the import feature (spec 010).
/// The top-level key is `"data"` matching `ManifestDto.data` in the import DTO.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `selection` - Selected entity types to export
///
/// # Returns
/// A JSON manifest ready for archiving
pub async fn build_manifest(
    pool: &SqlitePool,
    selection: &ExportEntitySelection,
    media_dir: &Path,
) -> Result<Value, ExportError> {
    let mut data = json!({});

    // Export manufacturers when railway models or track inventory are selected (FK dependency)
    if selection.include_railway_models || selection.include_track_inventory {
        let rows = sqlx::query(
            "SELECT id, name, registered_company_name, country_code, status, website_url \
             FROM manufacturers ORDER BY name",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let manufacturers: Vec<Value> = rows
            .iter()
            .map(|row| {
                strip_null_fields(json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "name": row.try_get::<String, _>("name").ok(),
                    "registeredCompanyName": row.try_get::<Option<String>, _>("registered_company_name").ok().flatten(),
                    "countryCode": row.try_get::<Option<String>, _>("country_code").ok().flatten(),
                    "status": enum_value(row.try_get::<String, _>("status").ok(), db_manufacturer_status_to_schema),
                    "websiteUrl": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
                }))
            })
            .collect();
        data["manufacturers"] = json!(manufacturers);
    }

    if selection.include_railway_models {
        // Railway companies (referenced by rolling stocks)
        let rc_rows = sqlx::query(
            "SELECT id, name, country_code, status FROM railway_companies ORDER BY name",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let railway_companies: Vec<Value> = rc_rows
            .iter()
            .map(|row| {
                strip_null_fields(json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "name": row.try_get::<String, _>("name").ok(),
                    "countryCode": row.try_get::<Option<String>, _>("country_code").ok().flatten(),
                    "status": enum_value(row.try_get::<Option<String>, _>("status").ok().flatten(), db_railway_company_status_to_schema),
                }))
            })
            .collect();
        data["railwayCompanies"] = json!(railway_companies);

        // Railway models with description/details from translations + nested rolling stocks
        let model_rows = sqlx::query(
            "SELECT rm.id, rm.manufacturer_id, rm.product_code, rm.category, rm.scale, \
                    rm.power_method, rm.epoch, rm.delivery_date, rm.availability_status, \
                    rmt.description, rmt.details \
             FROM railway_models rm \
             LEFT JOIN railway_model_translations rmt \
               ON rmt.railway_model_id = rm.id AND rmt.language_code = 'en' \
             ORDER BY rm.id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let mut models: Vec<Value> = Vec::new();
        for row in &model_rows {
            let model_id: String = row
                .try_get("id")
                .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            // Fetch nested rolling stocks
            let rs_rows = sqlx::query(
                "SELECT railway_company_id, series_code, road_number, livery, \
                        friendly_name, is_dummy \
                 FROM rolling_stocks WHERE railway_model_id = ? ORDER BY series_code",
            )
            .bind(&model_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            let rolling_stocks: Vec<Value> = rs_rows
                .iter()
                .map(|rs| {
                    strip_null_fields(json!({
                        "railwayCompanyId": rs.try_get::<String, _>("railway_company_id").ok(),
                        "seriesCode": rs.try_get::<String, _>("series_code").ok(),
                        "roadNumber": rs.try_get::<Option<String>, _>("road_number").ok().flatten(),
                        "livery": rs.try_get::<Option<String>, _>("livery").ok().flatten(),
                        "friendlyName": rs.try_get::<Option<String>, _>("friendly_name").ok().flatten(),
                        "isDummy": rs.try_get::<i64, _>("is_dummy").ok().map(|v| v != 0),
                    }))
                })
                .collect();

            let category_db: String = row
                .try_get("category")
                .unwrap_or_else(|_| "LOCOMOTIVES".to_string());
            let power_method_db: String = row
                .try_get("power_method")
                .unwrap_or_else(|_| "DC".to_string());
            let product_code: String = row.try_get("product_code").unwrap_or_default();
            let description: String = row
                .try_get::<Option<String>, _>("description")
                .ok()
                .flatten()
                .unwrap_or_else(|| product_code.clone());

            let availability_status = enum_value(
                row.try_get::<Option<String>, _>("availability_status")
                    .ok()
                    .flatten(),
                db_availability_status_to_schema,
            );

            models.push(strip_null_fields(json!({
                "id": model_id,
                "manufacturerId": row.try_get::<String, _>("manufacturer_id").ok(),
                "productCode": product_code,
                "description": description,
                "details": row.try_get::<Option<String>, _>("details").ok().flatten(),
                "scale": row.try_get::<String, _>("scale").ok(),
                "epoch": row.try_get::<String, _>("epoch").ok(),
                "category": {
                    "type": db_category_to_schema(&category_db),
                },
                "powerMethod": db_power_method_to_schema(&power_method_db),
                "deliveryDate": row.try_get::<Option<String>, _>("delivery_date").ok().flatten(),
                "availabilityStatus": availability_status,
                "rollingStocks": rolling_stocks,
            })));
        }
        data["railwayModels"] = json!(models);
    }

    if selection.include_collection_items {
        let item_rows = sqlx::query(
            "SELECT ci.id, ci.railway_model_id, ci.added_date, ci.removed_date, \
                    ci.purchase_condition, ci.model_condition, ci.box_condition, ci.notes, \
                    pi.purchase_type, pi.purchase_date, pi.seller_id, \
                    pi.purchased_price_amount, pi.purchased_price_currency, \
                    pi.sale_date, pi.sale_price_amount, pi.sale_price_currency, \
                    pi.deposit_amount, pi.deposit_currency, pi.expected_date \
             FROM collection_items ci \
             LEFT JOIN purchase_infos pi ON pi.collection_item_id = ci.id \
             ORDER BY ci.id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let items: Vec<Value> = item_rows
            .iter()
            .map(|row| {
                let purchase_type_raw: Option<String> = row
                    .try_get::<Option<String>, _>("purchase_type")
                    .ok()
                    .flatten();

                // Only build a purchase object when type maps to a known schema value.
                // Purchase.type is required by the schema, so without it we omit the object.
                let schema_purchase_type =
                    purchase_type_raw.as_deref().and_then(db_purchase_type_to_schema);

                let purchase = if let Some(pt) = schema_purchase_type {
                    let purchase_date: Option<String> = row
                        .try_get::<Option<String>, _>("purchase_date")
                        .ok()
                        .flatten();
                    let price_amount: Option<i64> = row
                        .try_get::<Option<i64>, _>("purchased_price_amount")
                        .ok()
                        .flatten();
                    let price_currency: Option<String> = row
                        .try_get::<Option<String>, _>("purchased_price_currency")
                        .ok()
                        .flatten();
                    let price = match (price_amount, price_currency) {
                        (Some(amount), Some(currency)) => {
                            json!({ "amount": amount, "currency": currency })
                        }
                        _ => Value::Null,
                    };

                    let sale_price_amount: Option<i64> = row
                        .try_get::<Option<i64>, _>("sale_price_amount")
                        .ok()
                        .flatten();
                    let sale_price_currency: Option<String> = row
                        .try_get::<Option<String>, _>("sale_price_currency")
                        .ok()
                        .flatten();
                    let sale_price = match (sale_price_amount, sale_price_currency) {
                        (Some(amount), Some(currency)) => {
                            json!({ "amount": amount, "currency": currency })
                        }
                        _ => Value::Null,
                    };

                    let deposit_amount_val: Option<i64> = row
                        .try_get::<Option<i64>, _>("deposit_amount")
                        .ok()
                        .flatten();
                    let deposit_currency: Option<String> = row
                        .try_get::<Option<String>, _>("deposit_currency")
                        .ok()
                        .flatten();
                    let deposit = match (deposit_amount_val, deposit_currency) {
                        (Some(amount), Some(currency)) => {
                            json!({ "amount": amount, "currency": currency })
                        }
                        _ => Value::Null,
                    };

                    strip_null_fields(json!({
                        "type": pt,
                        "purchaseDate": purchase_date,
                        "sellerId": row.try_get::<Option<String>, _>("seller_id").ok().flatten(),
                        "price": price,
                        "salePrice": sale_price,
                        "depositAmount": deposit,
                        "saleDate": row.try_get::<Option<String>, _>("sale_date").ok().flatten(),
                        "expectedDelivery": row.try_get::<Option<String>, _>("expected_date").ok().flatten(),
                    }))
                } else {
                    Value::Null
                };

                let model_id_for_image: Option<String> =
                    row.try_get::<String, _>("railway_model_id").ok();
                let image = model_id_for_image
                    .as_deref()
                    .and_then(|id| probe_model_image(media_dir, id));
                strip_null_fields(json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "railwayModelId": row.try_get::<String, _>("railway_model_id").ok(),
                    "addedDate": row.try_get::<String, _>("added_date").ok(),
                    "removedDate": row.try_get::<Option<String>, _>("removed_date").ok().flatten(),
                    "purchaseCondition": enum_value(
                        row.try_get::<Option<String>, _>("purchase_condition").ok().flatten(),
                        db_purchase_condition_to_schema,
                    ),
                    "modelCondition": enum_value(
                        row.try_get::<Option<String>, _>("model_condition").ok().flatten(),
                        db_model_condition_to_schema,
                    ),
                    "boxCondition": enum_value(
                        row.try_get::<Option<String>, _>("box_condition").ok().flatten(),
                        db_box_condition_to_schema,
                    ),
                    "notes": row.try_get::<Option<String>, _>("notes").ok().flatten(),
                    "image": image,
                    "purchase": purchase,
                }))
            })
            .collect();
        data["collectionItems"] = json!(items);
    }

    if selection.include_sellers {
        let rows = sqlx::query(
            "SELECT id, name, type, email, phone, website_url FROM sellers ORDER BY name",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let sellers: Vec<Value> = rows
            .iter()
            .map(|row| {
                strip_null_fields(json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "name": row.try_get::<String, _>("name").ok(),
                    "sellerType": enum_value(row.try_get::<String, _>("type").ok(), db_seller_type_to_schema),
                    "email": row.try_get::<Option<String>, _>("email").ok().flatten(),
                    "phone": row.try_get::<Option<String>, _>("phone").ok().flatten(),
                    "websiteUrl": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
                }))
            })
            .collect();
        data["sellers"] = json!(sellers);
    }

    if selection.include_maintenance_logs {
        // Query maintenance cards joined to owned_rolling_stocks to get collection_item_id.
        // The schema requires MaintenanceCard.collectionItemId; the DB stores
        // maintenance_cards.owned_rolling_stock_id → owned_rolling_stocks.collection_item_id.
        let card_rows = sqlx::query(
            "SELECT mc.id, ors.collection_item_id, \
                    mc.last_maintenance_date, mc.next_maintenance_date \
             FROM maintenance_cards mc \
             JOIN owned_rolling_stocks ors ON ors.id = mc.owned_rolling_stock_id \
             ORDER BY mc.id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let mut maintenance_cards: Vec<Value> = Vec::new();
        for card_row in &card_rows {
            let card_id: String = card_row
                .try_get("id")
                .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            let event_rows = sqlx::query(
                "SELECT id, date_performed, maintenance_type, notes \
                 FROM maintenance_events \
                 WHERE maintenance_card_id = ? \
                 ORDER BY date_performed",
            )
            .bind(&card_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            let events: Vec<Value> = event_rows
                .iter()
                .map(|ev| {
                    let schema_type = ev
                        .try_get::<Option<String>, _>("maintenance_type")
                        .ok()
                        .flatten()
                        .as_deref()
                        .and_then(db_maintenance_type_to_schema)
                        .unwrap_or("repair"); // "OTHER" maps to "repair"; safe fallback for NULL
                    strip_null_fields(json!({
                        "id": ev.try_get::<String, _>("id").ok(),
                        "date": ev.try_get::<String, _>("date_performed").ok(),
                        "type": schema_type,
                        "description": ev.try_get::<Option<String>, _>("notes").ok().flatten(),
                    }))
                })
                .collect();

            maintenance_cards.push(strip_null_fields(json!({
                "id": card_id,
                "collectionItemId": card_row.try_get::<String, _>("collection_item_id").ok(),
                "lastMaintenanceDate": card_row.try_get::<Option<String>, _>("last_maintenance_date").ok().flatten(),
                "nextMaintenanceDate": card_row.try_get::<Option<String>, _>("next_maintenance_date").ok().flatten(),
                "events": events,
            })));
        }
        data["maintenanceCards"] = json!(maintenance_cards);
    }

    if selection.include_track_inventory {
        // Track products
        let tp_rows = sqlx::query(
            "SELECT track_id, manufacturer_id, product_code, description, \
                    track_type, track_code, with_roadbed, length_mm, radius_mm \
             FROM track_products ORDER BY track_id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let track_products: Vec<Value> = tp_rows
            .iter()
            .map(|row| {
                strip_null_fields(json!({
                    "trackId": row.try_get::<String, _>("track_id").ok(),
                    "manufacturerId": row.try_get::<String, _>("manufacturer_id").ok(),
                    "productCode": row.try_get::<String, _>("product_code").ok(),
                    "description": row.try_get::<String, _>("description").ok(),
                    "trackType": row.try_get::<String, _>("track_type").ok(),
                    "trackCode": row.try_get::<String, _>("track_code").ok(),
                    "withRoadbed": row.try_get::<i64, _>("with_roadbed").ok().map(|v| v != 0),
                    "length": row.try_get::<Option<i64>, _>("length_mm").ok().flatten(),
                    "radius": row.try_get::<Option<i64>, _>("radius_mm").ok().flatten(),
                }))
            })
            .collect();
        data["trackProducts"] = json!(track_products);

        // Track inventories with nested items and purchases
        let inv_rows =
            sqlx::query("SELECT id, name, description FROM track_inventories ORDER BY id")
                .fetch_all(pool)
                .await
                .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        // Collect seller_ids referenced by track purchases to auto-include those sellers
        let mut referenced_seller_ids: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        let mut track_inventories: Vec<Value> = Vec::new();
        for inv_row in &inv_rows {
            let inv_id: String = inv_row
                .try_get("id")
                .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            // Nested items
            let item_rows = sqlx::query(
                "SELECT track_id, quantity, required \
                 FROM track_inventory_items WHERE inventory_id = ? ORDER BY track_id",
            )
            .bind(&inv_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            let items: Vec<Value> = item_rows
                .iter()
                .map(|row| {
                    strip_null_fields(json!({
                        "trackId": row.try_get::<String, _>("track_id").ok(),
                        "quantity": row.try_get::<i64, _>("quantity").ok(),
                        "required": row.try_get::<i64, _>("required").ok(),
                    }))
                })
                .collect();

            // Nested purchases
            let purchase_rows = sqlx::query(
                "SELECT id, track_id, quantity, price_amount, price_currency, \
                        seller_id, purchase_date \
                 FROM track_purchases WHERE inventory_id = ? ORDER BY purchase_date",
            )
            .bind(&inv_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

            let purchases: Vec<Value> = purchase_rows
                .iter()
                .map(|row| {
                    let seller_id: Option<String> =
                        row.try_get::<Option<String>, _>("seller_id").ok().flatten();
                    if let Some(ref sid) = seller_id {
                        referenced_seller_ids.insert(sid.clone());
                    }
                    let price_amount: i64 = row.try_get::<i64, _>("price_amount").unwrap_or(0);
                    let price_currency: String = row
                        .try_get::<String, _>("price_currency")
                        .unwrap_or_else(|_| "EUR".to_string());
                    strip_null_fields(json!({
                        "id": row.try_get::<String, _>("id").ok(),
                        "trackId": row.try_get::<String, _>("track_id").ok(),
                        "quantity": row.try_get::<i64, _>("quantity").ok(),
                        "price": { "amount": price_amount, "currency": price_currency },
                        "sellerId": seller_id,
                        "purchaseDate": row.try_get::<String, _>("purchase_date").ok(),
                    }))
                })
                .collect();

            track_inventories.push(strip_null_fields(json!({
                "id": inv_id,
                "name": inv_row.try_get::<String, _>("name").ok(),
                "description": inv_row.try_get::<Option<String>, _>("description").ok().flatten(),
                "items": items,
                "purchases": purchases,
            })));
        }
        data["trackInventories"] = json!(track_inventories);

        // Auto-include sellers referenced by track purchases (merge with any already exported)
        if !referenced_seller_ids.is_empty() {
            let existing_sellers: Vec<Value> =
                data["sellers"].as_array().cloned().unwrap_or_default();
            let existing_ids: std::collections::HashSet<String> = existing_sellers
                .iter()
                .filter_map(|s| s["id"].as_str().map(|id| id.to_string()))
                .collect();

            let missing_ids: Vec<&String> = referenced_seller_ids
                .iter()
                .filter(|id| !existing_ids.contains(*id))
                .collect();

            if !missing_ids.is_empty() {
                let placeholders = missing_ids
                    .iter()
                    .map(|_| "?")
                    .collect::<Vec<_>>()
                    .join(", ");
                let query = format!(
                    "SELECT id, name, type, email, phone, website_url \
                     FROM sellers WHERE id IN ({}) ORDER BY name",
                    placeholders
                );
                let mut q = sqlx::query(&query);
                for id in &missing_ids {
                    q = q.bind(id.as_str());
                }
                let seller_rows = q
                    .fetch_all(pool)
                    .await
                    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

                let mut all_sellers = existing_sellers;
                for row in &seller_rows {
                    all_sellers.push(strip_null_fields(json!({
                        "id": row.try_get::<String, _>("id").ok(),
                        "name": row.try_get::<String, _>("name").ok(),
                        "sellerType": enum_value(row.try_get::<String, _>("type").ok(), db_seller_type_to_schema),
                        "email": row.try_get::<Option<String>, _>("email").ok().flatten(),
                        "phone": row.try_get::<Option<String>, _>("phone").ok().flatten(),
                        "websiteUrl": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
                    })));
                }
                data["sellers"] = json!(all_sellers);
            }
        }
    }

    // Build final manifest — "data" key matches ManifestDto.data in the import feature
    let manifest = json!({
        "$schema": "https://rusty-shed.app/schemas/manifest/v1.json",
        "version": "1.0",
        "exportedAt": chrono::Utc::now().to_rfc3339(),
        "source": "rusty-shed",
        "data": data
    });

    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ─── strip_null_fields ────────────────────────────────────────────────────

    #[test]
    fn test_strip_null_fields_removes_null_values() {
        let input = json!({ "a": "hello", "b": null, "c": 42 });
        let result = strip_null_fields(input);
        assert_eq!(result, json!({ "a": "hello", "c": 42 }));
    }

    #[test]
    fn test_strip_null_fields_nested_object() {
        let input = json!({ "outer": { "keep": "yes", "drop": null } });
        let result = strip_null_fields(input);
        assert_eq!(result, json!({ "outer": { "keep": "yes" } }));
    }

    #[test]
    fn test_strip_null_fields_array_items_processed() {
        let input = json!([{ "x": 1, "y": null }, { "x": 2, "y": null }]);
        let result = strip_null_fields(input);
        assert_eq!(result, json!([{ "x": 1 }, { "x": 2 }]));
    }

    #[test]
    fn test_strip_null_fields_preserves_non_object_scalars() {
        let input = json!("hello");
        let result = strip_null_fields(input);
        assert_eq!(result, json!("hello"));

        let input2 = json!(123);
        let result2 = strip_null_fields(input2);
        assert_eq!(result2, json!(123));
    }

    // ─── enum_value ───────────────────────────────────────────────────────────

    #[test]
    fn test_enum_value_known_input() {
        let result = enum_value(Some("ACTIVE".to_string()), db_manufacturer_status_to_schema);
        assert_eq!(result, Value::String("active".to_string()));
    }

    #[test]
    fn test_enum_value_unknown_input_returns_null() {
        let result = enum_value(
            Some("UNKNOWN".to_string()),
            db_manufacturer_status_to_schema,
        );
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_enum_value_none_returns_null() {
        let result = enum_value(None, db_manufacturer_status_to_schema);
        assert_eq!(result, Value::Null);
    }

    // ─── db_category_to_schema ────────────────────────────────────────────────

    #[test]
    fn test_db_category_to_schema_all_variants() {
        assert_eq!(db_category_to_schema("LOCOMOTIVES"), "locomotive");
        assert_eq!(db_category_to_schema("PASSENGER_CARS"), "passengerCar");
        assert_eq!(db_category_to_schema("FREIGHT_CARS"), "freightCar");
        assert_eq!(
            db_category_to_schema("ELECTRIC_MULTIPLE_UNITS"),
            "electricMultipleUnit"
        );
        assert_eq!(db_category_to_schema("RAILCARS"), "railcar");
        assert_eq!(db_category_to_schema("TRAIN_SETS"), "trainSet");
        assert_eq!(db_category_to_schema("STARTER_SETS"), "trainSet");
        assert_eq!(db_category_to_schema("UNKNOWN"), "locomotive");
    }

    // ─── db_power_method_to_schema ────────────────────────────────────────────

    #[test]
    fn test_db_power_method_to_schema_all_variants() {
        assert_eq!(db_power_method_to_schema("AC"), "ac");
        assert_eq!(db_power_method_to_schema("TRIX_EXPRESS"), "trixExpress");
        assert_eq!(db_power_method_to_schema("DC"), "dc");
        assert_eq!(db_power_method_to_schema("UNKNOWN"), "dc");
    }

    // ─── db_seller_type_to_schema ─────────────────────────────────────────────

    #[test]
    fn test_db_seller_type_to_schema_all_variants() {
        assert_eq!(db_seller_type_to_schema("SHOP"), Some("shop"));
        assert_eq!(db_seller_type_to_schema("PRIVATE"), Some("private"));
        assert_eq!(db_seller_type_to_schema("MARKETPLACE"), Some("marketplace"));
        assert_eq!(db_seller_type_to_schema("DISTRIBUTOR"), Some("distributor"));
        assert_eq!(db_seller_type_to_schema("UNKNOWN"), None);
    }

    // ─── db_maintenance_type_to_schema ───────────────────────────────────────

    #[test]
    fn test_db_maintenance_type_to_schema_all_variants() {
        assert_eq!(
            db_maintenance_type_to_schema("WHEEL_CLEANING"),
            Some("cleaning")
        );
        assert_eq!(
            db_maintenance_type_to_schema("TRACK_CLEANING"),
            Some("cleaning")
        );
        assert_eq!(
            db_maintenance_type_to_schema("CONTACT_CLEANING"),
            Some("cleaning")
        );
        assert_eq!(
            db_maintenance_type_to_schema("LUBRICATION"),
            Some("lubrication")
        );
        assert_eq!(
            db_maintenance_type_to_schema("GEAR_GREASE"),
            Some("lubrication")
        );
        assert_eq!(
            db_maintenance_type_to_schema("MOTOR_BRUSH_REPLACEMENT"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("TRACTION_TIRE_REPLACEMENT"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("SPEAKER_REPAIR"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("COUPLER_ADJUSTMENT"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("DETAIL_REPAIR"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("DECODER_INSTALL"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("FIRMWARE_UPDATE"),
            Some("repair")
        );
        assert_eq!(
            db_maintenance_type_to_schema("STAY_ALIVE_INSTALL"),
            Some("repair")
        );
        assert_eq!(db_maintenance_type_to_schema("OTHER"), Some("repair"));
        assert_eq!(
            db_maintenance_type_to_schema("WEATHERING"),
            Some("modification")
        );
        assert_eq!(
            db_maintenance_type_to_schema("GENERAL_INSPECTION"),
            Some("inspection")
        );
        // unknown / future variants return None; caller provides fallback
        assert_eq!(db_maintenance_type_to_schema("UNKNOWN"), None);
    }

    // ─── db_model_condition_to_schema ─────────────────────────────────────────

    #[test]
    fn test_db_model_condition_to_schema_all_variants() {
        assert_eq!(db_model_condition_to_schema("MINT"), Some("mint"));
        assert_eq!(db_model_condition_to_schema("NEAR_MINT"), Some("mint"));
        assert_eq!(db_model_condition_to_schema("EXCELLENT"), Some("excellent"));
        assert_eq!(db_model_condition_to_schema("VERY_GOOD"), Some("excellent"));
        assert_eq!(db_model_condition_to_schema("GOOD"), Some("good"));
        assert_eq!(db_model_condition_to_schema("FAIR"), Some("fair"));
        assert_eq!(db_model_condition_to_schema("POOR"), Some("poor"));
        assert_eq!(db_model_condition_to_schema("FOR_PARTS"), Some("poor"));
        assert_eq!(db_model_condition_to_schema("UNKNOWN"), None);
    }

    // ─── db_purchase_condition_to_schema ──────────────────────────────────────

    #[test]
    fn test_db_purchase_condition_to_schema_all_variants() {
        assert_eq!(db_purchase_condition_to_schema("NEW"), Some("new"));
        assert_eq!(
            db_purchase_condition_to_schema("PRE_OWNED"),
            Some("preowned")
        );
        assert_eq!(db_purchase_condition_to_schema("USED"), Some("used"));
        assert_eq!(db_purchase_condition_to_schema("UNKNOWN"), None);
    }
}
