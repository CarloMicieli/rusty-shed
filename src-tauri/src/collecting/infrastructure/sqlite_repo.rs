use crate::collecting::domain::collection::Collection;
use crate::collecting::domain::collection_id::CollectionId;
use crate::collecting::domain::collection_item::CollectionItem;
use crate::collecting::domain::collection_item_id::CollectionItemId;
use crate::collecting::domain::owned_rolling_stock::OwnedRollingStock;
use crate::collecting::domain::purchase_info::PurchaseInfo;
use crate::collecting::domain::repository::CollectionRepository;
use crate::collecting::domain::summary::CollectionSummary;
use crate::collecting::infrastructure::entities::{
    CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
};
use crate::collecting::infrastructure::sqlite;
use crate::core::domain::MonetaryAmount;
use anyhow::{Context, Result, anyhow};
use itertools::Itertools;
use sqlx::SqlitePool;
use std::collections::HashMap;

pub struct SqliteCollectionRepository {
    pool: SqlitePool,
}

impl SqliteCollectionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl SqliteCollectionRepository {
    // Helper to build Collection from CollectionRow and items
    fn build_collection(row: CollectionRow, items: Vec<CollectionItem>) -> Result<Collection> {
        let collection_id = CollectionId::try_from(row.id).map_err(|e| anyhow!(e))?;

        Ok(Collection {
            id: collection_id,
            name: row.name,
            summary: CollectionSummary {
                locomotives_count: row.locomotives_count as u16,
                passenger_cars_count: row.passenger_cars_count as u16,
                freight_cars_count: row.freight_cars_count as u16,
                train_sets_count: row.train_sets_count as u16,
                railcars_count: row.railcars_count as u16,
                electric_multiple_units_count: row.electric_multiple_units_count as u16,
            },
            total_value: MonetaryAmount::from_db(
                row.total_value_amount,
                Some(&row.total_value_currency),
            )
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to parse collection total value from DB")?,
            items,
        })
    }

