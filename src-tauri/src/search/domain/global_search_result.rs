use crate::catalog::domain::railway_model::RailwayModelId;

/// A single search hit returned by the global search use case.
///
/// Each item carries enough information for the frontend to render
/// a result card and route the user to the correct detail page.
#[derive(Debug, Clone)]
pub struct GlobalSearchResult {
    /// UUID of the underlying railway model.
    pub railway_model_id: RailwayModelId,
    /// Where this result comes from (collection or wishlist).
    pub source: SearchSource,
    /// The ID of the `collection_item` or `wishlist_item` (not the railway model).
    pub item_id: String,
    /// For wishlist items: the parent `wishlist_id`. `None` for collection items.
    pub parent_id: Option<String>,
    /// Language-resolved model description (falls back to English).
    pub display_name: String,
    /// Manufacturer brand name (e.g. "A.C.M.E.", "Fleischmann").
    pub manufacturer_name: String,
}

/// Discriminator for which list this search result originated from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchSource {
    /// The result was found in the user's collection.
    Collection,
    /// The result was found in a wishlist.
    Wishlist,
}
