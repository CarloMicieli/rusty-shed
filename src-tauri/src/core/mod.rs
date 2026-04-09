//! Core module — cross-cutting infrastructure shared by all feature modules.
//!
//! Provides the application's foundational concerns that have no business
//! domain of their own:
//!
//! - **infrastructure**: SQLite connection pool initialisation, WAL-mode
//!   configuration, migration runner, and the `CommandError` type that is
//!   returned from every Tauri command.
//! - **domain**: shared value objects (e.g. `DomainError`) used across bounded
//!   contexts.
//! - **interface**: Tauri commands for database lifecycle operations (init,
//!   health-check).
//!
//! All other modules depend on `core::infrastructure` for database access;
//! none of them are depended on by `core` in return (no circular dependencies).

pub mod domain;
pub mod infrastructure;
pub mod interface;
