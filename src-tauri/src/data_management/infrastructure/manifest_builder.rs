use serde_json::{Map, Value, json};
use sqlx::{Row, SqlitePool};
use std::path::Path;

use crate::data_management::domain::{ExportEntitySelection, ExportError};

// ─── DB-to-schema value converters ──────────────────────────────────────────

fn db_category_to_schema(db_value: &str) -> &'static str {
    match db_value {
        "LOCOMOTIVES" => "LOCOMOTIVES",
        "PASSENGER_CARS" => "PASSENGER_CARS",
        "FREIGHT_CARS" => "FREIGHT_CARS",
        "ELECTRIC_MULTIPLE_UNITS" => "ELECTRIC_MULTIPLE_UNITS",
        "RAILCARS" => "RAILCARS",
        "TRAIN_SETS" => "TRAIN_SETS",
        "STARTER_SETS" => "STARTER_SETS",
        _ => "LOCOMOTIVES",
    }
}

fn db_power_method_to_schema(db_value: &str) -> &'static str {
    match db_value {
        "AC" => "AC",
        "TRIX_EXPRESS" => "TRIX_EXPRESS",
        _ => "DC",
    }
}

fn db_manufacturer_status_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "ACTIVE" => Some("ACTIVE"),
        "MERGED" => Some("MERGED"),
        "OUT_OF_BUSINESS" => Some("OUT_OF_BUSINESS"),
        _ => None,
    }
}

fn db_railway_company_status_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "ACTIVE" => Some("ACTIVE"),
        "INACTIVE" => Some("INACTIVE"),
        "MERGED" => Some("MERGED"),
        _ => None,
    }
}

fn db_availability_status_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "AVAILABLE" => Some("AVAILABLE"),
        "ANNOUNCED" => Some("ANNOUNCED"),
        "CANCELLED" => Some("CANCELLED"),
        "DISCONTINUED" => Some("DISCONTINUED"),
        _ => None,
    }
}

fn db_seller_type_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "SHOP" => Some("SHOP"),
        "PRIVATE" => Some("PRIVATE"),
        "MARKETPLACE" => Some("MARKETPLACE"),
        "DISTRIBUTOR" => Some("DISTRIBUTOR"),
        _ => None,
    }
}

fn db_purchase_type_to_schema(s: &str) -> Option<&'static str> {
    match s.to_ascii_uppercase().as_str() {
        "PURCHASED" => Some("purchased"),
        "SOLD" => Some("sold"),
        "PRE_ORDERED" | "PREORDERED" => Some("preOrdered"),
        _ => None,
    }
}

fn db_purchase_condition_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "NEW" => Some("NEW"),
        "PRE_OWNED" | "USED" => Some("PRE_OWNED"),
        _ => None,
    }
}

fn db_model_condition_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "MINT" => Some("MINT"),
        "NEAR_MINT" => Some("NEAR_MINT"),
        "EXCELLENT" => Some("EXCELLENT"),
        "VERY_GOOD" => Some("VERY_GOOD"),
        "GOOD" => Some("GOOD"),
        "FAIR" => Some("FAIR"),
        "POOR" => Some("POOR"),
        "FOR_PARTS" => Some("FOR_PARTS"),
        _ => None,
    }
}

