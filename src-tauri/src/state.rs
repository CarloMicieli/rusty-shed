use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// Lightweight sync operation state stored independent of domain types.
pub struct SyncState {
    pub operation_id: Option<String>,
    pub is_syncing: bool,
    pub progress_percent: f32,
    pub status_message: String,
}

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
    models_dir: PathBuf,
    /// Resolved path to the SQLite database file.
    db_path: PathBuf,
    /// Email of the currently connected Google account (None if not connected).
    connected_email: Mutex<Option<String>>,
    /// Current cloud backup sync operation state.
    sync_state: Mutex<SyncState>,
    /// ISO 8601 timestamp of the last successful cloud backup sync.
    last_sync_at: Mutex<Option<String>>,
}

impl AppState {
    /// Create a new `AppState` wrapping an existing `SqlitePool`.
    pub fn new(db_pool: SqlitePool, models_dir: PathBuf, db_path: PathBuf) -> Self {
        Self {
            initialized: AtomicBool::new(false),
            db_pool,
            models_dir,
            db_path,
            connected_email: Mutex::new(None),
            sync_state: Mutex::new(SyncState {
                operation_id: None,
                is_syncing: false,
                progress_percent: 0.0,
                status_message: String::new(),
            }),
            last_sync_at: Mutex::new(None),
        }
    }

    /// Mark the database as initialized.
    pub fn set_initialized(&self) {
        self.initialized.store(true, Ordering::SeqCst);
    }

    /// Return whether the database is considered initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    /// Return a cloned `SqlitePool` handle for use by callers.
    pub fn db_pool(&self) -> SqlitePool {
        self.db_pool.clone()
    }

    /// Return the configured models directory path.
    pub fn models_dir(&self) -> PathBuf {
        self.models_dir.clone()
    }

    /// Return the resolved database file path.
    pub fn db_path(&self) -> PathBuf {
        self.db_path.clone()
    }

    /// Create a new `SqliteUnitOfWork` using the internal database pool.
    pub async fn unit_of_work<'conn>(&'conn self) -> Result<SqliteUnitOfWork<'conn>, CommandError> {
        SqliteUnitOfWork::new(&self.db_pool())
            .await
            .map_err(|e| CommandError::DatabaseError(e.to_string()))
    }

    /// Return the email of the currently connected Google account.
    pub fn connected_email(&self) -> Option<String> {
        self.connected_email
            .lock()
            .expect("connected_email lock poisoned")
            .clone()
    }

    /// Set (or clear) the connected Google account email.
    pub fn set_connected_email(&self, email: Option<String>) {
        *self
            .connected_email
            .lock()
            .expect("connected_email lock poisoned") = email;
    }

    /// Return a snapshot of the current sync operation state.
    pub fn sync_state(&self) -> (Option<String>, bool, f32, String) {
        let s = self.sync_state.lock().expect("sync_state lock poisoned");
        (
            s.operation_id.clone(),
            s.is_syncing,
            s.progress_percent,
            s.status_message.clone(),
        )
    }

    /// Update the current sync operation state.
    pub fn set_sync_state(
        &self,
        operation_id: Option<String>,
        is_syncing: bool,
        progress_percent: f32,
        status_message: impl Into<String>,
    ) {
        let mut s = self.sync_state.lock().expect("sync_state lock poisoned");
        s.operation_id = operation_id;
        s.is_syncing = is_syncing;
        s.progress_percent = progress_percent;
        s.status_message = status_message.into();
    }

    /// Return the ISO 8601 timestamp of the last successful cloud backup.
    pub fn last_sync_at(&self) -> Option<String> {
        self.last_sync_at
            .lock()
            .expect("last_sync_at lock poisoned")
            .clone()
    }

    /// Set the last successful cloud backup timestamp.
    pub fn set_last_sync_at(&self, timestamp: Option<String>) {
        *self
            .last_sync_at
            .lock()
            .expect("last_sync_at lock poisoned") = timestamp;
    }
}
