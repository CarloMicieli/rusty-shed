use crate::catalog::domain::{Epoch, PowerMethod, ProductCode, Scale};
use crate::collecting::domain::collection::{
    Collection, CollectionItem, CollectionRepository, OwnedRollingStock, PurchaseInfo,
};
use crate::core::domain::MonetaryAmount;
use anyhow::{Context, Result, anyhow};
use chrono::{Local, NaiveDate};
use sqlx::{Row, SqlitePool};
use std::convert::TryFrom;

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
                    summary: crate::collecting::domain::collection::CollectionSummary::default(),
                    total_value: None,
                    items: vec![],
                });
            }
        };

        let collection_id: String = collection_rec.get("id");

        // Fetch items
        let item_rows = sqlx::query(
            r#"
            SELECT id, manufacturer, product_code, description, power_method, scale, epoch
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
            // Parse power method using TryFrom; fall back to DC when missing/invalid
            let power_method = power_method_str
                .as_deref()
                .and_then(|s| PowerMethod::try_from(s).ok())
                .unwrap_or(PowerMethod::DC);
            let scale_str: Option<String> = row.get("scale");
            // Parse scale using TryFrom; fall back to H0 on missing/invalid
            let scale = scale_str
                .as_deref()
                .and_then(|s| Scale::try_from(s).ok())
                .unwrap_or(Scale::H0);

            // Fetch rolling stocks
            let rolling_stock_rows = sqlx::query(
                r#"
                SELECT id, notes, railway_id
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
                    // Only keep minimal view fields for OwnedRollingStock: id, rolling_stock_id, railway, epoch, notes
                    OwnedRollingStock {
                        id: rs_row.get("id"),
                        rolling_stock_id: rs_row.get("id"),
                        notes: rs_row.get("notes"),
                        railway_id: rs_row.get("railway_id"),
                        epoch: Epoch(row.get("epoch")),
                    }
                })
                .collect();

            // Fetch purchase info
            let purchase_info_row = sqlx::query(
                r#"
                SELECT id, purchase_date, price_amount, price_currency, seller
                FROM purchase_infos
                WHERE item_id = ?
                LIMIT 1
                "#,
            )
            .bind(&item_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch purchase info")?;

            let purchase_info = match purchase_info_row {
                Some(pi_row) => {
                    let pd_str: String = pi_row.get("purchase_date");
                    let purchase_date = NaiveDate::parse_from_str(&pd_str, "%Y-%m-%d")
                        .unwrap_or_else(|_| Local::now().naive_local().date());

                    // Build optional MonetaryAmount from DB parts: price_amount (i64) and price_currency (nullable TEXT)
                    let price_amount: i64 = pi_row.get("price_amount");
                    let price_currency: Option<String> = pi_row.get("price_currency");
                    let price = MonetaryAmount::from_db(price_amount, price_currency.as_deref())
                        .map_err(|e| anyhow!(e.to_string()))
                        .context("Failed to parse purchase price from DB")?;

                    Some(PurchaseInfo {
                        id: pi_row.get("id"),
                        item_id: item_id.clone(),
                        purchase_date,
                        price,
                        seller: pi_row.get("seller"),
                    })
                }
                None => None,
            };

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
            summary: crate::collecting::domain::collection::CollectionSummary {
                locomotives_count: collection_rec.get("locomotives_count"),
                passenger_cars_count: collection_rec.get("passenger_cars_count"),
                freight_cars_count: collection_rec.get("freight_cars_count"),
                train_sets_count: collection_rec.get("train_sets_count"),
                railcars_count: collection_rec.get("railcars_count"),
                electric_multiple_units_count: collection_rec.get("electric_multiple_units_count"),
            },
            total_value: MonetaryAmount::from_db(
                collection_rec.get("total_value_amount"),
                collection_rec
                    .get::<Option<String>, _>("total_value_currency")
                    .as_deref(),
            )
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to parse collection total value from DB")?,
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
            INSERT INTO collection_items (id, collection_id, manufacturer, product_code, description, power_method, scale, epoch, railway_model_id) 
            VALUES ('item1', 'col1', 'ACME', '12345', 'Test Loc', 'DC', 'H0', 'IV', 'item1');
            "#
        ).execute(&pool).await.expect("Failed to seed item");

        sqlx::query(
            r#"
            INSERT INTO owned_rolling_stocks (id, item_id, notes, railway_id) 
            VALUES ('rs1', 'item1', 'Caimano', 'FS');
            "#,
        )
        .execute(&pool)
        .await
        .expect("Failed to seed rolling stock");

        let repo = SqliteCollectionRepository::new(pool.clone());
        let collection = repo
            .get_collection()
            .await
            .expect("Failed to get collection");

        assert_eq!(collection.id, "col1");
        assert_eq!(collection.summary.locomotives_count, 1u16);
        assert_eq!(collection.items.len(), 1);
        assert_eq!(collection.items[0].product_code.0, "12345");
        // Check power method enum mapping
        assert_eq!(collection.items[0].power_method, PowerMethod::DC);
        // Check scale enum mapping
        assert_eq!(collection.items[0].scale, Scale::H0);

        assert_eq!(collection.items[0].rolling_stocks.len(), 1);
        assert_eq!(collection.items[0].rolling_stocks[0].railway_id, "FS");
        assert_eq!(
            collection.items[0].rolling_stocks[0].rolling_stock_id,
            "rs1"
        );
        assert_eq!(
            collection.items[0].rolling_stocks[0].epoch,
            Epoch("IV".to_string())
        );
    }
}
