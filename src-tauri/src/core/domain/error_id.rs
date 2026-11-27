//! Strongly typed identifiers for backend error occurrences.

use serde::Serialize;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// A stable identifier for one backend error occurrence.
///
/// The wire and display format is `ERR-NNNN-X`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, specta::Type)]
#[serde(transparent)]
pub struct ErrorId(String);

impl ErrorId {
    /// Generate a process-unique error identifier.
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        let combined = millis.wrapping_mul(10_000).wrapping_add(seq);
        let numeric = (combined % 9000 + 1000) as u16;
        let letter = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[(combined / 9000 % 26) as usize] as char;

        Self(format!("ERR-{numeric:04}-{letter}"))
    }
}

impl fmt::Display for ErrorId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for ErrorId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashSet;

    #[test]
    fn format_is_stable() {
        let id = ErrorId::generate();
        let binding = id.to_string();
        let parts: Vec<&str> = binding.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "ERR");
        assert_eq!(parts[1].len(), 4);
        assert!(parts[1].chars().all(|character| character.is_ascii_digit()));
        assert_eq!(parts[2].len(), 1);
        assert!(
            parts[2]
                .chars()
                .all(|character| character.is_ascii_uppercase())
        );
    }

    #[test]
    fn generated_ids_are_unique() {
        let ids: HashSet<ErrorId> = (0..500).map(|_| ErrorId::generate()).collect();
        assert_eq!(ids.len(), 500);
    }

    #[test]
    fn serializes_as_a_string() {
        let id = ErrorId::generate();
        let json = serde_json::to_string(&id).expect("error id should serialize");
        assert_eq!(json, format!("\"{id}\""));
    }
}
