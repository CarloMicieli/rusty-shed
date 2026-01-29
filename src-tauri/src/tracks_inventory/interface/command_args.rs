use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::error::CommandError;
use crate::sellers::domain::seller_id::SellerId;
use crate::tracks_inventory::application::{
    AddTrackPurchaseInput, NewTrackInventoryInput, RenameTrackInventoryInput,
    SetTrackItemQuantityInput,
};
use crate::tracks_inventory::domain::TrackId;
use crate::tracks_inventory::domain::TrackInventoryId;
use chrono::NaiveDate;
use garde::Validate;
use serde::Deserialize;

/// Command argument to create a new track inventory
#[derive(Debug, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(allow_unvalidated)]
pub struct NewTrackInventoryArgs {
    /// Name of the track inventory
    pub name: String,
    /// Description of the track inventory
    pub description: Option<String>,
}

/// Command argument to add a track purchase to an inventory
#[derive(Debug, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(allow_unvalidated)]
pub struct AddTrackPurchaseArgs {
    /// Inventory ID to which the purchase is added
    pub id: TrackInventoryId,
    /// Track ID of the purchased track
    pub track_id: TrackId,
    /// Quantity of tracks purchased
    pub quantity: i64,
    /// Price paid for the purchase
    pub price: MonetaryAmount,
    /// Optional seller ID from whom the track was purchased
    pub seller_id: Option<SellerId>,
    /// Date of the purchase
    pub purchase_date: NaiveDate,
}

/// Command argument to rename a track inventory
#[derive(Debug, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(allow_unvalidated)]
pub struct RenameTrackInventoryArgs {
    /// Inventory ID to be renamed
    pub id: TrackInventoryId,
    /// New name for the track inventory
    pub new_name: String,
}

/// Command argument to set the quantity of a track item in an inventory
#[derive(Debug, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(allow_unvalidated)]
pub struct SetTrackItemQuantityArgs {
    /// Inventory ID containing the track item
    pub inventory_id: TrackInventoryId,
    /// Track ID of the item whose quantity is to be set
    pub track_id: TrackId,
    /// New quantity for the track item
    pub quantity: i64,
}

impl TryFrom<NewTrackInventoryArgs> for NewTrackInventoryInput {
    type Error = CommandError;

    fn try_from(value: NewTrackInventoryArgs) -> Result<Self, Self::Error> {
        Ok(NewTrackInventoryInput {
            name: value.name,
            description: value.description,
        })
    }
}

impl TryFrom<AddTrackPurchaseArgs> for AddTrackPurchaseInput {
    type Error = CommandError;

    fn try_from(value: AddTrackPurchaseArgs) -> Result<Self, Self::Error> {
        Ok(AddTrackPurchaseInput {
            id: value.id,
            track_id: value.track_id,
            quantity: value.quantity,
            price: value.price,
            seller_id: value.seller_id,
            purchase_date: value.purchase_date,
        })
    }
}

impl TryFrom<RenameTrackInventoryArgs> for RenameTrackInventoryInput {
    type Error = CommandError;

    fn try_from(value: RenameTrackInventoryArgs) -> Result<Self, Self::Error> {
        Ok(RenameTrackInventoryInput {
            id: value.id,
            new_name: value.new_name,
        })
    }
}

impl TryFrom<SetTrackItemQuantityArgs> for SetTrackItemQuantityInput {
    type Error = CommandError;

    fn try_from(value: SetTrackItemQuantityArgs) -> Result<Self, Self::Error> {
        Ok(SetTrackItemQuantityInput {
            inventory_id: value.inventory_id,
            track_id: value.track_id,
            quantity: value.quantity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::currency::Currency;
    use crate::core::domain::monetary_amount::MonetaryAmount;
    use crate::sellers::domain::seller_id::SellerId;
    use crate::tracks_inventory::domain::{TrackId, TrackInventoryId};
    use chrono::NaiveDate;

    #[test]
    fn new_track_inventory_args_to_input() {
        let args = NewTrackInventoryArgs {
            name: "My Inventory".to_string(),
            description: Some("desc".to_string()),
        };

        let input: NewTrackInventoryInput = args.try_into().expect("conversion");

        assert_eq!(input.name, "My Inventory");
        assert_eq!(input.description.unwrap(), "desc");
    }

    #[test]
    fn add_purchase_args_to_input() {
        let args = AddTrackPurchaseArgs {
            id: TrackInventoryId::default(),
            track_id: TrackId::try_from("trn:track:acme:p-100").unwrap(),
            quantity: 3,
            price: MonetaryAmount::new(100, Currency::EUR),
            seller_id: Some(SellerId::try_from("trn:seller:seller-1").unwrap()),
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let input: AddTrackPurchaseInput = args.try_into().expect("conversion");

        assert_eq!(input.quantity, 3);
    }

    #[test]
    fn rename_args_to_input() {
        let args = RenameTrackInventoryArgs {
            id: TrackInventoryId::default(),
            new_name: "New name".to_string(),
        };

        let input: RenameTrackInventoryInput = args.try_into().expect("conversion");

        assert_eq!(input.new_name, "New name");
    }

    #[test]
    fn set_quantity_args_to_input() {
        let args = SetTrackItemQuantityArgs {
            inventory_id: TrackInventoryId::default(),
            track_id: TrackId::try_from("trn:track:acme:p-300").unwrap(),
            quantity: 7,
        };

        let input: SetTrackItemQuantityInput = args.try_into().expect("conversion");

        assert_eq!(input.quantity, 7);
    }
}
