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
