use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Status of a manufacturer lifecycle.
///
/// Serialized as SCREAMING_SNAKE_CASE; parsing is case-insensitive.
#[derive(
    Debug,
    Copy,
    Clone,
    Default,
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManufacturerStatus {
    #[default]
    Active,
    Merged,
    OutOfBusiness,
}

/// Garde validator for ManufacturerStatus values used in command arguments.
#[allow(dead_code)]
pub fn validate_manufacturer_status(value: &str, _ctx: &()) -> garde::Result {
    if ManufacturerStatus::try_from(value).is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_manufacturer_status"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("ACTIVE", Ok(ManufacturerStatus::Active))]
    #[case("active", Ok(ManufacturerStatus::Active))]
    #[case("MERGED", Ok(ManufacturerStatus::Merged))]
    #[case("OUT_OF_BUSINESS", Ok(ManufacturerStatus::OutOfBusiness))]
    fn it_should_parse_string_as_manufacturer_status(
        #[case] input: &str,
        #[case] expected: Result<ManufacturerStatus, ParseError>,
    ) {
        let result = input.parse::<ManufacturerStatus>();
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(ManufacturerStatus::Active, "ACTIVE")]
    #[case(ManufacturerStatus::Merged, "MERGED")]
    #[case(ManufacturerStatus::OutOfBusiness, "OUT_OF_BUSINESS")]
    fn it_should_display_manufacturer_status(
        #[case] input: ManufacturerStatus,
        #[case] expected: &str,
    ) {
        assert_eq!(expected, input.to_string());
    }

    #[test]
    fn it_should_try_from_invalid_returns_error() {
        let res = ManufacturerStatus::try_from("unknown");
        assert!(res.is_err());
    }

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("ACTIVE")]
        #[case("MERGED")]
        #[case("OUT_OF_BUSINESS")]
        fn validate_manufacturer_status_accepts_all(#[case] input: &str) {
            assert!(validate_manufacturer_status(input, &()).is_ok());
            assert!(validate_manufacturer_status(&input.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_manufacturer_status_rejects_invalid() {
            let err = validate_manufacturer_status("INVALID", &()).unwrap_err();
            assert!(
                err.to_string()
                    .contains("error_invalid_manufacturer_status")
            );
        }
    }
}