fn db_box_condition_to_schema(s: &str) -> Option<&'static str> {
    match s {
        "ORIGINAL_MINT" => Some("ORIGINAL_MINT"),
        "ORIGINAL_GOOD" => Some("ORIGINAL_GOOD"),
        "ORIGINAL_WORN" => Some("ORIGINAL_WORN"),
        "REPLACEMENT_BOX" => Some("REPLACEMENT_BOX"),
        "NO_BOX" => Some("NO_BOX"),
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

/// Build a seller JSON value from a database row.
///
/// Centralises the field mapping so both the explicit-sellers export and the
/// auto-include-sellers-for-track-purchases path produce identical output.
/// Expects the row to contain: id, name, type, email, phone, website_url,
/// street_address, city, state_region, postal_code, country_code.
fn build_seller_value(row: &sqlx::sqlite::SqliteRow) -> Value {
    let address_obj = strip_null_fields(json!({
        "street": row.try_get::<Option<String>, _>("street_address").ok().flatten(),
        "city": row.try_get::<Option<String>, _>("city").ok().flatten(),
        "region": row.try_get::<Option<String>, _>("state_region").ok().flatten(),
        "postalCode": row.try_get::<Option<String>, _>("postal_code").ok().flatten(),
        "countryCode": row.try_get::<Option<String>, _>("country_code").ok().flatten(),
    }));
    let address = if address_obj
        .as_object()
        .map(|o| !o.is_empty())
        .unwrap_or(false)
    {
        address_obj
    } else {
        Value::Null
    };
    strip_null_fields(json!({
        "id": row.try_get::<String, _>("id").ok(),
        "name": row.try_get::<String, _>("name").ok(),
        "sellerType": enum_value(row.try_get::<String, _>("type").ok(), db_seller_type_to_schema),
        "email": row.try_get::<Option<String>, _>("email").ok().flatten(),
        "phone": row.try_get::<Option<String>, _>("phone").ok().flatten(),
        "websiteUrl": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
        "address": address,
    }))
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

#[derive(Debug, Clone, Copy)]
struct ExportInclusions {
    include_maintenance_logs: bool,
    include_collection_items: bool,
    include_wishlists: bool,
    include_railway_models: bool,
    include_track_inventory: bool,
    include_sellers: bool,
    include_dcc_roster: bool,
}

fn resolve_export_inclusions(selection: &ExportEntitySelection) -> ExportInclusions {
    let include_maintenance_logs = selection.include_maintenance_logs;
    let include_collection_items = selection.include_collection_items || include_maintenance_logs;
    let include_wishlists = selection.include_wishlists;
    let include_railway_models =
        selection.include_railway_models || include_collection_items || include_wishlists;
    let include_track_inventory = selection.include_track_inventory;
    let include_sellers = selection.include_sellers;
    let include_dcc_roster = selection.include_dcc_roster;

    ExportInclusions {
        include_maintenance_logs,
        include_collection_items,
        include_wishlists,
        include_railway_models,
        include_track_inventory,
        include_sellers,
        include_dcc_roster,
    }
}

async fn export_manufacturers_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let rows = sqlx::query(
        "SELECT id, name, registered_company_name, country_code, status, website_url, \
                street_address, extended_address, city, state_region, postal_code \
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
                "streetAddress": row.try_get::<Option<String>, _>("street_address").ok().flatten(),
                "extendedAddress": row.try_get::<Option<String>, _>("extended_address").ok().flatten(),
                "city": row.try_get::<Option<String>, _>("city").ok().flatten(),
                "stateRegion": row.try_get::<Option<String>, _>("state_region").ok().flatten(),
                "postalCode": row.try_get::<Option<String>, _>("postal_code").ok().flatten(),
            }))
        })
        .collect();
    data["manufacturers"] = json!(manufacturers);

    Ok(())
}

