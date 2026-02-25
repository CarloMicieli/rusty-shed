use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::localized_field::LocalizedField;
use crate::catalog::domain::railway_model::Category;
use crate::catalog::domain::railway_model::DeliveryDate;
use crate::catalog::domain::railway_model::Epoch;
use crate::catalog::domain::railway_model::PowerMethod;
use crate::catalog::domain::railway_model::ProductCode;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::{AvailabilityStatus, RailwayModelUowExt};
use crate::catalog::domain::railway_model::{
    RailwayModel, RailwayModelEvent, RailwayModelParams, RollingStockParams,
};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;
use chrono::Utc;
use uuid::Uuid;

/// Use case for creating a new railway model.
pub struct AddRailwayModel;

impl AddRailwayModel {
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

        // Aggregate-first approach: construct the RailwayModel aggregate,
        // emit a Created event (carrying the params), then persist via save().
        let product_code_string = railway_model_params.product_code.to_string();
        let railway_model_id =
            RailwayModelId::new(&railway_model_params.manufacturer_id, &product_code_string)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

        let mut aggregate = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer_id: railway_model_params.manufacturer_id.clone(),
            product_code: railway_model_params.product_code.clone(),
            description: LocalizedField {
                lang: "en".to_string(),
                value: railway_model_params.description.clone(),
            },
            details: railway_model_params
                .details
                .clone()
                .map(|v| LocalizedField { lang: "en".to_string(), value: v }),
            power_method: railway_model_params.power_method,
            scale: railway_model_params.scale.clone(),
            epoch: railway_model_params.epoch.clone(),
            category: railway_model_params.category,
            delivery_date: railway_model_params.delivery_date.clone(),
            availability_status: railway_model_params.availability_status,
            rolling_stocks: Vec::new(),
            pending_events: Vec::new(),
        };

        let created_event = RailwayModelEvent::RailwayModelCreated {
            event_id: Uuid::new_v4(),
            railway_model_id: railway_model_id.clone(),
            timestamp: Utc::now().naive_utc(),
            params: railway_model_params.clone(),
        };

        aggregate.push_event(created_event);

        // Persist aggregate by applying its events.
        repository
            .save(&mut aggregate)
            .await
            .map(|_| railway_model_id)
    }
}

/// Input for creating a new railway model.
#[derive(Debug, Clone)]
pub struct CreateRailwayModelInput {
    /// Manufacturer identifier as a string.
    pub manufacturer_id: String,
    /// Display name of the manufacturer.
    pub product_code: String,
    /// Description of the railway model.
    pub description: String,
    /// Additional details about the railway model.
    pub details: Option<String>,
    /// Power method used by the railway model.
    pub power_method: String,
    /// Scale of the railway model.
    pub scale: String,
    /// Epoch of the railway model.
    pub epoch: String,
    /// Category of the railway model.
    pub category: String,
    /// Optional delivery date of the railway model.
    pub delivery_date: Option<String>,
    /// Optional availability status of the railway model.
    pub availability_status: Option<String>,
    /// Rolling stocks associated with the railway model.
    pub rolling_stocks: Vec<CreateRollingStockInput>,
}

