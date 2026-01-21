use std::convert::TryFrom;

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use crate::wishlist::interface::{
    AddToWishlistInput, CreateWishlistInput, MoveWishlistItemInput, RenameWishlistInput,
};
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct CreateWishlistCommand {
    pub name: String,
    pub notes: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct RenameWishlistCommand {
    pub id: WishlistId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DeleteWishlistCommand {
    pub id: WishlistId,
}

#[derive(Debug, Clone)]
pub struct SetDefaultWishlistCommand {
    pub id: WishlistId,
}

#[derive(Debug, Clone)]
pub struct AddToWishlistCommand {
    pub wishlist_id: WishlistId,
    pub railway_model_id: RailwayModelId,
    pub priority: WishlistPriority,
    pub status: WishlistStatus,
    pub desired_price: Option<MonetaryAmount>,
    pub notes: Option<String>,
    pub added_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct RemoveWishlistItemCommand {
    pub item_id: WishlistItemId,
}

#[derive(Debug, Clone)]
pub struct MoveWishlistItemCommand {
    pub item_id: WishlistItemId,
    pub destination_wishlist_id: WishlistId,
}

impl TryFrom<CreateWishlistInput> for CreateWishlistCommand {
    type Error = DomainError;

    fn try_from(input: CreateWishlistInput) -> Result<Self, Self::Error> {
        Ok(CreateWishlistCommand {
            name: input.name,
            notes: input.notes,
            is_default: input.is_default.unwrap_or(false),
        })
    }
}

impl TryFrom<RenameWishlistInput> for RenameWishlistCommand {
    type Error = DomainError;

    fn try_from(input: RenameWishlistInput) -> Result<Self, Self::Error> {
        let id = WishlistId::try_from(input.id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(RenameWishlistCommand {
            id,
            name: input.name,
        })
    }
}

impl TryFrom<String> for DeleteWishlistCommand {
    type Error = DomainError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let wid = WishlistId::try_from(id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(DeleteWishlistCommand { id: wid })
    }
}

impl TryFrom<String> for SetDefaultWishlistCommand {
    type Error = DomainError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let wid = WishlistId::try_from(id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(SetDefaultWishlistCommand { id: wid })
    }
}

impl TryFrom<AddToWishlistInput> for AddToWishlistCommand {
    type Error = DomainError;

    fn try_from(input: AddToWishlistInput) -> Result<Self, Self::Error> {
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

        Ok(AddToWishlistCommand {
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

impl TryFrom<String> for RemoveWishlistItemCommand {
    type Error = DomainError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let iid = WishlistItemId::try_from(id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(RemoveWishlistItemCommand { item_id: iid })
    }
}

impl TryFrom<MoveWishlistItemInput> for MoveWishlistItemCommand {
    type Error = DomainError;

    fn try_from(input: MoveWishlistItemInput) -> Result<Self, Self::Error> {
        let item_id = WishlistItemId::try_from(input.item_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        let dest = WishlistId::try_from(input.destination_wishlist_id.as_str())
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        Ok(MoveWishlistItemCommand {
            item_id,
            destination_wishlist_id: dest,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wishlist::interface::CreateWishlistInput;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_create_wishlist_try_from_sets_defaults() {
        let input = CreateWishlistInput {
            name: "My list".to_string(),
            notes: Some("notes".to_string()),
            is_default: None,
        };

        let cmd = CreateWishlistCommand::try_from(input).expect("conversion should succeed");

        assert_eq!(cmd.name, "My list");
        assert_eq!(cmd.notes, Some("notes".to_string()));
        assert!(!cmd.is_default);
    }

    #[test]
    fn it_should_create_wishlist_try_from_with_true() {
        let input = CreateWishlistInput {
            name: "List2".to_string(),
            notes: None,
            is_default: Some(true),
        };

        let cmd = CreateWishlistCommand::try_from(input).expect("conversion should succeed");

        assert_eq!(cmd.name, "List2");
        assert!(cmd.is_default);
    }
}