async fn export_railway_models_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let rc_rows = sqlx::query(
        "SELECT id, name, country_code, status, operating_since, operating_until \
         FROM railway_companies ORDER BY name",
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
                "operatingSince": row.try_get::<Option<String>, _>("operating_since").ok().flatten(),
                "operatingUntil": row.try_get::<Option<String>, _>("operating_until").ok().flatten(),
            }))
        })
        .collect();
    data["railwayCompanies"] = json!(railway_companies);

    let model_rows = sqlx::query(
        "SELECT rm.id, rm.manufacturer_id, rm.product_code, rm.category, rm.scale, \
                                    rm.power_method, rm.epoch, rm.delivery_date, rm.availability_status \
                     FROM railway_models rm \
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

        let rs_rows = sqlx::query(
            "SELECT id, railway_company_id, series_code, series, road_number, \
                friendly_name, depot, livery, electric_multiple_unit_type, \
                freight_car_type, locomotive_type, passenger_car_type, railcar_type, \
                service_level, length_inches, length_millimeters, \
                technical_minimum_radius_mm, technical_coupling_socket, \
                technical_coupling_close_couplers, technical_coupling_digital_shunting, \
                technical_flywheel_fitted, technical_body_shell, technical_chassis, \
                technical_interior_lights, technical_lights, technical_sprung_buffers, \
                dcc_interface, control, is_dummy \
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
                    "id": rs.try_get::<String, _>("id").ok(),
                    "railwayCompanyId": rs.try_get::<String, _>("railway_company_id").ok(),
                    "seriesCode": rs.try_get::<String, _>("series_code").ok(),
                    "series": rs.try_get::<Option<String>, _>("series").ok().flatten(),
                    "roadNumber": rs.try_get::<Option<String>, _>("road_number").ok().flatten(),
                    "depot": rs.try_get::<Option<String>, _>("depot").ok().flatten(),
                    "livery": rs.try_get::<Option<String>, _>("livery").ok().flatten(),
                    "friendlyName": rs.try_get::<Option<String>, _>("friendly_name").ok().flatten(),
                    "electricMultipleUnitType": rs.try_get::<Option<String>, _>("electric_multiple_unit_type").ok().flatten(),
                    "freightCarType": rs.try_get::<Option<String>, _>("freight_car_type").ok().flatten(),
                    "locomotiveType": rs.try_get::<Option<String>, _>("locomotive_type").ok().flatten(),
                    "passengerCarType": rs.try_get::<Option<String>, _>("passenger_car_type").ok().flatten(),
                    "railcarType": rs.try_get::<Option<String>, _>("railcar_type").ok().flatten(),
                    "serviceLevel": rs.try_get::<Option<String>, _>("service_level").ok().flatten(),
                    "isDummy": rs.try_get::<i64, _>("is_dummy").ok().map(|v| v != 0),
                    "lengthInches": rs.try_get::<Option<f64>, _>("length_inches").ok().flatten(),
                    "lengthMillimeters": rs.try_get::<Option<f64>, _>("length_millimeters").ok().flatten(),
                    "technicalMinimumRadiusMm": rs.try_get::<Option<f64>, _>("technical_minimum_radius_mm").ok().flatten(),
                    "technicalCouplingSocket": rs.try_get::<Option<String>, _>("technical_coupling_socket").ok().flatten(),
                    "technicalCouplingCloseCouplers": rs.try_get::<Option<String>, _>("technical_coupling_close_couplers").ok().flatten(),
                    "technicalCouplingDigitalShunting": rs.try_get::<Option<String>, _>("technical_coupling_digital_shunting").ok().flatten(),
                    "technicalFlywheelFitted": rs.try_get::<Option<String>, _>("technical_flywheel_fitted").ok().flatten(),
                    "technicalBodyShell": rs.try_get::<Option<String>, _>("technical_body_shell").ok().flatten(),
                    "technicalChassis": rs.try_get::<Option<String>, _>("technical_chassis").ok().flatten(),
                    "technicalInteriorLights": rs.try_get::<Option<String>, _>("technical_interior_lights").ok().flatten(),
                    "technicalLights": rs.try_get::<Option<String>, _>("technical_lights").ok().flatten(),
                    "technicalSprungBuffers": rs.try_get::<Option<String>, _>("technical_sprung_buffers").ok().flatten(),
                    "dccInterface": rs.try_get::<Option<String>, _>("dcc_interface").ok().flatten(),
                    "control": rs.try_get::<Option<String>, _>("control").ok().flatten(),
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

        let translation_rows = sqlx::query(
            "SELECT language_code, description, details \
             FROM railway_model_translations WHERE railway_model_id = ?",
        )
        .bind(&model_id)
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let mut description = Map::new();
        let mut details = Map::new();
        for tr in &translation_rows {
            let language = tr
                .try_get::<String, _>("language_code")
                .unwrap_or_else(|_| "en".to_string());
            if language != "en" && language != "it" {
                continue;
            }
            if let Some(text) = tr
                .try_get::<Option<String>, _>("description")
                .ok()
                .flatten()
            {
                description.insert(language.clone(), Value::String(text));
            }
            if let Some(text) = tr.try_get::<Option<String>, _>("details").ok().flatten() {
                details.insert(language, Value::String(text));
            }
        }
        if !description.contains_key("en") {
            description.insert("en".to_string(), Value::String(product_code.clone()));
        }

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
            "description": Value::Object(description),
            "details": if details.is_empty() { Value::Null } else { Value::Object(details) },
            "scale": row.try_get::<String, _>("scale").ok(),
            "epoch": row.try_get::<String, _>("epoch").ok(),
            "category": db_category_to_schema(&category_db),
            "powerMethod": db_power_method_to_schema(&power_method_db),
            "deliveryDate": row.try_get::<Option<String>, _>("delivery_date").ok().flatten(),
            "availabilityStatus": availability_status,
            "rollingStocks": rolling_stocks,
        })));
    }
    data["railwayModels"] = json!(models);

    Ok(())
}

