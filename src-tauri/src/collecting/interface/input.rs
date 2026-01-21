use crate::catalog::domain::railway_model::{Category, RailwayModelId, RollingStockId};
use crate::collecting::domain::{
    AddCollectionItem, BoxCondition, ModelCondition, PurchaseCondition,
};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;
use serde::Deserialize;

/// Input structure for removing an item from the collection.
#[derive(Debug, Clone, Deserialize, specta::Type)]
pub struct RemoveCollectionItemInput {
    /// The ID of the collection item to remove.
    pub collection_item_id: String,
    /// The category of the item.
    pub category: String,
    /// The date the item was removed from the collection (YYYY-MM-DD).
    pub removed_date: String,
}

/// Input structure for adding an item to the collection.
#[derive(Debug, Clone, Deserialize, specta::Type)]
pub struct AddCollectionItemInput {
    /// The railway model ID of the item to add.
    pub railway_model_id: String,
    /// The rolling stock IDs associated with the item.
    pub rolling_stock_ids: Vec<String>,
    /// The category of the item.
    pub category: String,
    /// The price amount in the smallest currency unit (e.g., cents).
    pub price_amount: i64,
    /// The currency code for the price (e.g., "USD").
    pub price_currency: String,
    /// The seller ID (optional).
    pub seller_id: Option<String>,
    /// The date the item was added to the collection (YYYY-MM-DD).
    pub added_date: NaiveDate,
    /// The date the item was purchased (YYYY-MM-DD).
    pub purchase_date: NaiveDate,
    /// The purchase condition (optional).
    pub purchase_condition: Option<String>,
    /// The model condition (optional).
    pub model_condition: Option<String>,
    /// The box condition (optional).
    pub box_condition: Option<String>,
    /// Additional notes about the item (optional).
    pub notes: Option<String>,
}

impl TryFrom<AddCollectionItemInput> for AddCollectionItem {
    type Error = DomainError;

    fn try_from(input: AddCollectionItemInput) -> Result<Self, Self::Error> {
        let mut ctx = ValidationContext::default();

        let railway_model_id =
            ctx.validate_try_from::<RailwayModelId>("railway_model_id", input.railway_model_id);

        // Rolling stock validation: Ensure we don't just "skip" invalid ones
        let rolling_stock_ids = if input.rolling_stock_ids.is_empty() {
            ctx.push_error("rolling_stock_ids", "required", "cannot be empty");
            None // Mark as None so we know it failed
        } else {
            let items = ctx.validate_vec_try_from::<RollingStockId>(
                "rolling_stock_ids",
                input.rolling_stock_ids,
            );
            // If lengths don't match, some items failed validation
            Some(items)
        };

        let category = ctx.validate_parse::<Category>("category", input.category);

        // Currency and Price
        let currency = ctx.collect("price_currency", Currency::from_code(&input.price_currency));
        if input.price_amount < 0 {
            ctx.push_error(
                "price_amount",
                "invalid_range",
                "price_amount cannot be negative",
            );
        }

        let seller_id = ctx.validate_opt_try_from::<SellerId>("seller_id", input.seller_id);

        // Enums
        let purchase_condition = ctx.validate_opt_parse::<PurchaseCondition>(
            "purchase_condition",
            input.purchase_condition,
        );
        let model_condition =
            ctx.validate_opt_parse::<ModelCondition>("model_condition", input.model_condition);
        let box_condition =
            ctx.validate_opt_parse::<BoxCondition>("box_condition", input.box_condition);

        // Check for errors before unwrapping
        ctx.finish()?;

        // SAFE UNWRAPS: Guaranteed by ctx.finish()?
        Ok(AddCollectionItem {
            railway_model_id: railway_model_id.unwrap(),
            rolling_stock_ids: rolling_stock_ids.unwrap(),
            category: category.unwrap(),
            price: MonetaryAmount::new(input.price_amount, currency.unwrap()),
            seller_id,
            added_date: input.added_date,
            purchase_date: input.purchase_date,
            purchase_condition,
            model_condition,
            box_condition,
            notes: input.notes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_collection_item_try_from_valid() {
        let input = AddCollectionItemInput {
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            rolling_stock_ids: vec![
                "trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8".to_string(),
            ],
            category: "Locomotives".to_string(),
            price_amount: 1234,
            price_currency: "USD".to_string(),
            seller_id: Some("trn:seller:model-train-shop".to_string()),
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: Some("NEW".to_string()),
            model_condition: Some("MINT".to_string()),
            box_condition: Some("ORIGINAL_MINT".to_string()),
            notes: Some("Inserted by test".to_string()),
        };

        let cmd = AddCollectionItem::try_from(input).expect("conversion should succeed");
        assert_eq!(cmd.price.amount, 1234);
        assert_eq!(cmd.price.currency.to_code(), "USD");
        assert_eq!(cmd.rolling_stock_ids.len(), 1);
    }

    #[test]
    fn it_should_add_collection_item_try_from_empty_rolling_stock() {
        let input = AddCollectionItemInput {
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            rolling_stock_ids: vec![],
            category: "Locomotives".to_string(),
            price_amount: 1234,
            price_currency: "USD".to_string(),
            seller_id: None,
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
            notes: None,
        };

        let res = AddCollectionItem::try_from(input);
        assert!(res.is_err());
        match res.err().unwrap() {
            crate::core::domain::domain_error::DomainError::ValidationError(errors) => {
                assert!(errors.contains_key("rolling_stock_ids"));
            }
            _ => panic!("expected Validation error"),
        }
    }
}
