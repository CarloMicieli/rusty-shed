//! Wishlist module — manages the user's model railway wish lists.
//!
//! A wishlist is a named list of catalogue entries the user intends to acquire.
//! Items can be moved between lists or promoted to the collection once purchased.
//! This module depends on `catalog` for model reference data.
//!
//! ## Layers
//! - **domain**: `Wishlist`, `WishlistItem` aggregates and repository traits
//! - **application**: use-case services (create/rename/delete lists, add/move/remove items)
//! - **infrastructure**: SQLite repository implementations
//! - **interface**: Tauri command handlers + specta-annotated DTOs

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;

pub use interface::command_handlers;
