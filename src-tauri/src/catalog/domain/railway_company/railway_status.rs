use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The lifecycle status of a railway company.
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
pub enum RailwayStatus {
    /// The railway company is active and operational.
    #[default]
    Active,
    /// The railway company is inactive and no longer operational.
    Inactive,
    /// The railway company has merged with another entity.
    Merged,
}

/// Garde validator for `RailwayStatus`.
#[allow(dead_code)]
pub fn validate_railway_status(value: &str, _ctx: &()) -> garde::Result {
    if RailwayStatus::try_from(value).is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_railway_status"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("ACTIVE", Ok(RailwayStatus::Active))]
    #[case("INACTIVE", Ok(RailwayStatus::Inactive))]
    #[case("MERGED", Ok(RailwayStatus::Merged))]
    #[case("invalid", Err(ParseError::VariantNotFound))]
    fn it_should_parse_string_as_railway_status(
        #[case] input: &str,
        #[case] expected: Result<RailwayStatus, ParseError>,
    ) {
        let status = input.parse::<RailwayStatus>();
        assert_eq!(expected, status);
    }

    #[rstest]
    #[case(RailwayStatus::Active, "ACTIVE")]
    #[case(RailwayStatus::Inactive, "INACTIVE")]
    #[case(RailwayStatus::Merged, "MERGED")]
    fn it_should_display_railway_status(#[case] input: RailwayStatus, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("ACTIVE")]
        #[case("INACTIVE")]
        #[case("MERGED")]
        fn validate_railway_status_accepts_all(#[case] s: &str) {
            assert!(validate_railway_status(s, &()).is_ok());
            assert!(validate_railway_status(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_railway_status_rejects_invalid() {
            let err = validate_railway_status("XYZ", &()).unwrap_err();
            assert!(err.to_string().contains("error_invalid_railway_status"));
        }
    }
}
