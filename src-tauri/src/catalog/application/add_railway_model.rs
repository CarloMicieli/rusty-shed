use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::Category;
use crate::catalog::domain::railway_model::DeliveryDate;
use crate::catalog::domain::railway_model::Epoch;
use crate::catalog::domain::railway_model::PowerMethod;
use crate::catalog::domain::railway_model::ProductCode;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::localized_field::LocalizedField;
use crate::catalog::domain::railway_model::{AvailabilityStatus, RailwayModelUowExt};
use crate::catalog::domain::railway_model::{
    RailwayModel, RailwayModelEvent, RailwayModelParams, RollingStockParams,
};
use crate::catalog::domain::scale::Scale;
use crate::core::domain::{Language, domain_error::DomainError};
use chrono::Utc;
use garde::Validate;
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
        input.validate().map_err(DomainError::from)?;

        let manufacturer_id = ManufacturerId::try_from(&input.manufacturer_id).map_err(|e| {
            DomainError::validation_general(format!("invalid manufacturer_id: {e}"))
        })?;
        let product_code = ProductCode::try_from(input.product_code.as_str())
            .map_err(|e| DomainError::validation_general(format!("invalid product_code: {e}")))?;
        let power_method = input
            .power_method
            .parse::<PowerMethod>()
            .map_err(|e| DomainError::validation_general(format!("invalid power_method: {e}")))?;
        let scale = Scale::try_from(input.scale.as_str())
            .map_err(|e| DomainError::validation_general(format!("invalid scale: {e}")))?;
        let category = input
            .category
            .parse::<Category>()
            .map_err(|e| DomainError::validation_general(format!("invalid category: {e}")))?;
        let delivery_date = input
            .delivery_date
            .as_ref()
            .map(|s| DeliveryDate::parse(s))
            .transpose()
            .map_err(|e| DomainError::validation_general(format!("invalid delivery_date: {e}")))?;
        let availability_status = input
            .availability_status
            .as_ref()
            .map(|s| s.parse::<AvailabilityStatus>())
            .transpose()
            .map_err(|e| {
                DomainError::validation_general(format!("invalid availability_status: {e}"))
            })?;

        let rolling_stocks = input
            .rolling_stocks
            .into_iter()
            .map(RollingStockParams::try_from)
            .collect::<Result<Vec<RollingStockParams>, DomainError>>()?;

        let railway_model_params = RailwayModelParams {
            manufacturer_id,
            product_code,
            power_method,
            scale,
            category,
            epoch: Epoch::from(input.epoch.as_str()),
            delivery_date,
            availability_status,
            description: input.description,
            details: input.details,
            rolling_stocks,
        };

        // Build the event first (moving railway_model_params, no full-struct clone),
        // then borrow from the event to construct the aggregate.
        let product_code_string = railway_model_params.product_code.to_string();
        let railway_model_id =
            RailwayModelId::new(&railway_model_params.manufacturer_id, &product_code_string)
                .map_err(|e| DomainError::Validation(e.to_string()))?;

        // Extract Copy fields before the move.
        let power_method = railway_model_params.power_method;
        let category = railway_model_params.category;
        let availability_status = railway_model_params.availability_status;

        let created_event = RailwayModelEvent::RailwayModelCreated {
            event_id: Uuid::new_v4(),
            railway_model_id: railway_model_id.clone(),
            timestamp: Utc::now().naive_utc(),
            params: railway_model_params, // moved — eliminates the whole-struct clone
        };

        // Borrow params from the event to build the aggregate.
        let params =
            if let RailwayModelEvent::RailwayModelCreated { ref params, .. } = created_event {
                params
            } else {
                unreachable!()
            };

        let mut aggregate = RailwayModel {
            id: railway_model_id.clone(),
            manufacturer_id: params.manufacturer_id.clone(),
            product_code: params.product_code.clone(),
            description: LocalizedField {
                lang: Language::English,
                value: params.description.clone(),
            },
            details: params.details.clone().map(|v| LocalizedField {
                lang: Language::English,
                value: v,
            }),
            power_method,
            scale: params.scale.clone(),
            epoch: params.epoch.clone(),
            category,
            delivery_date: params.delivery_date.clone(),
            availability_status,
            rolling_stocks: Vec::new(),
            pending_events: Vec::new(),
        };

        aggregate.push_event(created_event);

        repository
            .save(&mut aggregate)
            .await
            .map(|_| railway_model_id)
    }
}

/// Input for creating a new railway model.
#[derive(Debug, Clone, Validate)]
// `details` and `rolling_stocks` are intentionally validated in later domain conversion steps.
#[garde(allow_unvalidated)]
pub struct CreateRailwayModelInput {
    /// Manufacturer identifier as a string.
    #[garde(
        length(min = 1),
        custom(crate::catalog::domain::manufacturer::validate_manufacturer_id)
    )]
    pub manufacturer_id: String,
    /// Display name of the manufacturer.
    #[garde(length(min = 1, max = 20))]
    pub product_code: String,
    /// Description of the railway model.
    #[garde(length(min = 1, max = 500))]
    pub description: String,
    /// Additional details about the railway model.
    pub details: Option<String>,
    /// Power method used by the railway model.
    #[garde(custom(crate::catalog::domain::railway_model::power_method::validate_power_method))]
    pub power_method: String,
    /// Scale of the railway model.
    #[garde(custom(crate::catalog::domain::scale::scale::validate_scale))]
    pub scale: String,
    /// Epoch of the railway model.
    #[garde(length(min = 1, max = 10))]
    pub epoch: String,
    /// Category of the railway model.
    #[garde(custom(crate::catalog::domain::railway_model::category::validate_category))]
    pub category: String,
    /// Optional delivery date of the railway model.
    #[garde(custom(validate_opt_delivery_date))]
    pub delivery_date: Option<String>,
    /// Optional availability status of the railway model.
    #[garde(custom(
        crate::catalog::domain::railway_model::availability_status::validate_opt_availability_status
    ))]
    pub availability_status: Option<String>,
    /// Rolling stocks associated with the railway model.
    pub rolling_stocks: Vec<CreateRollingStockInput>,
}

