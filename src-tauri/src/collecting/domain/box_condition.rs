use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The condition of the original packaging box for a collectible item.
///
/// ### Notes
/// In the railway hobby, the box can sometimes represent 30%–50% of the total value, especially for vintage brands like Hornby Dublo or Lionel.
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
pub enum BoxCondition {
    /// Box is crisp, no tears, no "shelf wear."
    #[default]
    OriginalMint,
    /// Some corner scuffing or minor creases.
    OriginalGood,
    /// Significant tears, tape repairs, or faded colors.
    OriginalWorn,
    /// Not the original box, but a suitable storage box.
    ReplacementBox,
    /// "Loose" model with no packaging at all.
    NoBox,
}

/// Garde validator for `BoxCondition`.
#[allow(dead_code)]
pub fn validate_box_condition(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<BoxCondition>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_box_condition"))
    }
}

#[cfg(test)]
mod validator_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ORIGINAL_MINT")]
    #[case("ORIGINAL_GOOD")]
    #[case("NO_BOX")]
    fn validate_box_condition_accepts_all(#[case] s: &str) {
        assert!(validate_box_condition(s, &()).is_ok());
        assert!(validate_box_condition(&s.to_lowercase(), &()).is_ok());
    }

    #[test]
    fn validate_box_condition_rejects_invalid() {
        let err = validate_box_condition("INVALID", &()).unwrap_err();
        assert!(err.to_string().contains("error_invalid_box_condition"));
    }
}
