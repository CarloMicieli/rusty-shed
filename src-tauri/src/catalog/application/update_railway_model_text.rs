use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;
use serde::Deserialize;
use specta::Type;

/// Identifies which free-text field on a [`RailwayModel`] is being updated.
///
/// Used by both the application use case input and the Tauri command args.
#[derive(Debug, Clone, Deserialize, Type, PartialEq, Eq)]
pub enum RailwayModelTextField {
    /// The `description` field (required, non-empty).
    Description,
    /// The `details` field (optional; an empty string is stored as `NULL`).
    Details,
}

/// Input for [`UpdateRailwayModelText::execute`].
pub struct UpdateRailwayModelTextInput {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// Which field to update.
    pub field: RailwayModelTextField,
    /// New value.  For `Details`, an empty string means "clear to `None`".
    /// For `Description`, must be non-empty (validated by the domain).
    pub value: String,
    /// Language code for the translation to update ("en" or "it").
    pub lang: String,
}

/// Use case that updates a single free-text field on a [`RailwayModel`] aggregate.
pub struct UpdateRailwayModelText;

impl UpdateRailwayModelText {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::Validation`] when `field == Description` and `value` is empty.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRailwayModelTextInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, &input.lang)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        match input.field {
            RailwayModelTextField::Description => {
                model.update_description(input.value)?;
            }
            RailwayModelTextField::Details => {
                let details = if input.value.is_empty() {
                    None
                } else {
                    Some(input.value)
                };
                model.update_details(details);
            }
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

    fn make_model(id: RailwayModelId, description: &str) -> RailwayModel {
        let manufacturer = ManufacturerId::try_from("trn:manufacturer:acme").unwrap();
        let product = ProductCode::try_from("P100").unwrap();
        RailwayModel {
            id,
            manufacturer_id: manufacturer,
            product_code: product,
            description: LocalizedField {
                lang: "en".to_string(),
                value: description.to_string(),
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
    async fn update_description_saves_new_value() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let model = make_model(id.clone(), "Old description");

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        UpdateRailwayModelText::execute(
            &mut uow,
            UpdateRailwayModelTextInput {
                railway_model_id: id,
                field: RailwayModelTextField::Description,
                value: "New description".to_string(),
                lang: "en".to_string(),
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn update_description_empty_value_returns_validation_error() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let model = make_model(id.clone(), "Existing desc");

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        // save should NOT be called when validation fails
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = UpdateRailwayModelText::execute(
            &mut uow,
            UpdateRailwayModelTextInput {
                railway_model_id: id,
                field: RailwayModelTextField::Description,
                value: "".to_string(),
                lang: "en".to_string(),
            },
        )
        .await
        .expect_err("empty description should fail");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation error, got {err:?}"
        );
    }

    #[tokio::test]
    async fn update_details_empty_value_clears_to_none() {
        let id = RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P100",
        )
        .unwrap();
        let mut model = make_model(id.clone(), "Desc");
        model.details = Some(LocalizedField {
            lang: "en".to_string(),
            value: "Old details".to_string(),
        });

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);

        UpdateRailwayModelText::execute(
            &mut uow,
            UpdateRailwayModelTextInput {
                railway_model_id: id,
                field: RailwayModelTextField::Details,
                value: "".to_string(),
                lang: "en".to_string(),
            },
        )
        .await
        .expect("clearing details should succeed");
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

        let err = UpdateRailwayModelText::execute(
            &mut uow,
            UpdateRailwayModelTextInput {
                railway_model_id: id,
                field: RailwayModelTextField::Description,
                value: "anything".to_string(),
                lang: "en".to_string(),
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
