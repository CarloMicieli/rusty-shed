use crate::catalog::domain::railway_company::{RailwayCompanyId, RailwayCompanyUowExt};
use crate::catalog::domain::railway_model::localized_field::LocalizedField;
use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt, RollingStockId};
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRollingStockRailwayCompany::execute`].
pub struct UpdateRollingStockRailwayCompanyInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// The new railway company id (must exist in the database).
    pub railway_company_id: RailwayCompanyId,
}

/// Use case that updates the railway company of a single rolling stock unit.
///
/// Verifies that the target railway company exists in the database before
/// applying the change to the aggregate.
pub struct UpdateRollingStockRailwayCompany;

impl UpdateRollingStockRailwayCompany {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when `railway_company_id` does not exist in the database.
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::NotFound`] when no rolling stock with `rolling_stock_id` exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRollingStockRailwayCompanyInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + RailwayCompanyUowExt + Send,
    {
        // Verify the target railway company exists.
        {
            let mut company_repo = unit_of_work.railway_companies_repo();
            company_repo
                .find_by_id(&input.railway_company_id)
                .await?
                .ok_or_else(|| DomainError::NotFound {
                    resource: "RailwayCompany".to_string(),
                    identifier: input.railway_company_id.to_string(),
                })?;
        }

        let mut model_repo = unit_of_work.railway_model_repository();

        let mut model = model_repo
            .find_by_id(&input.railway_model_id, "en")
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.update_rolling_stock_railway_company(
            &input.rolling_stock_id,
            input.railway_company_id,
        )?;

        model_repo.save(&mut model).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_company::{
        MockRailwayCompanyRepository, RailwayCompany, RailwayCompanyId,
    };
    use crate::catalog::domain::railway_model::LocomotiveType;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
        RailwayModelId, RollingStock,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::metadata::Metadata;

    fn make_model_with_locomotive(
        model_id: RailwayModelId,
        rs_id: RollingStockId,
        railway: RailwayCompanyId,
    ) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P100").unwrap();
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
                lang: "en".to_string(),
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

    fn make_railway_company(id: RailwayCompanyId) -> RailwayCompany {
        RailwayCompany {
            id,
            name: "Test Company".to_string(),
            registered_company_name: None,
            country_code: None,
            metadata: Metadata::default(),
            period_of_activity: None,
        }
    }

    #[tokio::test]
    async fn updates_railway_company_successfully() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let old_company = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
        let new_company = RailwayCompanyId::try_from("trn:railway-company:sncf").unwrap();
        let model = make_model_with_locomotive(model_id.clone(), rs_id.clone(), old_company);
        let company = make_railway_company(new_company.clone());

        let mut mock_company = MockRailwayCompanyRepository::new();
        mock_company
            .expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some(company.clone())));

        let mut mock_model = MockRailwayModelRepository::new();
        mock_model
            .expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock_model.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_company_and_model_repos(mock_company, mock_model);

        UpdateRollingStockRailwayCompany::execute(
            &mut uow,
            UpdateRollingStockRailwayCompanyInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                railway_company_id: new_company,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn returns_not_found_when_company_missing() {
        let model_id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let rs_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let company_id = RailwayCompanyId::try_from("trn:railway-company:unknown").unwrap();

        let mut mock_company = MockRailwayCompanyRepository::new();
        mock_company
            .expect_find_by_id()
            .times(1)
            .returning(|_| Ok(None));

        let mut mock_model = MockRailwayModelRepository::new();
        mock_model.expect_find_by_id().times(0);
        mock_model.expect_save().times(0);

        let mut uow = FakeUow::with_company_and_model_repos(mock_company, mock_model);

        let err = UpdateRollingStockRailwayCompany::execute(
            &mut uow,
            UpdateRollingStockRailwayCompanyInput {
                railway_model_id: model_id,
                rolling_stock_id: rs_id,
                railway_company_id: company_id,
            },
        )
        .await
        .expect_err("missing company should fail");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }
}
