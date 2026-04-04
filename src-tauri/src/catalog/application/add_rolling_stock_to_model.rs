use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::{
    Control, Coupling, CouplingSocket, DccInterface, ElectricMultipleUnitType, FeatureFlag,
    FreightCarType, LocomotiveType, PassengerCarType, RailcarType, RailwayModelId,
    RailwayModelUowExt, RollingStockCategory, RollingStockId, RollingStockParams,
    TechnicalSpecifications, TechnicalSpecificationsBuilder,
};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;

/// Input for [`AddRollingStockToModel::execute`].
#[derive(Debug)]
pub struct AddRollingStockToModelInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The railway company that operated this rolling stock.
    pub railway_company_id: RailwayCompanyId,
    /// Category of the rolling stock (determines the variant to create).
    pub category: RollingStockCategory,
    /// Series code identifying this variant (required, non-empty).
    pub series_code: String,
    /// Optional road/fleet number.
    pub road_number: Option<String>,
    /// Optional livery description.
    pub livery: Option<String>,
    /// Optional depot name.
    pub depot: Option<String>,
    /// Optional control type.
    pub control: Option<Control>,
    /// Optional DCC decoder interface connector.
    pub dcc_interface: Option<DccInterface>,
    /// Optional coupling socket standard (e.g. "NEM_362").
    pub coupling_socket: Option<String>,
    /// Optional short-coupler flag. Only meaningful when `coupling_socket` is `Some`.
    pub close_couplers: Option<bool>,
    /// Optional category-specific sub-type string.
    pub sub_type: Option<String>,
    /// Optional display/friendly name (falls back to series_code if absent).
    pub friendly_name: Option<String>,
    /// Optional prototype this rolling stock is linked to.
    pub prototype_id: Option<String>,
}

/// Use case that adds a new rolling stock variant to an existing [`RailwayModel`] aggregate.
pub struct AddRollingStockToModel;

impl AddRollingStockToModel {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: AddRollingStockToModelInput,
    ) -> Result<RollingStockId, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, "en")
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        let params = build_params(input);
        let rs_id = model.add_rolling_stock(params);
        repo.save(&mut model).await?;

        Ok(rs_id)
    }
}

/// Build `RollingStockParams` from the input using per-category defaults.
fn build_params(input: AddRollingStockToModelInput) -> RollingStockParams {
    let coupling_socket = input
        .coupling_socket
        .as_deref()
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<CouplingSocket>().ok());

    let technical_specifications: Option<TechnicalSpecifications> = coupling_socket.map(|socket| {
        let close_couplers = input
            .close_couplers
            .map(|v| if v { FeatureFlag::Yes } else { FeatureFlag::No });
        let coupling = Coupling {
            socket: Some(socket),
            close_couplers,
            digital_shunting: None,
        };
        TechnicalSpecificationsBuilder::default()
            .with_coupling(coupling)
            .build()
    });

    match input.category {
        RollingStockCategory::Locomotive => {
            let locomotive_type = input
                .sub_type
                .as_deref()
                .and_then(|s| s.parse::<LocomotiveType>().ok())
                .unwrap_or(LocomotiveType::ElectricLocomotive);
            RollingStockParams::LocomotiveParams {
                railway_company_id: input.railway_company_id,
                friendly_name: input
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| input.series_code.clone()),
                series_code: Some(input.series_code),
                road_number: input.road_number.unwrap_or_default(),
                series: None,
                depot: input.depot,
                livery: input.livery,
                locomotive_type,
                dcc_interface: input.dcc_interface,
                control: input.control,
                is_dummy: false,
                length_over_buffers: None,
                technical_specifications,
            }
        }
        RollingStockCategory::ElectricMultipleUnit => {
            let electric_multiple_unit_type = input
                .sub_type
                .as_deref()
                .and_then(|s| s.parse::<ElectricMultipleUnitType>().ok())
                .unwrap_or(ElectricMultipleUnitType::MotorCar);
            RollingStockParams::ElectricMultipleUnitParams {
                railway_company_id: input.railway_company_id,
                friendly_name: input
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| input.series_code.clone()),
                series_code: Some(input.series_code),
                road_number: input.road_number,
                series: None,
                depot: input.depot,
                livery: input.livery,
                electric_multiple_unit_type,
                dcc_interface: input.dcc_interface,
                control: input.control,
                is_dummy: false,
                length_over_buffers: None,
                technical_specifications,
            }
        }
        RollingStockCategory::Railcar => {
            let railcar_type = input
                .sub_type
                .as_deref()
                .and_then(|s| s.parse::<RailcarType>().ok())
                .unwrap_or(RailcarType::PowerCar);
            RollingStockParams::RailcarParams {
                railway_company_id: input.railway_company_id,
                friendly_name: input
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| input.series_code.clone()),
                series_code: Some(input.series_code),
                road_number: input.road_number,
                series: None,
                depot: input.depot,
                livery: input.livery,
                railcar_type,
                dcc_interface: input.dcc_interface,
                control: input.control,
                is_dummy: false,
                length_over_buffers: None,
                technical_specifications,
            }
        }
        RollingStockCategory::PassengerCar => {
            let passenger_car_type = input
                .sub_type
                .as_deref()
                .and_then(|s| s.parse::<PassengerCarType>().ok());
            RollingStockParams::PassengerCarParams {
                railway_company_id: input.railway_company_id,
                friendly_name: input
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| input.series_code.clone()),
                series_code: Some(input.series_code),
                road_number: input.road_number,
                series: None,
                livery: input.livery,
                passenger_car_type,
                service_level: None,
                length_over_buffers: None,
                technical_specifications,
            }
        }
        RollingStockCategory::FreightCar => {
            let freight_car_type = input
                .sub_type
                .as_deref()
                .and_then(|s| s.parse::<FreightCarType>().ok());
            RollingStockParams::FreightCarParams {
                railway_company_id: input.railway_company_id,
                friendly_name: input
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| input.series_code.clone()),
                series_code: Some(input.series_code),
                road_number: input.road_number,
                series: None,
                livery: input.livery,
                freight_car_type,
                length_over_buffers: None,
                technical_specifications,
            }
        }
    }
}

