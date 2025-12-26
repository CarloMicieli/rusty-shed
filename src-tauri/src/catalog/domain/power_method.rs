use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Power method used by rolling stocks.
///
/// This enum represents how a model locomotive obtains electrical power.
/// The `Display` implementation returns a human-friendly name for each variant.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerMethod {
    /// Alternating current (AC) power collection.
    AC,
    /// Direct current (DC) power collection.
    DC,
    /// Trix Express three-rail power pickup system.
    TrixExpress,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(PowerMethod::AC, "AC")]
    #[case(PowerMethod::DC, "DC")]
    #[case(PowerMethod::TrixExpress, "TRIX_EXPRESS")]
    fn display_variants(#[case] pm: PowerMethod, #[case] expected: &str) {
        assert_eq!(pm.to_string(), expected);
    }

    #[rstest]
    #[case("AC", PowerMethod::AC)]
    #[case("DC", PowerMethod::DC)]
    #[case("TRIX_EXPRESS", PowerMethod::TrixExpress)]
    fn try_from_valid(#[case] input: &str, #[case] expected: PowerMethod) {
        let parsed = PowerMethod::try_from(input).expect("should parse");
        assert_eq!(parsed, expected);
    }

    #[test]
    fn try_from_invalid_returns_error() {
        let res = PowerMethod::try_from("unknown");
        assert!(res.is_err());
    }
}
