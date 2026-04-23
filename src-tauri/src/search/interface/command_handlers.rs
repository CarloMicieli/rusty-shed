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
