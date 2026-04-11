use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::{Language, domain_error::DomainError};

/// Input for [`UpsertRailwayModelTranslation::execute`].
pub struct UpsertRailwayModelTranslationInput {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// Language code.
    pub lang: Language,
    /// Description text. Required non-empty for "en".
    pub description: Option<String>,
    /// Details text. Optional for all languages.
    pub details: Option<String>,
}

/// Use case that creates or replaces a translation for a single language on a railway model.
pub struct UpsertRailwayModelTranslation;

impl UpsertRailwayModelTranslation {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::Validation`] when `lang == "en"` and `description` is empty.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpsertRailwayModelTranslationInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        if input.lang == Language::English {
            let desc = input.description.as_deref().unwrap_or("").trim();
            if desc.is_empty() {
                return Err(DomainError::Validation(
                    "English description is required".to_string(),
                ));
            }
        }

        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, input.lang)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.upsert_translation(input.lang, input.description, input.details);

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
    };
    use crate::catalog::domain::scale::Scale;

    fn make_model(model_id: RailwayModelId) -> RailwayModel {
        RailwayModel {
            id: model_id.clone(),
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            product_code: ProductCode::try_from("P300").unwrap(),
            description: LocalizedField {
                lang: Language::English,
                value: "Original description".to_string(),
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

    fn model_id() -> RailwayModelId {
        RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").unwrap(),
            "P300",
        )
        .unwrap()
    }

    #[tokio::test]
    async fn it_upserts_translation_successfully() {
        let mid = model_id();
        let model = make_model(mid.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        UpsertRailwayModelTranslation::execute(
            &mut uow,
            UpsertRailwayModelTranslationInput {
                railway_model_id: mid,
                lang: Language::English,
                description: Some("Updated description".to_string()),
                details: None,
            },
        )
        .await
        .expect("should succeed");
    }

    #[tokio::test]
    async fn it_returns_not_found_when_model_is_missing() {
        let mid = model_id();

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        mock.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let err = UpsertRailwayModelTranslation::execute(
            &mut uow,
            UpsertRailwayModelTranslationInput {
                railway_model_id: mid,
                lang: Language::Italian,
                description: Some("Descrizione".to_string()),
                details: None,
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
    async fn it_returns_validation_error_for_empty_english_description() {
        // Validation fires before the repo is even touched
        let mock = MockRailwayModelRepository::new();
        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = UpsertRailwayModelTranslation::execute(
            &mut uow,
            UpsertRailwayModelTranslationInput {
                railway_model_id: model_id(),
                lang: Language::English,
                description: None,
                details: None,
            },
        )
        .await
        .expect_err("empty English description should fail validation");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation, got {err:?}"
        );
    }

    #[tokio::test]
    async fn it_allows_empty_description_for_italian() {
        let mid = model_id();
        let model = make_model(mid.clone());

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_, _| Ok(Some(model.clone())));
        mock.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        UpsertRailwayModelTranslation::execute(
            &mut uow,
            UpsertRailwayModelTranslationInput {
                railway_model_id: mid,
                lang: Language::Italian,
                description: None,
                details: Some("Dettagli aggiuntivi".to_string()),
            },
        )
        .await
        .expect("Italian with no description should be allowed");
    }
}