async fn export_collection_items_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    media_dir: &Path,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

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

            let schema_purchase_type = purchase_type_raw.as_deref().and_then(db_purchase_type_to_schema);

            let purchase = if let Some(pt) = schema_purchase_type {
                let purchase_date: Option<String> =
                    row.try_get::<Option<String>, _>("purchase_date").ok().flatten();
                let sale_date: Option<String> =
                    row.try_get::<Option<String>, _>("sale_date").ok().flatten();
                let seller_id: Option<String> =
                    row.try_get::<Option<String>, _>("seller_id").ok().flatten();

                let conditions_met = match pt {
                    "purchased" => purchase_date.is_some(),
                    "sold" => purchase_date.is_some() && sale_date.is_some(),
                    "preOrdered" => seller_id.is_some(),
                    _ => true,
                };

                if conditions_met {
                    let price = match (
                        row.try_get::<Option<i64>, _>("purchased_price_amount")
                            .ok()
                            .flatten(),
                        row.try_get::<Option<String>, _>("purchased_price_currency")
                            .ok()
                            .flatten(),
                    ) {
                        (Some(amount), Some(currency)) => {
                            json!({ "amount": amount, "currency": currency })
                        }
                        _ => Value::Null,
                    };

                    let sale_price = match (
                        row.try_get::<Option<i64>, _>("sale_price_amount")
                            .ok()
                            .flatten(),
                        row.try_get::<Option<String>, _>("sale_price_currency")
                            .ok()
                            .flatten(),
                    ) {
                        (Some(amount), Some(currency)) => {
                            json!({ "amount": amount, "currency": currency })
                        }
                        _ => Value::Null,
                    };

                    let deposit = match (
                        row.try_get::<Option<i64>, _>("deposit_amount")
                            .ok()
                            .flatten(),
                        row.try_get::<Option<String>, _>("deposit_currency")
                            .ok()
                            .flatten(),
                    ) {
                        (Some(amount), Some(currency)) => {
                            json!({ "amount": amount, "currency": currency })
                        }
                        _ => Value::Null,
                    };

                    strip_null_fields(json!({
                        "type": pt,
                        "purchaseDate": purchase_date,
                        "sellerId": seller_id,
                        "price": price,
                        "salePrice": sale_price,
                        "depositAmount": deposit,
                        "saleDate": sale_date,
                        "expectedDelivery": row.try_get::<Option<String>, _>("expected_date").ok().flatten(),
                    }))
                } else {
                    Value::Null
                }
            } else {
                Value::Null
            };

            let model_id_for_image: Option<String> = row.try_get::<String, _>("railway_model_id").ok();
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

    let ors_rows = sqlx::query(
        "SELECT id, collection_item_id, rolling_stock_id, notes, dcc_address, \
                installed_decoder_id, current_coupler_id \
         FROM owned_rolling_stocks \
         WHERE collection_item_id IN (SELECT id FROM collection_items) \
         ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let owned_rolling_stocks: Vec<Value> = ors_rows
        .iter()
        .map(|row| {
            strip_null_fields(json!({
                "id": row.try_get::<String, _>("id").ok(),
                "collectionItemId": row.try_get::<String, _>("collection_item_id").ok(),
                "rollingStockId": row.try_get::<Option<String>, _>("rolling_stock_id").ok().flatten(),
                "notes": row.try_get::<Option<String>, _>("notes").ok().flatten(),
                "dccAddress": row.try_get::<Option<i64>, _>("dcc_address").ok().flatten(),
                "installedDecoderId": row.try_get::<Option<String>, _>("installed_decoder_id").ok().flatten(),
                "currentCouplerId": row.try_get::<Option<String>, _>("current_coupler_id").ok().flatten(),
            }))
        })
        .collect();
    data["ownedRollingStocks"] = json!(owned_rolling_stocks);

    Ok(())
}

async fn export_sellers_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let rows = sqlx::query(
        "SELECT id, name, type, email, phone, website_url, \
                street_address, city, state_region, postal_code, country_code \
         FROM sellers ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let sellers: Vec<Value> = rows.iter().map(build_seller_value).collect();
    data["sellers"] = json!(sellers);

    Ok(())
}

async fn export_maintenance_logs_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let card_rows = sqlx::query(
        "SELECT mc.id, ors.collection_item_id, ors.id AS owned_rolling_stock_id, \
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
                    .unwrap_or("repair");
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
            "ownedRollingStockId": card_row.try_get::<String, _>("owned_rolling_stock_id").ok(),
            "lastMaintenanceDate": card_row.try_get::<Option<String>, _>("last_maintenance_date").ok().flatten(),
            "nextMaintenanceDate": card_row.try_get::<Option<String>, _>("next_maintenance_date").ok().flatten(),
            "events": events,
        })));
    }
    data["maintenanceCards"] = json!(maintenance_cards);

    Ok(())
}

async fn export_track_inventory_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

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

    let inv_rows = sqlx::query("SELECT id, name, description FROM track_inventories ORDER BY id")
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let mut referenced_seller_ids: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut track_inventories: Vec<Value> = Vec::new();
    for inv_row in &inv_rows {
        let inv_id: String = inv_row
            .try_get("id")
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

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

    if !referenced_seller_ids.is_empty() {
        let existing_sellers: Vec<Value> = data["sellers"].as_array().cloned().unwrap_or_default();
        let existing_ids: std::collections::HashSet<String> = existing_sellers
            .iter()
            .filter_map(|seller| seller["id"].as_str().map(|id| id.to_string()))
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
                "SELECT id, name, type, email, phone, website_url, \
                        street_address, city, state_region, postal_code, country_code \
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
                all_sellers.push(build_seller_value(row));
            }
            data["sellers"] = json!(all_sellers);
        }
    }

    Ok(())
}

