use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::CollectionRailwayModel;
use crate::collecting::domain::CollectionSummary;
use crate::collecting::domain::DigitalSetup;
use crate::collecting::domain::PurchaseInfo;
use crate::collecting::domain::{CollectionItemView, CollectionView, OwnedRollingStockView};
use crate::collecting::infrastructure::entities::{
    CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
};
use crate::core::domain::MonetaryAmount;
use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::domain::DecoderId;
use anyhow::anyhow;
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
        items: Vec<CollectionItemView>,
    ) -> Result<CollectionView, DomainError> {
        let total_value =
            MonetaryAmount::from_db(row.total_value_amount, Some(&row.total_value_currency))
                .map_err(|err| DomainError::Validation(err.to_string()))?;

        let summary = CollectionSummary {
            locomotives_count: row.locomotives_count as u16,
            passenger_cars_count: row.passenger_cars_count as u16,
            freight_cars_count: row.freight_cars_count as u16,
            train_sets_count: row.train_sets_count as u16,
            railcars_count: row.railcars_count as u16,
            electric_multiple_units_count: row.electric_multiple_units_count as u16,
        };

        Ok(CollectionView {
            id: row.id,
            name: row.name,
            summary,
            total_value,
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
    ///   repository behavior).
    ///
    /// Returns the mapped `CollectionItem` or an error when the item's id
    /// cannot be parsed.
    pub fn row_to_collection_item(
        row: CollectionItemRow,
        owned_rolling_stocks_map: &HashMap<CollectionItemId, Vec<OwnedRollingStockRow>>,
        purchase_info_map: &HashMap<CollectionItemId, Vec<PurchaseInfoRow>>,
    ) -> Result<CollectionItemView, DomainError> {
        let collection_item_id = row.id;

        let owned_rolling_stocks = owned_rolling_stocks_map
            .get(&collection_item_id)
            .map(|owned_rs_list| {
                owned_rs_list
                    .iter()
                    .map(|rs_row| {
                        // Basic fields
                        let mut ors = OwnedRollingStockView {
                            id: rs_row.id.clone(),
                            rolling_stock_id: rs_row.rolling_stock_id.clone().unwrap(),
                            notes: rs_row.notes.clone(),
                            digital: None,
                        };

                        // If a decoder is installed (installed_decoder_id present), try to build DigitalSetup
                        if let Some(installed_id) = &rs_row.installed_decoder_id {
                            // Parse dcc_address if present
                            if let Some(addr_i64) = rs_row.dcc_address {
                                let addr_u16 = addr_i64 as u16;

                                // decoder_interface must be present in the joined columns
                                if let Some(interface) = &rs_row.decoder_interface {
                                    ors.digital = Some(DigitalSetup {
                                        // `DccInterface` implements `Copy` so avoid cloning
                                        interface: *interface,
                                        dcc_address: addr_u16,
                                        installed_decoder_id: installed_id.parse().unwrap_or_else(
                                            |_| DecoderId::new(installed_id.clone()),
                                        ),
                                    });
                                }
                            }
                        }

                        ors
                    })
                    .collect()
            })
            .unwrap_or_default();

        let purchase_info = purchase_info_map
            .get(&collection_item_id)
            .and_then(|pi_list| pi_list.first())
            .and_then(|pi_row| Self::row_to_purchase_info(pi_row).ok());

        let railway_model = CollectionRailwayModel {
            category: row.category,
            scale: row.scale,
            epoch: row.epoch,
            description: row.description,
            product_code: row.product_code,
            manufacturer: row.manufacturer,
            railway_model_id: row.railway_model_id,
        };

        Ok(CollectionItemView {
            id: collection_item_id,
            railway_model,
            purchase_condition: row.purchase_condition,
            model_condition: row.model_condition,
            box_condition: row.box_condition,
            added_date: row.added_date,
            removed_date: row.removed_date,
            notes: row.notes,
            rolling_stocks: owned_rolling_stocks,
            purchase_info,
        })
    }

    /// Convert a single `PurchaseInfoRow` into the domain `PurchaseInfo`
    /// enum.
    ///
    /// The function recognizes the purchase types: `purchased`, `sold` and
    /// `preorder`. For each supported type the corresponding domain variant
    /// is returned. When optional numeric/currency fields are absent the
    /// function uses the domain helpers (`MonetaryAmount::from_db`) which may
    /// return `None` for empty currency or return an error if values are
    /// invalid.
    ///
    /// Errors
    /// - If the `purchase_type` is not recognized an error is returned.
    /// - If currency/amount conversions fail, the underlying `MonetaryAmount`
    ///   parsing error is propagated.
    fn row_to_purchase_info(pi_row: &PurchaseInfoRow) -> anyhow::Result<PurchaseInfo> {
        // Normalize purchase_type to lowercase for case-insensitive matching.
        // Keep the owned lowercase String in scope so `as_deref()` yields a
        // stable reference for pattern matching.
        let purchase_type_lower: Option<String> =
            pi_row.purchase_type.clone().map(|s| s.to_ascii_lowercase());
        let purchase_type = purchase_type_lower.as_deref();
        let purchase_date = pi_row.purchase_date;
        // debug: log purchase row fields to understand failures in mapping from fixtures
        eprintln!(
            "DEBUG: pi_row.id={}, purchase_type={:?}, purchased_currency={:?}",
            pi_row.id, pi_row.purchase_type, pi_row.purchased_price_currency
        );
        match purchase_type {
            Some("purchased") => {
                let price = MonetaryAmount::from_db(
                    pi_row.purchased_price_amount.unwrap_or(0),
                    pi_row.purchased_price_currency.as_deref(),
                )?;
                Ok(PurchaseInfo::Purchased(
                    crate::collecting::domain::PurchasedInfo {
                        id: pi_row.id.clone(),
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
                Ok(PurchaseInfo::Sold(crate::collecting::domain::SoldInfo {
                    id: pi_row.id.clone(),
                    purchase_date,
                    purchase_price,
                    sale_date: pi_row.sale_date.unwrap_or(purchase_date),
                    sale_price: sale_price.unwrap_or_default(),
                    buyer: pi_row.buyer_id.clone(),
                    seller: pi_row.seller_id.clone(),
                }))
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
                    crate::collecting::domain::PreOrderInfo {
                        id: pi_row.id.clone(),
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
    use crate::catalog::domain::railway_model::{Category, RailwayModelId, RollingStockId};
    use crate::catalog::domain::scale::Scale;
    use crate::collecting::domain::CollectionItemId;
    use crate::collecting::domain::OwnedRollingStockId;
    use crate::collecting::domain::PurchaseInfoId;
    use crate::collecting::domain::{
        BoxCondition, CollectionId, ModelCondition, PurchaseCondition,
    };
    use crate::collecting::infrastructure::entities::{
        CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
    };
    use crate::core::domain::currency::Currency;
    use crate::sellers::domain::seller_id::SellerId;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn it_should_map_collection() {
        let collection_id = CollectionId::default();
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
            created_at: NaiveDate::from_ymd_opt(2025, 12, 26)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            updated_at: NaiveDate::from_ymd_opt(2025, 12, 27)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        };

        let mapped = CollectionMapper::row_to_collection(collection_row, vec![])
            .expect("mapping should succeed");

        assert_eq!(mapped.id, collection_id);
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
        let item_id = CollectionItemId::default();
        let railway_model_id = RailwayModelId::try_from("trn:railway-model:acme:60100").unwrap();
        let collection_item = CollectionItemRow {
            id: item_id.clone(),
            collection_id: CollectionId::default(),
            category: Category::Locomotives,
            scale: Scale::H0,
            epoch: "VI".into(),
            description: "Some description".to_string(),
            product_code: "60100".to_string(),
            manufacturer: "Acme".to_string(),
            railway_model_id: railway_model_id.clone(),
            added_date: NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
            removed_date: None,
            purchase_condition: Some(PurchaseCondition::New),
            model_condition: Some(ModelCondition::Mint),
            box_condition: Some(BoxCondition::OriginalMint),
            notes: Some("My notes go here".to_string()),
        };

        let owned_item_id = OwnedRollingStockId::new(
            "trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-2c7ce47bc770".to_string(),
        );
        let rolling_stock_id = RollingStockId::new();
        let owned_rolling_stock = OwnedRollingStockRow {
            id: owned_item_id,
            collection_item_id: item_id.clone(),
            rolling_stock_id: Some(rolling_stock_id),
            notes: Some("My rolling stock notes go here".to_string()),
            dcc_address: None,
            installed_decoder_id: None,
            decoder_id: None,
            decoder_manufacturer_id: None,
            decoder_product_code: None,
            decoder_type: None,
            decoder_protocol: None,
            decoder_interface: None,
        };

        let purchase_id =
            PurchaseInfoId::try_from("trn:purchase:59adc26d-0274-4d6b-8c14-61e598d3fe0e").unwrap();
        let seller_id = SellerId::new_from_name("shop-1");
        let collection_item_id = item_id.clone();
        let purchase_info = PurchaseInfoRow {
            id: purchase_id,
            collection_item_id: collection_item_id.clone(),
            purchase_type: Some("purchased".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
            seller_id: Some(seller_id.clone()),
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

        let mut owned_rolling_stocks_map: HashMap<CollectionItemId, Vec<OwnedRollingStockRow>> =
            HashMap::new();
        owned_rolling_stocks_map.insert(collection_item_id.clone(), vec![owned_rolling_stock]);

        let mut purchase_infos_map: HashMap<CollectionItemId, Vec<PurchaseInfoRow>> =
            HashMap::new();
        purchase_infos_map.insert(collection_item_id.clone(), vec![purchase_info]);

        let mapped_item = CollectionMapper::row_to_collection_item(
            collection_item,
            &owned_rolling_stocks_map,
            &purchase_infos_map,
        )
        .expect("mapping item should succeed");

        assert_eq!(mapped_item.id, collection_item_id);
        assert_eq!(mapped_item.purchase_condition, Some(PurchaseCondition::New));
        assert_eq!(mapped_item.model_condition, Some(ModelCondition::Mint));
        assert_eq!(mapped_item.box_condition, Some(BoxCondition::OriginalMint));
        assert_eq!(mapped_item.notes, Some("My notes go here".to_string()));

        let mapped_railway_model = &mapped_item.railway_model;
        assert_eq!(mapped_railway_model.railway_model_id, railway_model_id);
        assert_eq!(mapped_railway_model.category, Category::Locomotives);
        assert_eq!(mapped_railway_model.scale, Scale::H0);
        assert_eq!(mapped_railway_model.epoch, "VI".into());
        assert_eq!(
            mapped_railway_model.description,
            "Some description".to_string()
        );
        assert_eq!(mapped_railway_model.product_code, "60100".to_string());
        assert_eq!(mapped_railway_model.manufacturer, "Acme".to_string());

        assert_eq!(mapped_item.rolling_stocks.len(), 1);
        let ors = &mapped_item.rolling_stocks[0];
        assert_eq!(
            ors.id.to_string(),
            "trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-2c7ce47bc770".to_string()
        );
        assert!(
            ors.rolling_stock_id
                .to_string()
                .starts_with("trn:rolling-stock")
        );
        assert_eq!(
            ors.notes,
            Some("My rolling stock notes go here".to_string())
        );

        let pi = mapped_item.purchase_info.expect("purchase info present");
        match pi {
            PurchaseInfo::Purchased(p) => {
                assert_eq!(
                    p.id.to_string(),
                    "trn:purchase:59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string()
                );
                assert_eq!(p.purchase_date.to_string(), "2025-12-26");
                let price = p.price.expect("price present");
                assert_eq!(price.amount, 17500u64);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(p.seller, Some(seller_id));
            }
            _ => panic!("expected Purchased variant"),
        }
    }

    #[test]
    fn it_should_map_row_to_purchase_info_purchased() {
        let pi_row = PurchaseInfoRow {
            id: PurchaseInfoId::new("59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string()),
            collection_item_id: CollectionItemId::try_from(
                "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730",
            )
            .unwrap(),
            purchase_type: Some("purchased".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
            seller_id: Some(SellerId::try_from("shop-1").unwrap()),
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
                assert_eq!(
                    p.id.to_string(),
                    "59adc26d-0274-4d6b-8c14-61e598d3fe0e".to_string()
                );
                assert_eq!(p.purchase_date.to_string(), "2025-12-26");
                let price = p.price.expect("price present");
                assert_eq!(price.amount, 17500u64);
                assert_eq!(price.currency, Currency::EUR);
                assert_eq!(p.seller, Some(SellerId::try_from("shop-1").unwrap()));
            }
            _ => panic!("expected Purchased variant"),
        }
    }

    #[test]
    fn it_should_map_row_to_purchase_info_sold() {
        let purchase_info_id =
            PurchaseInfoId::try_from("trn:purchase:e647e791-c56b-4018-acdb-5d7891f17c34")
                .expect("should parse purchase info id");
        let seller_id = SellerId::try_from("trn:seller:8d3cf2ec-ae2b-46ba-8f7d-cce3969d78b8")
            .expect("should parse seller id");
        let collection_item_id =
            CollectionItemId::try_from("trn:collection-item:1d1ad112-6080-4d3c-8c03-d694d30e2786")
                .expect("should parse collection item id");
        let pi_row = PurchaseInfoRow {
            id: purchase_info_id.clone(),
            collection_item_id: collection_item_id.clone(),
            purchase_type: Some("sold".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2024, 5, 10).unwrap(),
            seller_id: Some(seller_id.clone()),
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
                assert_eq!(s.id, purchase_info_id);
                assert_eq!(s.purchase_date.to_string(), "2024-05-10");
                let purchase_price = s.purchase_price.expect("purchase price present");
                assert_eq!(purchase_price.amount, 20000u64);
                assert_eq!(purchase_price.currency, Currency::EUR);
                assert_eq!(s.sale_price.amount, 25000u64);
                assert_eq!(s.sale_price.currency, Currency::EUR);
                assert_eq!(s.sale_date.to_string(), "2025-01-01");
                assert_eq!(s.buyer, Some("buyer-1".to_string()));
                assert_eq!(s.seller, Some(seller_id));
            }
            _ => panic!("expected Sold variant"),
        }
    }

    #[test]
    fn it_should_map_row_to_purchase_info_preorder() {
        let purchase_info_id =
            PurchaseInfoId::try_from("trn:purchase:e647e791-c56b-4018-acdb-5d7891f17c34")
                .expect("should parse purchase info id");
        let seller_id = SellerId::try_from("trn:seller:8d3cf2ec-ae2b-46ba-8f7d-cce3969d78b8")
            .expect("should parse seller id");
        let collection_item_id =
            CollectionItemId::try_from("trn:collection-item:1d1ad112-6080-4d3c-8c03-d694d30e2786")
                .expect("should parse collection item id");
        let pi_row = PurchaseInfoRow {
            id: purchase_info_id.clone(),
            collection_item_id: collection_item_id.clone(),
            purchase_type: Some("preorder".to_string()),
            purchase_date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            seller_id: Some(seller_id.clone()),
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

        let purchase_info =
            CollectionMapper::row_to_purchase_info(&pi_row).expect("mapping purchase info");
        match purchase_info {
            PurchaseInfo::PreOrdered(pre_order_info) => {
                assert_eq!(pre_order_info.id, purchase_info_id);
                assert_eq!(pre_order_info.order_date.to_string(), "2025-06-01");
                assert_eq!(pre_order_info.deposit.amount, 500u64);
                assert_eq!(pre_order_info.deposit.currency, Currency::EUR);
                assert_eq!(pre_order_info.total_price.amount, 1000u64);
                assert_eq!(pre_order_info.total_price.currency, Currency::EUR);
                assert_eq!(pre_order_info.seller, Some(seller_id));
                assert_eq!(
                    pre_order_info.expected_date.map(|d| d.to_string()),
                    Some("2025-12-01".to_string())
                );
            }
            _ => panic!("expected PreOrdered variant"),
        }
    }
}
