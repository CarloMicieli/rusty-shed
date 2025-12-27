use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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
    specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RailwayStatus {
    #[default]
    Active,
    Inactive,
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
    fn it_should_display_railway_status(#[case] input: RailwayStatus, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
