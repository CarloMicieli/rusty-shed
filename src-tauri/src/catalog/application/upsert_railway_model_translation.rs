use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::{domain_error::DomainError, Language};

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

        let lang_str = match input.lang {
            Language::English => "en",
            Language::Italian => "it",
        };

        let mut model = repo
            .find_by_id(&input.railway_model_id, lang_str)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.upsert_translation(input.lang, input.description, input.details);

        repo.save(&mut model).await
    }
}
