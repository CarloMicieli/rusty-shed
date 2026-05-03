use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::interface::SimplifiedRailwayModelArgs;
use crate::core::domain::currency::{validate_currency_code, validate_opt_currency_code};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::validate_opt_not_future_date;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::wishlist::application::inputs::{
    AddToWishlistInput, CreateWishlistInput, DeleteWishlistInput, MoveWishlistItemInput,
    RemoveWishlistItemInput, RenameWishlistInput, SetDefaultWishlistInput, UpdateWishlistItemInput,
};
use crate::wishlist::domain::wishlist_id::{WishlistId, validate_wishlist_id};
use crate::wishlist::domain::wishlist_item_id::{WishlistItemId, validate_wishlist_item_id};
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};

/// Monetary amount exposed through wishlist command contracts.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WishlistMonetaryAmountDto {
    pub amount: i64,
    pub currency: Currency,
}

/// Wishlist item contract for transport responses.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WishlistItem {
    pub id: WishlistItemId,
    pub railway_model_id: RailwayModelId,
    pub priority: WishlistPriority,
    pub status: WishlistStatus,
    pub added_date: NaiveDate,
    pub removed_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub desired_price: Option<WishlistMonetaryAmountDto>,
    pub purchased_price: Option<WishlistMonetaryAmountDto>,
}

impl From<crate::wishlist::domain::wishlist_item::WishlistItem> for WishlistItem {
    fn from(value: crate::wishlist::domain::wishlist_item::WishlistItem) -> Self {
        Self {
            id: value.id,
            railway_model_id: value.railway_model_id,
            priority: value.priority,
            status: value.status,
            added_date: value.added_date,
            removed_date: value.removed_date,
            notes: value.notes,
            desired_price: value.desired_price.map(|price| WishlistMonetaryAmountDto {
                amount: price.amount,
                currency: price.currency,
            }),
            purchased_price: value
                .purchased_price
                .map(|price| WishlistMonetaryAmountDto {
                    amount: price.amount,
                    currency: price.currency,
                }),
        }
    }
}

/// Arguments structure for adding an item to a wishlist.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddToWishlistArgs {
    /// The ID of the wishlist to which the item will be added.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub wishlist_id: String,
    /// The railway model ID of the item to add.
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::railway_model::validate_railway_model_id)
    )]
    pub railway_model_id: String,
    /// The priority of the wishlist item (optional).
    pub priority: Option<WishlistPriority>,
    /// The status of the wishlist item (optional).
    pub status: Option<WishlistStatus>,
    /// The desired price amount in the smallest currency unit (e.g., cents). Must be >= 0.
    #[garde(range(min = 0))]
    pub desired_price_amount: Option<i64>,
    /// The desired price currency code (e.g., "USD"). Must be 3 characters (ISO 4217).
    #[garde(length(min = 3, max = 3), ascii, custom(validate_opt_currency_code))]
    pub desired_price_currency: Option<String>,
    /// Additional notes about the wishlist item (optional).
    #[garde(length(max = 2000))]
    pub notes: Option<String>,
    /// The date the item was added to the wishlist (optional).
    #[garde(custom(validate_opt_not_future_date))]
    pub added_date: Option<NaiveDate>,
}

/// Arguments structure for moving an item between wishlists.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MoveWishlistItemArgs {
    /// The ID of the wishlist item to move.
    #[garde(length(min = 1), custom(validate_wishlist_item_id))]
    pub item_id: String,
    /// The ID of the destination wishlist.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub destination_wishlist_id: String,
    /// The ID of the source wishlist the item currently belongs to.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub wishlist_id: String,
}

/// Arguments structure for creating a new wishlist.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CreateWishlistArgs {
    /// The name of the new wishlist (1–200 characters).
    #[garde(length(min = 1, max = 200))]
    pub name: String,
    /// Optional notes about the new wishlist.
    #[garde(length(max = 2000))]
    pub notes: Option<String>,
    /// Whether the new wishlist should be set as the default.
    pub is_default: Option<bool>,
}

/// Arguments structure for renaming an existing wishlist.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct RenameWishlistArgs {
    /// The ID of the wishlist to rename.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub wishlist_id: String,
    /// The new name for the wishlist (1–200 characters).
    #[garde(length(min = 1, max = 200))]
    pub name: String,
}

