use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt, RollingStockId};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRollingStockSubcategory::execute`].
pub struct UpdateRollingStockSubcategoryInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// The new subcategory string (e.g. "ELECTRIC_LOCOMOTIVE", "GONDOLA").
    pub subcategory: String,
}

/// Use case that changes the subcategory (type field) of a single rolling stock unit.
pub struct UpdateRollingStockSubcategory;

impl UpdateRollingStockSubcategory {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::NotFound`] when no rolling stock with `rolling_stock_id` exists.
    /// - [`DomainError::Validation`] when `subcategory` is not valid for the current category.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRollingStockSubcategoryInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut model_repo = unit_of_work.railway_model_repository();

        let mut model = model_repo
            .find_by_id(&input.railway_model_id, Language::English)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.update_rolling_stock_subcategory(&input.rolling_stock_id, input.subcategory)?;

        model_repo.save(&mut model).await
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
    use crate::core::domain::Language;

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
    async fn updates_subcategory_successfully() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let model = make_model_with_locomotive(model_id.clone(), rs_id.clone());

        let mut mock_model = MockRailwayModelRepository::new();
        mock_model
            .expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock_model.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock_model);

        UpdateRollingStockSubcategory::execute(
            &mut uow,
            UpdateRollingStockSubcategoryInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                subcategory: "DIESEL_LOCOMOTIVE".to_string(),
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn returns_validation_error_for_invalid_subcategory() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let model = make_model_with_locomotive(model_id.clone(), rs_id.clone());

        let mut mock_model = MockRailwayModelRepository::new();
        mock_model
            .expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock_model.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock_model);

        let err = UpdateRollingStockSubcategory::execute(
            &mut uow,
            UpdateRollingStockSubcategoryInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                subcategory: "GONDOLA".to_string(), // freight car type, not locomotive type
            },
        )
        .await
        .expect_err("invalid subcategory should fail");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation, got {err:?}"
        );
    }

    #[tokio::test]
    async fn returns_not_found_when_model_missing() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());

        let mut mock_model = MockRailwayModelRepository::new();
        mock_model
            .expect_find_by_id()
            .times(1)
            .returning(|_, _| Ok(None));
        mock_model.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock_model);

        let err = UpdateRollingStockSubcategory::execute(
            &mut uow,
            UpdateRollingStockSubcategoryInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                subcategory: "DIESEL_LOCOMOTIVE".to_string(),
            },
        )
        .await
        .expect_err("missing model should fail");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }

    #[tokio::test]
    async fn updates_subcategory_for_non_locomotive_category() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P200",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P200").unwrap();
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
        let freight = RollingStock::FreightCar {
            id: rs_id.clone(),
            railway_id: railway,
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "FC-1".to_string(),
            road_number: None,
            freight_car_type: None,
        };
        let model = RailwayModel {
            id: model_id.clone(),
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
            category: Category::FreightCars,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![freight],
            pending_events: vec![],
        };

        let mut mock_model = MockRailwayModelRepository::new();
        mock_model
            .expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock_model.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock_model);

        UpdateRollingStockSubcategory::execute(
            &mut uow,
            UpdateRollingStockSubcategoryInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                subcategory: "GONDOLA".to_string(),
            },
        )
        .await
        .expect("should succeed");
    }
}
