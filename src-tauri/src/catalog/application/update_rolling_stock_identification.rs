use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt, RollingStockId};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRollingStockIdentification::execute`].
pub struct UpdateRollingStockIdentificationInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// New series code (required, non-empty).
    pub series_code: String,
    /// Optional road number; `None` clears the field.
    pub road_number: Option<String>,
    /// Optional livery; `None` clears the field.
    pub livery: Option<String>,
    /// Optional depot; `None` clears the field.
    pub depot: Option<String>,
}

/// Use case that updates the identification fields (series_code, road_number, livery, depot)
/// of a single rolling stock unit within a [`RailwayModel`] aggregate.
pub struct UpdateRollingStockIdentification;

impl UpdateRollingStockIdentification {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::NotFound`] when no rolling stock with `rolling_stock_id` exists.
    /// - [`DomainError::Validation`] when `series_code` is empty.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRollingStockIdentificationInput,
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

        model.update_rolling_stock_identification(
            &input.rolling_stock_id,
            input.series_code,
            input.road_number,
            input.livery,
            input.depot,
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
    use crate::catalog::domain::railway_model::LocomotiveType;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
        RailwayModelId, RollingStock,
    };
    use crate::catalog::domain::scale::Scale;

    fn make_model_with_locomotive(model_id: RailwayModelId, rs_id: RollingStockId) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P100").unwrap();
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
        let loco = RollingStock::Locomotive {
            id: rs_id,
            railway_id: railway,
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "SC-1".to_string(),
            road_number: Some("100".to_string()),
            series: None,
            depot: None,
            locomotive_type: LocomotiveType::ElectricLocomotive,
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
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![loco],
            pending_events: vec![],
        }
    }

    #[tokio::test]
    async fn updates_identification_fields() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let model = make_model_with_locomotive(model_id.clone(), rs_id.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        UpdateRollingStockIdentification::execute(
            &mut uow,
            UpdateRollingStockIdentificationInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                series_code: "SC-NEW".to_string(),
                road_number: Some("456".to_string()),
                livery: None,
                depot: None,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn empty_series_code_returns_validation_error() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let model = make_model_with_locomotive(model_id.clone(), rs_id.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = UpdateRollingStockIdentification::execute(
            &mut uow,
            UpdateRollingStockIdentificationInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                series_code: "".to_string(),
                road_number: None,
                livery: None,
                depot: None,
            },
        )
        .await
        .expect_err("empty series_code should fail");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation error, got {err:?}"
        );
    }

    #[tokio::test]
    async fn returns_not_found_when_model_is_missing() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P999",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = UpdateRollingStockIdentification::execute(
            &mut uow,
            UpdateRollingStockIdentificationInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                series_code: "SC-1".to_string(),
                road_number: None,
                livery: None,
                depot: None,
            },
        )
        .await
        .expect_err("missing model should return NotFound");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }
}
