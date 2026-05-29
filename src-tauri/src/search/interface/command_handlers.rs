use garde::Validate;
use tracing::info;

use crate::core::infrastructure::error::CommandError;
use crate::search::application::global_search::{GlobalSearch, GlobalSearchInput};
use crate::search::domain::global_search_result::SearchSource;
use crate::search::interface::command_args::{GlobalSearchArgs, GlobalSearchResultView};
use crate::state::AppState;

pub async fn global_search_inner(
    state: &AppState,
    args: GlobalSearchArgs,
) -> Result<Vec<GlobalSearchResultView>, CommandError> {
    info!(
        "Global search with query: {:?} lang: {:?}",
        args.query, args.lang
    );

    args.validate().map_err(CommandError::from)?;

    let input = GlobalSearchInput {
        query: args.query,
        lang: args.lang,
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let results = GlobalSearch::execute(&mut unit_of_work, input).await?;
    unit_of_work.commit().await?;

    let views = results
        .into_iter()
        .map(|r| GlobalSearchResultView {
            railway_model_id: r.railway_model_id.to_string(),
            source: match r.source {
                SearchSource::Collection => "collection".to_string(),
                SearchSource::Wishlist => "wishlist".to_string(),
            },
            item_id: r.item_id,
            parent_id: r.parent_id,
            display_name: r.display_name,
            manufacturer_name: r.manufacturer_name,
        })
        .collect();

    Ok(views)
}

/// Perform a cross-domain full-text search over the user's collection and wishlist.
///
/// # Arguments
/// - `state` - Tauri-managed application state providing the DB pool.
/// - `args`  - Validated search input (query string and locale).
///
/// # Returns
/// An ordered list of at most 50 `GlobalSearchResultView` items, ranked by
/// FTS5 BM25 relevance. A model appearing in both collection and wishlist
/// produces two separate result entries.
#[tauri::command]
#[specta::specta]
pub async fn global_search(
    state: tauri::State<'_, AppState>,
    args: GlobalSearchArgs,
) -> Result<Vec<GlobalSearchResultView>, CommandError> {
    global_search_inner(&state, args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::core::domain::Language;
    use crate::core::domain::identifiers::Identifier;
    use crate::search::domain::global_search_result::{GlobalSearchResult, SearchSource};
    use crate::search::domain::repository::MockGlobalSearchRepository;
    use sqlx::SqlitePool;
    use std::sync::Arc;

    async fn state_with_uow(uow: MockAppUow) -> AppState {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");
        AppState::new_with_factory(pool, Arc::new(OneShotFactory::new(uow)))
    }

    #[tokio::test]
    async fn global_search_inner_short_query_returns_validation_error() {
        let state = AppState::for_test(
            SqlitePool::connect(":memory:")
                .await
                .expect("in-memory pool"),
        );
        let args = GlobalSearchArgs {
            query: "x".to_string(),
            lang: Language::English,
        };

        let result = global_search_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[tokio::test]
    async fn global_search_inner_maps_wishlist_result_view() {
        let mut repo = MockGlobalSearchRepository::new();
        repo.expect_search().times(1).returning(|_, _| {
            Ok(vec![GlobalSearchResult {
                railway_model_id: RailwayModelId::from_string_unchecked(
                    "trn:railway-model:acme:p100".to_string(),
                ),
                source: SearchSource::Wishlist,
                item_id: "wishlist-item-1".to_string(),
                parent_id: Some("wishlist-1".to_string()),
                display_name: "E.656".to_string(),
                manufacturer_name: "ACME".to_string(),
            }])
        });

        let uow = MockAppUow::new().with_global_search(repo);
        let state = state_with_uow(uow).await;
        let args = GlobalSearchArgs {
            query: "diesel".to_string(),
            lang: Language::English,
        };

        let results = global_search_inner(&state, args)
            .await
            .expect("global search should succeed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].source, "wishlist");
        assert_eq!(results[0].item_id, "wishlist-item-1");
        assert_eq!(results[0].parent_id.as_deref(), Some("wishlist-1"));
    }
}
