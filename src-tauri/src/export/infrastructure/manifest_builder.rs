/// Archive manifest builder
use serde_json::{Value, json};
use sqlx::{Row, SqlitePool};

use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;

/// Build export manifest from selected entities
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
    let mut entities = json!({});

    // Query and include selected entity types
    if selection.include_railway_models {
        let rows = sqlx::query(
            "SELECT id, manufacturer_id, product_code, description, category, scale, power_method, epoch FROM railway_models ORDER BY id"
        )
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let models: Vec<Value> = rows
            .iter()
            .map(|row| {
                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "manufacturer_id": row.try_get::<String, _>("manufacturer_id").ok(),
                    "product_code": row.try_get::<String, _>("product_code").ok(),
                    "description": row.try_get::<String, _>("description").ok(),
                    "category": row.try_get::<String, _>("category").ok(),
                    "scale": row.try_get::<String, _>("scale").ok(),
                    "power_method": row.try_get::<String, _>("power_method").ok(),
                    "epoch": row.try_get::<String, _>("epoch").ok(),
                })
            })
            .collect();
        entities["railway_models"] = json!(models);
    }

    if selection.include_collection_items {
        let rows = sqlx::query(
            "SELECT id, collection_id, railway_model_id, added_date, removed_date, purchase_condition, model_condition FROM collection_items ORDER BY id"
        )
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let items: Vec<Value> = rows.iter().map(|row| {
            json!({
                "id": row.try_get::<String, _>("id").ok(),
                "collection_id": row.try_get::<String, _>("collection_id").ok(),
                "railway_model_id": row.try_get::<String, _>("railway_model_id").ok(),
                "added_date": row.try_get::<String, _>("added_date").ok(),
                "removed_date": row.try_get::<Option<String>, _>("removed_date").ok().flatten(),
                "purchase_condition": row.try_get::<Option<String>, _>("purchase_condition").ok().flatten(),
                "model_condition": row.try_get::<Option<String>, _>("model_condition").ok().flatten(),
            })
        }).collect();
        entities["collection_items"] = json!(items);
    }

    if selection.include_sellers {
        let rows = sqlx::query(
            "SELECT id, name, type, email, website_url, country_code FROM sellers ORDER BY id",
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
                    "type": row.try_get::<String, _>("type").ok(),
                    "email": row.try_get::<Option<String>, _>("email").ok().flatten(),
                    "website_url": row.try_get::<Option<String>, _>("website_url").ok().flatten(),
                    "country_code": row.try_get::<Option<String>, _>("country_code").ok().flatten(),
                })
            })
            .collect();
        entities["sellers"] = json!(sellers);
    }

    if selection.include_maintenance_logs {
        let rows = sqlx::query(
            "SELECT id, maintenance_card_id, date_performed, maintenance_type, notes FROM maintenance_events ORDER BY id"
        )
            .fetch_all(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let logs: Vec<Value> = rows.iter().map(|row| {
            json!({
                "id": row.try_get::<String, _>("id").ok(),
                "maintenance_card_id": row.try_get::<String, _>("maintenance_card_id").ok(),
                "date_performed": row.try_get::<String, _>("date_performed").ok(),
                "maintenance_type": row.try_get::<Option<String>, _>("maintenance_type").ok().flatten(),
                "notes": row.try_get::<Option<String>, _>("notes").ok().flatten(),
            })
        }).collect();
        entities["maintenance_events"] = json!(logs);
    }

    if selection.include_dcc_roster {
        let rows =
            sqlx::query("SELECT id, rolling_stock_id, series_code FROM rolling_stocks ORDER BY id")
                .fetch_all(pool)
                .await
                .map_err(|e| ExportError::DatabaseError(e.to_string()))?;

        let dcc: Vec<Value> = rows
            .iter()
            .map(|row| {
                json!({
                    "id": row.try_get::<String, _>("id").ok(),
                    "rolling_stock_id": row.try_get::<String, _>("rolling_stock_id").ok(),
                    "series_code": row.try_get::<String, _>("series_code").ok(),
                })
            })
            .collect();
        entities["rolling_stocks"] = json!(dcc);
    }

    // Build final manifest
    let manifest = json!({
        "version": "1.0",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "entities": entities
    });

    Ok(manifest)
}
