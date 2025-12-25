use thiserror::Error;

/// Error types for core domain operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Unsupported or unknown currency code.
    #[error("Unsupported currency code: {0}")]
    UnsupportedCurrency(String),

    /// Negative amount read from the database where only non-negative values are allowed.
    #[error("Negative monetary amount: {0}")]
    NegativeAmount(i64),

    /// Attempt to combine amounts with different currencies.
    #[error("Cannot add MonetaryAmount with different currencies")]
    CurrencyMismatch,

    /// Arithmetic overflow while adding monetary amounts.
    #[error("Monetary amount overflow when adding")]
    Overflow,
}
