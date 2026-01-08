use chrono::{DateTime, Utc};

/// The metadata information for the current resource
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Metadata {
    pub version: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Metadata {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Metadata {
            version: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
