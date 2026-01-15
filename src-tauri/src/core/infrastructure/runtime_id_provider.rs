use crate::core::domain::IdProvider;

/// A runtime provider that uses `T::default()` as the source of new ids.
///
/// This simple provider is useful for production code where IDs are
/// represented by types that implement `Default` (for example newtypes that
/// generate a fresh value on `Default`, or numeric types where `Default` is
/// zero). For more sophisticated strategies, implement a custom provider in
/// infrastructure.
pub struct RuntimeIdProvider;

impl RuntimeIdProvider {
    /// Construct a new runtime provider.
    pub fn new() -> Self {
        Self
    }
}

impl Default for RuntimeIdProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> IdProvider<T> for RuntimeIdProvider {
    fn next_id(&self) -> T {
        T::default()
    }
}
