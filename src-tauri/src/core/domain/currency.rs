use serde::{Deserialize, Serialize};

/// Currency codes supported by the application.
///
/// The enum uses a small, explicit set of currencies for now. Use
/// `Currency::from_code` to obtain a `Currency` value from an ISO-style
/// currency code (case-insensitive).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, specta::Type,
)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
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
    pub fn from_code(code: &str) -> Result<Currency, CurrencyError> {
        match code.to_uppercase().as_str() {
            "EUR" => Ok(Currency::EUR),
            "USD" => Ok(Currency::USD),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            other => Err(CurrencyError::UnsupportedCurrency(other.to_string())),
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

    /// Return the ISO-style code for this currency.
    pub fn to_code(&self) -> &'static str {
        match self {
            Currency::EUR => "EUR",
            Currency::USD => "USD",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CurrencyError {
    #[error("Unsupported currency code: {0}")]
    UnsupportedCurrency(String),
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
