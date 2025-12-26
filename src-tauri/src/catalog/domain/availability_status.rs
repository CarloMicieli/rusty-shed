use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvailabilityStatus {
    /// the railway model is just announced, hence not yet available
    Announced,

    /// the railway model is available
    Available,

    /// the railway model is delayed
    Cancelled,

    /// the railway model is discontinued
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
