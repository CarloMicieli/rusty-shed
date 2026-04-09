use crate::catalog::domain::railway_model::{DeliveryDate, RailwayModelId, RailwayModelUowExt};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRailwayModelDeliveryDate::execute`].
pub struct UpdateRailwayModelDeliveryDateInput {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// New delivery date, or `None` to clear it.
    pub delivery_date: Option<DeliveryDate>,
}

/// Use case that updates the delivery date of a [`RailwayModel`] aggregate.
pub struct UpdateRailwayModelDeliveryDate;

impl UpdateRailwayModelDeliveryDate {
    /// Execute the use case.
    ///
    /// Passing `None` as `delivery_date` clears the existing value.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRailwayModelDeliveryDateInput,
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

        model.update_delivery_date(input.delivery_date);

        repo.save(&mut model).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
        RailwayModelId,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::Language;

    fn make_model(id: RailwayModelId) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P100").unwrap();
        RailwayModel {
            id,
            manufacturer_id: manufacturer,
            product_code: product,
            description: LocalizedField {
                lang: Language::English,
                value: "Test model".to_string(),
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

    #[tokio::test]
    async fn sets_delivery_date() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let model = make_model(id.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        UpdateRailwayModelDeliveryDate::execute(
            &mut uow,
            UpdateRailwayModelDeliveryDateInput {
                railway_model_id: id,
                delivery_date: Some(DeliveryDate::Year(2026)),
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn clears_delivery_date() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let mut model = make_model(id.clone());
        model.delivery_date = Some(DeliveryDate::Year(2025));

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        UpdateRailwayModelDeliveryDate::execute(
            &mut uow,
            UpdateRailwayModelDeliveryDateInput {
                railway_model_id: id,
                delivery_date: None,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn returns_not_found_when_model_is_missing() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P999",
        )
        .unwrap();

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = UpdateRailwayModelDeliveryDate::execute(
            &mut uow,
            UpdateRailwayModelDeliveryDateInput {
                railway_model_id: id,
                delivery_date: Some(DeliveryDate::Year(2026)),
            },
        )
        .await
        .expect_err("missing model should fail");

        assert!(
            matches!(err, DomainError::NotFound { .. }),
            "expected NotFound, got {err:?}"
        );
    }
}