async fn export_train_formations_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let cat_rows =
        sqlx::query("SELECT id, name, is_custom FROM formation_categories ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let formation_categories: Vec<Value> = cat_rows
        .iter()
        .map(|row| {
            json!({
                "id": row.try_get::<String, _>("id").ok(),
                "name": row.try_get::<String, _>("name").ok(),
                "isCustom": row.try_get::<i64, _>("is_custom").ok().map(|v| v != 0).unwrap_or(false),
            })
        })
        .collect();
    data["formationCategories"] = json!(formation_categories);

    let proto_rows = sqlx::query(
        "SELECT DISTINCT p.id, p.railway_company_id, p.series_code, p.friendly_name, \
                p.specification_type, \
                p.locomotive_type, p.locomotive_series, \
                p.service_level, p.passenger_car_type, \
                p.freight_car_type, p.railcar_type, \
                p.electric_multiple_unit_type, p.elements_count, p.is_permanently_coupled, \
                p.is_motorized, p.is_custom \
         FROM prototypes p \
         WHERE p.is_custom = 1 \
            OR p.id IN ( \
                SELECT DISTINCT fe.prototype_id \
                FROM formation_elements fe \
                JOIN train_formations tf ON tf.id = fe.formation_id \
            ) \
         ORDER BY p.id",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let prototypes: Vec<Value> = proto_rows
        .iter()
        .map(|row| {
            strip_null_fields(json!({
                "id": row.try_get::<String, _>("id").ok(),
                "railwayCompanyId": row.try_get::<String, _>("railway_company_id").ok(),
                "seriesCode": row.try_get::<String, _>("series_code").ok(),
                "friendlyName": row.try_get::<Option<String>, _>("friendly_name").ok().flatten(),
                "specificationType": row.try_get::<String, _>("specification_type").ok(),
                "locomotiveType": row.try_get::<Option<String>, _>("locomotive_type").ok().flatten(),
                "locomotiveSeries": row.try_get::<Option<String>, _>("locomotive_series").ok().flatten(),
                "serviceLevel": row.try_get::<Option<String>, _>("service_level").ok().flatten(),
                "passengerCarType": row.try_get::<Option<String>, _>("passenger_car_type").ok().flatten(),
                "freightCarType": row.try_get::<Option<String>, _>("freight_car_type").ok().flatten(),
                "railcarType": row.try_get::<Option<String>, _>("railcar_type").ok().flatten(),
                "electricMultipleUnitType": row.try_get::<Option<String>, _>("electric_multiple_unit_type").ok().flatten(),
                "elementsCount": row.try_get::<Option<i64>, _>("elements_count").ok().flatten(),
                "isPermanentlyCoupled": row.try_get::<Option<i64>, _>("is_permanently_coupled").ok().flatten().map(|v| v != 0),
                "isMotorized": row.try_get::<i64, _>("is_motorized").ok().map(|v| v != 0).unwrap_or(false),
                "isCustom": row.try_get::<i64, _>("is_custom").ok().map(|v| v != 0).unwrap_or(false),
            }))
        })
        .collect();
    data["prototypes"] = json!(prototypes);

    if data["railwayCompanies"].is_null() || data["railwayCompanies"] == json!(null) {
        let rc_rows = sqlx::query(
            "SELECT id, name, country_code, status, operating_since, operating_until \
             FROM railway_companies ORDER BY name",
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
                    "operatingSince": row.try_get::<Option<String>, _>("operating_since").ok().flatten(),
                    "operatingUntil": row.try_get::<Option<String>, _>("operating_until").ok().flatten(),
                }))
            })
            .collect();
        data["railwayCompanies"] = json!(railway_companies);
    }

    let tf_rows = sqlx::query(
        "SELECT id, name, category_id, start_year, end_year, epoch, notes \
         FROM train_formations ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let mut train_formations: Vec<Value> = Vec::new();
    for tf_row in &tf_rows {
        let tf_id: String = tf_row
            .try_get("id")
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let elem_rows = sqlx::query(
            "SELECT id, prototype_id, owned_rolling_stock_id, position_order, traction_override \
             FROM formation_elements \
             WHERE formation_id = ? ORDER BY position_order",
        )
        .bind(&tf_id)
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let elements: Vec<Value> = elem_rows
            .iter()
            .map(|row| {
                strip_null_fields(json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "prototypeId": row.try_get::<String, _>("prototype_id").ok(),
                    "ownedRollingStockId": row.try_get::<Option<String>, _>("owned_rolling_stock_id").ok().flatten(),
                    "positionOrder": row.try_get::<i64, _>("position_order").ok(),
                    "tractionOverride": row.try_get::<i64, _>("traction_override").ok().unwrap_or(0),
                }))
            })
            .collect();

        train_formations.push(strip_null_fields(json!({
            "id": tf_id,
            "name": tf_row.try_get::<String, _>("name").ok(),
            "categoryId": tf_row.try_get::<Option<String>, _>("category_id").ok().flatten(),
            "startYear": tf_row.try_get::<Option<i64>, _>("start_year").ok().flatten(),
            "endYear": tf_row.try_get::<Option<i64>, _>("end_year").ok().flatten(),
            "epoch": tf_row.try_get::<Option<String>, _>("epoch").ok().flatten(),
            "notes": tf_row.try_get::<Option<String>, _>("notes").ok().flatten(),
            "elements": elements,
        })));
    }
    data["trainFormations"] = json!(train_formations);

    Ok(())
}

