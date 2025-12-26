//! Domain model for the `Collection` aggregate.
//!
//! This module contains the in-memory representation of a user's collection of
//! items. The `Collection` struct is (de)serializable and used across the
//! application, persisted by the collecting infrastructure and returned by
//! collecting use-cases and command handlers.

use crate::collecting::domain::collection::{CollectionItem, CollectionSummary};
use crate::core::domain::MonetaryAmount;
use serde::{Deserialize, Serialize};

/// Represents a user-owned collection of items.
///
/// A `Collection` contains identifying information, a few aggregated summary
/// values and the list of `CollectionItem` entries that make up the
/// collection. It is intentionally lightweight to keep IPC payloads small.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Collection {
    /// Unique identifier for the collection (typically a UUID stored as a string).
    pub id: String,

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
