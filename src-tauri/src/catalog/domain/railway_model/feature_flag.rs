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
    sqlx::Type,
    specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
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

/// Garde validator for `FeatureFlag`.
#[allow(dead_code)]
pub fn validate_feature_flag(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<FeatureFlag>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_feature_flag"))
    }
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

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("YES")]
        #[case("NO")]
        fn validate_feature_flag_accepts_all(#[case] s: &str) {
            assert!(validate_feature_flag(s, &()).is_ok());
            assert!(validate_feature_flag(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_feature_flag_rejects_invalid() {
            let err = validate_feature_flag("BAD", &()).unwrap_err();
            assert!(err.to_string().contains("error_invalid_feature_flag"));
        }
    }
}
