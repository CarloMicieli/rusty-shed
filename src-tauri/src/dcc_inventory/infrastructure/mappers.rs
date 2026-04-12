use crate::catalog::domain::railway_model::{PowerMethod, RollingStockCategory};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::domain_error::DomainError;
use crate::dcc_inventory::application::{
    DecoderView, DigitalRollingStockView, DigitalSummary, InstallableRollingStockView,
};
use crate::dcc_inventory::domain::{DccAddress, Decoder, DigitalRollingStock};
use crate::dcc_inventory::infrastructure::entities::{
    DecoderRow, DigitalRollingStockRow, EnrichedRow, InstallableRow, SummaryRow,
};

impl TryFrom<DigitalRollingStockRow> for DigitalRollingStock {
    type Error = DomainError;

    fn try_from(row: DigitalRollingStockRow) -> Result<Self, Self::Error> {
        let dcc_address =
            DccAddress::new(row.dcc_address).map_err(|e| DomainError::Validation(e.to_string()))?;

        let decoder_id = row.installed_decoder_id.ok_or_else(|| {
            DomainError::Validation("missing decoder for digital rolling stock".to_string())
        })?;

        Ok(DigitalRollingStock::reconstitute(
            row.id,
            row.owned_rolling_stock_id,
            dcc_address,
            decoder_id,
        ))
    }
}

impl From<DecoderRow> for Decoder {
    fn from(row: DecoderRow) -> Self {
        Decoder {
            id: row.id,
            manufacturer_id: row.manufacturer_id,
            product_code: row.product_code,
            decoder_type: row.decoder_type,
            protocol: row.protocol,
            decoder_interface: row.decoder_interface,
        }
    }
}

impl TryFrom<EnrichedRow> for DigitalRollingStockView {
    type Error = DomainError;

    fn try_from(row: EnrichedRow) -> Result<Self, Self::Error> {
        let dcc_address =
            DccAddress::new(row.dcc_address).map_err(|e| DomainError::Validation(e.to_string()))?;

        let decoder = DecoderView {
            id: row.decoder_id,
            manufacturer: row
                .manufacturer_name
                .unwrap_or_else(|| "Unknown".to_string()),
            product_code: row.decoder_product_code,
            decoder_type: row.decoder_type,
            protocol: row.decoder_protocol,
            decoder_interface: row.decoder_interface,
        };

        let category = row
            .category
            .ok_or_else(|| {
                DomainError::Validation("missing category for digital rolling stock".to_string())
            })
            .and_then(|c| {
                c.parse::<RollingStockCategory>().map_err(|_| {
                    DomainError::Validation(format!("unknown rolling stock category: {c}"))
                })
            })?;

        let scale = row.scale.as_deref().and_then(|s| Scale::try_from(s).ok());
        let power_method = row
            .power_method
            .as_deref()
            .and_then(|p| PowerMethod::try_from(p).ok());

        Ok(DigitalRollingStockView {
            id: row.id,
            owned_rolling_stock_id: row.owned_rolling_stock_id,
            dcc_address,
            decoder,
            category,
            railway_company_name: row.railway_company_name,
            scale,
            power_method,
            road_number: row.road_number,
            series_code: row.series_code,
            description: row.description,
        })
    }
}

impl From<InstallableRow> for InstallableRollingStockView {
    fn from(row: InstallableRow) -> Self {
        let category = row
            .category
            .and_then(|c| c.parse::<RollingStockCategory>().ok())
            .unwrap_or(RollingStockCategory::Locomotive);

        InstallableRollingStockView {
            owned_rolling_stock_id: row.owned_rolling_stock_id,
            category,
            railway_company_name: row.railway_company_name,
            road_number: row.road_number,
            series_code: row.series_code,
            has_decoder: row.has_decoder != 0,
            dcc_interface: row.dcc_interface,
        }
    }
}

impl From<SummaryRow> for DigitalSummary {
    fn from(row: SummaryRow) -> Self {
        let total_non_dummy = row.total_non_dummy as u32;
        let digital_count = row.digital_count as u32;
        let percentage = if total_non_dummy > 0 {
            (digital_count as f32 / total_non_dummy as f32) * 100.0
        } else {
            0.0
        };

        DigitalSummary {
            total_non_dummy,
            digital_count,
            percentage,
        }
    }
}
