use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::interface::SimplifiedRailwayModelArgs;
use crate::collecting::application::AddCollectionItemInput;
use crate::collecting::domain::box_condition::validate_opt_box_condition;
use crate::collecting::domain::model_condition::validate_opt_model_condition;
use crate::collecting::domain::purchase_condition::validate_opt_purchase_condition;
use crate::collecting::domain::validate_collection_item_id;
use crate::collecting::domain::{BoxCondition, ModelCondition, PurchaseCondition};
use crate::core::domain::currency::{validate_currency_code, validate_opt_currency_code};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::{
    ValidationContext, validate_not_future_iso_date, validate_opt_not_future_date,
};
use crate::core::domain::{Currency, MonetaryAmount};
use crate::sellers::domain::seller_id::SellerId;
use chrono::NaiveDate;
use garde::Validate;
use serde::Deserialize;

/// Arguments structure for removing an item from the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCollectionItemArgs {
    /// The ID of the collection item to remove.
    #[garde(length(min = 1), custom(validate_collection_item_id))]
    pub collection_item_id: String,
    /// The category of the item.
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::railway_model::category::validate_category)
    )]
    pub category: String,
    /// The date the item was removed from the collection (YYYY-MM-DD).
    #[garde(custom(validate_iso_date), custom(validate_not_future_iso_date))]
    pub removed_date: String,
}

/// Arguments structure for selling an owned item from the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SellCollectionItemArgs {
    /// The ID of the collection item to sell.
    #[garde(length(min = 1), custom(validate_collection_item_id))]
    pub item_id: String,
    /// The date the item was sold (YYYY-MM-DD).
    #[garde(custom(validate_iso_date), custom(validate_not_future_iso_date))]
    pub sale_date: String,
    /// The sale amount in minor units (e.g. cents).
    #[garde(range(min = 0))]
    pub amount: i64,
    /// The ISO-4217 currency code (e.g. EUR, USD).
    #[garde(length(min = 3, max = 3), ascii, custom(validate_currency_code))]
    pub currency: String,
    /// Optional buyer identifier.
    #[garde(skip)]
    pub buyer_id: Option<String>,
}

/// Arguments structure for updating a single mutable field of a collection item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionItemArgs {
    /// The ID of the collection item to update.
    #[garde(length(min = 1), custom(validate_collection_item_id))]
    pub collection_item_id: String,
    /// The concrete field update payload.
    #[garde(dive)]
    pub update: CollectionItemUpdateArgs,
}

/// Tagged payload for a collection item update operation.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(tag = "kind", content = "data", rename_all = "camelCase")]
pub enum CollectionItemUpdateArgs {
    /// Set or clear seller id in purchase info.
    Seller {
        #[garde(custom(crate::sellers::domain::seller_id::validate_opt_seller_trn))]
        seller_id: Option<String>,
    },
    /// Set or clear purchased price in minor units.
    Price {
        #[garde(range(min = 0))]
        amount: Option<i64>,
        #[garde(custom(validate_opt_currency_code))]
        currency: Option<String>,
    },
    /// Set or clear purchase date.
    PurchaseDate {
        #[garde(custom(validate_opt_not_future_date))]
        purchase_date: Option<NaiveDate>,
    },
    /// Set or clear added date.
    AddedDate {
        #[garde(custom(validate_opt_not_future_date))]
        added_date: Option<NaiveDate>,
    },
    /// Set or clear notes.
    Notes {
        #[garde(length(max = 2000))]
        notes: Option<String>,
    },
    /// Set or clear purchase condition.
    PurchaseCondition {
        #[garde(custom(validate_opt_purchase_condition))]
        purchase_condition: Option<String>,
    },
    /// Set or clear model condition.
    ModelCondition {
        #[garde(custom(validate_opt_model_condition))]
        model_condition: Option<String>,
    },
    /// Set or clear box condition.
    BoxCondition {
        #[garde(custom(validate_opt_box_condition))]
        box_condition: Option<String>,
    },
}

