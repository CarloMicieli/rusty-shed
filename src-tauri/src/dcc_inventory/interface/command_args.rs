use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;
use crate::dcc_inventory::application::change_dcc_address::ChangeDccAddressInput;
use crate::dcc_inventory::application::change_decoder::ChangeDecoderInput;
use crate::dcc_inventory::application::new_digital_rolling_stock::NewDigitalRollingStockInput;
use crate::dcc_inventory::domain::{DccAddress, DecoderId, DigitalRollingStockId};
use garde::Validate;
use serde::Deserialize;

/// Arguments for creating a new Digital Rolling Stock.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct NewDigitalRollingStockArgs {
    /// The owned rolling stock id.
    pub owned_rolling_stock_id: String,
    /// The DCC address.
    pub dcc_address: u16,
    /// The decoder id.
    pub decoder_id: String,
}

/// Response for created digital rolling stock: returns the new id.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct ResponseNewDigitalRollingStock {
    /// The new digital rolling stock id.
    pub id: DigitalRollingStockId,
}

/// Arguments for changing a DCC address.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDccAddressArgs {
    /// The digital rolling stock id.
    pub id: String,
    /// The new DCC address.
    pub new_dcc_address: u16,
}

/// Arguments for changing a decoder.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDecoderArgs {
    pub id: String,
    pub decoder_id: String,
}

/// Arguments for checking DCC address duplicates.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CheckDccAddressDuplicateArgs {
    /// The DCC address to check (1-9999).
    pub dcc_address: u16,
    /// Optional: exclude this ID from the check (for edit mode).
    pub exclude_id: Option<String>,
}

// TryFrom conversions
impl TryFrom<NewDigitalRollingStockArgs> for NewDigitalRollingStockInput {
    type Error = DomainError;

    fn try_from(input: NewDigitalRollingStockArgs) -> Result<Self, Self::Error> {
        let mut ctx = ValidationContext::default();

        let owned_rolling_stock_id = ctx.validate_try_from::<OwnedRollingStockId>(
            "owned_rolling_stock_id",
            input.owned_rolling_stock_id,
        );

        let dcc_address = ctx.collect("dcc_address", DccAddress::new(input.dcc_address));

        let decoder_id = ctx.validate_try_from::<DecoderId>("decoder_id", input.decoder_id);

        ctx.finish()?;

        Ok(NewDigitalRollingStockInput {
            owned_rolling_stock_id: owned_rolling_stock_id.unwrap(),
            dcc_address: dcc_address.unwrap(),
            decoder_id: decoder_id.unwrap(),
        })
    }
}

impl TryFrom<ChangeDccAddressArgs> for ChangeDccAddressInput {
    type Error = DomainError;

    fn try_from(input: ChangeDccAddressArgs) -> Result<Self, Self::Error> {
        let mut ctx = ValidationContext::default();

        let id = ctx.validate_try_from::<DigitalRollingStockId>("id", input.id);
        let new_dcc_address =
            ctx.collect("new_dcc_address", DccAddress::new(input.new_dcc_address));

        ctx.finish()?;

        Ok(ChangeDccAddressInput {
            id: id.unwrap(),
            new_dcc_address: new_dcc_address.unwrap(),
        })
    }
}

impl TryFrom<ChangeDecoderArgs> for ChangeDecoderInput {
    type Error = DomainError;

    fn try_from(input: ChangeDecoderArgs) -> Result<Self, Self::Error> {
        let mut ctx = ValidationContext::default();

        let id = ctx.validate_try_from::<DigitalRollingStockId>("id", input.id);
        let decoder_id = ctx.validate_try_from::<DecoderId>("decoder_id", input.decoder_id);

        ctx.finish()?;

        Ok(ChangeDecoderInput {
            id: id.unwrap(),
            decoder_id: decoder_id.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dcc_inventory::application::new_digital_rolling_stock::NewDigitalRollingStockInput;
    use uuid::Uuid;

    #[test]
    fn new_digital_rolling_stock_args_try_from_ok() {
        let args = NewDigitalRollingStockArgs {
            owned_rolling_stock_id: format!("trn:owned-rolling-stock:{}", Uuid::new_v4()),
            dcc_address: 123,
            decoder_id: "trn:decoder:acme:d-100".to_string(),
        };

        let _input =
            NewDigitalRollingStockInput::try_from(args).expect("conversion should succeed");
    }

    #[test]
    fn change_dcc_address_args_try_from_ok() {
        let args = ChangeDccAddressArgs {
            id: format!("trn:digital-rolling-stock:{}", Uuid::new_v4()),
            new_dcc_address: 500,
        };

        let _input = ChangeDccAddressInput::try_from(args).expect("conversion should succeed");
    }

    #[test]
    fn change_decoder_args_try_from_ok() {
        let args = ChangeDecoderArgs {
            id: format!("trn:digital-rolling-stock:{}", Uuid::new_v4()),
            decoder_id: "trn:decoder:acme:d-100".to_string(),
        };

        let _input = ChangeDecoderInput::try_from(args).expect("conversion should succeed");
    }
}