/// Arguments for purchasing a wishlist item and moving it to the collection.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseWishlistArgs {
    /// The ID of the wishlist containing the item.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub wishlist_id: String,
    /// The ID of the wishlist item being purchased.
    #[garde(length(min = 1), custom(validate_wishlist_item_id))]
    pub wishlist_item_id: String,
    /// Purchase price amount in the smallest currency unit (e.g., cents). Must be >= 0.
    #[garde(range(min = 0))]
    pub price_amount: i64,
    /// Purchase price currency code (e.g., "EUR", "USD", "GBP", "JPY"). Must be 3 characters.
    #[garde(length(min = 3, max = 3), ascii, custom(validate_currency_code))]
    pub price_currency: String,
    /// The date the purchase occurred (ISO 8601: YYYY-MM-DD).
    pub purchase_date: NaiveDate,
    /// Optional seller id string.
    #[garde(custom(crate::sellers::domain::seller_id::validate_opt_seller_trn))]
    pub seller_id: Option<String>,
    /// Purchase condition. Valid values: NEW | PRE_OWNED.
    #[garde(custom(
        crate::collecting::domain::purchase_condition::validate_opt_purchase_condition
    ))]
    pub purchase_condition: Option<String>,
    /// Model condition grade. Valid values: MINT | NEAR_MINT | EXCELLENT | VERY_GOOD | GOOD | FAIR | POOR | FOR_PARTS.
    #[garde(custom(crate::collecting::domain::model_condition::validate_opt_model_condition))]
    pub model_condition: Option<String>,
    /// Box/packaging condition. Valid values: ORIGINAL_MINT | ORIGINAL_GOOD | ORIGINAL_WORN | REPLACEMENT_BOX | NO_BOX.
    #[garde(custom(crate::collecting::domain::box_condition::validate_opt_box_condition))]
    pub box_condition: Option<String>,
}

impl TryFrom<CreateWishlistArgs> for CreateWishlistInput {
    type Error = DomainError;

    fn try_from(input: CreateWishlistArgs) -> Result<Self, Self::Error> {
        Ok(CreateWishlistInput {
            name: input.name,
            notes: input.notes,
            is_default: input.is_default.unwrap_or(false),
        })
    }
}

impl TryFrom<RenameWishlistArgs> for RenameWishlistInput {
    type Error = DomainError;

    fn try_from(input: RenameWishlistArgs) -> Result<Self, Self::Error> {
        let id = WishlistId::try_from(input.wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(RenameWishlistInput {
            id,
            name: input.name,
        })
    }
}

impl TryFrom<String> for DeleteWishlistInput {
    type Error = DomainError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let wid = WishlistId::try_from(id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(DeleteWishlistInput { id: wid })
    }
}

impl TryFrom<String> for SetDefaultWishlistInput {
    type Error = DomainError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let wid = WishlistId::try_from(id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(SetDefaultWishlistInput { id: wid })
    }
}

impl TryFrom<AddToWishlistArgs> for AddToWishlistInput {
    type Error = DomainError;

