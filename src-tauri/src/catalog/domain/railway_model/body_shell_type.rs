use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The construction type of rolling stock's body shell.
///
/// This enum describes the material / manufacturing technique used for the outer
/// body shell of a model (for example, a plastic injection-moulded shell vs a
/// metal die-cast shell). It is used in technical specifications to describe
/// the build of the vehicle's exterior.
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
    sqlx::Type,
    specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BodyShellType {
    /// Plastic body shell construction (typically injection moulded plastic).
    Plastic,

    /// Metal die-cast body shell construction (heavier, metal cast components).
    MetalDieCast,
}

/// Garde validator for `BodyShellType`.
#[allow(dead_code)]
pub fn validate_body_shell_type(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<BodyShellType>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_body_shell_type"))
    }
}

/// Garde validator for `Option<String>` that must parse as `BodyShellType` when present.
#[allow(dead_code)]
pub fn validate_opt_body_shell_type(value: &Option<String>, _ctx: &()) -> garde::Result {
    match value {
        Some(s) => {
            if s.parse::<BodyShellType>().is_ok() {
                Ok(())
            } else {
                Err(garde::Error::new("error_invalid_body_shell_type"))
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
    fn it_should_parse_invalid_returns_error() {
        let result = "NOT_A_BODY_SHELL_TYPE".parse::<BodyShellType>();
        assert_eq!(Err(ParseError::VariantNotFound), result);
    }

    #[test]
    fn it_should_parse_lowercase() {
        let result = "plastic".parse::<BodyShellType>();
        assert_eq!(Ok(BodyShellType::Plastic), result);
    }

    #[rstest]
    #[case(BodyShellType::Plastic, "PLASTIC")]
    #[case(BodyShellType::MetalDieCast, "METAL_DIE_CAST")]
    fn display_variants(#[case] input: BodyShellType, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }

    mod validator_tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("PLASTIC")]
        #[case("METAL_DIE_CAST")]
        fn validate_body_shell_type_accepts_all(#[case] s: &str) {
            assert!(validate_body_shell_type(s, &()).is_ok());
            assert!(validate_body_shell_type(&s.to_lowercase(), &()).is_ok());
        }

        #[test]
        fn validate_body_shell_type_rejects_invalid() {
            let err = validate_body_shell_type("BAD", &()).unwrap_err();
            assert!(err.to_string().contains("error_invalid_body_shell_type"));
        }
    }
}
