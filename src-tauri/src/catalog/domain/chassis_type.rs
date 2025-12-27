use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The construction type of a rolling stock's chassis.
///
/// This enum indicates the material or manufacturing technique used for the
/// chassis (the structural frame and underbody) of a model. It is part of
/// the technical specifications describing the build quality and expected
/// properties of the model's underframe.
#[derive(
    Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChassisType {
    /// Plastic chassis construction (typically injection-moulded plastic).
    Plastic,

    /// Metal die-cast chassis construction (heavier, metal cast chassis parts).
    MetalDieCast,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case("PLASTIC", Ok(ChassisType::Plastic))]
    #[case("METAL_DIE_CAST", Ok(ChassisType::MetalDieCast))]
    // verify ascii case-insensitive parsing
    #[case("metal_die_cast", Ok(ChassisType::MetalDieCast))]
    fn parse_chassis_type(#[case] input: &str, #[case] expected: Result<ChassisType, ParseError>) {
        let result = input.parse::<ChassisType>();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_invalid_returns_error() {
        let result = "NOT_A_CHASSIS_TYPE".parse::<ChassisType>();
        assert_eq!(Err(ParseError::VariantNotFound), result);
    }

    #[rstest]
    #[case(ChassisType::Plastic, "PLASTIC")]
    #[case(ChassisType::MetalDieCast, "METAL_DIE_CAST")]
    fn display_chassis_type(#[case] input: ChassisType, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