/// Input for creating a rolling stock.
#[derive(Debug, Clone)]
pub enum CreateRollingStockInput {
    /// Locomotive-specific input fields.
    Locomotive {
        /// Railway company identifier as a string.
        railway_company_id: String,
        /// Friendly name of the rolling stock.
        friendly_name: String,
        /// Series code of the rolling stock.
        series_code: String,
        /// Road number of the rolling stock.
        road_number: String,
        /// Series of the rolling stock.
        series: Option<String>,
        /// Depot of the rolling stock.
        depot: Option<String>,
        /// Livery of the rolling stock.
        livery: Option<String>,
        /// Type of the locomotive.
        locomotive_type: String,
        /// Indicates if the locomotive is a dummy.
        is_dummy: Option<bool>,
        /// Control type of the locomotive.
        control: Option<String>,
        /// DCC interface of the locomotive.
        dcc_interface: Option<String>,
        /// Length over buffers of the rolling stock.
        length_over_buffers: Option<LengthOverBuffersInput>,
        /// Technical specifications of the rolling stock.
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Passenger car-specific input fields.
    PassengerCar {
        /// Railway company identifier as a string.
        railway_company_id: String,
        /// Friendly name of the rolling stock.
        friendly_name: String,
        /// Series code of the rolling stock.
        series_code: String,
        /// Road number of the rolling stock.
        road_number: Option<String>,
        /// Series of the rolling stock.
        series: Option<String>,
        /// Livery of the rolling stock.
        livery: Option<String>,
        /// Type of the passenger car.
        passenger_car_type: String,
        /// Service level of the passenger car.
        service_level: Option<String>,
        /// Length over buffers of the rolling stock.
        length_over_buffers: Option<LengthOverBuffersInput>,
        /// Technical specifications of the rolling stock.
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Freight car-specific input fields.
    FreightCar {
        /// Railway company identifier as a string.
        railway_company_id: String,
        /// Friendly name of the rolling stock.
        friendly_name: String,
        /// Series code of the rolling stock.
        series_code: String,
        /// Road number of the rolling stock.
        road_number: Option<String>,
        /// Series of the rolling stock.
        series: Option<String>,
        /// Livery of the rolling stock.
        livery: Option<String>,
        /// Type of the freight car.
        freight_car_type: Option<String>,
        /// Length over buffers of the rolling stock.
        length_over_buffers: Option<LengthOverBuffersInput>,
        /// Technical specifications of the rolling stock.
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Railcar-specific input fields.
    Railcar {
        /// Railway company identifier as a string.
        railway_company_id: String,
        /// Friendly name of the rolling stock.
        friendly_name: String,
        /// Series code of the rolling stock.
        series_code: String,
        /// Road number of the rolling stock.
        road_number: Option<String>,
        /// Series of the rolling stock.
        series: Option<String>,
        /// Depot of the rolling stock.
        depot: Option<String>,
        /// Livery of the rolling stock.
        livery: Option<String>,
        /// Type of the railcar.
        railcar_type: Option<String>,
        /// Indicates if the railcar is a dummy.
        is_dummy: Option<bool>,
        /// Control type of the railcar.
        control: Option<String>,
        /// DCC interface of the railcar.
        dcc_interface: Option<String>,
        /// Length over buffers of the rolling stock.
        length_over_buffers: Option<LengthOverBuffersInput>,
        /// Technical specifications of the rolling stock.
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
    /// Electric multiple unit-specific input fields.
    ElectricMultipleUnit {
        /// Railway company identifier as a string.
        railway_company_id: String,
        /// Friendly name of the rolling stock.
        friendly_name: String,
        /// Series code of the rolling stock.
        series_code: String,
        /// Road number of the rolling stock.
        road_number: Option<String>,
        /// Series of the rolling stock.
        series: Option<String>,
        /// Depot of the rolling stock.
        depot: Option<String>,
        /// Livery of the rolling stock.
        livery: Option<String>,
        /// Type of the electric multiple unit.
        electric_multiple_unit_type: String,
        /// Indicates if the electric multiple unit is a dummy.
        is_dummy: Option<bool>,
        /// Control type of the electric multiple unit.
        control: Option<String>,
        /// DCC interface of the electric multiple unit.
        dcc_interface: Option<String>,
        /// Length over buffers of the rolling stock.
        length_over_buffers: Option<LengthOverBuffersInput>,
        /// Technical specifications of the rolling stock.
        technical_specifications: Option<TechnicalSpecificationsInput>,
    },
}

/// Length measurements over buffers for a rolling stock item.
#[derive(Debug, Clone)]
pub struct LengthOverBuffersInput {
    /// Length in millimeters.
    pub millimeters: Option<f64>,
    /// Length in inches.
    pub inches: Option<f64>,
}

/// Optional technical specifications for a rolling stock item.
#[derive(Debug, Clone)]
pub struct TechnicalSpecificationsInput {
    /// Minimum radius in millimeters.
    pub minimum_radius: Option<f64>,
    /// Coupling details.
    pub coupling: Option<CouplingInput>,
    /// Indicates if a flywheel is fitted.
    pub flywheel_fitted: Option<String>,
    /// Body shell details.
    pub body_shell: Option<String>,
    /// Chassis details.
    pub chassis: Option<String>,
    /// Interior lights details.
    pub interior_lights: Option<String>,
    /// Lights details.
    pub lights: Option<String>,
    /// Sprung buffers details.
    pub sprung_buffers: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CouplingInput {
    /// Socket type of the coupling.
    pub socket: String,
    /// Optional coupling behaviours.
    pub close_couplers: Option<String>,
    /// Optional digital shunting details.
    pub digital_shunting: Option<String>,
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

        let result = AddRailwayModel::execute(&mut unit_of_work, input).await;

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
