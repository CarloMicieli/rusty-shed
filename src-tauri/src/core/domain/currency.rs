//! Core currency types used across the application.
//!
//! This module provides the `Currency` enum for a small set of supported
//! currencies and helpers to parse and format currency codes and symbols.

use crate::core::domain::error::Error;
use serde::{Deserialize, Serialize};

/// Currency codes supported by the application.
///
/// The enum uses a small, explicit set of currencies for now. Use
/// `Currency::from_code` to obtain a `Currency` value from an ISO-style
/// currency code (case-insensitive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum Currency {
    /// Euro
    EUR,
    /// United States Dollar
    USD,
    /// Great Britain Pound
    GBP,
    /// Japanese Yen
    JPY,
}

impl Currency {
    /// Parse an ISO-style currency code (case-insensitive) into a `Currency`.
    ///
    /// Returns `Ok(Currency)` for known codes (`"EUR"`, `"USD"`, `"GBP"`,
    /// `"JPY"`) or an error for unsupported/unknown codes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rusty_shed_lib::core::domain::Currency;
    /// let c = Currency::from_code("eur").unwrap();
    /// assert_eq!(c, Currency::EUR);
    /// ```
    pub fn from_code(code: &str) -> Result<Currency, Error> {
        match code.to_uppercase().as_str() {
            "EUR" => Ok(Currency::EUR),
            "USD" => Ok(Currency::USD),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            other => Err(Error::UnsupportedCurrency(other.to_string())),
        }
    }

    /// Return the Unicode symbol commonly used for this currency.
    ///
    /// Note: this is a simple helper for UI formatting; for full localization
    /// you might want to use a dedicated i18n/locale library.
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::EUR => "€",
            Currency::USD => "$",
            Currency::GBP => "£",
            Currency::JPY => "¥",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_from_code_ok() {
        assert_eq!(Currency::from_code("EUR").unwrap(), Currency::EUR);
        assert_eq!(Currency::from_code("usd").unwrap(), Currency::USD);
        assert_eq!(Currency::from_code("Gbp").unwrap(), Currency::GBP);
        assert_eq!(Currency::from_code("JPY").unwrap(), Currency::JPY);
    }

    #[test]
    fn currency_from_code_err() {
        assert!(Currency::from_code("ABC").is_err());
    }
}
