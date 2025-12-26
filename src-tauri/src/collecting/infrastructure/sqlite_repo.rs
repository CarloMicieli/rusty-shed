use crate::collecting::domain::collection::{
    Collection, CollectionItem, CollectionRepository, OwnedRollingStock, PurchaseInfo,
};
use crate::core::domain::MonetaryAmount;
use anyhow::{Context, Result, anyhow};
use chrono::{Local, NaiveDate};
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
            SELECT id, railway_model_id, conditions, notes
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

            // Fetch rolling stocks
            let rolling_stock_rows = sqlx::query(
                r#"
                SELECT id, rolling_stock_id, notes
                FROM owned_rolling_stocks
                WHERE collection_item_id = ?
                "#,
            )
            .bind(&item_id)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch rolling stocks")?;

            let rolling_stocks = rolling_stock_rows
                .into_iter()
                .map(|rs_row| {
                    #[cfg(test)]
                    println!(
                        "DEBUG: rs_id={} rs_notes={:?}",
                        rs_row.get::<String, _>("id"),
                        rs_row.get::<Option<String>, _>("notes")
                    );
                    OwnedRollingStock {
                        id: rs_row.get("id"),
                        rolling_stock_id: rs_row
                            .get::<Option<String>, _>("rolling_stock_id")
                            .unwrap_or_else(|| rs_row.get("id")),
                        notes: rs_row.get("notes"),
                    }
                })
                .collect();

            // Fetch purchase info from dedicated table
            let purchase_info_row = sqlx::query(
                r#"
                SELECT purchase_id, purchase_type, purchase_date, seller_id, buyer_id,
                       sale_date, purchased_price_amount, purchased_price_currency,
                       sale_price_amount, sale_price_currency, deposit_amount, deposit_currency,
                       preorder_total_amount, preorder_total_currency, expected_date
                FROM purchase_infos
                WHERE collection_item_id = ?
                LIMIT 1
                "#,
            )
            .bind(&item_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch purchase info")?;

            let purchase_info: Option<PurchaseInfo> = match purchase_info_row {
                None => None,
                Some(pi_row) => {
                    let ptype: Option<String> = pi_row.get("purchase_type");
                    // purchase_date is required in the table; fall back to today if parse fails
                    let pd_str: String = pi_row.get("purchase_date");
                    let purchase_date = NaiveDate::parse_from_str(&pd_str, "%Y-%m-%d")
                        .unwrap_or_else(|_| Local::now().naive_local().date());

                    match ptype.as_deref() {
                        Some("purchased") => {
                            let price_amount: i64 = pi_row.get("purchased_price_amount");
                            let price_currency: Option<String> =
                                pi_row.get("purchased_price_currency");
                            let price =
                                MonetaryAmount::from_db(price_amount, price_currency.as_deref())
                                    .map_err(|e| anyhow!(e.to_string()))
                                    .context("Failed to parse purchased price from DB")?;

                            Some(PurchaseInfo::Purchased(crate::collecting::domain::collection::purchase_info::PurchasedInfo {
                                id: pi_row.get("purchase_id"),
                                purchase_date,
                                price,
                                seller: pi_row.get::<Option<String>, _>("seller_id"),
                            }))
                        }
                        Some("sold") => {
                            let sale_date_str: Option<String> = pi_row.get("sale_date");
                            let sale_date = sale_date_str
                                .as_deref()
                                .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
                                .unwrap_or_else(|| Local::now().naive_local().date());

                            // original purchase price (optional)
                            let purchase_amount: i64 = pi_row.get("purchased_price_amount");
                            let purchase_currency: Option<String> =
                                pi_row.get("purchased_price_currency");
                            let purchase_price = MonetaryAmount::from_db(
                                purchase_amount,
                                purchase_currency.as_deref(),
                            )
                            .map_err(|e| anyhow!(e.to_string()))
                            .context("Failed to parse original purchase price from DB")?;

                            let sale_amount: i64 = pi_row.get("sale_price_amount");
                            let sale_currency: Option<String> = pi_row.get("sale_price_currency");
                            let sale_price =
                                MonetaryAmount::from_db(sale_amount, sale_currency.as_deref())
                                    .map_err(|e| anyhow!(e.to_string()))
                                    .context("Failed to parse sale price from DB")?
                                    .ok_or_else(|| anyhow!("Missing sale price for sold item"))?;

                            Some(PurchaseInfo::Sold(
                                crate::collecting::domain::collection::purchase_info::SoldInfo {
                                    id: pi_row.get("purchase_id"),
                                    purchase_date,
                                    purchase_price,
                                    sale_date,
                                    sale_price,
                                    buyer: pi_row.get::<Option<String>, _>("buyer_id"),
                                    seller: pi_row.get::<Option<String>, _>("seller_id"),
                                },
                            ))
                        }
                        Some("preorder") => {
                            let deposit_amount: i64 = pi_row.get("deposit_amount");
                            let deposit_currency: Option<String> = pi_row.get("deposit_currency");
                            let deposit = MonetaryAmount::from_db(
                                deposit_amount,
                                deposit_currency.as_deref(),
                            )
                            .map_err(|e| anyhow!(e.to_string()))
                            .context("Failed to parse deposit from DB")?
                            .ok_or_else(|| anyhow!("Missing deposit for preorder"))?;

                            let total_amount: i64 = pi_row.get("preorder_total_amount");
                            let total_currency: Option<String> =
                                pi_row.get("preorder_total_currency");
                            let total_price =
                                MonetaryAmount::from_db(total_amount, total_currency.as_deref())
                                    .map_err(|e| anyhow!(e.to_string()))
                                    .context("Failed to parse preorder total price from DB")?
                                    .ok_or_else(|| anyhow!("Missing preorder total price"))?;

                            // ensure currencies match
                            if deposit.currency != total_price.currency {
                                return Err(anyhow!(
                                    "Preorder deposit and total price have different currencies"
                                ));
                            }

                            let expected_date_str: Option<String> = pi_row.get("expected_date");
                            let expected_date = expected_date_str
                                .as_deref()
                                .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

                            Some(PurchaseInfo::PreOrdered(crate::collecting::domain::collection::purchase_info::PreOrderInfo {
                                id: pi_row.get("purchase_id"),
                                order_date: purchase_date,
                                deposit,
                                total_price,
                                seller: pi_row.get::<Option<String>, _>("seller_id"),
                                expected_date,
                            }))
                        }
                        _ => None,
                    }
                }
            };

            items.push(CollectionItem {
                id: item_id.clone(),
                railway_model_id: row.get("railway_model_id"),
                conditions: row.get::<Option<String>, _>("conditions"),
                notes: row.get::<Option<String>, _>("notes"),
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
    use crate::catalog::infrastructure::testing::CatalogTestDb;
    use crate::collecting::infrastructure::testing::CollectingTestDb;
    use crate::core::domain::currency::Currency;
    use pretty_assertions::assert_eq;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_collection_empty(pool: SqlitePool) {
        let repo = SqliteCollectionRepository::new(pool.clone());
        let collection = repo
            .get_collection()
            .await
            .expect("Failed to get collection");

        // As per current implementation logic: "return Ok(Collection { ... })" if not found
        assert_eq!(collection.name, "My Collection");
        assert_eq!(collection.items.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_collection_with_data(pool: SqlitePool) -> Result<()> {
        let catalog_db = CatalogTestDb::new(pool.clone());
        let catalog_test_data = catalog_db.setup_railway_model().await?;

        let collecting_db = CollectingTestDb::new(pool.clone());
        let railway_model_id = &catalog_test_data.railway_model_id;
        let rolling_stock_ids: Vec<&str> = catalog_test_data
            .rolling_stock_ids
            .iter()
            .map(|s| s.as_str()) // or .map(|s| &**s)
            .collect();
        let collection_test_data = collecting_db
            .setup_minimal_collection(railway_model_id, rolling_stock_ids.clone())
            .await?;
        let collection_id = collection_test_data.collection_id;

        let repo = SqliteCollectionRepository::new(pool.clone());
        let collection = repo
            .get_collection()
            .await
            .expect("Failed to get collection");

        assert_eq!(collection.id, collection_id);
        assert_eq!(collection.summary.locomotives_count, 0);
        assert_eq!(collection.summary.passenger_cars_count, 0);
        assert_eq!(collection.summary.freight_cars_count, 0);
        assert_eq!(collection.summary.train_sets_count, 0);
        assert_eq!(collection.summary.railcars_count, 0);
        assert_eq!(collection.summary.electric_multiple_units_count, 0);
        assert!(collection.total_value.is_some());
        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0].railway_model_id,
            railway_model_id.to_string()
        );

        assert_eq!(collection.items[0].rolling_stocks.len(), 1);
        assert_eq!(
            collection.items[0].rolling_stocks[0].rolling_stock_id,
            rolling_stock_ids[0].to_string()
        );

        assert!(collection.items[0].purchase_info.is_some());
        let purchase_info = collection.items[0].purchase_info.as_ref().unwrap();
        match purchase_info {
            PurchaseInfo::Purchased(purchased_info) => {
                assert_eq!(purchased_info.id, collection_test_data.purchase_info_id);
                let price = purchased_info.price.as_ref().expect("price present");
                assert_eq!(price.amount, 0);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(purchased_info.seller, None);
            }
            other => panic!("Expected purchase info to be Purchased, got: {:?}", other),
        }

        Ok(())
    }
}
