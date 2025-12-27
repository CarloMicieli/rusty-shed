use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Represents the availability or relevance of a specific model feature.
///
/// This is often used for technical specifications where a feature might
/// exist, be intentionally absent, or simply not be relevant to that
/// specific class of locomotive or rolling stock.
#[derive(
    Debug,
    Eq,
    PartialEq,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    Default,
    specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeatureFlag {
    /// The feature is present and functional on the model.
    Yes,

    /// The feature is not present on the model, though it might be
    /// expected or available on similar models.
    No,

    /// The feature is not relevant for this type of equipment.
    ///
    /// For example, a "Pantograph Type" flag would be `NotApplicable`
    /// for a Steam Locomotive.
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
