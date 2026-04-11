use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;

/// Input for [`SearchRailwayModels::execute`].
pub struct SearchRailwayModelsInput {
    /// The search query (minimum 2 characters).
    pub query: String,
}

/// Use case that searches railway models using FTS5 full-text search across all languages.
pub struct SearchRailwayModels;

impl SearchRailwayModels {
    /// Execute the search use case.
    ///
    /// # Errors
    /// - [`DomainError::Validation`] when `query` is shorter than 2 characters.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SearchRailwayModelsInput,
    ) -> Result<Vec<RailwayModelId>, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        if input.query.trim().len() < 2 {
            return Err(DomainError::Validation(
                "search query must be at least 2 characters".to_string(),
            ));
        }

        let mut repository = unit_of_work.railway_model_repository();
        repository.search(&input.query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;

    fn make_id(slug: &str) -> RailwayModelId {
        RailwayModelId::try_from(format!("trn:railway-model:acme:{slug}").as_str()).unwrap()
    }

    #[tokio::test]
    async fn it_returns_results_for_valid_query() {
        let expected_id = make_id("1234");

        let mut mock = MockRailwayModelRepository::new();
        mock.expect_search()
            .withf(|q| q == "BR 50")
            .times(1)
            .returning(move |_| Ok(vec![expected_id.clone()]));

        let mut uow = FakeUow::with_railway_models_repo(mock);
        let result = SearchRailwayModels::execute(
            &mut uow,
            SearchRailwayModelsInput {
                query: "BR 50".to_string(),
            },
        )
        .await
        .expect("should return search results");

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn it_returns_validation_error_for_empty_query() {
        let mock = MockRailwayModelRepository::new();
        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = SearchRailwayModels::execute(
            &mut uow,
            SearchRailwayModelsInput {
                query: "".to_string(),
            },
        )
        .await
        .expect_err("empty query should fail validation");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation, got {err:?}"
        );
    }

    #[tokio::test]
    async fn it_returns_validation_error_for_single_char_query() {
        let mock = MockRailwayModelRepository::new();
        let mut uow = FakeUow::with_railway_models_repo(mock);

        let err = SearchRailwayModels::execute(
            &mut uow,
            SearchRailwayModelsInput {
                query: "B".to_string(),
            },
        )
        .await
        .expect_err("single-char query should fail validation");

        assert!(
            matches!(err, DomainError::Validation(_)),
            "expected Validation, got {err:?}"
        );
    }
}
