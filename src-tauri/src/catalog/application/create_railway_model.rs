use crate::catalog::application::create_railway_model_input::CreateRailwayModelInput;
use crate::catalog::domain::RailwayModel;
use crate::catalog::domain::availability_status::AvailabilityStatus;
use crate::catalog::domain::category::Category;
use crate::catalog::domain::delivery_date::DeliveryDate;
use crate::catalog::domain::epoch::Epoch;
use crate::catalog::domain::power_method::PowerMethod;
use crate::catalog::domain::product_code::ProductCode;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::rolling_stock_id::RollingStockId;
use crate::catalog::domain::scale::Scale;
use crate::catalog::infrastructure::repository;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use uuid::Uuid;

/// Use case for creating a new railway model.
pub struct CreateRailwayModelUseCase;

impl CreateRailwayModelUseCase {
    /// Execute the use case within a Unit of Work.
    /// Returns the ID of the created railway model.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        input: CreateRailwayModelInput,
    ) -> Result<String, String> {
        // Generate ID for railway model
        let model_id = RailwayModelId::try_from(Uuid::new_v4().to_string())
            .map_err(|e| format!("Failed to generate railway model ID: {}", e))?;

        // Parse and validate domain types
        let product_code = ProductCode::try_from(input.product_code)
            .map_err(|e| format!("Invalid product code: {}", e))?;

        let power_method = input
            .power_method
            .parse::<PowerMethod>()
            .map_err(|e| format!("Invalid power method: {}", e))?;

        let scale =
            Scale::try_from(input.scale.as_str()).map_err(|e| format!("Invalid scale: {}", e))?;

        let epoch = Epoch::from(input.epoch.as_str());

        let category = input
            .category
            .parse::<Category>()
            .map_err(|e| format!("Invalid category: {}", e))?;

        let delivery_date = match input.delivery_date {
            Some(ref s) => {
                Some(DeliveryDate::parse(s).map_err(|e| format!("Invalid delivery date: {}", e))?)
            }
            None => None,
        };

        let availability_status = match input.availability_status {
            Some(ref s) => Some(
                s.parse::<AvailabilityStatus>()
                    .map_err(|e| format!("Invalid availability status: {}", e))?,
            ),
            None => None,
        };

        // Parse rolling stocks (domain models created in repository layer)
        // For now, just collect the inputs
        let rolling_stock_inputs = input.rolling_stocks;

        // Create railway model
        let railway_model = RailwayModel {
            id: model_id.clone(),
            manufacturer: input.manufacturer_id,
            product_code,
            description: input.description,
            details: input.details,
            power_method,
            scale,
            epoch,
            category,
            delivery_date,
            availability_status,
            rolling_stocks: vec![], // Will be populated after insert
        };

        // Insert railway model
        repository::insert_railway_model(&mut unit_of_work.tx, &railway_model)
            .await
            .map_err(|e| format!("Failed to insert railway model: {}", e))?;

        // Insert rolling stocks
        for rs_input in rolling_stock_inputs {
            let rs_id = RollingStockId::new(); // Generate UUID

            repository::insert_rolling_stock(&mut unit_of_work.tx, &model_id, &rs_id, rs_input)
                .await
                .map_err(|e| format!("Failed to insert rolling stock: {}", e))?;
        }

        Ok(model_id.to_string())
    }
}
