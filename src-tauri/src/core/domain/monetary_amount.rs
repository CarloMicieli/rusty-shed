type Result<T> = std::result::Result<T, MonetaryAmountError>;

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::core::domain::currency::{Currency, CurrencyError};

/// A monetary amount in the smallest currency unit together with its currency.
///
/// `MonetaryAmount` stores the raw integer amount (e.g. cents) in `amount` and
/// the `currency` that the amount is denominated in. Prefer using the provided
/// constructors rather than populating fields directly.
///
/// `MonetaryAmount` provides helpers to build an instance from database parts
/// (`MonetaryAmount::from_db`), to add values when currencies match
/// (`add_same_currency`) and to format the value for display.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct MonetaryAmount {
    /// Amount stored in the smallest unit (e.g. cents for EUR/USD/GBP).
    pub amount: u64,

    /// Currency of the amount.
    pub currency: Currency,
}

impl Default for MonetaryAmount {
    fn default() -> Self {
        Self {
            amount: 0,
            currency: Currency::USD,
        }
    }
}

impl MonetaryAmount {
    /// Create a new `MonetaryAmount` from a raw amount and currency.
    pub fn new(amount: u64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Construct from DB parts.
    ///
    /// Interprets `amount_i64` (signed integer read from the DB) and an
    /// optional `currency_code`. If `currency_code` is `None`, this function
    /// returns `Ok(None)` (the domain field becomes absent). If `currency_code`
    /// is present but unrecognized, or the amount is negative, an error is
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns an error when the currency code is unsupported or the amount is
    /// negative.
    pub fn from_db(amount_i64: i64, currency_code: Option<&str>) -> Result<Option<MonetaryAmount>> {
        match currency_code {
            None => Ok(None),
            Some(code) => {
                if amount_i64 < 0 {
                    return Err(MonetaryAmountError::NegativeAmount(amount_i64));
                }
                let currency = Currency::from_code(code)?;
                Ok(Some(MonetaryAmount::new(amount_i64 as u64, currency)))
            }
        }
    }

    /// Add two `MonetaryAmount` values with the same currency.
    ///
    /// Returns an error when the currencies differ or when the addition would
    /// overflow the `u64` range.
    pub fn add_same_currency(&self, other: &MonetaryAmount) -> Result<MonetaryAmount> {
        if self.currency != other.currency {
            return Err(MonetaryAmountError::CurrencyMismatch);
        }
        let sum = self
            .amount
            .checked_add(other.amount)
            .ok_or(MonetaryAmountError::Overflow)?;
        Ok(MonetaryAmount::new(sum, self.currency))
    }

    /// Convenience helper to combine two optional monetary amounts.
    ///
    /// - If both are `None` -> `Ok(None)`
    /// - If one is `Some` -> clone and return that value
    /// - If both `Some` -> add them if currencies match, otherwise return an error
    pub fn add_optional(
        a: Option<&MonetaryAmount>,
        b: Option<&MonetaryAmount>,
    ) -> Result<Option<MonetaryAmount>> {
        match (a, b) {
            (None, None) => Ok(None),
            (Some(x), None) => Ok(Some(x.clone())),
            (None, Some(y)) => Ok(Some(y.clone())),
            (Some(x), Some(y)) => Ok(Some(x.add_same_currency(y)?)),
        }
    }
}

impl fmt::Display for MonetaryAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency {
            Currency::JPY => {
                // No decimals for JPY
                write!(f, "{}{}", self.currency.symbol(), self.amount)
            }
            Currency::EUR => {
                // EUR: symbol after with space (e.g. "10.50 €")
                let major = self.amount / 100;
                let minor = self.amount % 100;
                write!(f, "{}.{:02} {}", major, minor, self.currency.symbol())
            }
            Currency::USD | Currency::GBP => {
                // symbol before, two decimals
                let major = self.amount / 100;
                let minor = self.amount % 100;
                write!(f, "{}{}.{:02}", self.currency.symbol(), major, minor)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MonetaryAmountError {
    /// Unsupported or unknown currency code.
    #[error("{0}")]
    UnsupportedCurrency(#[from] CurrencyError),

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::currency::Currency;
    use rstest::rstest;

    #[rstest]
    #[case(1050, Currency::EUR, "10.50 €")]
    #[case(1234, Currency::USD, "$12.34")]
    #[case(500, Currency::GBP, "£5.00")]
    #[case(1000, Currency::JPY, "¥1000")]
    fn monetary_display_formats(
        #[case] amount: u64,
        #[case] currency: Currency,
        #[case] expected: &str,
    ) {
        let m = MonetaryAmount::new(amount, currency);
        assert_eq!(m.to_string(), expected);
    }

    #[test]
    fn monetary_from_db_none() {
        let monetary_amount = MonetaryAmount::from_db(0, None).unwrap();
        assert!(monetary_amount.is_none());
    }

    #[test]
    fn monetary_from_db_negative() {
        let result = MonetaryAmount::from_db(-1, Some("EUR"));
        assert!(result.is_err());
        let err = result.err().unwrap();
        match err {
            MonetaryAmountError::NegativeAmount(_) => {
                assert_eq!(err.to_string(), "Negative monetary amount: -1");
            }
            _ => panic!("Expected UnsupportedCurrency error"),
        }
    }

    #[test]
    fn monetary_from_db_invalid_currency() {
        let result = MonetaryAmount::from_db(100, Some("INVALID"));
        assert!(result.is_err());

        let err = result.err().unwrap();
        match err {
            MonetaryAmountError::UnsupportedCurrency(ref code) => {
                assert_eq!(err.to_string(), "Unsupported currency code: INVALID");
                assert_eq!(code.to_string(), "Unsupported currency code: INVALID");
            }
            _ => panic!("Expected UnsupportedCurrency error"),
        }
    }

    #[rstest]
    #[case(100, 250, Currency::EUR, 350)]
    fn add_same_currency_ok(
        #[case] a: u64,
        #[case] b: u64,
        #[case] currency: Currency,
        #[case] expected: u64,
    ) {
        let a = MonetaryAmount::new(a, currency);
        let b = MonetaryAmount::new(b, currency);
        let s = a.add_same_currency(&b).unwrap();
        assert_eq!(s.amount, expected);
        assert_eq!(s.currency, currency);
    }

    #[rstest]
    fn add_same_currency_mismatch() {
        let a = MonetaryAmount::new(100, Currency::EUR);
        let b = MonetaryAmount::new(100, Currency::USD);
        assert!(a.add_same_currency(&b).is_err());
    }
}
