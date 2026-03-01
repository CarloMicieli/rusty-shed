use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::core::domain::Language;

/// Transport arguments for the `global_search` Tauri command.
///
/// Validated with `garde` at the transport boundary before any database access.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSearchArgs {
    /// Raw search term entered by the user. Must be 2–500 characters.
    #[garde(length(min = 2, max = 500))]
    pub query: String,
    /// Language for the search.
    #[garde(skip)]
    pub lang: Language,
}

/// A single search result item returned by the `global_search` command.
///
/// Contains enough information for the frontend to render a result card
/// and navigate to the correct detail page.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSearchResultView {
    /// UUID of the underlying railway model.
    pub railway_model_id: String,
    /// `"collection"` or `"wishlist"` — where this result was found.
    pub source: String,
    /// UUID of the `collection_item` or `wishlist_item`.
    pub item_id: String,
    /// For wishlist items: the parent `wishlist_id`. `None` for collection items.
    pub parent_id: Option<String>,
    /// Language-resolved model description (falls back to English).
    pub display_name: String,
    /// Manufacturer brand name (e.g. "A.C.M.E.", "Fleischmann").
    pub manufacturer_name: String,
}
