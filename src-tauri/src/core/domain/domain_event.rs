use chrono::NaiveDateTime;

/// Defines the essential contract for all Domain Events within the system.
///
/// A **Domain Event** represents something that has happened in the past
/// that domain experts care about. They are produced by Aggregate Roots
/// as a result of a state change and are typically used for:
///
/// 1. **Consistency:** Ensuring side effects happen in other aggregates.
/// 2. **Audit Trails:** Tracking the history of changes over time.
/// 3. **Integration:** Notifying external systems via a Message Bus or Outbox.
///
/// ### Type Parameters
/// * `T`: The type used for the `aggregate_id`. Usually [Uuid] or a specialized [String].
///
/// ### Implementation Notes
/// Events should be treated as **immutable**. Once created, the data
/// within an event must not change, as it represents a historical fact.
pub trait DomainEvent<T> {
    /// Returns the identifier of the aggregate that produced this event.
    fn aggregate_id(&self) -> &T;

    /// Returns the unique identifier for this event instance.
    fn event_id(&self) -> &uuid::Uuid;

    /// Returns the timestamp indicating when the event occurred.
    fn timestamp(&self) -> NaiveDateTime;
}