async fn export_wishlists_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let wl_rows = sqlx::query("SELECT id, name, notes, is_default FROM wishlists ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let mut wishlists: Vec<Value> = Vec::new();
    for wl_row in &wl_rows {
        let wl_id: String = wl_row
            .try_get("id")
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let item_rows = sqlx::query(
            "SELECT id, railway_model_id, priority, status, added_date, removed_date, \
                    notes, desired_price_amount, desired_price_currency, \
                    purchased_price_amount, purchased_price_currency \
             FROM wishlist_items WHERE wishlist_id = ? ORDER BY added_date",
        )
        .bind(&wl_id)
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let items: Vec<Value> = item_rows
            .iter()
            .map(|row| {
                let desired_price = match (
                    row.try_get::<Option<i64>, _>("desired_price_amount")
                        .ok()
                        .flatten(),
                    row.try_get::<Option<String>, _>("desired_price_currency")
                        .ok()
                        .flatten(),
                ) {
                    (Some(amount), Some(currency)) => {
                        json!({ "amount": amount, "currency": currency })
                    }
                    _ => Value::Null,
                };
                let purchased_price = match (
                    row.try_get::<Option<i64>, _>("purchased_price_amount")
                        .ok()
                        .flatten(),
                    row.try_get::<Option<String>, _>("purchased_price_currency")
                        .ok()
                        .flatten(),
                ) {
                    (Some(amount), Some(currency)) => {
                        json!({ "amount": amount, "currency": currency })
                    }
                    _ => Value::Null,
                };
                strip_null_fields(json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "railwayModelId": row.try_get::<String, _>("railway_model_id").ok(),
                    "priority": row.try_get::<String, _>("priority").ok(),
                    "status": row.try_get::<String, _>("status").ok(),
                    "addedDate": row.try_get::<String, _>("added_date").ok(),
                    "removedDate": row.try_get::<Option<String>, _>("removed_date").ok().flatten(),
                    "notes": row.try_get::<Option<String>, _>("notes").ok().flatten(),
                    "desiredPrice": desired_price,
                    "purchasedPrice": purchased_price,
                }))
            })
            .collect();

        wishlists.push(strip_null_fields(json!({
            "id": wl_id,
            "name": wl_row.try_get::<String, _>("name").ok(),
            "notes": wl_row.try_get::<Option<String>, _>("notes").ok().flatten(),
            "isDefault": wl_row.try_get::<i64, _>("is_default").ok().map(|v| v != 0).unwrap_or(false),
            "items": items,
        })));
    }
    data["wishlists"] = json!(wishlists);

    Ok(())
}