/// Arguments structure for adding an item to the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddCollectionItemArgs {
    /// The railway model ID of the item to add.
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::railway_model::validate_railway_model_id)
    )]
    pub railway_model_id: String,
    /// The category and rolling stock are determined from the referenced railway model.
    /// The price amount in the smallest currency unit (e.g., cents). Must be >= 0.
    #[garde(range(min = 0))]
    pub price_amount: i64,
    /// The currency code for the price (e.g., "USD"). Must be 3 characters (ISO 4217).
    #[garde(length(min = 3, max = 3), ascii, custom(validate_currency_code))]
    pub price_currency: String,
    /// The seller ID (optional).
    #[garde(custom(crate::sellers::domain::seller_id::validate_opt_seller_trn))]
    pub seller_id: Option<String>,
    /// The date the item was added to the collection (YYYY-MM-DD).
    pub added_date: NaiveDate,
    /// The date the item was purchased (YYYY-MM-DD).
    pub purchase_date: NaiveDate,
    /// The purchase condition (optional). Valid values: NEW, PRE_OWNED.
    #[garde(custom(validate_opt_purchase_condition))]
    pub purchase_condition: Option<String>,
    /// The model condition (optional). Valid values: MINT, NEAR_MINT, EXCELLENT, etc.
    #[garde(custom(validate_opt_model_condition))]
    pub model_condition: Option<String>,
    /// The box condition (optional). Valid values: ORIGINAL_MINT, ORIGINAL_GOOD, etc.
    #[garde(custom(validate_opt_box_condition))]
    pub box_condition: Option<String>,
    /// Additional notes about the item (optional).
    #[garde(length(max = 2000))]
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

    fn valid_sell_args() -> SellCollectionItemArgs {
        SellCollectionItemArgs {
            item_id: "trn:collection-item:11111111-1111-1111-1111-111111111111".to_string(),
            sale_date: "2025-01-15".to_string(),
            amount: 5000,
            currency: "EUR".to_string(),
            buyer_id: None,
        }
    }

    #[test]
    fn sell_args_valid_passes_validation() {
        let args = valid_sell_args();
        assert!(args.validate().is_ok());
    }

    #[test]
    fn sell_args_future_date_fails_validation() {
        let args = SellCollectionItemArgs {
            sale_date: "2099-12-31".to_string(),
            ..valid_sell_args()
        };
        assert!(args.validate().is_err());
    }

    #[test]
    fn sell_args_negative_amount_fails_validation() {
        let args = SellCollectionItemArgs {
            amount: -1,
            ..valid_sell_args()
        };
        assert!(args.validate().is_err());
    }

    #[test]
    fn sell_args_invalid_currency_fails_validation() {
        let args = SellCollectionItemArgs {
            currency: "XYZ".to_string(),
            ..valid_sell_args()
        };
        assert!(args.validate().is_err());
    }

    #[test]
    fn sell_args_empty_item_id_fails_validation() {
        let args = SellCollectionItemArgs {
            item_id: "".to_string(),
            ..valid_sell_args()
        };
        assert!(args.validate().is_err());
    }

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
    #[garde(dive)]
    pub railway_model: SimplifiedRailwayModelArgs,

    /// The category and rolling stock are determined from the referenced railway model.
    /// The price amount in the smallest currency unit (e.g., cents). Must be >= 0.
    #[garde(range(min = 0))]
    pub price_amount: i64,
    /// The currency code for the price (e.g., "USD"). Must be 3 characters (ISO 4217).
    #[garde(length(min = 3, max = 3), ascii, custom(validate_currency_code))]
    pub price_currency: String,
    /// The seller ID (optional).
    #[garde(custom(crate::sellers::domain::seller_id::validate_opt_seller_trn))]
    pub seller_id: Option<String>,
    /// The date the item was added to the collection (YYYY-MM-DD).
    pub added_date: NaiveDate,
    /// The date the item was purchased (YYYY-MM-DD).
    pub purchase_date: NaiveDate,
    /// The purchase condition (optional). Valid values: NEW, PRE_OWNED.
    #[garde(custom(validate_opt_purchase_condition))]
    pub purchase_condition: Option<String>,
    /// The model condition (optional). Valid values: MINT, NEAR_MINT, EXCELLENT, etc.
    #[garde(custom(validate_opt_model_condition))]
    pub model_condition: Option<String>,
    /// The box condition (optional). Valid values: ORIGINAL_MINT, ORIGINAL_GOOD, etc.
    #[garde(custom(validate_opt_box_condition))]
    pub box_condition: Option<String>,
    /// Additional notes about the item (optional).
    #[garde(length(max = 2000))]
    pub notes: Option<String>,
    /// Purchase type: "STANDARD" (default) or "PREORDER".
    #[garde(skip)]
    pub purchase_type: Option<String>,
    /// Deposit amount for preorders (minor units). Required when purchase_type == "PREORDER".
    #[garde(range(min = 0))]
    pub deposit_amount: Option<i64>,
    /// Currency for the deposit. Required when purchase_type == "PREORDER".
    #[garde(custom(validate_opt_currency_code))]
    pub deposit_currency: Option<String>,
    /// Total preorder amount in minor units.
    #[garde(range(min = 0))]
    pub preorder_total_amount: Option<i64>,
    /// Currency for the preorder total.
    #[garde(custom(validate_opt_currency_code))]
    pub preorder_total_currency: Option<String>,
    /// Expected delivery date for preorders (YYYY-MM-DD).
    #[garde(skip)]
    pub expected_date: Option<NaiveDate>,
}

