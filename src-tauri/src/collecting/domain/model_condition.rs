use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// This represents the physical and mechanical state of the locomotive or rolling stock.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumString,
    Display,
    Default,
    Serialize,
    Deserialize,
    sqlx::Type,
    specta::Type,
)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelCondition {
    /// Brand new, no signs of use, factory fresh.
    #[default]
    Mint,
    /// Almost like new, with only very minor signs of handling.
    NearMint,
    /// Clean, very light use, no missing detail parts.
    Excellent,
    /// Minor wear from use; paint is still strong.
    VeryGood,
    /// Visible wear, small scratches, but functional.
    Good,
    /// Significant wear, missing small parts (couplers, buffers).
    Fair,
    /// Heavy damage, non-functional, or heavily modified.
    Poor,
    /// Not a runner; only useful for salvaging components.
    ForParts,
}

/// Garde validator for `ModelCondition`.
#[allow(dead_code)]
pub fn validate_model_condition(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<ModelCondition>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_model_condition"))
    }
}

#[cfg(test)]
mod validator_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("MINT")]
    #[case("NEAR_MINT")]
    #[case("FAIR")]
    fn validate_model_condition_accepts_all(#[case] s: &str) {
        assert!(validate_model_condition(s, &()).is_ok());
        assert!(validate_model_condition(&s.to_lowercase(), &()).is_ok());
    }

    #[test]
    fn validate_model_condition_rejects_invalid() {
        let err = validate_model_condition("BADVALUE", &()).unwrap_err();
        assert!(err.to_string().contains("error_invalid_model_condition"));
    }
}
