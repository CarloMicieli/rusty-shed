use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

/// Represents the service class(es) for a rolling stock or service.
///
/// | **Variant**                      | **Description**           |
/// |:---------------------------------|:--------------------------|
/// | `ServiceLevel::First`            | `1st class`               |
/// | `ServiceLevel::Second`           | `2nd class`               |
/// | `ServiceLevel::Third`            | `3rd class`               |
/// | `ServiceLevel::FirstSecond`      | `Mixed 1st/2nd class`     |
/// | `ServiceLevel::SecondThird`      | `Mixed 2nd/3rd class`     |
/// | `ServiceLevel::FirstSecondThird` | `Mixed 1st/2nd/3rd class` |
///
/// Parsing: `TryFrom<&str>` is implemented and accepts the string forms above
/// (whitespace is trimmed). Formatting: `Display` is implemented and produces
/// the corresponding string representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceLevel {
    First,
    Second,
    Third,
    FirstSecond,
    SecondThird,
    FirstSecondThird,
}

impl Display for ServiceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ServiceLevel::First => write!(f, "1"),
            ServiceLevel::Second => write!(f, "2"),
            ServiceLevel::Third => write!(f, "3"),
            ServiceLevel::FirstSecond => write!(f, "1/2"),
            ServiceLevel::SecondThird => write!(f, "2/3"),
            ServiceLevel::FirstSecondThird => write!(f, "1/2/3"),
        }
    }
}

// Static error message used when parsing fails
const INVALID_SERVICE_LEVEL: &str = "invalid service level";

impl TryFrom<&str> for ServiceLevel {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "1" => Ok(ServiceLevel::First),
            "2" => Ok(ServiceLevel::Second),
            "3" => Ok(ServiceLevel::Third),
            "1/2" => Ok(ServiceLevel::FirstSecond),
            "2/3" => Ok(ServiceLevel::SecondThird),
            "1/2/3" => Ok(ServiceLevel::FirstSecondThird),
            _ => Err(anyhow::anyhow!(INVALID_SERVICE_LEVEL)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(ServiceLevel::First, "1")]
    #[case(ServiceLevel::Second, "2")]
    #[case(ServiceLevel::Third, "3")]
    #[case(ServiceLevel::FirstSecond, "1/2")]
    #[case(ServiceLevel::SecondThird, "2/3")]
    #[case(ServiceLevel::FirstSecondThird, "1/2/3")]
    fn display_service_level(#[case] input: ServiceLevel, #[case] expected: &str) {
        assert_eq!(input.to_string(), expected);
    }

    #[rstest]
    #[case("1", ServiceLevel::First)]
    #[case("2", ServiceLevel::Second)]
    #[case("3", ServiceLevel::Third)]
    #[case("1/2", ServiceLevel::FirstSecond)]
    #[case("2/3", ServiceLevel::SecondThird)]
    #[case("1/2/3", ServiceLevel::FirstSecondThird)]
    fn try_from_valid_values(#[case] input: &str, #[case] expected: ServiceLevel) {
        let parsed = ServiceLevel::try_from(input).expect("should parse");
        assert_eq!(parsed, expected);
    }

    #[test]
    fn try_from_invalid_value_returns_error() {
        let err = ServiceLevel::try_from("invalid");
        assert!(err.is_err());
        // Ensure the error contains the static message
        let err = err.unwrap_err();
        assert!(format!("{}", err).contains(INVALID_SERVICE_LEVEL));
    }
}
