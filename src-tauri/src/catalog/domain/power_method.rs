use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Power method used by rolling stocks.
///
/// This enum represents how a model locomotive obtains electrical power.
/// The `Display` implementation returns a human-friendly name for each variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerMethod {
    /// Alternating current (AC) power collection.
    AC,
    /// Direct current (DC) power collection.
    DC,
    /// Trix Express three-rail power pickup system.
    TrixExpress,
}

/// Human-friendly formatting for `PowerMethod`.
///
/// Output values:
/// - `AC` -> "AC"
/// - `DC` -> "DC"
/// - `TrixExpress` -> "Trix Express"
impl fmt::Display for PowerMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerMethod::AC => write!(f, "AC"),
            PowerMethod::DC => write!(f, "DC"),
            PowerMethod::TrixExpress => write!(f, "Trix Express"),
        }
    }
}

// Static error message used when parsing fails
const INVALID_POWER_METHOD: &str = "invalid power method";

impl TryFrom<&str> for PowerMethod {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "AC" => Ok(PowerMethod::AC),
            "DC" => Ok(PowerMethod::DC),
            "Trix Express" => Ok(PowerMethod::TrixExpress),
            // Accept the variant name without space as convenience
            "TrixExpress" => Ok(PowerMethod::TrixExpress),
            _ => Err(anyhow::anyhow!(INVALID_POWER_METHOD)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(PowerMethod::AC, "AC")]
    #[case(PowerMethod::DC, "DC")]
    #[case(PowerMethod::TrixExpress, "Trix Express")]
    fn display_variants(#[case] pm: PowerMethod, #[case] expected: &str) {
        assert_eq!(pm.to_string(), expected);
    }

    #[rstest]
    #[case("AC", PowerMethod::AC)]
    #[case("DC", PowerMethod::DC)]
    #[case("Trix Express", PowerMethod::TrixExpress)]
    #[case("TrixExpress", PowerMethod::TrixExpress)]
    fn try_from_valid(#[case] input: &str, #[case] expected: PowerMethod) {
        let parsed = PowerMethod::try_from(input).expect("should parse");
        assert_eq!(parsed, expected);
    }

    #[test]
    fn try_from_invalid_returns_error() {
        let res = PowerMethod::try_from("unknown");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(format!("{}", err).contains(INVALID_POWER_METHOD));
    }
}
