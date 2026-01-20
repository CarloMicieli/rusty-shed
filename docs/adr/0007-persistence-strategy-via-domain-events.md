# ADR 7: Persistence Strategy via Domain Events

Status: Accepted

Date: 2026-01-20

Deciders: Project Lead

## 1. Context and Problem Statement

In a Hexagonal Architecture using Domain-Driven Design (DDD), we need a strategy to persist changes made to complex Aggregates and their embedded entities within our Rust/Tauri command handlers.

We want to avoid "Anemic Domain Models" where the repository dictates the domain structure. Instead, we need a way to track state changes within the Aggregate and ensure the database reflects those specific changes without exposing the persistence logic to the Domain layer

## 2. Decision Drivers

- **Encapsulation**: The Aggregate should not know about SQL or database schemas.
- **Auditability**: Clear tracking of what business actions occurred.
- **Consistency**: Ensuring embedded entities are updated in sync with the root.
- **Performance vs. Precision**: Balancing the number of DB operations against the accuracy of updates.

## 3. Considered Options

### Option 1: State-Based Mapping (Snapshot)

The repository compares the entire new state of the aggregate with the old state (or simply overwrites it).

- Good, because: Simple to implement; works well for simple CRUD.
- Bad, because: Hard to determine what changed in a large aggregate (e.g., did an item in a list get updated, or was a new one added?). Often leads to "save-all" logic which is inefficient.

### Option 2: Domain Event Tracking (The Proposed Design)

The aggregate records "what happened" in an internal queue; the repository iterates through these events to execute specific DB mutations.

Aggregates contain a `pending_events: Vec<DomainEvent>` field.

- Good, because: High precision. The repository knows exactly which row to update or delete based on the event type (e.g., `ItemAdded` vs `ItemRenamed`).
- Bad, because: ncreased complexity in the Repository; the repository must _"double-map"_ both the state and the events.

### Option 3: Event Sourcing

The state of the aggregate is never stored directly; only the stream of events is stored and replayed.

- Good, because: Perfect audit trail; "time travel" debugging.
- Bad, because: Significant overhead for a Tauri app; requires complex infrastructure and handling of "projections" for queries.

## 4. Decision Outcome

Chosen Option: **Option 2: Domain Event Tracking**

### Justification

This provides the best balance of strict typing and architectural flexibility. It allows the Repository to treat every event as a standard unit of work while allowing the Domain to remain expressive through Enums.

### Consequences

- Positive: The Domain Layer remains pure. We can trigger side effects (like UI notifications in Tauri) by observing these same events.
- Positive: Optimized database writes—we only touch what actually changed.

- Negative: The Repository becomes more "intelligent" and slightly harder to maintain as it must understand the mapping of every event type.
- Negative: Potential for "out-of-sync" bugs if a developer forgets to push an event after changing a field.

### Technical Details

#### The Immutable Envelope

We define a generic `EventEnvelope<T>` that restricts `T` to types implementing `Debug`. Access to the payload is provided via the `Deref` trait to allow the envelope to be used directly in match statements.

```rust
/// Immutable metadata wrapper for Domain Events
#[derive(Debug, Clone)]
pub struct EventEnvelope<T: Debug> {
    id: Uuid,
    occurred_at: DateTime<Utc>,
    payload: T,
}

impl<T: Debug> EventEnvelope<T> {
    /// Constructor that captures infrastructure metadata
    pub fn new(payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            occurred_at: Utc::now(),
            payload,
        }
    }
    
    pub fn id(&self) -> Uuid { self.id }
    pub fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
}

impl<T: Debug> std::ops::Deref for EventEnvelope<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.payload }
}
```

#### Domain Integration

Each aggregate will define a type alias for its specific events to improve readability.

```rust
pub enum CollectionEvent {
    CollectionRenamed { ... },
    CollectionItemAdded { ... },
}

/// Type alias for ergonomic use in the Aggregate
pub type CollectionDomainEvent = EventEnvelope<CollectionEvent>;

pub struct Collection {
    // ...
    pending_events: Vec<CollectionDomainEvent>,
}
```

#### Repository Logic

The repository consumes events by iterating over the `Vec<ProjectDomainEvent>` and using a match statement. Because of the `Deref` implementation, the code remains clean, treating the envelope as the event itself for matching while still accessing its metadata.

Key logic steps:

- **Drain the Events**: The repository calls `collection.pull_events()` to take ownership of the domain events, ensuring they aren't processed twice.
- **Exhaustive Matching**: Using Rust’s match, the compiler ensures that every possible `CollectionEvent` variant is handled.
- **Atomic Persistence**: All SQL commands generated by the events are executed within the same database transaction.


## 5. More Information

By using this approach in Rust, we can leverage the match expression to ensure that all `DomainEvent` variants are handled by the repository at compile-time, reducing the risk of unhandled state changes.