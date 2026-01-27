use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The condition under which a collectible item was purchased.
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
pub enum PurchaseCondition {
    /// The item was purchased brand new from a retailer.
    #[default]
    New,
    /// The item was purchased second-hand from another collector or seller.
    PreOwned,
}

/// Garde validator for `PurchaseCondition`.
#[allow(dead_code)]
pub fn validate_purchase_condition(value: &str, _ctx: &()) -> garde::Result {
    if value.parse::<PurchaseCondition>().is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("error_invalid_purchase_condition"))
    }
}

#[cfg(test)]
mod validator_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("NEW")]
    #[case("PRE_OWNED")]
    fn validate_purchase_condition_all(#[case] s: &str) {
        assert!(validate_purchase_condition(s, &()).is_ok());
        assert!(validate_purchase_condition(&s.to_lowercase(), &()).is_ok());
    }

    #[test]
    fn validate_purchase_condition_rejects_invalid() {
        let err = validate_purchase_condition("BADVALUE", &()).unwrap_err();
        assert!(err.to_string().contains("error_invalid_purchase_condition"));
    }
}
