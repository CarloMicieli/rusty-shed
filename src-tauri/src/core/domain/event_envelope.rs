use chrono::{DateTime, Utc};
use std::fmt::Debug;
use std::ops::Deref;
use uuid::Uuid;

/// Immutable metadata wrapper for Domain Events.
///
/// `EventEnvelope` wraps domain events with infrastructure metadata (ID, timestamp)
/// while allowing the payload to be accessed directly via `Deref`. This pattern
/// ensures that domain events remain pure business logic while still carrying
/// the metadata needed for persistence, auditing, and integration.
///
/// ### Type Parameters
/// * `T`: The domain event type, must implement `Debug` for error reporting.
#[derive(Debug, Clone)]
pub struct EventEnvelope<T: Debug> {
    id: Uuid,
    occurred_at: DateTime<Utc>,
    payload: T,
}

impl<T: Debug> EventEnvelope<T> {
    /// Constructor that captures infrastructure metadata.
    ///
    /// Creates a new envelope with a generated UUID and current UTC timestamp.
    /// The payload is wrapped but remains accessible via `Deref`.
    pub fn new(payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            occurred_at: Utc::now(),
            payload,
        }
    }

    /// Returns the unique identifier for this event instance.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the timestamp indicating when the event occurred.
    pub fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

impl<T: Debug> Deref for EventEnvelope<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}