    fn build_collection_item(
        row: CollectionItemRow,
        owned_rolling_stocks_map: &HashMap<CollectionItemId, Vec<OwnedRollingStockRow>>,
        purchase_info_map: &HashMap<CollectionItemId, Vec<PurchaseInfoRow>>,
    ) -> Result<CollectionItem> {
        let collection_item_id = CollectionItemId::try_from(&row.id).map_err(|e| anyhow!(e))?;

        let owned_rolling_stocks = owned_rolling_stocks_map
            .get(&collection_item_id)
            .map(|owned_rs_list| {
                owned_rs_list
                    .iter()
                    .map(|rs_row| OwnedRollingStock {
                        id: rs_row.id.clone(),
                        rolling_stock_id: rs_row
                            .rolling_stock_id
                            .clone()
                            .unwrap_or_else(|| rs_row.id.clone()),
                        notes: rs_row.notes.clone().unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(CollectionItem {
            id: collection_item_id.clone(),
            railway_model_id: row.railway_model_id,
            conditions: row.conditions.clone(),
            notes: row.notes.clone(),
            rolling_stocks: owned_rolling_stocks,
            purchase_info: purchase_info_map
                .get(&collection_item_id)
                .and_then(|pi_list| pi_list.first())
                .and_then(|pi_row| Self::build_purchase_info(pi_row).ok()),
        })
    }

    fn build_purchase_info(pi_row: &PurchaseInfoRow) -> Result<PurchaseInfo> {
        let purchase_type = pi_row.purchase_type.as_deref();
        let purchase_date = pi_row.purchase_date;
        match purchase_type {
            Some("purchased") => {
                let price = MonetaryAmount::from_db(
                    pi_row.purchased_price_amount.unwrap_or(0),
                    pi_row.purchased_price_currency.as_deref(),
                )?;
                Ok(PurchaseInfo::Purchased(
                    crate::collecting::domain::purchase_info::PurchasedInfo {
                        id: pi_row.purchase_id.clone(),
                        purchase_date,
                        price,
                        seller: pi_row.seller_id.clone(),
                    },
                ))
            }
            Some("sold") => {
                let purchase_price = MonetaryAmount::from_db(
                    pi_row.purchased_price_amount.unwrap_or(0),
                    pi_row.purchased_price_currency.as_deref(),
                )?;
                let sale_price = MonetaryAmount::from_db(
                    pi_row.sale_price_amount.unwrap_or(0),
                    pi_row.sale_price_currency.as_deref(),
                )?;
                Ok(PurchaseInfo::Sold(
                    crate::collecting::domain::purchase_info::SoldInfo {
                        id: pi_row.purchase_id.clone(),
                        purchase_date,
                        purchase_price,
                        sale_date: pi_row.sale_date.unwrap_or(purchase_date),
                        sale_price: sale_price.unwrap_or_default(),
                        buyer: pi_row.buyer_id.clone(),
                        seller: pi_row.seller_id.clone(),
                    },
                ))
            }
            Some("preorder") => {
                let deposit = MonetaryAmount::from_db(
                    pi_row.deposit_amount.unwrap_or(0),
                    pi_row.deposit_currency.as_deref(),
                )?;
                let total_price = MonetaryAmount::from_db(
                    pi_row.preorder_total_amount.unwrap_or(0),
                    pi_row.preorder_total_currency.as_deref(),
                )?;
                Ok(PurchaseInfo::PreOrdered(
                    crate::collecting::domain::purchase_info::PreOrderInfo {
                        id: pi_row.purchase_id.clone(),
                        order_date: purchase_date,
                        deposit: deposit.unwrap_or_default(),
                        total_price: total_price.unwrap_or_default(),
                        seller: pi_row.seller_id.clone(),
                        expected_date: pi_row.expected_date,
                    },
                ))
            }
            _ => Err(anyhow!("Invalid purchase type")),
        }
    }
}

#[async_trait::async_trait]
impl CollectionRepository for SqliteCollectionRepository {
    async fn get_collection(&self) -> Result<Collection> {
        // For simplicity and matching the use case "get collection", we assume a single user collection for now
        // or getting the first one found. If none exists, we might need to return a default or error.
        // For this iteration, let's try to fetch the first collection.
        let collection_id = CollectionId::default();

        let collection_row = sqlite::get_collection(&self.pool, collection_id).await?;
        if collection_row.is_none() {
            // Return an empty collection structure if no DB entry exists yet
            return Ok(Collection::default());
        }

        let collection_row =
            collection_row.expect("Expect collection row to be present after None check");
        let collection_id = CollectionId::try_from(&collection_row.id).map_err(|e| anyhow!(e))?;
        let collection_item_rows = sqlite::get_collection_items(&self.pool, &collection_id).await?;

        let owned_rolling_stock_rows =
            sqlite::get_owned_rolling_stocks(&self.pool, &collection_id).await?;
        let owned_rolling_stocks_map = owned_rolling_stock_rows
            .into_iter()
            .map(|owned_rs| {
                (
                    CollectionItemId::try_from(&owned_rs.collection_item_id).unwrap(),
                    owned_rs,
                )
            })
            .into_group_map();

        let purchase_info_rows = sqlite::get_purchase_infos(&self.pool, &collection_id).await?;
        let purchase_info_map = purchase_info_rows
            .into_iter()
            .map(|purchase_info| {
                (
                    CollectionItemId::try_from(&purchase_info.collection_item_id).unwrap(),
                    purchase_info,
                )
            })
            .into_group_map();

        let mut collection_items = Vec::new();
        for collection_item_row in collection_item_rows {
            let item = Self::build_collection_item(
                collection_item_row,
                &owned_rolling_stocks_map,
                &purchase_info_map,
            )?;
            collection_items.push(item);
        }

        Self::build_collection(collection_row, collection_items)
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

    // TODO: Enable this test after fixing the issues with test data setup
    #[ignore]
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

        assert_eq!(collection.id.to_string(), collection_id);
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
