use super::IdProvider;
use std::{cell::RefCell, collections::VecDeque};

/// Test utilities for deterministic id generation.
///
/// These helpers live in the `core::domain` test utilities module and are
/// intended only for use inside unit/integration tests.
pub struct MockIdProvider<T: Clone> {
    value: T,
}

impl<T: Clone> MockIdProvider<T> {
    /// Create a new `MockIdProvider` that always returns `value`.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> IdProvider<T> for MockIdProvider<T> {
    fn next_id(&self) -> T {
        self.value.clone()
    }
}

/// A mock ID provider that returns the `Default` value for `T`.
///
/// Useful in tests where calling `Default::default()` on the provider
/// should yield a deterministic identifier (the `T::default()` value).
#[derive(Debug, Clone)]
pub struct DefaultMockIdProvider<T: Clone + Default> {
    value: T,
}

impl<T: Clone + Default> DefaultMockIdProvider<T> {
    /// Create a new provider that will return `T::default()`.
    pub fn new() -> Self {
        Self {
            value: T::default(),
        }
    }

    /// Get the value that will be returned by the provider.
    pub fn value(&self) -> T {
        self.value.clone()
    }
}

impl<T: Clone + Default> Default for DefaultMockIdProvider<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Default> IdProvider<T> for DefaultMockIdProvider<T> {
    fn next_id(&self) -> T {
        self.value.clone()
    }
}

/// Provider that yields a predetermined sequence of values.
///
/// Uses a `RefCell<VecDeque<T>>` so the provider can be used through a
/// shared reference (`&self`) while still mutating internal state.
pub struct SequentialIdProvider<T> {
    queue: RefCell<VecDeque<T>>,
}

impl<T> SequentialIdProvider<T> {
    /// Create a new sequential provider from the provided vector of items.
    pub fn new(items: Vec<T>) -> Self {
        Self {
            queue: RefCell::new(VecDeque::from(items)),
        }
    }
}

impl<T> IdProvider<T> for SequentialIdProvider<T> {
    fn next_id(&self) -> T {
        self.queue
            .borrow_mut()
            .pop_front()
            .expect("SequentialIdProvider exhausted")
    }
}
