//! Data management module — import, export, and local backup of the collection.
//!
//! Handles the full lifecycle of portable archive files (`.zip` / `.tar.gz`)
//! that contain a JSON manifest plus model images. Provides three user flows:
//!
//! 1. **Export** — serialise the database to a compressed archive.
//! 2. **Import** — validate, preview, and apply an archive into the database.
//!    Uses a session store to cache the parsed manifest between preview and
//!    execution steps.
//! 3. **Backup** — on-demand snapshot of the raw SQLite database file.
//!
//! ## Layers
//! - **domain**: archive format enum, `ImportSession`, manifest DTOs, error types
//! - **application**: use-case services (`ValidatePackage`, `PreviewImport`,
//!   `ExecuteImport`, `ExecuteExport`) and the `ImportSessionStore`
//! - **infrastructure**: `ArchiveExtractor`, `ArchiveWriter`, `SqliteImportRepository`,
//!   file-picker helpers, disk-space checker
//! - **interface**: Tauri command handlers for import, export, and backup flows

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
