/// Archive manifest builder
use serde_json::{Value, json};
use sqlx::{Row, SqlitePool};

use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;

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
) -> Result<Value, ExportError> {
    let mut data = json!({});

    // Always export manufacturers when railway models are selected (FK dependency)
    if selection.include_railway_models {
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
                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "name": row.try_get::<String, _>("name").ok(),
                    "registeredCompanyName": row.try_get::<Option<String>, _>("registered_company_name").ok().flatten(),
                    "countryCode": row.try_get::<Option<String>, _>("country_code").ok().flatten(),
                    "status": row.try_get::<String, _>("status").ok(),
                    "websiteUrl": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
                })
            })
            .collect();
        data["manufacturers"] = json!(manufacturers);

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
                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "name": row.try_get::<String, _>("name").ok(),
                    "countryCode": row.try_get::<Option<String>, _>("country_code").ok().flatten(),
                    "status": row.try_get::<Option<String>, _>("status").ok().flatten(),
                })
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
                    json!({
                        "railwayCompanyId": rs.try_get::<String, _>("railway_company_id").ok(),
                        "seriesCode": rs.try_get::<String, _>("series_code").ok(),
                        "roadNumber": rs.try_get::<Option<String>, _>("road_number").ok().flatten(),
                        "livery": rs.try_get::<Option<String>, _>("livery").ok().flatten(),
                        "friendlyName": rs.try_get::<Option<String>, _>("friendly_name").ok().flatten(),
                        "isDummy": rs.try_get::<i64, _>("is_dummy").ok().map(|v| v != 0),
                    })
                })
                .collect();

            let category_type: String = row
                .try_get("category")
                .unwrap_or_else(|_| "LOCOMOTIVES".to_string());

            models.push(json!({
                "id": model_id,
                "manufacturerId": row.try_get::<String, _>("manufacturer_id").ok(),
                "productCode": row.try_get::<String, _>("product_code").ok(),
                "description": row.try_get::<Option<String>, _>("description").ok().flatten(),
                "details": row.try_get::<Option<String>, _>("details").ok().flatten(),
                "scale": row.try_get::<String, _>("scale").ok(),
                "epoch": row.try_get::<String, _>("epoch").ok(),
                "category": {
                    "type": category_type,
                },
                "powerMethod": row.try_get::<String, _>("power_method").ok(),
                "deliveryDate": row.try_get::<Option<String>, _>("delivery_date").ok().flatten(),
                "availabilityStatus": row.try_get::<Option<String>, _>("availability_status").ok().flatten(),
                "rollingStocks": rolling_stocks,
            }));
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
                let purchase_date: Option<String> = row
                    .try_get::<Option<String>, _>("purchase_date")
                    .ok()
                    .flatten();
                let purchase_type: Option<String> = row
                    .try_get::<Option<String>, _>("purchase_type")
                    .ok()
                    .flatten();
                let has_purchase = purchase_date.is_some() || purchase_type.is_some();

                let purchase = if has_purchase {
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

                    json!({
                        "type": purchase_type,
                        "purchaseDate": purchase_date,
                        "sellerId": row.try_get::<Option<String>, _>("seller_id").ok().flatten(),
                        "price": price,
                        "saleDate": row.try_get::<Option<String>, _>("sale_date").ok().flatten(),
                        "expectedDelivery": row.try_get::<Option<String>, _>("expected_date").ok().flatten(),
                    })
                } else {
                    Value::Null
                };

                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "railwayModelId": row.try_get::<String, _>("railway_model_id").ok(),
                    "addedDate": row.try_get::<String, _>("added_date").ok(),
                    "removedDate": row.try_get::<Option<String>, _>("removed_date").ok().flatten(),
                    "purchaseCondition": row.try_get::<Option<String>, _>("purchase_condition").ok().flatten(),
                    "modelCondition": row.try_get::<Option<String>, _>("model_condition").ok().flatten(),
                    "boxCondition": row.try_get::<Option<String>, _>("box_condition").ok().flatten(),
                    "notes": row.try_get::<Option<String>, _>("notes").ok().flatten(),
                    "purchase": purchase,
                })
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
                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "name": row.try_get::<String, _>("name").ok(),
                    "sellerType": row.try_get::<String, _>("type").ok(),
                    "email": row.try_get::<Option<String>, _>("email").ok().flatten(),
                    "phone": row.try_get::<Option<String>, _>("phone").ok().flatten(),
                    "websiteUrl": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
                })
            })
            .collect();
        data["sellers"] = json!(sellers);
    }

    if selection.include_maintenance_logs {
        let rows = sqlx::query(
            "SELECT id, maintenance_card_id, date_performed, maintenance_type, notes \
             FROM maintenance_events ORDER BY id",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let logs: Vec<Value> = rows
            .iter()
            .map(|row| {
                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "maintenanceCardId": row.try_get::<String, _>("maintenance_card_id").ok(),
                    "date": row.try_get::<String, _>("date_performed").ok(),
                    "type": row.try_get::<Option<String>, _>("maintenance_type").ok().flatten(),
                    "description": row.try_get::<Option<String>, _>("notes").ok().flatten(),
                })
            })
            .collect();
        data["maintenanceCards"] = json!(logs);
    }

    // Build final manifest — "data" key matches ManifestDto.data in the import feature
    let manifest = json!({
        "version": "1.0",
        "exportedAt": chrono::Utc::now().to_rfc3339(),
        "source": "rusty-shed",
        "data": data
    });

    Ok(manifest)
}