/// Arguments to mark a preordered item as received (converting it to a purchased item).
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReceivePreorderArgs {
    /// The ID of the collection item to convert from preorder to purchased.
    #[garde(length(min = 1), custom(validate_collection_item_id))]
    pub item_id: String,
    /// The date the item was received (YYYY-MM-DD, not in the future).
    #[garde(custom(validate_iso_date), custom(validate_not_future_iso_date))]
    pub received_date: String,
}

#[cfg(test)]
mod garde_tests {
    use super::*;
    use chrono::NaiveDate;
    use garde::Validate;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn valid_item() -> AcquisitionItemArgs {
        AcquisitionItemArgs {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "60100".to_string(),
            description: "Steam locomotive".to_string(),
            category: "LOCOMOTIVES".to_string(),
            scale: "H0".to_string(),
            epoch: "IV".to_string(),
            power_method: "DC".to_string(),
            price_amount: 5000,
            price_currency: "EUR".to_string(),
        }
    }

    fn valid_acquisition() -> RecordAcquisitionArgs {
        RecordAcquisitionArgs {
            seller_id: None,
            purchase_date: "2025-06-01".to_string(),
            items: vec![valid_item()],
        }
    }

    fn valid_add_collection_item() -> AddCollectionItemArgs {
        AddCollectionItemArgs {
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            price_amount: 1000,
            price_currency: "EUR".to_string(),
            seller_id: None,
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
            notes: None,
        }
    }

    // ── RecordAcquisitionArgs ────────────────────────────────────────────────

    #[test]
    fn record_acquisition_valid_passes() {
        assert!(valid_acquisition().validate().is_ok());
    }

