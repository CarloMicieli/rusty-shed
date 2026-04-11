use crate::catalog::domain::railway_model::railway_model_translation::RailwayModelTranslations;
use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve all stored translations for a railway model.
pub struct GetRailwayModelTranslations;

impl GetRailwayModelTranslations {
    /// Execute the query to retrieve all translations for a railway model.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_model_id` - The identifier of the railway model.
    ///
    /// # Returns
    /// - `Ok(Some(RailwayModelTranslations))` when the model exists with translations.
    /// - `Ok(None)` when the model does not exist.
    /// - `Err(DomainError)` on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        railway_model_id: &RailwayModelId,
    ) -> Result<Option<RailwayModelTranslations>, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        repository.find_translations(railway_model_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::railway_model::{
        MockRailwayModelRepository, RailwayModelTranslationEntry,
    };

    fn model_id() -> RailwayModelId {
        RailwayModelId::try_from("trn:railway-model:acme:1234").unwrap()
    }

    #[tokio::test]
    async fn it_returns_translations_when_found() {
        let id = model_id();
        let translations = RailwayModelTranslations {
            railway_model_id: id.clone(),
            en: Some(RailwayModelTranslationEntry {
                description: Some("A locomotive".to_string()),
                details: None,
            }),
            it: None,
        };

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_translations()
            .times(1)
            .returning(move |_| Ok(Some(translations.clone())));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let result = GetRailwayModelTranslations::execute(&mut uow, &id)
            .await
            .expect("should succeed");

        assert!(result.is_some());
        let t = result.unwrap();
        assert!(t.en.is_some());
    }

    #[tokio::test]
    async fn it_returns_none_when_model_not_found() {
        let id = model_id();

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_find_translations()
            .times(1)
            .returning(|_| Ok(None));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let result = GetRailwayModelTranslations::execute(&mut uow, &id)
            .await
            .expect("should succeed");

        assert!(result.is_none());
    }
}
