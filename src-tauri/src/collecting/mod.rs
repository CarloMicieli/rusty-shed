//! Collecting module — tracks the user's physical model railway collection.
//!
//! A *collection item* represents a purchased catalogue entry together with
//! condition, purchase details, and the individual rolling-stock units it
//! contains. This module depends on `catalog` for model lookups and `sellers`
//! for purchase origin data.
//!
//! ## Layers
//! - **domain**: `CollectionItem`, `RollingStock` aggregates and repository traits
//! - **application**: use-case services (add/update/remove items, rolling-stock ops)
//! - **infrastructure**: SQLite repository implementations with FK-safe inserts
//! - **interface**: Tauri command handlers + specta-annotated DTOs

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
