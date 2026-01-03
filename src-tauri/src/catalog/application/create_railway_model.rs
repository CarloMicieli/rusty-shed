use crate::catalog::application::create_railway_model_input::CreateRailwayModelInput;
use crate::catalog::domain::availability_status::AvailabilityStatus;
use crate::catalog::domain::category::Category;
use crate::catalog::domain::delivery_date::DeliveryDate;
use crate::catalog::domain::epoch::Epoch;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::params::RailwayModelParams;
use crate::catalog::domain::power_method::PowerMethod;
use crate::catalog::domain::product_code::ProductCode;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::scale::Scale;
use crate::catalog::infrastructure::repository::CatalogUowExt;
use crate::core::application::validation::ValidationContext;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

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
    /// * `Err(CommandError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        input: CreateRailwayModelInput,
    ) -> Result<RailwayModelId, DomainError> {
        let mut repository = unit_of_work.catalog_repository();
        let mut validation_context = ValidationContext::default();

        let manufacturer_id = ManufacturerId::new(&input.manufacturer_id);

        // Collect all potential failures
        let railway_model_id = validation_context.collect(
            "railway_model_id",
            RailwayModelId::new(&manufacturer_id, &input.product_code),
        );
        let product_code = validation_context.collect("product_code", ProductCode::try_from(input.product_code));
        let power_method = validation_context.collect("power_method", input.power_method.parse::<PowerMethod>());
        let scale = validation_context.collect("scale", Scale::try_from(input.scale.as_str()));
        let category = validation_context.collect("category", input.category.parse::<Category>());

        let delivery_date = input
            .delivery_date
            .as_ref()
            .and_then(|s| validation_context.collect("delivery_date", DeliveryDate::parse(s)));

        let availability_status = input
            .availability_status
            .as_ref()
            .and_then(|s| validation_context.collect("availability_status", s.parse::<AvailabilityStatus>()));

        // Checkpoint: Stop if validation failed
        validation_context.finish()?;

        // Final Assembly (Safe unwraps because ctx.finish() passed)
        let railway_model_params = RailwayModelParams {
            manufacturer_id,
            product_code: product_code.unwrap(),
            power_method: power_method.unwrap(),
            scale: scale.unwrap(),
            category: category.unwrap(),
            epoch: Epoch::from(input.epoch.as_str()),
            delivery_date,
            availability_status,
            description: input.description,
            details: input.details,
            rolling_stocks: vec![],
        };

        repository
            .create(&railway_model_params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_railway_model_use_case(pool: sqlx::SqlitePool) {
        let mut unit_of_work = SqliteUnitOfWork::new(&pool).await.unwrap();

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
