use crate::core::domain::{Language, domain_error::DomainError};
use crate::search::domain::global_search_result::GlobalSearchResult;
use crate::search::domain::repository::GlobalSearchUowExt;

/// Input for [`GlobalSearch::execute`].
pub struct GlobalSearchInput {
    /// Raw query string, already validated (min 2, max 500 chars).
    pub query: String,
    /// Language for the search.
    pub lang: Language,
}

/// Use case that performs a cross-domain full-text search over the collection and wishlist.
///
/// The search matches against railway model descriptions, details, manufacturer names,
/// and rolling stock fields (road number, series code, livery, depot) using SQLite FTS5.
pub struct GlobalSearch;

impl GlobalSearch {
    /// Execute the global search use case.
    ///
    /// Transforms the raw user query into a FTS5 prefix query and delegates
    /// to the repository. Returns at most 50 results ordered by BM25 relevance.
    ///
    /// # Arguments
    /// * `unit_of_work` - Unit of work providing the `GlobalSearchRepository`.
    /// * `input`        - Validated search input (query and language).
    ///
    /// # Errors
    /// - [`DomainError::Validation`] when `query` is shorter than 2 characters.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: GlobalSearchInput,
    ) -> Result<Vec<GlobalSearchResult>, DomainError>
    where
        U: GlobalSearchUowExt + Send,
    {
        if input.query.trim().len() < 2 {
            return Err(DomainError::Validation(
                "search query must be at least 2 characters".to_string(),
            ));
        }

        // Transform query to FTS5 prefix format: wrap in quotes and append `*`
        // so "diesel loco" becomes `"diesel loco"*` (phrase + prefix matching).
        let fts_query = format!("\"{}\"*", input.query.trim());

        let mut repo = unit_of_work.global_search_repo();
        repo.search(&fts_query, input.lang).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::MockAppUow;
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::core::domain::identifiers::Identifier;
    use crate::search::domain::global_search_result::{GlobalSearchResult, SearchSource};
    use crate::search::domain::repository::MockGlobalSearchRepository;

    fn make_result() -> GlobalSearchResult {
        GlobalSearchResult {
            railway_model_id: RailwayModelId::from_string_unchecked(
                "trn:railway-model:acme:p100".into(),
            ),
            source: SearchSource::Collection,
            item_id: "item-1".into(),
            parent_id: None,
            display_name: "E.656".into(),
            manufacturer_name: "ACME".into(),
        }
    }

    #[tokio::test]
    async fn rejects_short_query() {
        let mut repo = MockGlobalSearchRepository::new();
        repo.expect_search().times(0);

        let mut uow = MockAppUow::new().with_global_search(repo);
        let result = GlobalSearch::execute(
            &mut uow,
            GlobalSearchInput {
                query: "a".into(),
                lang: Language::English,
            },
        )
        .await;

        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[tokio::test]
    async fn transforms_query_to_fts_prefix_and_returns_results() {
        let expected = "\"diesel loco\"*".to_string();
        let mut repo = MockGlobalSearchRepository::new();
        repo.expect_search()
            .times(1)
            .withf(move |query, lang| *query == expected && *lang == Language::Italian)
            .return_once(|_, _| Ok(vec![make_result()]));

        let mut uow = MockAppUow::new().with_global_search(repo);
        let result = GlobalSearch::execute(
            &mut uow,
            GlobalSearchInput {
                query: " diesel loco ".into(),
                lang: Language::Italian,
            },
        )
        .await;

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap().len(), 1);
    }
}
