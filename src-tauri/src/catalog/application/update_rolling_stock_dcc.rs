use crate::catalog::domain::railway_model::{
    Control, DccInterface, LengthOverBuffers, RailwayModelId, RailwayModelUowExt,
    RollingStockDccPatch, RollingStockId,
};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRollingStockDcc::execute`].
pub struct UpdateRollingStockDccInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// Optional control type; `None` clears the field.
    pub control: Option<Control>,
    /// Optional DCC interface connector; `None` clears the field.
    pub dcc_interface: Option<DccInterface>,
    /// Optional length over buffers; `None` clears the field.
    pub length_over_buffers: Option<LengthOverBuffers>,
}

/// Use case that updates only the control type, DCC interface, and length of a single
/// rolling stock unit without touching any other technical specification fields.
pub struct UpdateRollingStockDcc;

impl UpdateRollingStockDcc {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::NotFound`] when no rolling stock with `rolling_stock_id` exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRollingStockDccInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, Language::English)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.update_rolling_stock_dcc(
            &input.rolling_stock_id,
            RollingStockDccPatch {
                control: input.control,
                dcc_interface: input.dcc_interface,
                length_over_buffers: input.length_over_buffers,
            },
        )?;

        repo.save(&mut model).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_company::RailwayCompanyId;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        Category, LocomotiveType, MockRailwayModelRepository, PowerMethod, ProductCode,
        RailwayModel, RollingStock, RollingStockId,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::Language;

    fn make_model_with_locomotive(model_id: RailwayModelId, rs_id: RollingStockId) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P200").unwrap();
        let railway = RailwayCompanyId::try_from("trn:railway-company:db").unwrap();
        let loco = RollingStock::Locomotive {
            id: rs_id,
            railway_id: railway,
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "BR50".to_string(),
            road_number: Some("50 001".to_string()),
            series: None,
            depot: None,
            locomotive_type: LocomotiveType::SteamLocomotive,
            dcc_interface: None,
            control: None,
            is_dummy: false,
        };
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
            epoch: "III".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![loco],
            pending_events: vec![],
        }
    }

    fn model_id() -> RailwayModelId {
        RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P200",
        )
        .unwrap()
    }

    #[tokio::test]
    async fn it_updates_dcc_fields_successfully() {
        let mid = model_id();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let model = make_model_with_locomotive(mid.clone(), rs_id.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        UpdateRollingStockDcc::execute(
            &mut uow,
            UpdateRollingStockDccInput {
                railway_model_id: mid,
                rolling_stock_id: rs_id,
                control: Some(Control::DccReady),
                dcc_interface: Some(DccInterface::Nem651),
                length_over_buffers: None,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn it_returns_not_found_when_model_is_missing() {
        let mid = model_id();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let err = UpdateRollingStockDcc::execute(
            &mut uow,
            UpdateRollingStockDccInput {
                railway_model_id: mid,
                rolling_stock_id: rs_id,
                control: None,
                dcc_interface: None,
                length_over_buffers: None,
            },
        )
        .await
        .expect_err("missing model should return NotFound");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }

    #[tokio::test]
    async fn it_returns_not_found_when_rolling_stock_is_missing() {
        let mid = model_id();
        let rs_id_in_model = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let rs_id_unknown = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let model = make_model_with_locomotive(mid.clone(), rs_id_in_model);

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let err = UpdateRollingStockDcc::execute(
            &mut uow,
            UpdateRollingStockDccInput {
                railway_model_id: mid,
                rolling_stock_id: rs_id_unknown,
                control: None,
                dcc_interface: None,
                length_over_buffers: None,
            },
        )
        .await
        .expect_err("unknown rs_id should return NotFound");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }
}
