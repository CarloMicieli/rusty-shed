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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::domain::{ArchiveFormat, ImportState};
    use std::path::PathBuf;

    #[tokio::test]
    async fn insert_get_update_and_remove_session() {
        let store = ImportSessionStore::new();
        let mut session = ImportSession::new(PathBuf::from("/tmp/import.zip"), ArchiveFormat::Zip);
        let session_id = session.id.clone();
        session.transition(ImportState::Analyzed);

        store.insert(session.clone()).await;

        let loaded = store.get(&session_id).await;
        assert!(loaded.is_some());
        assert!(matches!(
            loaded.expect("session should exist").state,
            ImportState::Analyzed
        ));

        store
            .update(&session_id, |s| s.transition(ImportState::Completed))
            .await;

        let updated = store
            .get(&session_id)
            .await
            .expect("session should still exist");
        assert!(matches!(updated.state, ImportState::Completed));

        let removed = store.remove(&session_id).await;
        assert!(removed.is_some());
        assert!(store.get(&session_id).await.is_none());
    }

    #[tokio::test]
    async fn any_returns_true_only_when_predicate_matches() {
        let store = ImportSessionStore::new();
        let session = ImportSession::new(PathBuf::from("/tmp/import.tar.gz"), ArchiveFormat::TarGz);
        let session_id = session.id.clone();
        store.insert(session).await;

        let has_importing = store
            .any(|s| matches!(s.state, ImportState::Importing))
            .await;
        assert!(!has_importing);

        store
            .update(&session_id, |s| s.transition(ImportState::Importing))
            .await;
        let has_importing = store
            .any(|s| matches!(s.state, ImportState::Importing))
            .await;
        assert!(has_importing);
    }
}
