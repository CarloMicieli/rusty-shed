use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Lifecycle availability status of a railway model.
///
/// The enum variants represent common product lifecycle states. When
/// serialized via `serde` the variants use SCREAMING_SNAKE_CASE; likewise
/// string parsing via `strum` expects SCREAMING_SNAKE_CASE but is
/// case-insensitive.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    EnumString,
    Default,
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
pub enum AvailabilityStatus {
    /// The railway model is just announced and not yet available.
    Announced,

    /// The railway model is available for purchase.
    #[default]
    Available,

    /// The railway model production / release has been cancelled or delayed
    /// (not proceeding as previously announced).
    Cancelled,

    /// The railway model has been discontinued and is no longer produced.
    Discontinued,
}

/// Garde validator for `AvailabilityStatus`.
#[allow(dead_code)]
pub fn validate_availability_status(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<AvailabilityStatus>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_availability_status"))
    }
}

/// Garde validator for `Option<String>` that must parse as `AvailabilityStatus` when present.
#[allow(dead_code)]
pub fn validate_opt_availability_status(value: &Option<String>, _ctx: &()) -> garde::Result {
    match value {
        Some(s) => {
            if s.parse::<AvailabilityStatus>().is_ok() {
                Ok(())
            } else {
                Err(garde::Error::new("error_invalid_availability_status"))
            }
        }
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("ANNOUNCED", Ok(AvailabilityStatus::Announced))]
    #[case("AVAILABLE", Ok(AvailabilityStatus::Available))]
    #[case("CANCELLED", Ok(AvailabilityStatus::Cancelled))]
    #[case("DISCONTINUED", Ok(AvailabilityStatus::Discontinued))]
    fn it_should_parse_string_as_availability_status(
        #[case] input: &str,
        #[case] expected: Result<AvailabilityStatus, ParseError>,
    ) {
        let result = input.parse::<AvailabilityStatus>();
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(AvailabilityStatus::Announced, "ANNOUNCED")]
    #[case(AvailabilityStatus::Available, "AVAILABLE")]
    #[case(AvailabilityStatus::Cancelled, "CANCELLED")]
    #[case(AvailabilityStatus::Discontinued, "DISCONTINUED")]
    fn it_should_display_dcc_interfaces(#[case] input: AvailabilityStatus, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("AVAILABLE")]
        #[case("ANNOUNCED")]
        fn validate_availability_status_accepts_all(#[case] s: &str) {
            assert!(validate_availability_status(s, &()).is_ok());
            assert!(validate_availability_status(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_availability_status_rejects_invalid() {
            let err = validate_availability_status("NONE", &()).unwrap_err();
            assert!(
                err.to_string()
                    .contains("error_invalid_availability_status")
            );
        }
    }
}
