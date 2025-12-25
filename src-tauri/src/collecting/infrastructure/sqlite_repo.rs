use crate::catalog::domain::{
    Epoch,
    PowerMethod,
    ProductCode,
    RailwayCompany,
    Scale, // ServiceLevel, SubCategory removed
};
use crate::collecting::domain::collection::{
    Collection, CollectionItem, CollectionRepository, OwnedRollingStock, PurchaseInfo,
};
// use crate::db::DB_POOL; removed
use anyhow::{Context, Result};
use sqlx::{Row, SqlitePool};

pub struct SqliteCollectionRepository {
    pool: SqlitePool,
}

impl SqliteCollectionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CollectionRepository for SqliteCollectionRepository {
    async fn get_collection(&self) -> Result<Collection> {
        // For simplicity and matching the use case "get collection", we assume a single user collection for now
        // or getting the first one found. If none exists, we might need to return a default or error.
        // For this iteration, let's try to fetch the first collection.

        let collection_row = sqlx::query(
            r#"
            SELECT 
                id, name, locomotives_count, passenger_cars_count, freight_cars_count, 
                train_sets_count, railcars_count, electric_multiple_units_count, 
                total_value_amount, total_value_currency
            FROM collections
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch collection summary")?;

        let collection_rec = match collection_row {
            Some(row) => row,
            None => {
                // Return an empty collection structure if no DB entry exists yet?
                // Or maybe the seed data logic should enforce one.
                // Let's return a default empty collection for now if strictly needed,
                // or error if the use case expects data.
                // Given "read from sqlite", let's assume if it is empty we return empty object.
                return Ok(Collection {
                    id: "".to_string(),
                    name: "My Collection".to_string(),
                    locomotives_count: 0,
                    passenger_cars_count: 0,
                    freight_cars_count: 0,
                    train_sets_count: 0,
                    railcars_count: 0,
                    electric_multiple_units_count: 0,
                    total_value_amount: 0,
                    total_value_currency: "EUR".to_string(),
                    items: vec![],
                });
            }
        };

        let collection_id: String = collection_rec.get("id");

        // Fetch items
        let item_rows = sqlx::query(
            r#"
            SELECT id, manufacturer, product_code, description, power_method, scale, epoch, delivery_date
            FROM collection_items
            WHERE collection_id = ?
            "#,
        )
        .bind(&collection_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch collection items")?;

        let mut items = Vec::new();

        for row in item_rows {
            let item_id: String = row.get("id");
            let product_code_str: String = row.get("product_code");
            let power_method_str: Option<String> = row.get("power_method");
            let power_method = match power_method_str.as_deref() {
                Some("AC") => PowerMethod::AC,
                _ => PowerMethod::DC, // Default or handle error? Defaulting to DC for now
            };
            let scale_str: Option<String> = row.get("scale");
            let scale = match scale_str.as_deref() {
                Some("H0") => Scale::H0,
                Some("N") => Scale::N,
                // ... handle others as needed or map string
                _ => Scale::H0, // Default
            };

            // Fetch rolling stocks
            let rolling_stock_rows = sqlx::query(
                r#"
                SELECT id, road_number, type_name, series, railway_name, railway_registered_name, 
                       railway_country_code, category, sub_category, depot, length, livery, service_level
                FROM owned_rolling_stocks
                WHERE item_id = ?
                "#,
            )
            .bind(&item_id)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch rolling stocks")?;

            let rolling_stocks = rolling_stock_rows
                .into_iter()
                .map(|rs_row| {
                    // Only keep minimal view fields for OwnedRollingStock: id, rolling_stock_id, railway, epoch, description
                    OwnedRollingStock {
                        id: rs_row.get("id"),
                        rolling_stock_id: rs_row.get("id"),
                        description: rs_row.get("type_name"),
                        railway: RailwayCompany {
                            name: rs_row.get("railway_name"),
                            registered_company_name: rs_row.get("railway_registered_name"),
                            country_code: rs_row.get("railway_country_code"),
                        },
                        epoch: Epoch(row.get("epoch")),
                    }
                })
                .collect();

            // Fetch purchase info
            let purchase_info_row = sqlx::query(
                r#"
                SELECT id, date, price_amount, price_currency, seller
                FROM purchase_infos
                WHERE item_id = ?
                LIMIT 1
                "#,
            )
            .bind(&item_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch purchase info")?;

            let purchase_info = purchase_info_row.map(|pi_row| PurchaseInfo {
                id: pi_row.get("id"),
                item_id: item_id.clone(),
                date: pi_row.get("date"),
                price_amount: pi_row.get("price_amount"),
                price_currency: pi_row.get("price_currency"),
                seller: pi_row.get("seller"),
            });

            items.push(CollectionItem {
                id: item_id.clone(),
                railway_model_id: item_id.clone(),
                manufacturer: row.get("manufacturer"),
                product_code: ProductCode(product_code_str),
                description: row.get("description"),
                power_method,
                scale,
                epoch: Epoch(row.get("epoch")),
                rolling_stocks,
                purchase_info,
            });
        }

        Ok(Collection {
            id: collection_id,
            name: collection_rec.get("name"),
            locomotives_count: collection_rec.get("locomotives_count"),
            passenger_cars_count: collection_rec.get("passenger_cars_count"),
            freight_cars_count: collection_rec.get("freight_cars_count"),
            train_sets_count: collection_rec.get("train_sets_count"),
            railcars_count: collection_rec.get("railcars_count"),
            electric_multiple_units_count: collection_rec.get("electric_multiple_units_count"),
            total_value_amount: collection_rec.get("total_value_amount"),
            total_value_currency: collection_rec.get("total_value_currency"),
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::MIGRATOR;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_get_collection_empty() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory pool");

        MIGRATOR.run(&pool).await.expect("Failed to run migrations");

        // Seed empty collection ? Or assumes code handles it.
        // The current code fetches "LIMIT 1". If empty, it returns a default empty object.

        let repo = SqliteCollectionRepository::new(pool.clone());
        let collection = repo
            .get_collection()
            .await
            .expect("Failed to get collection");

        // As per current implementation logic: "return Ok(Collection { ... })" if not found
        assert_eq!(collection.name, "My Collection");
        assert_eq!(collection.items.len(), 0);
    }

    #[tokio::test]
    async fn test_get_collection_with_data() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory pool");

        MIGRATOR.run(&pool).await.expect("Failed to run migrations");

        // Seed data
        sqlx::query(
            r#"
            INSERT INTO collections (id, name, locomotives_count) VALUES ('col1', 'Test Collection', 1);
            "#
        ).execute(&pool).await.expect("Failed to seed collection");

        sqlx::query(
            r#"
            INSERT INTO collection_items (id, collection_id, manufacturer, product_code, description, power_method, scale, epoch, delivery_date) 
            VALUES ('item1', 'col1', 'ACME', '12345', 'Test Loc', 'DC', 'H0', 'IV', '2020');
            "#
        ).execute(&pool).await.expect("Failed to seed item");

        sqlx::query(
            r#"
            INSERT INTO owned_rolling_stocks (id, item_id, road_number, type_name, railway_name, category)
            VALUES ('rs1', 'item1', 'E656', 'Caimano', 'FS', 'LOCOMOTIVE');
            "#
        ).execute(&pool).await.expect("Failed to seed rolling stock");

        let repo = SqliteCollectionRepository::new(pool.clone());
        let collection = repo
            .get_collection()
            .await
            .expect("Failed to get collection");

        assert_eq!(collection.id, "col1");
        assert_eq!(collection.locomotives_count, 1);
        assert_eq!(collection.items.len(), 1);
        assert_eq!(collection.items[0].product_code.0, "12345");
        // Check power method enum mapping
        assert_eq!(collection.items[0].power_method, PowerMethod::DC);
        // Check scale enum mapping
        assert_eq!(collection.items[0].scale, Scale::H0);

        assert_eq!(collection.items[0].rolling_stocks.len(), 1);
        assert_eq!(collection.items[0].rolling_stocks[0].railway.name, "FS");
        assert_eq!(collection.items[0].rolling_stocks[0].rolling_stock_id, "rs1");
        assert_eq!(collection.items[0].rolling_stocks[0].epoch, Epoch("IV".to_string()));
    }
}