/// Parse `AddRollingStockToModelArgs` strings into domain types, collecting all errors.
#[allow(clippy::too_many_arguments)]
pub fn parse_add_rolling_stock_args(
    railway_model_id: String,
    railway_company_id: String,
    category: String,
    series_code: String,
    road_number: Option<String>,
    livery: Option<String>,
    depot: Option<String>,
    control: Option<String>,
    dcc_interface: Option<String>,
    coupling_socket: Option<String>,
    close_couplers: Option<bool>,
    sub_type: Option<String>,
    friendly_name: Option<String>,
    prototype_id: Option<String>,
) -> Result<AddRollingStockToModelInput, DomainError> {
    let mut ctx = ValidationContext::default();

    let model_id = ctx.collect(
        "railway_model_id",
        RailwayModelId::try_from(railway_model_id.as_str()),
    );
    let company_id = ctx.collect(
        "railway_company_id",
        RailwayCompanyId::try_from(railway_company_id.as_str()),
    );
    let cat = ctx.collect("category", category.parse::<RollingStockCategory>());
    let control_val = control.and_then(|s| ctx.collect("control", s.parse::<Control>()));

    // DCC interface is optional and non-fatal — silently ignore unrecognised values.
    let dcc_interface_val = dcc_interface
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<DccInterface>().ok());

    if series_code.trim().is_empty() {
        ctx.push_error("series_code", "length", "series_code must not be empty");
    }

    ctx.finish()?;

    Ok(AddRollingStockToModelInput {
        railway_model_id: model_id.unwrap(),
        railway_company_id: company_id.unwrap(),
        category: cat.unwrap(),
        series_code,
        road_number,
        livery,
        depot,
        control: control_val,
        dcc_interface: dcc_interface_val,
        coupling_socket: coupling_socket.filter(|s| !s.is_empty()),
        close_couplers,
        sub_type: sub_type.filter(|s| !s.is_empty()),
        friendly_name: friendly_name.filter(|s| !s.is_empty()),
        prototype_id: prototype_id.filter(|s| !s.is_empty()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::Language;

    fn make_empty_model(model_id: RailwayModelId) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P100").unwrap();
        RailwayModel {
            id: model_id,
            manufacturer_id: manufacturer,
            product_code: product,
            description: LocalizedField {
                lang: Language::English,
                value: "Test".to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: vec![],
        }
    }

    fn base_input(
        category: RollingStockCategory,
        model_id: RailwayModelId,
    ) -> AddRollingStockToModelInput {
        AddRollingStockToModelInput {
            railway_model_id: model_id,
            railway_company_id: RailwayCompanyId::try_from("trn:railway-company:fs").unwrap(),
            category,
            series_code: "E.656".to_string(),
            road_number: None,
            livery: None,
            depot: None,
            control: None,
            dcc_interface: None,
            coupling_socket: None,
            close_couplers: None,
            sub_type: None,
            friendly_name: None,
            prototype_id: None,
        }
    }

    fn model_id() -> RailwayModelId {
        RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap()
    }

    #[tokio::test]
    async fn adds_locomotive_successfully() {
        let mid = model_id();
        let model = make_empty_model(mid.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let rs_id = AddRollingStockToModel::execute(
            &mut uow,
            base_input(RollingStockCategory::Locomotive, mid),
        )
        .await
        .expect("should succeed");

        assert!(!rs_id.to_string().is_empty());
    }

    #[tokio::test]
    async fn adds_passenger_car_successfully() {
        let mid = model_id();
        let model = make_empty_model(mid.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        AddRollingStockToModel::execute(
            &mut uow,
            base_input(RollingStockCategory::PassengerCar, mid),
        )
        .await
        .expect("passenger car should succeed");
    }

    #[tokio::test]
    async fn adds_freight_car_successfully() {
        let mid = model_id();
        let model = make_empty_model(mid.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        AddRollingStockToModel::execute(
            &mut uow,
            base_input(RollingStockCategory::FreightCar, mid),
        )
        .await
        .expect("freight car should succeed");
    }

    #[tokio::test]
    async fn returns_not_found_when_model_missing() {
        let mid = model_id();

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let err = AddRollingStockToModel::execute(
            &mut uow,
            base_input(RollingStockCategory::Locomotive, mid),
        )
        .await
        .expect_err("missing model should fail");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }

    #[test]
    fn parse_rejects_empty_series_code() {
        let err = parse_add_rolling_stock_args(
            "trn:railway-model:acme:p100".to_string(),
            "trn:railway-company:fs".to_string(),
            "LOCOMOTIVE".to_string(),
            "".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .expect_err("empty series_code should fail");

        assert!(
            matches!(err, DomainError::ValidationError(_)),
            "expected ValidationError, got {err:?}"
        );
    }
}