    #[test]
    fn record_acquisition_bad_date_fails() {
        let args = RecordAcquisitionArgs {
            purchase_date: "not-a-date".to_string(),
            ..valid_acquisition()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "purchase_date"
                && e.to_string().contains("error_invalid_date_format")),
            "{errors:?}"
        );
    }

    #[test]
    fn record_acquisition_invalid_seller_trn_fails() {
        let args = RecordAcquisitionArgs {
            seller_id: Some("not-a-trn".to_string()),
            ..valid_acquisition()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "seller_id"
                && e.to_string().contains("error_invalid_seller_id")),
            "{errors:?}"
        );
    }

    #[test]
    fn record_acquisition_empty_items_fails() {
        let args = RecordAcquisitionArgs {
            items: vec![],
            ..valid_acquisition()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "items"),
            "{errors:?}"
        );
    }

    // ── AcquisitionItemArgs ──────────────────────────────────────────────────

    #[test]
    fn acquisition_item_valid_passes() {
        assert!(valid_item().validate().is_ok());
    }

    #[test]
    fn acquisition_item_negative_price_fails() {
        let item = AcquisitionItemArgs {
            price_amount: -1,
            ..valid_item()
        };
        let report = item.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "price_amount"),
            "{errors:?}"
        );
    }

    #[test]
    fn acquisition_item_bad_currency_length_fails() {
        let item = AcquisitionItemArgs {
            price_currency: "EU".to_string(),
            ..valid_item()
        };
        let report = item.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "price_currency"),
            "{errors:?}"
        );
    }

    #[test]
    fn acquisition_item_invalid_category_fails() {
        let item = AcquisitionItemArgs {
            category: "NOT_A_CATEGORY".to_string(),
            ..valid_item()
        };
        let report = item.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "category"
                && e.to_string().contains("error_invalid_category")),
            "{errors:?}"
        );
    }

    #[test]
    fn acquisition_item_invalid_scale_fails() {
        let item = AcquisitionItemArgs {
            scale: "NOSCALE".to_string(),
            ..valid_item()
        };
        let report = item.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, e)| p.to_string() == "scale"
                    && e.to_string().contains("error_invalid_scale")),
            "{errors:?}"
        );
    }

    #[test]
    fn acquisition_item_invalid_power_method_fails() {
        let item = AcquisitionItemArgs {
            power_method: "STEAM".to_string(),
            ..valid_item()
        };
        let report = item.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "power_method"
                && e.to_string().contains("error_invalid_power_method")),
            "{errors:?}"
        );
    }

    // ── AddCollectionItemArgs ────────────────────────────────────────────────

    #[test]
    fn add_collection_item_valid_passes() {
        assert!(valid_add_collection_item().validate().is_ok());
    }

    #[test]
    fn add_collection_item_negative_price_fails() {
        let args = AddCollectionItemArgs {
            price_amount: -100,
            ..valid_add_collection_item()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "price_amount"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_collection_item_bad_currency_fails() {
        let args = AddCollectionItemArgs {
            price_currency: "EU".to_string(),
            ..valid_add_collection_item()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "price_currency"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_collection_item_invalid_purchase_condition_fails() {
        let args = AddCollectionItemArgs {
            purchase_condition: Some("BROKEN".to_string()),
            ..valid_add_collection_item()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "purchase_condition"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_collection_item_invalid_model_condition_fails() {
        let args = AddCollectionItemArgs {
            model_condition: Some("PERFECT".to_string()),
            ..valid_add_collection_item()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "model_condition"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_collection_item_invalid_box_condition_fails() {
        let args = AddCollectionItemArgs {
            box_condition: Some("SMASHED".to_string()),
            ..valid_add_collection_item()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "box_condition"),
            "{errors:?}"
        );
    }
}

// Reuse `SimplifiedRailwayModelArgs` from the `catalog::interface` module.

/// Validates that a string represents a valid ISO 8601 date (YYYY-MM-DD).
fn validate_iso_date(v: &str, _: &()) -> garde::Result {
    chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| garde::Error::new("error_invalid_date_format"))
}

/// Top-level args for the record_acquisition command.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct RecordAcquisitionArgs {
    /// Optional seller id (TRN string).
    #[garde(custom(crate::sellers::domain::seller_id::validate_opt_seller_trn))]
    pub seller_id: Option<String>,
    /// Purchase date as YYYY-MM-DD string.
    #[garde(custom(validate_iso_date), custom(validate_not_future_iso_date))]
    pub purchase_date: String,
    /// At least one item required.
    #[garde(length(min = 1), dive)]
    pub items: Vec<AcquisitionItemArgs>,
}

/// Per-item args within a single acquisition batch.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AcquisitionItemArgs {
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::manufacturer::validate_manufacturer_id)
    )]
    pub manufacturer_id: String,
    #[garde(length(min = 1, max = 20))]
    pub product_code: String,
    #[garde(length(min = 1, max = 500))]
    pub description: String,
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::railway_model::category::validate_category)
    )]
    pub category: String,
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::scale::scale::validate_scale)
    )]
    pub scale: String,
    #[garde(
        length(min = 1, max = 10),
        custom(crate::catalog::domain::railway_model::epoch::validate_epoch)
    )]
    pub epoch: String,
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::railway_model::power_method::validate_power_method)
    )]
    pub power_method: String,
    /// Price in cents; 0 means no price recorded. Must be >= 0.
    #[garde(range(min = 0))]
    pub price_amount: i64,
    /// ISO 4217 currency code (3 characters).
    #[garde(length(min = 3, max = 3), ascii, custom(validate_currency_code))]
    pub price_currency: String,
}
