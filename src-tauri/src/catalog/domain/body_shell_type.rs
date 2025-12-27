use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The construction type of rolling stock's body shell.
///
/// This enum describes the material / manufacturing technique used for the outer
/// body shell of a model (for example, a plastic injection-moulded shell vs a
/// metal die-cast shell). It is used in technical specifications to describe
/// the build of the vehicle's exterior.
#[derive(
    Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BodyShellType {
    /// Plastic body shell construction (typically injection moulded plastic).
    Plastic,

    /// Metal die-cast body shell construction (heavier, metal cast components).
    MetalDieCast,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("PLASTIC", Ok(BodyShellType::Plastic))]
    #[case("METAL_DIE_CAST", Ok(BodyShellType::MetalDieCast))]
    fn parse_variants_shouting_case(
        #[case] input: &str,
        #[case] expected: Result<BodyShellType, ParseError>,
    ) {
        let result = input.parse::<BodyShellType>();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_invalid_returns_error() {
        let result = "NOT_A_BODY_SHELL_TYPE".parse::<BodyShellType>();
        assert_eq!(Err(ParseError::VariantNotFound), result);
    }

    #[test]
    fn parse_lowercase() {
        let result = "plastic".parse::<BodyShellType>();
        assert_eq!(Ok(BodyShellType::Plastic), result);
    }

    #[rstest]
    #[case(BodyShellType::Plastic, "PLASTIC")]
    #[case(BodyShellType::MetalDieCast, "METAL_DIE_CAST")]
    fn display_variants(#[case] input: BodyShellType, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
