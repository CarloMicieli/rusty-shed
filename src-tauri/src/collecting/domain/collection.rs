use crate::collecting::domain::collection_id::CollectionId;
use crate::collecting::domain::collection_item::CollectionItem;
use crate::collecting::domain::summary::CollectionSummary;
use crate::core::domain::MonetaryAmount;
use serde::{Deserialize, Serialize};

pub const DEFAULT_COLLECTION_ID: &str = "052cb8be-cc5c-460d-b72c-6cec595b91d7";

/// Represents a user-owned collection of items.
///
/// A `Collection` contains identifying information, a few aggregated summary
/// values and the list of `CollectionItem` entries that make up the
/// collection. It is intentionally lightweight to keep IPC payloads small.
///
/// Default behaviour:
/// - `Collection::default()` returns an empty collection with a generated id,
///   the name "My Collection", a `CollectionSummary::default()` and no
///   `total_value` (i.e. `None`). This mirrors previous code paths that
///   returned a default when no database row existed.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Collection {
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
    pub items: Vec<CollectionItem>,
}

impl Default for Collection {
    /// Returns a sensible default `Collection` matching existing code paths
    /// that expect a default when no collection is present in the database.
    fn default() -> Self {
        Collection {
            id: CollectionId::try_from(DEFAULT_COLLECTION_ID).expect("Invalid collection ID"),
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
    fn default_collection_has_expected_values() {
        let d = Collection::default();

        assert_eq!(d.name, "My Collection");
        assert!(d.items.is_empty());
        assert!(d.total_value.is_none());
        assert_eq!(d.summary, CollectionSummary::default());
    }
}
