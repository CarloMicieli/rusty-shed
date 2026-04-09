//! Search module — full-text search across the railway model catalogue.
//!
//! Uses SQLite's FTS5 virtual table to index model identifiers, names, and
//! related metadata. The FTS index is rebuilt incrementally whenever a model
//! is created or updated. User queries are wrapped in FTS5 phrase-quote syntax
//! (`"term"*`) and passed as bound parameters to prevent injection.
//!
//! ## Layers
//! - **domain**: `SearchResult`, `SearchQuery` value types and repository trait
//! - **application**: `GlobalSearchService` — query formatting + result ranking
//! - **infrastructure**: `SqliteGlobalSearchRepository` — FTS5 queries and index
//!   rebuild logic
//! - **interface**: `search_models` Tauri command

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