async fn export_dcc_roster_if_needed(
    data: &mut Value,
    pool: &SqlitePool,
    should_export: bool,
) -> Result<(), ExportError> {
    if !should_export {
        return Ok(());
    }

    let decoder_rows = sqlx::query(
        "SELECT id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface \
         FROM decoders ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let decoders: Vec<Value> = decoder_rows
        .iter()
        .map(|row| {
            json!({
                "id": row.try_get::<String, _>("id").ok(),
                "manufacturerId": row.try_get::<String, _>("manufacturer_id").ok(),
                "productCode": row.try_get::<String, _>("product_code").ok(),
                "decoderType": row.try_get::<String, _>("decoder_type").ok(),
                "protocol": row.try_get::<String, _>("protocol").ok(),
                "decoderInterface": row.try_get::<String, _>("decoder_interface").ok(),
            })
        })
        .collect();
    data["decoders"] = json!(decoders);

    let roster_rows = sqlx::query(
        "SELECT \
            REPLACE(id, 'trn:owned-rolling-stock:', 'trn:digital-rolling-stock:') AS id, \
            id AS owned_rolling_stock_id, \
            dcc_address, \
            installed_decoder_id \
         FROM owned_rolling_stocks \
         WHERE dcc_address IS NOT NULL OR installed_decoder_id IS NOT NULL \
         ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

    let digital_rolling_stocks: Vec<Value> = roster_rows
        .iter()
        .map(|row| {
            strip_null_fields(json!({
                "id": row.try_get::<String, _>("id").ok(),
                "ownedRollingStockId": row.try_get::<String, _>("owned_rolling_stock_id").ok(),
                "dccAddress": row.try_get::<i64, _>("dcc_address").ok(),
                "decoderId": row.try_get::<Option<String>, _>("installed_decoder_id").ok().flatten(),
            }))
        })
        .collect();
    data["digitalRollingStocks"] = json!(digital_rolling_stocks);

    Ok(())
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

    // Resolve FK dependencies so the exported manifest is always self-consistent.
    // Each entity must be present whenever something that references it is exported.
    let inclusions = resolve_export_inclusions(selection);

    export_manufacturers_if_needed(
        &mut data,
        pool,
        inclusions.include_railway_models
            || inclusions.include_track_inventory
            || inclusions.include_dcc_roster,
    )
    .await?;

    export_railway_models_if_needed(&mut data, pool, inclusions.include_railway_models).await?;

    export_collection_items_if_needed(
        &mut data,
        pool,
        media_dir,
        inclusions.include_collection_items,
    )
    .await?;

    export_sellers_if_needed(&mut data, pool, inclusions.include_sellers).await?;

    export_maintenance_logs_if_needed(&mut data, pool, inclusions.include_maintenance_logs).await?;

    export_track_inventory_if_needed(&mut data, pool, inclusions.include_track_inventory).await?;

    export_train_formations_if_needed(&mut data, pool, selection.include_train_formations).await?;

    export_wishlists_if_needed(&mut data, pool, inclusions.include_wishlists).await?;

    export_dcc_roster_if_needed(&mut data, pool, inclusions.include_dcc_roster).await?;

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
    use sqlx::SqlitePool;

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
        assert_eq!(result, Value::String("ACTIVE".to_string()));
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
        assert_eq!(db_category_to_schema("LOCOMOTIVES"), "LOCOMOTIVES");
        assert_eq!(db_category_to_schema("PASSENGER_CARS"), "PASSENGER_CARS");
        assert_eq!(db_category_to_schema("FREIGHT_CARS"), "FREIGHT_CARS");
        assert_eq!(
            db_category_to_schema("ELECTRIC_MULTIPLE_UNITS"),
            "ELECTRIC_MULTIPLE_UNITS"
        );
        assert_eq!(db_category_to_schema("RAILCARS"), "RAILCARS");
        assert_eq!(db_category_to_schema("TRAIN_SETS"), "TRAIN_SETS");
        assert_eq!(db_category_to_schema("STARTER_SETS"), "STARTER_SETS");
        assert_eq!(db_category_to_schema("UNKNOWN"), "LOCOMOTIVES");
    }

    // ─── db_power_method_to_schema ────────────────────────────────────────────

    #[test]
    fn test_db_power_method_to_schema_all_variants() {
        assert_eq!(db_power_method_to_schema("AC"), "AC");
        assert_eq!(db_power_method_to_schema("TRIX_EXPRESS"), "TRIX_EXPRESS");
        assert_eq!(db_power_method_to_schema("DC"), "DC");
        assert_eq!(db_power_method_to_schema("UNKNOWN"), "DC");
    }

    // ─── db_seller_type_to_schema ─────────────────────────────────────────────

    #[test]
    fn test_db_seller_type_to_schema_all_variants() {
        assert_eq!(db_seller_type_to_schema("SHOP"), Some("SHOP"));
        assert_eq!(db_seller_type_to_schema("PRIVATE"), Some("PRIVATE"));
        assert_eq!(db_seller_type_to_schema("MARKETPLACE"), Some("MARKETPLACE"));
        assert_eq!(db_seller_type_to_schema("DISTRIBUTOR"), Some("DISTRIBUTOR"));
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
        assert_eq!(db_model_condition_to_schema("MINT"), Some("MINT"));
        assert_eq!(db_model_condition_to_schema("NEAR_MINT"), Some("NEAR_MINT"));
        assert_eq!(db_model_condition_to_schema("EXCELLENT"), Some("EXCELLENT"));
        assert_eq!(db_model_condition_to_schema("VERY_GOOD"), Some("VERY_GOOD"));
        assert_eq!(db_model_condition_to_schema("GOOD"), Some("GOOD"));
        assert_eq!(db_model_condition_to_schema("FAIR"), Some("FAIR"));
        assert_eq!(db_model_condition_to_schema("POOR"), Some("POOR"));
        assert_eq!(db_model_condition_to_schema("FOR_PARTS"), Some("FOR_PARTS"));
        assert_eq!(db_model_condition_to_schema("UNKNOWN"), None);
    }

    // ─── db_purchase_condition_to_schema ──────────────────────────────────────

    #[test]
    fn test_db_purchase_condition_to_schema_all_variants() {
        assert_eq!(db_purchase_condition_to_schema("NEW"), Some("NEW"));
        assert_eq!(
            db_purchase_condition_to_schema("PRE_OWNED"),
            Some("PRE_OWNED")
        );
        assert_eq!(db_purchase_condition_to_schema("USED"), Some("PRE_OWNED"));
        assert_eq!(db_purchase_condition_to_schema("UNKNOWN"), None);
    }

    #[test]
    fn test_db_availability_status_to_schema_all_variants() {
        assert_eq!(
            db_availability_status_to_schema("AVAILABLE"),
            Some("AVAILABLE")
        );
        assert_eq!(
            db_availability_status_to_schema("ANNOUNCED"),
            Some("ANNOUNCED")
        );
        assert_eq!(
            db_availability_status_to_schema("CANCELLED"),
            Some("CANCELLED")
        );
        assert_eq!(
            db_availability_status_to_schema("DISCONTINUED"),
            Some("DISCONTINUED")
        );
        assert_eq!(db_availability_status_to_schema("UNKNOWN"), None);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn export_track_inventory_if_needed_returns_early_when_disabled(pool: SqlitePool) {
        let mut data = json!({});

        export_track_inventory_if_needed(&mut data, &pool, false)
            .await
            .expect("export should no-op when disabled");

        assert!(data["trackProducts"].is_null());
        assert!(data["trackInventories"].is_null());
        assert!(data["sellers"].is_null());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn export_track_inventory_if_needed_exports_and_merges_seller_references(
        pool: SqlitePool,
    ) {
        sqlx::query("INSERT INTO manufacturers (id, name, status) VALUES (?, ?, ?)")
            .bind("manufacturer-1")
            .bind("Manufacturer")
            .bind("ACTIVE")
            .execute(&pool)
            .await
            .expect("manufacturer insert should succeed");

        sqlx::query(
            "INSERT INTO sellers (id, name, type, email, phone, website_url, street_address, city, state_region, postal_code, country_code) \
             VALUES (?, ?, ?, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL)",
        )
        .bind("seller-existing")
        .bind("Existing Seller")
        .bind("SHOP")
        .execute(&pool)
        .await
        .expect("existing seller insert should succeed");

        sqlx::query(
            "INSERT INTO sellers (id, name, type, email, phone, website_url, street_address, city, state_region, postal_code, country_code) \
             VALUES (?, ?, ?, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL)",
        )
        .bind("seller-from-purchase")
        .bind("Purchase Seller")
        .bind("SHOP")
        .execute(&pool)
        .await
        .expect("purchase seller insert should succeed");

        sqlx::query(
            "INSERT INTO track_products \
             (id, track_id, manufacturer_id, product_code, description, track_type, track_code, with_roadbed, length_mm, radius_mm, created_at, updated_at, version) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)",
        )
        .bind("track-product-1")
        .bind("track-1")
        .bind("manufacturer-1")
        .bind("TR-1")
        .bind("Straight")
        .bind("STRAIGHT")
        .bind("ST")
        .bind(1_i64)
        .bind(Some(231_i64))
        .bind(Option::<i64>::None)
        .execute(&pool)
        .await
        .expect("track product insert should succeed");

        sqlx::query(
            "INSERT INTO track_inventories (id, name, description, created_at, updated_at, version) \
             VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)",
        )
        .bind("inventory-1")
        .bind("Layout A")
        .bind(Some("Main inventory"))
        .execute(&pool)
        .await
        .expect("track inventory insert should succeed");

        sqlx::query(
            "INSERT INTO track_inventory_items (inventory_id, track_id, quantity, required) \
             VALUES (?, ?, ?, ?)",
        )
        .bind("inventory-1")
        .bind("track-1")
        .bind(4_i64)
        .bind(2_i64)
        .execute(&pool)
        .await
        .expect("track inventory item insert should succeed");

        sqlx::query(
            "INSERT INTO track_purchases \
             (id, inventory_id, track_id, quantity, price_amount, price_currency, seller_id, purchase_date, created_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)",
        )
        .bind("purchase-1")
        .bind("inventory-1")
        .bind("track-1")
        .bind(1_i64)
        .bind(1299_i64)
        .bind("EUR")
        .bind(Some("seller-from-purchase"))
        .bind("2025-01-10")
        .execute(&pool)
        .await
        .expect("track purchase insert should succeed");

        let mut data = json!({
            "sellers": [
                {
                    "id": "seller-existing",
                    "name": "Existing Seller",
                    "sellerType": "SHOP"
                }
            ]
        });

        export_track_inventory_if_needed(&mut data, &pool, true)
            .await
            .expect("track inventory export should succeed");

        assert_eq!(
            data["trackProducts"]
                .as_array()
                .expect("track products should be an array")
                .len(),
            1
        );
        assert_eq!(
            data["trackInventories"]
                .as_array()
                .expect("track inventories should be an array")
                .len(),
            1
        );

        let exported_sellers = data["sellers"]
            .as_array()
            .expect("sellers should be an array after merge");
        let mut exported_ids: Vec<String> = exported_sellers
            .iter()
            .filter_map(|seller| seller["id"].as_str().map(ToOwned::to_owned))
            .collect();
        exported_ids.sort();
        assert_eq!(
            exported_ids,
            vec![
                "seller-existing".to_string(),
                "seller-from-purchase".to_string()
            ]
        );
    }
}