    fn try_from(input: AddToWishlistArgs) -> Result<Self, Self::Error> {
        let wishlist_id = WishlistId::try_from(input.wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        let railway_model_id = RailwayModelId::try_from(input.railway_model_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let desired_price = match (input.desired_price_amount, input.desired_price_currency) {
            (Some(amount), Some(code)) => {
                let currency = Currency::from_code(&code)
                    .map_err(|e| DomainError::Validation(e.to_string()))?;
                Some(MonetaryAmount::new(amount, currency))
            }
            _ => None,
        };

        Ok(AddToWishlistInput {
            wishlist_id,
            railway_model_id,
            priority: input.priority.unwrap_or_default(),
            status: input.status.unwrap_or_default(),
            desired_price,
            notes: input.notes,
            added_date: input.added_date.unwrap_or(chrono::Utc::now().date_naive()),
        })
    }
}

impl TryFrom<String> for RemoveWishlistItemInput {
    type Error = DomainError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let iid = WishlistItemId::try_from(id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(RemoveWishlistItemInput { item_id: iid })
    }
}

impl TryFrom<MoveWishlistItemArgs> for MoveWishlistItemInput {
    type Error = DomainError;

    fn try_from(input: MoveWishlistItemArgs) -> Result<Self, Self::Error> {
        let item_id = WishlistItemId::try_from(input.item_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        let dest = WishlistId::try_from(input.destination_wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        let wid = WishlistId::try_from(input.wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(MoveWishlistItemInput {
            item_id,
            destination_wishlist_id: dest,
            wishlist_id: wid,
        })
    }
}

impl TryFrom<PurchaseWishlistArgs>
    for crate::wishlist::application::purchase_wishlist_item::PurchaseWishlistItemCommand
{
    type Error = DomainError;

    fn try_from(input: PurchaseWishlistArgs) -> Result<Self, Self::Error> {
        use crate::collecting::domain::BoxCondition;
        use crate::collecting::domain::ModelCondition;
        use crate::collecting::domain::PurchaseCondition;
        use crate::sellers::domain::seller_id::SellerId;
        use crate::wishlist::application::purchase_wishlist_item::PurchaseWishlistItemCommand;

        let wishlist_id = WishlistId::try_from(input.wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let wishlist_item_id = WishlistItemId::try_from(input.wishlist_item_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let currency = Currency::from_code(&input.price_currency)
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let price = MonetaryAmount::new(input.price_amount, currency);

        let seller = match input.seller_id {
            Some(s) => Some(
                SellerId::try_from(s.as_str())
                    .map_err(|e| DomainError::Validation(e.to_string()))?,
            ),
            None => None,
        };

        let purchase_condition = input
            .purchase_condition
            .as_deref()
            .map(|s| {
                s.parse::<PurchaseCondition>().map_err(|_| {
                    DomainError::Validation(format!("Unknown purchase_condition: {s}"))
                })
            })
            .transpose()?;

        let model_condition = input
            .model_condition
            .as_deref()
            .map(|s| {
                s.parse::<ModelCondition>()
                    .map_err(|_| DomainError::Validation(format!("Unknown model_condition: {s}")))
            })
            .transpose()?;

        let box_condition = input
            .box_condition
            .as_deref()
            .map(|s| {
                s.parse::<BoxCondition>()
                    .map_err(|_| DomainError::Validation(format!("Unknown box_condition: {s}")))
            })
            .transpose()?;

        Ok(PurchaseWishlistItemCommand {
            wishlist_id,
            wishlist_item_id,
            purchase_price: price,
            purchase_date: input.purchase_date,
            seller_id: seller,
            purchase_condition,
            model_condition,
            box_condition,
        })
    }
}

/// Arguments for creating a simplified railway model and adding it to a wishlist.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddRailwayModelToWishListArgs {
    /// The simplified railway model data.
    #[garde(dive)]
    pub railway_model: SimplifiedRailwayModelArgs,
    /// Target wishlist id. The item will be added to this wishlist.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub wishlist_id: String,
    /// The priority of the wishlist item (optional).
    pub priority: Option<WishlistPriority>,
    /// The status of the wishlist item (optional).
    pub status: Option<WishlistStatus>,
    /// The desired price amount in the smallest currency unit (e.g., cents). Must be >= 0.
    #[garde(range(min = 0))]
    pub desired_price_amount: Option<i64>,
    /// The desired price currency code (e.g., "USD"). Must be 3 characters (ISO 4217).
    #[garde(length(min = 3, max = 3), ascii, custom(validate_opt_currency_code))]
    pub desired_price_currency: Option<String>,
    /// Additional notes about the wishlist item (optional).
    #[garde(length(max = 2000))]
    pub notes: Option<String>,
    /// The date the item was added to the wishlist (optional).
    #[garde(custom(validate_opt_not_future_date))]
    pub added_date: Option<NaiveDate>,
}

/// Serde helper: distinguishes an **absent** JSON key (`None` = "do not touch")
/// from an **explicit `null`** (`Some(None)` = "clear the value").
///
/// Used for `desired_price_amount` in `UpdateWishlistItemArgs`.
fn deserialize_double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

/// Garde validator for the double-option price amount: rejects negative inner values.
fn validate_opt_price_amount(value: &Option<Option<i64>>, _: &()) -> garde::Result {
    if matches!(value, Some(Some(n)) if *n < 0) {
        return Err(garde::Error::new("error_invalid_desired_price_amount"));
    }
    Ok(())
}

/// Arguments for updating editable fields on a specific wishlist item.
///
/// Only provided (non-`null`) fields are changed; omitted fields are left untouched.
/// For `desired_price_amount`: absent = unchanged, `null` = clear, number = set.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWishlistItemArgs {
    /// UUID of the parent wishlist.
    #[garde(length(min = 1), custom(validate_wishlist_id))]
    pub wishlist_id: String,
    /// UUID of the wishlist item to update.
    #[garde(length(min = 1), custom(validate_wishlist_item_id))]
    pub item_id: String,
    /// New priority; omit or `null` to leave unchanged.
    pub priority: Option<WishlistPriority>,
    /// New status; omit or `null` to leave unchanged.
    pub status: Option<WishlistStatus>,
    /// `null` clears the price; a number sets it (in smallest unit, must be >= 0); absent = unchanged.
    #[serde(default, deserialize_with = "deserialize_double_option")]
    #[specta(type = Option<i64>)]
    #[garde(custom(validate_opt_price_amount))]
    pub desired_price_amount: Option<Option<i64>>,
    /// ISO 4217 currency code (3 characters); required when `desired_price_amount` is a number.
    #[garde(length(min = 3, max = 3), ascii, custom(validate_opt_currency_code))]
    pub desired_price_currency: Option<String>,
    /// New added date (ISO 8601 YYYY-MM-DD); must be ≤ today. Omit to leave unchanged.
    #[garde(custom(validate_opt_not_future_date))]
    pub added_date: Option<NaiveDate>,
}

impl TryFrom<UpdateWishlistItemArgs> for UpdateWishlistItemInput {
    type Error = DomainError;

    fn try_from(input: UpdateWishlistItemArgs) -> Result<Self, Self::Error> {
        let wishlist_id = WishlistId::try_from(input.wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        let item_id = WishlistItemId::try_from(input.item_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        // Map double-option price amount + currency into a double-option MonetaryAmount.
        let desired_price: Option<Option<MonetaryAmount>> = match input.desired_price_amount {
            None => None,             // absent — do not touch
            Some(None) => Some(None), // explicit null — clear the price
            Some(Some(amount)) => {
                if amount < 0 {
                    return Err(DomainError::Validation(
                        "error_invalid_desired_price_amount".to_string(),
                    ));
                }
                let code = input.desired_price_currency.as_deref().ok_or_else(|| {
                    DomainError::Validation(
                        "desired_price_currency is required when desired_price_amount is set"
                            .to_string(),
                    )
                })?;
                let currency = Currency::from_code(code)
                    .map_err(|e| DomainError::Validation(e.to_string()))?;
                Some(Some(MonetaryAmount::new(amount, currency)))
            }
        };

        Ok(UpdateWishlistItemInput {
            wishlist_id,
            item_id,
            priority: input.priority,
            status: input.status,
            desired_price,
            added_date: input.added_date,
        })
    }
}

#[cfg(test)]
mod garde_tests {
    use super::*;
    use chrono::NaiveDate;
    use garde::Validate;

    // ── CreateWishlistArgs ───────────────────────────────────────────────────

    #[test]
    fn create_wishlist_valid_passes() {
        let args = CreateWishlistArgs {
            name: "My list".to_string(),
            notes: None,
            is_default: None,
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn create_wishlist_empty_name_fails() {
        let args = CreateWishlistArgs {
            name: String::new(),
            notes: None,
            is_default: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "name"),
            "{errors:?}"
        );
    }

    #[test]
    fn create_wishlist_name_too_long_fails() {
        let args = CreateWishlistArgs {
            name: "x".repeat(201),
            notes: None,
            is_default: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "name"),
            "{errors:?}"
        );
    }

    // ── RenameWishlistArgs ───────────────────────────────────────────────────

    #[test]
    fn rename_wishlist_empty_name_fails() {
        let args = RenameWishlistArgs {
            wishlist_id: "trn:wishlist:some-id".to_string(),
            name: String::new(),
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "name"),
            "{errors:?}"
        );
    }

    // ── AddToWishlistArgs ────────────────────────────────────────────────────

    #[test]
    fn add_to_wishlist_valid_passes() {
        let args = AddToWishlistArgs {
            wishlist_id: "trn:wishlist:550e8400-e29b-41d4-a716-446655440000".to_string(),
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            priority: None,
            status: None,
            desired_price_amount: Some(500),
            desired_price_currency: Some("EUR".to_string()),
            notes: None,
            added_date: None,
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn add_to_wishlist_negative_price_fails() {
        let args = AddToWishlistArgs {
            wishlist_id: "wl-1".to_string(),
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            priority: None,
            status: None,
            desired_price_amount: Some(-1),
            desired_price_currency: Some("EUR".to_string()),
            notes: None,
            added_date: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "desired_price_amount"),
            "{errors:?}"
        );
    }

    #[test]
    fn add_to_wishlist_bad_currency_fails() {
        let args = AddToWishlistArgs {
            wishlist_id: "wl-1".to_string(),
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            priority: None,
            status: None,
            desired_price_amount: None,
            desired_price_currency: Some("EU".to_string()),
            notes: None,
            added_date: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "desired_price_currency"),
            "{errors:?}"
        );
    }

    // ── PurchaseWishlistArgs ─────────────────────────────────────────────────

    fn valid_purchase() -> PurchaseWishlistArgs {
        PurchaseWishlistArgs {
            wishlist_id: "trn:wishlist:550e8400-e29b-41d4-a716-446655440000".to_string(),
            wishlist_item_id: "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440001".to_string(),
            price_amount: 2000,
            price_currency: "EUR".to_string(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            seller_id: None,
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
        }
    }

    #[test]
    fn purchase_wishlist_valid_passes() {
        assert!(valid_purchase().validate().is_ok());
    }

    #[test]
    fn purchase_wishlist_bad_currency_fails() {
        let args = PurchaseWishlistArgs {
            price_currency: "EU".to_string(),
            ..valid_purchase()
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
    fn purchase_wishlist_invalid_purchase_condition_fails() {
        let args = PurchaseWishlistArgs {
            purchase_condition: Some("REFURBISHED".to_string()),
            ..valid_purchase()
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
    fn purchase_wishlist_invalid_model_condition_fails() {
        let args = PurchaseWishlistArgs {
            model_condition: Some("LIKE_NEW".to_string()),
            ..valid_purchase()
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
    fn purchase_wishlist_invalid_box_condition_fails() {
        let args = PurchaseWishlistArgs {
            box_condition: Some("DAMAGED".to_string()),
            ..valid_purchase()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, _)| p.to_string() == "box_condition"),
            "{errors:?}"
        );
    }

    // ── UpdateWishlistItemArgs ───────────────────────────────────────────────

    #[test]
    fn update_wishlist_item_negative_price_fails() {
        let args = UpdateWishlistItemArgs {
            wishlist_id: "wl-1".to_string(),
            item_id: "item-1".to_string(),
            priority: None,
            status: None,
            desired_price_amount: Some(Some(-5)),
            desired_price_currency: Some("EUR".to_string()),
            added_date: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "desired_price_amount"),
            "{errors:?}"
        );
    }

    #[test]
    fn update_wishlist_item_bad_currency_fails() {
        let args = UpdateWishlistItemArgs {
            wishlist_id: "wl-1".to_string(),
            item_id: "item-1".to_string(),
            priority: None,
            status: None,
            desired_price_amount: None,
            desired_price_currency: Some("E".to_string()),
            added_date: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, _)| p.to_string() == "desired_price_currency"),
            "{errors:?}"
        );
    }

    #[test]
    fn update_wishlist_item_null_price_passes() {
        // Some(None) means "clear the price" — should be valid
        let args = UpdateWishlistItemArgs {
            wishlist_id: "trn:wishlist:550e8400-e29b-41d4-a716-446655440000".to_string(),
            item_id: "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440001".to_string(),
            priority: None,
            status: None,
            desired_price_amount: Some(None),
            desired_price_currency: None,
            added_date: None,
        };
        assert!(args.validate().is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wishlist::interface::CreateWishlistArgs;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_create_wishlist_try_from_sets_defaults() {
        let input = CreateWishlistArgs {
            name: "My list".to_string(),
            notes: Some("notes".to_string()),
            is_default: None,
        };

        let cmd = CreateWishlistInput::try_from(input).expect("conversion should succeed");

        assert_eq!(cmd.name, "My list");
        assert_eq!(cmd.notes, Some("notes".to_string()));
        assert!(!cmd.is_default);
    }

    #[test]
    fn it_should_create_wishlist_try_from_with_true() {
        let input = CreateWishlistArgs {
            name: "List2".to_string(),
            notes: None,
            is_default: Some(true),
        };

        let cmd = CreateWishlistInput::try_from(input).expect("conversion should succeed");

        assert_eq!(cmd.name, "List2");
        assert!(cmd.is_default);
    }
}
