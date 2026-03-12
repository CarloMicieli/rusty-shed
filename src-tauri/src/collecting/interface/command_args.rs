use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::interface::SimplifiedRailwayModelArgs;
use crate::collecting::application::AddCollectionItemInput;
use crate::collecting::domain::{BoxCondition, ModelCondition, PurchaseCondition};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;
use garde::Validate;
use serde::Deserialize;

/// Arguments structure for removing an item from the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCollectionItemArgs {
    /// The ID of the collection item to remove.
    pub collection_item_id: String,
    /// The category of the item.
    pub category: String,
    /// The date the item was removed from the collection (YYYY-MM-DD).
    pub removed_date: String,
}

/// Arguments structure for updating a single mutable field of a collection item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionItemArgs {
    /// The ID of the collection item to update.
    pub collection_item_id: String,
    /// The concrete field update payload.
    pub update: CollectionItemUpdateArgs,
}

/// Tagged payload for a collection item update operation.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(tag = "kind", content = "data", rename_all = "camelCase")]
pub enum CollectionItemUpdateArgs {
    /// Set or clear seller id in purchase info.
    Seller { seller_id: Option<String> },
    /// Set or clear purchased price in minor units.
    Price {
        amount: Option<i64>,
        currency: Option<String>,
    },
    /// Set or clear purchase date.
    PurchaseDate { purchase_date: Option<NaiveDate> },
    /// Set or clear added date.
    AddedDate { added_date: Option<NaiveDate> },
    /// Set or clear notes.
    Notes { notes: Option<String> },
    /// Set or clear purchase condition.
    PurchaseCondition { purchase_condition: Option<String> },
    /// Set or clear model condition.
    ModelCondition { model_condition: Option<String> },
    /// Set or clear box condition.
    BoxCondition { box_condition: Option<String> },
}

/// Arguments structure for adding an item to the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddCollectionItemArgs {
    /// The railway model ID of the item to add.
    pub railway_model_id: String,
    /// The category and rolling stock are determined from the referenced railway model.
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

impl TryFrom<AddCollectionItemArgs> for AddCollectionItemInput {
    type Error = DomainError;

    fn try_from(input: AddCollectionItemArgs) -> Result<Self, Self::Error> {
        let mut ctx = ValidationContext::default();

        let railway_model_id =
            ctx.validate_try_from::<RailwayModelId>("railway_model_id", input.railway_model_id);

        // category and rolling stock are derived from the RailwayModel; no validation here.

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
        Ok(AddCollectionItemInput {
            railway_model_id: railway_model_id.unwrap(),
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
        let input = AddCollectionItemArgs {
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
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

        let cmd = AddCollectionItemInput::try_from(input).expect("conversion should succeed");
        assert_eq!(cmd.price.amount, 1234);
        assert_eq!(cmd.price.currency.to_code(), "USD");
    }
}

/// Arguments for creating a simplified railway model and adding it to the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddRailwayModelToCollectionArgs {
    /// The simplified railway model data.
    pub railway_model: SimplifiedRailwayModelArgs,

    /// The category and rolling stock are determined from the referenced railway model.
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

// Reuse `SimplifiedRailwayModelArgs` from the `catalog::interface` module.

/// Top-level args for the record_acquisition command.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct RecordAcquisitionArgs {
    /// Optional seller id (TRN string).
    pub seller_id: Option<String>,
    /// Purchase date as YYYY-MM-DD string.
    pub purchase_date: String,
    /// At least one item required.
    #[garde(length(min = 1))]
    pub items: Vec<AcquisitionItemArgs>,
}

/// Per-item args within a single acquisition batch.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AcquisitionItemArgs {
    #[garde(length(min = 1))]
    pub manufacturer_id: String,
    #[garde(length(min = 1))]
    pub product_code: String,
    pub description: String,
    #[garde(length(min = 1))]
    pub category: String,
    #[garde(length(min = 1))]
    pub scale: String,
    pub epoch: String,
    #[garde(length(min = 1))]
    pub power_method: String,
    /// Price in cents; 0 means no price recorded.
    pub price_amount: i64,
    pub price_currency: String,
}