fn validate_opt_delivery_date(value: &Option<String>, _: &()) -> garde::Result {
    match value {
        Some(v) => DeliveryDate::parse(v)
            .map(|_| ())
            .map_err(|_| garde::Error::new("error_invalid_delivery_date")),
        None => Ok(()),
    }
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
    use crate::catalog::domain::railway_model::{
        MockRailwayModelRepository, RailwayModelEvent, RollingStockParams,
    };
    use pretty_assertions::assert_eq;

    fn valid_input() -> CreateRailwayModelInput {
        CreateRailwayModelInput {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "P999".to_string(),
            power_method: "DC".to_string(),
            scale: "H0".to_string(),
            category: "LOCOMOTIVES".to_string(),
            epoch: "IV".to_string(),
            delivery_date: None,
            availability_status: None,
            description: "A fine locomotive".to_string(),
            details: None,
            rolling_stocks: vec![],
        }
    }

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
            assert_eq!(e.len(), 8);
            let errors = e;
            assert!(errors.contains_key("product_code"));
            assert!(errors.contains_key("description"));
            assert!(errors.contains_key("power_method"));
            assert!(errors.contains_key("scale"));
            assert!(errors.contains_key("epoch"));
            assert!(errors.contains_key("category"));
            assert!(errors.contains_key("availability_status"));
            assert!(errors.contains_key("delivery_date"));
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn it_creates_railway_model_on_success_path() {
        let mut mock = MockRailwayModelRepository::new();
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut unit_of_work = FakeUow::with_railway_models_repo(mock);

        let input = valid_input();

        let id = AddRailwayModel::execute(&mut unit_of_work, input)
            .await
            .expect("should create railway model");

        assert!(
            id.to_string().contains("acme"),
            "returned id should reference the manufacturer"
        );
    }

    #[tokio::test]
    async fn it_rejects_invalid_nested_rolling_stock_data() {
        let mock = MockRailwayModelRepository::new();
        let mut unit_of_work = FakeUow::with_railway_models_repo(mock);

        let mut input = valid_input();
        input.rolling_stocks = vec![CreateRollingStockInput::Locomotive {
            railway_company_id: "not-a-trn".to_string(),
            friendly_name: "Test loco".to_string(),
            series_code: "E444".to_string(),
            road_number: "001".to_string(),
            series: None,
            depot: None,
            livery: None,
            locomotive_type: "ELECTRIC_LOCOMOTIVE".to_string(),
            is_dummy: Some(false),
            control: None,
            dcc_interface: None,
            length_over_buffers: None,
            technical_specifications: None,
        }];

        let result = AddRailwayModel::execute(&mut unit_of_work, input).await;

        match result {
            Err(DomainError::ValidationError(errors)) => {
                assert!(errors.contains_key("railway_company_id"), "{errors:?}");
            }
            other => panic!("expected nested validation error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn it_persists_converted_rolling_stock_in_created_aggregate() {
        let mut mock = MockRailwayModelRepository::new();
        mock.expect_save().times(1).returning(|aggregate| {
            assert_eq!(aggregate.pending_events.len(), 1);
            assert!(matches!(
                aggregate.pending_events.first(),
                Some(RailwayModelEvent::RailwayModelCreated { params, .. })
                    if matches!(
                        params.rolling_stocks.first(),
                        Some(RollingStockParams::LocomotiveParams {
                            railway_company_id,
                            series_code,
                            road_number,
                            ..
                        }) if railway_company_id.as_ref() == "trn:railway-company:fs"
                            && series_code.as_deref() == Some("E444")
                            && road_number == "001"
                    )
            ));
            Ok(())
        });

        let mut unit_of_work = FakeUow::with_railway_models_repo(mock);

        let mut input = valid_input();
        input.rolling_stocks = vec![CreateRollingStockInput::Locomotive {
            railway_company_id: "trn:railway-company:fs".to_string(),
            friendly_name: "Test loco".to_string(),
            series_code: "E444".to_string(),
            road_number: "001".to_string(),
            series: Some("E.444".to_string()),
            depot: Some("Roma Smistamento".to_string()),
            livery: Some("XMPR".to_string()),
            locomotive_type: "ELECTRIC_LOCOMOTIVE".to_string(),
            is_dummy: Some(false),
            control: None,
            dcc_interface: None,
            length_over_buffers: None,
            technical_specifications: None,
        }];

        let id = AddRailwayModel::execute(&mut unit_of_work, input)
            .await
            .expect("should create railway model");

        assert!(id.to_string().contains("acme"));
    }
}
