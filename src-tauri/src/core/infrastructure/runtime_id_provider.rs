use crate::core::domain::IdProvider;

/// A runtime provider that uses `T::default()` as the source of new ids.
///
/// This simple provider is useful for production code where IDs are
/// represented by types that implement `Default` (for example newtypes that
/// generate a fresh value on `Default`, or numeric types where `Default` is
/// zero). For more sophisticated strategies, implement a custom provider in
/// infrastructure.
#[allow(dead_code)]
pub(crate) struct RuntimeIdProvider;

#[allow(dead_code)]
impl RuntimeIdProvider {
    /// Construct a new runtime provider.
    pub fn new() -> Self {
        Self
    }
}

impl<T: Default> IdProvider<T> for RuntimeIdProvider {
    fn next_id(&self) -> T {
        T::default()
    }
}
