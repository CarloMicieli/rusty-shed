use crate::collecting::domain::collection::Collection;
use crate::collecting::domain::collection_id::CollectionId;
use crate::collecting::domain::collection_item::CollectionItem;
use crate::collecting::domain::collection_item_id::CollectionItemId;
use crate::collecting::domain::owned_rolling_stock::OwnedRollingStock;
use crate::collecting::domain::purchase_info::PurchaseInfo;
use crate::collecting::domain::summary::CollectionSummary;
use crate::collecting::infrastructure::entities::{
    CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
};
use crate::core::domain::MonetaryAmount;
use anyhow::{Context, anyhow};
use std::collections::HashMap;

/// Converts infrastructure row types into collecting domain types.
///
/// `CollectionMapper` is a zero-sized helper type that groups mapping-related
/// functions.
/// 
/// The mapping functions are intentionally pure (no DB access) and return
/// `anyhow::Result` to allow the caller to surface parsing/validation errors
/// (for example when UUIDs or monetary amounts are malformed).
#[derive(Debug)]
pub struct CollectionMapper;

impl CollectionMapper {
    /// Build a domain `Collection` from a `CollectionRow` and a list of
    /// already-mapped `CollectionItem` values.
    ///
    /// Parameters
    /// - `row`: Row-level representation of a collection read from the DB.
    /// - `items`: Pre-mapped `CollectionItem` values that belong to this
    ///   collection.
    ///
    /// Returns a `Collection` on success.
    ///
    /// Errors
    /// - When the `CollectionId` cannot be parsed from the row's `id` field.
    /// - When the `total_value` stored in the row cannot be converted into a
    ///   domain `MonetaryAmount` (for example if the currency code is
    ///   unsupported or the amount is negative).
    pub fn row_to_collection(
        row: CollectionRow,
        items: Vec<CollectionItem>,
    ) -> anyhow::Result<Collection> {
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

    /// Build a `CollectionItem` domain value from a `CollectionItemRow` and
    /// lookup maps for owned rolling stocks and purchase infos.
    ///
    /// Parameters
    /// - `row`: The DB row representing a collection item.
    /// - `owned_rolling_stocks_map`: Map from `CollectionItemId` to the rows
    ///   of `owned_rolling_stocks` related to that item. If the item has no
    ///   owned rolling stocks, an empty list will be used.
    /// - `purchase_info_map`: Map from `CollectionItemId` to the rows of
    ///   `purchase_infos` related to that item. When multiple purchase info
    ///   rows are present only the first is considered (matching existing
    ///   repository behaviour).
    ///
    /// Returns the mapped `CollectionItem` or an error when the item's id
    /// cannot be parsed.
    pub fn row_to_collection_item(
        row: CollectionItemRow,
        owned_rolling_stocks_map: &HashMap<CollectionItemId, Vec<OwnedRollingStockRow>>,
        purchase_info_map: &HashMap<CollectionItemId, Vec<PurchaseInfoRow>>,
    ) -> anyhow::Result<CollectionItem> {
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
                .and_then(|pi_row| Self::row_to_purchase_info(pi_row).ok()),
        })
    }

    /// Convert a single `PurchaseInfoRow` into the domain `PurchaseInfo`
    /// enum.
    ///
    /// The function recognises the purchase types: `purchased`, `sold` and
    /// `preorder`. For each supported type the corresponding domain variant
    /// is returned. When optional numeric/currency fields are absent the
    /// function uses the domain helpers (`MonetaryAmount::from_db`) which may
    /// return `None` for empty currency or return an error if values are
    /// invalid.
    ///
    /// Errors
    /// - If the `purchase_type` is not recognised an error is returned.
    /// - If currency/amount conversions fail, the underlying `MonetaryAmount`
    ///   parsing error is propagated.
    fn row_to_purchase_info(pi_row: &PurchaseInfoRow) -> anyhow::Result<PurchaseInfo> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use chrono::NaiveDate;
    use crate::collecting::domain::collection_id::CollectionId;
    use crate::collecting::domain::collection_item_id::CollectionItemId;
    use crate::collecting::infrastructure::entities::{
        CollectionRow, CollectionItemRow, OwnedRollingStockRow, PurchaseInfoRow,
    };
    use std::collections::HashMap;
    use crate::core::domain::currency::Currency;

    #[test]
    fn it_should_map_collection() {
        let collection_id = CollectionId::default().to_string();
        let collection_row = CollectionRow {
            id: collection_id.clone(),
            name: "My Test Collection".to_string(),
            locomotives_count: 2,
            passenger_cars_count: 3,
            freight_cars_count: 4,
            train_sets_count: 0,
            railcars_count: 1,
            electric_multiple_units_count: 0,
            total_value_amount: 12345,
            total_value_currency: "EUR".to_string(),
            created_at: NaiveDate::from_ymd_opt(2025, 12, 26).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            updated_at: NaiveDate::from_ymd_opt(2025, 12, 27).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        };

        let mapped = CollectionMapper::row_to_collection(collection_row, vec![]).expect("mapping should succeed");

        assert_eq!(mapped.id.to_string(), collection_id);
        assert_eq!(mapped.name, "My Test Collection");
        assert_eq!(mapped.summary.locomotives_count, 2);
        assert_eq!(mapped.summary.passenger_cars_count, 3);
        assert_eq!(mapped.summary.freight_cars_count, 4);
        assert_eq!(mapped.summary.train_sets_count, 0);
        assert_eq!(mapped.summary.railcars_count, 1);
        assert_eq!(mapped.summary.electric_multiple_units_count, 0);
        let total_value = mapped.total_value.expect("total value present");
        assert_eq!(total_value.amount, 12345u64);
        assert_eq!(total_value.currency, Currency::EUR);
        assert!(mapped.items.is_empty());
    }

    #[test]
    fn it_should_map_collection_item_with_owned_and_purchase_info() {
        let item_id_str = "d20a1a95-1ae4-4970-9e87-b4c84676e730".to_string();
        let collection_item = CollectionItemRow {
            id: item_id_str.clone(),
            collection_id: CollectionId::default().to_string(),
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            conditions: Some("new".to_string()),
            notes: Some("My notes go here".to_string()),
        };
        
        let owned_rolling_stock = OwnedRollingStockRow {
            id: "d3606635-4c4e-462b-ae9f-02c7ce47bc770".to_string(),
            collection_item_id: item_id_str.clone(),
            rolling_stock_id: Some("rs-001".to_string()),
            notes: Some("My rolling stock notes go here".to_string()),
        };
        
        let purchase_info = PurchaseInfoRow {
            purchase_id: "59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string(),
            collection_item_id: item_id_str.clone(),
            purchase_type: Some("purchased".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
            seller_id: Some("shop-1".to_string()),
            buyer_id: None,
            sale_date: None,
            purchased_price_amount: Some(17500),
            purchased_price_currency: Some("EUR".to_string()),
            sale_price_amount: None,
            sale_price_currency: None,
            deposit_amount: None,
            deposit_currency: None,
            preorder_total_amount: None,
            preorder_total_currency: None,
            expected_date: None,
        };

        let collection_item_id = CollectionItemId::try_from(&item_id_str).expect("valid uuid");
        let mut owned_rolling_stocks_map: HashMap<CollectionItemId, Vec<OwnedRollingStockRow>> = HashMap::new();
        owned_rolling_stocks_map.insert(collection_item_id.clone(), vec![owned_rolling_stock]);

        let mut purchase_infos_map: HashMap<CollectionItemId, Vec<PurchaseInfoRow>> = HashMap::new();
        purchase_infos_map.insert(collection_item_id.clone(), vec![purchase_info]);

        let mapped_item = CollectionMapper::row_to_collection_item(collection_item, &owned_rolling_stocks_map, &purchase_infos_map)
            .expect("mapping item should succeed");

        assert_eq!(mapped_item.id.to_string(), item_id_str);
        assert_eq!(mapped_item.railway_model_id, "trn:railway-model:acme:60100");
        assert_eq!(mapped_item.conditions, Some("new".to_string()));
        assert_eq!(mapped_item.notes, Some("My notes go here".to_string()));

        assert_eq!(mapped_item.rolling_stocks.len(), 1);
        let ors = &mapped_item.rolling_stocks[0];
        assert_eq!(ors.id, "d3606635-4c4e-462b-ae9f-02c7ce47bc770".to_string());
        assert_eq!(ors.rolling_stock_id, "rs-001".to_string());
        assert_eq!(ors.notes, "My rolling stock notes go here".to_string());

        let pi = mapped_item.purchase_info.expect("purchase info present");
        match pi {
            PurchaseInfo::Purchased(p) => {
                assert_eq!(p.id, "59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string());
                assert_eq!(p.purchase_date.to_string(), "2025-12-26");
                let price = p.price.expect("price present");
                assert_eq!(price.amount, 17500u64);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(p.seller, Some("shop-1".to_string()));
            }
            _ => panic!("expected Purchased variant"),
        }
    }

    #[test]
    fn it_should_map_row_to_purchase_info_purchased() {
        let pi_row = PurchaseInfoRow {
            purchase_id: "59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string(),
            collection_item_id: "d20a1a95-1ae4-4970-9e87-b4c84676e730".to_string(),
            purchase_type: Some("purchased".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
            seller_id: Some("shop-1".to_string()),
            buyer_id: None,
            sale_date: None,
            purchased_price_amount: Some(17500),
            purchased_price_currency: Some("EUR".to_string()),
            sale_price_amount: None,
            sale_price_currency: None,
            deposit_amount: None,
            deposit_currency: None,
            preorder_total_amount: None,
            preorder_total_currency: None,
            expected_date: None,
        };

        let pi = CollectionMapper::row_to_purchase_info(&pi_row).expect("mapping purchase info");
        match pi {
            PurchaseInfo::Purchased(p) => {
                assert_eq!(p.id, "59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string());
                assert_eq!(p.purchase_date.to_string(), "2025-12-26");
                let price = p.price.expect("price present");
                assert_eq!(price.amount, 17500u64);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(p.seller, Some("shop-1".to_string()));
            }
            _ => panic!("expected Purchased variant"),
        }
    }

    #[test]
    fn it_should_map_row_to_purchase_info_sold() {
        let pi_row = PurchaseInfoRow {
            purchase_id: "sold-purchase-0000-0000-0000-000000000000".to_string(),
            collection_item_id: "00000000-0000-0000-0000-000000000001".to_string(),
            purchase_type: Some("sold".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2024, 5, 10).unwrap(),
            seller_id: Some("original-seller".to_string()),
            buyer_id: Some("buyer-1".to_string()),
            sale_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            purchased_price_amount: Some(20000),
            purchased_price_currency: Some("EUR".to_string()),
            sale_price_amount: Some(25000),
            sale_price_currency: Some("EUR".to_string()),
            deposit_amount: None,
            deposit_currency: None,
            preorder_total_amount: None,
            preorder_total_currency: None,
            expected_date: None,
        };

        let pi = CollectionMapper::row_to_purchase_info(&pi_row).expect("mapping purchase info");
        match pi {
            PurchaseInfo::Sold(s) => {
                assert_eq!(s.id, "sold-purchase-0000-0000-0000-000000000000".to_string());
                assert_eq!(s.purchase_date.to_string(), "2024-05-10");
                let purchase_price = s.purchase_price.expect("purchase price present");
                assert_eq!(purchase_price.amount, 20000u64);
                assert_eq!(purchase_price.currency, Currency::EUR);
                assert_eq!(s.sale_price.amount, 25000u64);
                assert_eq!(s.sale_price.currency, Currency::EUR);
                assert_eq!(s.sale_date.to_string(), "2025-01-01");
                assert_eq!(s.buyer, Some("buyer-1".to_string()));
                assert_eq!(s.seller, Some("original-seller".to_string()));
            }
            _ => panic!("expected Sold variant"),
        }
    }

    #[test]
    fn it_should_map_row_to_purchase_info_preorder() {
        let pi_row = PurchaseInfoRow {
            purchase_id: "preorder-purchase-0000-0000-0000-000000000000".to_string(),
            collection_item_id: "00000000-0000-0000-0000-000000000002".to_string(),
            purchase_type: Some("preorder".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            seller_id: Some("preorder-shop".to_string()),
            buyer_id: None,
            sale_date: None,
            purchased_price_amount: None,
            purchased_price_currency: None,
            sale_price_amount: None,
            sale_price_currency: None,
            deposit_amount: Some(500),
            deposit_currency: Some("EUR".to_string()),
            preorder_total_amount: Some(1000),
            preorder_total_currency: Some("EUR".to_string()),
            expected_date: Some(NaiveDate::from_ymd_opt(2025, 12, 1).unwrap()),
        };

        let pi = CollectionMapper::row_to_purchase_info(&pi_row).expect("mapping purchase info");
        match pi {
            PurchaseInfo::PreOrdered(po) => {
                assert_eq!(po.id, "preorder-purchase-0000-0000-0000-000000000000".to_string());
                assert_eq!(po.order_date.to_string(), "2025-06-01");
                assert_eq!(po.deposit.amount, 500u64);
                assert_eq!(po.deposit.currency, Currency::EUR);
                assert_eq!(po.total_price.amount, 1000u64);
                assert_eq!(po.total_price.currency, Currency::EUR);
                assert_eq!(po.seller, Some("preorder-shop".to_string()));
                assert_eq!(po.expected_date.map(|d| d.to_string()), Some("2025-12-01".to_string()));
            }
            _ => panic!("expected PreOrdered variant"),
        }
    }
}
