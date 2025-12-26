use rand::distr::{Alphanumeric, SampleString};

/// Generate a random alphanumeric `String` of length `len`.
///
/// Returns an empty `String` when `len` is `0`.
pub fn random_str(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}
