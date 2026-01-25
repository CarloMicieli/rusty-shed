use crate::collecting::domain::CollectionSummary;
use crate::collecting::domain::{CollectionId, CollectionItemView};
use crate::core::domain::MonetaryAmount;
use serde::Serialize;

/// Represents a user-owned collection of items.
///
/// A `Collection` contains identifying information, a few aggregated summary
/// values and the list of `CollectionItem` entries that make up the
/// collection. It is intentionally lightweight to keep IPC payloads small.
///
/// Default behavior:
/// - `Collection::default()` returns an empty collection with a generated id,
///   the name "My Collection", a `CollectionSummary::default()` and no
///   `total_value` (i.e. `None`). This mirrors previous code paths that
///   returned a default when no database row existed.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CollectionView {
    /// Unique identifier for the collection (typically a UUID stored as a string).
    pub id: CollectionId,

    /// Display name for this collection.
    pub name: String,

    /// Precomputed summary counts (e.g. total items, tracked vs untracked).
    pub summary: CollectionSummary,

    /// Optional total monetary value of the collection. Use `MonetaryAmount`
    /// to preserve currency and decimal precision.
    pub total_value: Option<MonetaryAmount>,

    /// The list of items contained in this collection.
    pub items: Vec<CollectionItemView>,
}

impl Default for CollectionView {
    /// Returns a sensible default `Collection` matching existing code paths
    /// that expect a default when no collection is present in the database.
    fn default() -> Self {
        CollectionView {
            id: CollectionId::default(),
            name: "My Collection".to_string(),
            summary: CollectionSummary::default(),
            total_value: None,
            items: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_default_collection_has_expected_values() {
        let d = CollectionView::default();

        assert_eq!(d.name, "My Collection");
        assert!(d.items.is_empty());
        assert!(d.total_value.is_none());
        assert_eq!(d.summary, CollectionSummary::default());
    }
}
