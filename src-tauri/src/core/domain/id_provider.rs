//! ID provider trait for the core domain.
//!
//! This trait decouples the generation of identifiers from callers so that
//! production code can use real/runnable strategies while tests can inject
//! deterministic, repeatable providers. Use `IdProvider<T>` wherever code
//! needs to obtain a fresh `T` identifier without committing to how that
//! identifier is created.
//!
//! Example (production): implement a provider that returns `T::default()` or
//! delegates to a sequence generator. Example (testing): provide a
//! `MockIdProvider` that always returns a fixed value or a
//! `SequentialIdProvider` that yields a predetermined sequence of values.
//!
//! The trait intentionally takes `&self` rather than `&mut self` so that
//! implementations that require interior mutability (e.g. using
//! `RefCell<VecDeque<T>>`) can still be used through shared references.

/// A capability for producing identifiers of type `T`.
///
/// Implementations produce the next identifier when `next_id` is called.
/// This trait exists to make code that requires identifiers easier to test by
/// allowing injection of mock or deterministic providers.
pub trait IdProvider<T> {
    /// Return the next identifier of type `T`.
    fn next_id(&self) -> T;
}
