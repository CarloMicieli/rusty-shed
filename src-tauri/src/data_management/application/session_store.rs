use crate::data_management::domain::ImportSession;
use std::collections::HashMap;
use tokio::sync::Mutex;

/// Thread-safe in-process store for active import sessions.
///
/// Backed by a `tokio::sync::Mutex` to avoid blocking the async executor
/// and to eliminate the panic-on-poison risk of `std::sync::Mutex`.
pub struct ImportSessionStore {
    sessions: Mutex<HashMap<String, ImportSession>>,
}

impl ImportSessionStore {
    /// Create an empty session store.
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Insert a session, keyed by its ID.
    pub async fn insert(&self, session: ImportSession) {
        self.sessions
            .lock()
            .await
            .insert(session.id.clone(), session);
    }

    /// Retrieve a clone of the session with the given ID.
    pub async fn get(&self, session_id: &str) -> Option<ImportSession> {
        self.sessions.lock().await.get(session_id).cloned()
    }

    /// Apply a mutation to the session with the given ID.
    /// Does nothing if the session is not found.
    pub async fn update(&self, session_id: &str, f: impl FnOnce(&mut ImportSession)) {
        let mut map = self.sessions.lock().await;
        if let Some(session) = map.get_mut(session_id) {
            f(session);
        }
    }

    /// Remove and return the session with the given ID.
    pub async fn remove(&self, session_id: &str) -> Option<ImportSession> {
        self.sessions.lock().await.remove(session_id)
    }

    /// Return `true` if any session satisfies the predicate.
    pub async fn any<F>(&self, predicate: F) -> bool
    where
        F: Fn(&ImportSession) -> bool,
    {
        self.sessions.lock().await.values().any(predicate)
    }
}

impl Default for ImportSessionStore {
    fn default() -> Self {
        Self::new()
    }
}
