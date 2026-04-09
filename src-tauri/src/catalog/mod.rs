//! Catalogue module — manages the railway model catalogue.
//!
//! This is the central bounded context for all reference data. It owns the
//! master definitions of manufacturers, scales, railway companies, prototypes,
//! and railway models. Other modules (collecting, wishlist, etc.) reference
//! catalogue entities by ID but never mutate them.
//!
//! ## Layers
//! - **domain**: aggregate roots (`RailwayModel`), value objects, and repository traits
//! - **application**: use-case services (create/update/delete/query models)
//! - **infrastructure**: SQLite repository implementations
//! - **interface**: Tauri command handlers + specta-annotated input/output types

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
