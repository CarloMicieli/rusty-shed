use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// A flag to indicate the presence/absence of a given technical specification feature
#[derive(
    Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Default,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeatureFlag {
    /// Yes: the feature is present
    Yes,
    /// No: the feature is missing
    No,
    /// The feature is not applicable
    #[default]
    NotApplicable,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("YES", Ok(FeatureFlag::Yes))]
    #[case("NO", Ok(FeatureFlag::No))]
    #[case("NOT_APPLICABLE", Ok(FeatureFlag::NotApplicable))]
    #[case("invalid", Err(ParseError::VariantNotFound))]
    fn it_should_parse_strings_as_feature_flags(
        #[case] input: &str,
        #[case] expected: Result<FeatureFlag, ParseError>,
    ) {
        let flag = input.parse::<FeatureFlag>();
        assert_eq!(expected, flag);
    }

    #[rstest]
    #[case(FeatureFlag::Yes, "YES")]
    #[case(FeatureFlag::No, "NO")]
    #[case(FeatureFlag::NotApplicable, "NOT_APPLICABLE")]
    fn it_should_display_feature_flags(#[case] input: FeatureFlag, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
