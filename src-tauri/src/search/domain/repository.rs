use crate::core::domain::domain_error::DomainError;
use crate::search::domain::global_search_result::GlobalSearchResult;

/// Repository contract for global cross-domain search.
///
/// Implementors query the FTS5 index and join results with `collection_items`
/// and `wishlist_items` to attach source context to each hit.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait GlobalSearchRepository: Send {
    /// Search for railway models matching `query` in the given language.
    ///
    /// # Arguments
    /// * `query` - FTS5-formatted query string (prefix-transformed by the use case).
    /// * `lang`  - BCP-47 language tag used for display-name resolution.
    ///
    /// # Returns
    /// An ordered list of at most 50 `GlobalSearchResult` values, ranked by
    /// BM25 relevance (most relevant first). A model appearing in both
    /// collection and wishlist produces two separate results.
    async fn search(
        &mut self,
        query: &str,
        lang: &str,
    ) -> Result<Vec<GlobalSearchResult>, DomainError>;
}

/// Extension trait that attaches the `GlobalSearchRepository` to the Unit of Work.
pub trait GlobalSearchUowExt: Send {
    /// Returns a boxed `GlobalSearchRepository` tied to the current transaction.
    fn global_search_repo(&mut self) -> Box<dyn GlobalSearchRepository + '_>;
}
