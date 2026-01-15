use crate::catalog::application::railway_model_use_case_input::CreateRailwayModelInput;
use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::Category;
use crate::catalog::domain::railway_model::DeliveryDate;
use crate::catalog::domain::railway_model::Epoch;
use crate::catalog::domain::railway_model::PowerMethod;
use crate::catalog::domain::railway_model::ProductCode;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::{AvailabilityStatus, RailwayModelUowExt};
use crate::catalog::domain::railway_model::{RailwayModelParams, RollingStockParams};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;

/// Use case for creating a new railway model.
pub struct CreateRailwayModelUseCase;

impl CreateRailwayModelUseCase {
    /// Execute the use case to create a new railway model.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `input` - The input data for creating the railway model.
    ///
    /// # Returns
    /// * `Ok(RailwayModelId)` containing the new railway model ID on success,
    /// * `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// * `U` - The type of the unit of work, which must implement `RailwayModelUowExt` and be `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: CreateRailwayModelInput,
    ) -> Result<RailwayModelId, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        let mut validation_context = ValidationContext::default();

        // Collect all potential failures
        let manufacturer_id = validation_context.collect(
            "manufacturer_id",
            ManufacturerId::try_from(&input.manufacturer_id),
        );
        let product_code =
            validation_context.collect("product_code", ProductCode::try_from(input.product_code));
        let power_method =
            validation_context.collect("power_method", input.power_method.parse::<PowerMethod>());
        let scale = validation_context.collect("scale", Scale::try_from(input.scale.as_str()));
        let category = validation_context.collect("category", input.category.parse::<Category>());

        let delivery_date = input
            .delivery_date
            .as_ref()
            .and_then(|s| validation_context.collect("delivery_date", DeliveryDate::parse(s)));

        let availability_status = input.availability_status.as_ref().and_then(|s| {
            validation_context.collect("availability_status", s.parse::<AvailabilityStatus>())
        });

        // Checkpoint: Stop if validation failed
        validation_context.finish()?;

        let rolling_stocks = input
            .rolling_stocks
            .into_iter()
            .map(RollingStockParams::try_from)
            .collect::<Result<Vec<RollingStockParams>, DomainError>>()?;

        // Final Assembly (Safe unwraps because ctx.finish() passed)
        let railway_model_params = RailwayModelParams {
            manufacturer_id: manufacturer_id.unwrap(),
            product_code: product_code.unwrap(),
            power_method: power_method.unwrap(),
            scale: scale.unwrap(),
            category: category.unwrap(),
            epoch: Epoch::from(input.epoch.as_str()),
            delivery_date,
            availability_status,
            description: input.description,
            details: input.details,
            rolling_stocks,
        };

        repository.create(&railway_model_params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_should_validate_railway_model_inputs() {
        let mock = MockRailwayModelRepository::new();
        let mut unit_of_work = FakeUow::with_railway_models_repo(mock);

        let input = CreateRailwayModelInput {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "".to_string(),
            power_method: "".to_string(),
            scale: "".to_string(),
            category: "".to_string(),
            epoch: "".to_string(),
            delivery_date: Some("INVALID".to_string()),
            availability_status: Some("INVALID".to_string()),
            description: "".to_string(),
            details: Some("Detailed info about the test locomotive".to_string()),
            rolling_stocks: vec![],
        };

        let result = CreateRailwayModelUseCase::execute(&mut unit_of_work, input).await;

        if let Err(DomainError::ValidationError(e)) = result {
            assert_eq!(e.len(), 6);
            let errors = e;
            assert!(errors.contains_key("product_code"));
            assert!(errors.contains_key("power_method"));
            assert!(errors.contains_key("scale"));
            assert!(errors.contains_key("category"));
            assert!(errors.contains_key("availability_status"));
            assert!(errors.contains_key("delivery_date"));
        } else {
            panic!("Expected validation error");
        }
    }
}
