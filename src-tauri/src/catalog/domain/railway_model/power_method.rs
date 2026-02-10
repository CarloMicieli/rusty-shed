use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Power method used by rolling stocks.
///
/// This enum represents how a model locomotive obtains electrical power.
/// The `Display` implementation returns a human-friendly name for each variant.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    EnumString,
    Display,
    Serialize,
    Deserialize,
    sqlx::Type,
    specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerMethod {
    /// Alternating current (AC) power collection.
    #[serde(rename = "AC")]
    AC,
    /// Direct current (DC) power collection.
    #[serde(rename = "DC")]
    DC,
    /// Trix Express three-rail power pickup system.
    #[serde(rename = "TRIX_EXPRESS")]
    TrixExpress,
}

/// Garde validator for `PowerMethod` (case-insensitive parsing via `strum`).
#[allow(dead_code)]
pub fn validate_power_method(value: &str, _ctx: &()) -> garde::Result {
    if PowerMethod::try_from(value).is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_power_method"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
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
    fn it_should_try_from_invalid_returns_error() {
        let res = PowerMethod::try_from("unknown");
        assert!(res.is_err());
    }

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("AC")]
        #[case("DC")]
        #[case("TRIX_EXPRESS")]
        fn validate_power_method_accepts_all(#[case] s: &str) {
            assert!(validate_power_method(s, &()).is_ok());
            assert!(validate_power_method(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_power_method_rejects_invalid() {
            let e = validate_power_method("UNKNOWN", &()).unwrap_err();
            assert!(e.to_string().contains("error_invalid_power_method"));
        }
    }
}
