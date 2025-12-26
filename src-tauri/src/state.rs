use sqlx::sqlite::SqlitePool;
use std::sync::atomic::{AtomicBool, Ordering};

/// Application-wide state managed by Tauri.
///
/// `AppState` is intended to be stored via `tauri::Builder::manage(...)` and
/// accessed in command handlers using `tauri::State<'_, AppState>`.
///
/// The struct contains a lightweight readiness flag (`initialized`) and a
/// `SqlitePool` instance (`db_pool`) that is cloned for callers. The
/// `initialized` flag is an `AtomicBool` so reads/writes are lock-free and
/// safe to perform from multiple threads.
///
/// Concurrency notes:
/// - Tauri stores managed state behind `Arc`, so `tauri::State<'_, AppState>` is
///   a cheap reference and can be used in async command handlers.
/// - `SqlitePool` itself is a cloneable handle to an internal pool and is
///   designed to be shared across threads. The `db_pool()` accessor clones the
///   handle for the caller.
pub struct AppState {
    initialized: AtomicBool,
    db_pool: SqlitePool,
}

impl AppState {
    /// Create a new `AppState` wrapping an existing `SqlitePool`.
    ///
    /// The `initialized` flag will start as `false`; call
    /// `set_initialized()` after the application has completed any necessary
    /// startup steps (migrations, seeding, etc.).
    pub fn new(db_pool: SqlitePool) -> Self {
        Self {
            initialized: AtomicBool::new(false),
            db_pool,
        }
    }

    /// Mark the database as initialized.
    ///
    /// This sets the internal atomic flag to `true` using `SeqCst` ordering to
    /// provide strong cross-thread ordering guarantees during startup.
    pub fn set_initialized(&self) {
        self.initialized.store(true, Ordering::SeqCst);
    }

    /// Return whether the database is considered initialized.
    ///
    /// This performs an atomic load and returns a boolean. It is a fast,
    /// lock-free check suitable for guarding command handlers.
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    /// Return a cloned `SqlitePool` handle for use by callers.
    ///
    /// `SqlitePool` is a lightweight, cloneable handle; cloning it does not
    /// duplicate the underlying pool resources. Callers should use the cloned
    /// pool directly for database operations.
    pub fn db_pool(&self) -> SqlitePool {
        self.db_pool.clone()
    }
}
