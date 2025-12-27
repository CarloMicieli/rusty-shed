use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Lifecycle availability status of a railway model.
///
/// The enum variants represent common product lifecycle states. When
/// serialized via `serde` the variants use SCREAMING_SNAKE_CASE; likewise
/// string parsing via `strum` expects SCREAMING_SNAKE_CASE but is
/// case-insensitive.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvailabilityStatus {
    /// The railway model is just announced and not yet available.
    Announced,

    /// The railway model is available for purchase.
    Available,

    /// The railway model production / release has been cancelled or delayed
    /// (not proceeding as previously announced).
    Cancelled,

    /// The railway model has been discontinued and is no longer produced.
    Discontinued,
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
}
