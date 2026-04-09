use crate::catalog::domain::railway_model::{Category, Epoch, RailwayModelId, RailwayModelUowExt};
use crate::catalog::domain::scale::Scale;
#[allow(unused)]
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRailwayModelClassification::execute`].
pub struct UpdateRailwayModelClassificationInput {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// New scale, if being updated.
    pub scale: Option<Scale>,
    /// New epoch, if being updated.
    pub epoch: Option<Epoch>,
    /// New category, if being updated.
    pub category: Option<Category>,
}

/// Use case that updates the constrained classification fields (scale and/or epoch)
/// of a [`RailwayModel`] aggregate.
pub struct UpdateRailwayModelClassification;

impl UpdateRailwayModelClassification {
    /// Execute the use case.
    ///
    /// At least one of `scale` or `epoch` must be `Some`; providing neither is a
    /// [`DomainError::Validation`] error.
    ///
    /// # Errors
    /// - [`DomainError::Validation`] when both `scale` and `epoch` are `None`.
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRailwayModelClassificationInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        if input.scale.is_none() && input.epoch.is_none() && input.category.is_none() {
            return Err(DomainError::Validation(
                "at least one of scale, epoch, or category must be provided".to_string(),
            ));
        }

        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, Language::English)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        if let Some(scale) = input.scale {
            model.update_scale(scale);
        }
        if let Some(epoch) = input.epoch {
            model.update_epoch(epoch);
        }
        if let Some(category) = input.category {
            model.update_category(category);
        }

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
    async fn updates_scale_only() {
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

        UpdateRailwayModelClassification::execute(
            &mut uow,
            UpdateRailwayModelClassificationInput {
                railway_model_id: id,
                scale: Some(Scale::N),
                epoch: None,
                category: None,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn updates_epoch_only() {
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

        UpdateRailwayModelClassification::execute(
            &mut uow,
            UpdateRailwayModelClassificationInput {
                railway_model_id: id,
                scale: None,
                epoch: Some("III".into()),
                category: None,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn both_none_returns_validation_error() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(0);
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = UpdateRailwayModelClassification::execute(
            &mut uow,
            UpdateRailwayModelClassificationInput {
                railway_model_id: id,
                scale: None,
                epoch: None,
                category: None,
            },
        )
        .await
        .expect_err("should fail");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation error, got {err:?}"
        );
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

        let err = UpdateRailwayModelClassification::execute(
            &mut uow,
            UpdateRailwayModelClassificationInput {
                railway_model_id: id,
                scale: Some(Scale::N),
                epoch: None,
                category: None,
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
