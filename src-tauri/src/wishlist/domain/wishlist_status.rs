use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Status of a wishlist item.
///
/// Represents the current lifecycle state for an item on the wishlist. The
/// enum is serialized as SCREAMING_SNAKE_CASE (e.g. `"WANTED"`,
/// `"ON_ORDER"`, `"PURCHASED"`, `"IGNORED"`) and supports case-insensitive
/// parsing via `FromStr` through `strum_macros::EnumString`.
///
/// The default variant is `Wanted`.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Default, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WishlistStatus {
    /// The user wants the item and it's a candidate for purchase.
    ///
    /// This is the default status used when adding a new item to a wishlist.
    #[default]
    Wanted,

    /// The item has been ordered from a seller and is expected to arrive.
    OnOrder,

    /// The item has been acquired / purchased and is no longer actively wanted.
    Purchased,

    /// The item has been explicitly ignored by the user (not interested).
    Ignored,
}

// Tests for WishlistStatus
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn it_should_test_default_variant() {
        assert_eq!(WishlistStatus::default(), WishlistStatus::Wanted);
    }

    #[rstest]
    #[case(WishlistStatus::Wanted, "\"WANTED\"")]
    #[case(WishlistStatus::OnOrder, "\"ON_ORDER\"")]
    #[case(WishlistStatus::Purchased, "\"PURCHASED\"")]
    #[case(WishlistStatus::Ignored, "\"IGNORED\"")]
    fn test_serde_serialization_tokens(#[case] input: WishlistStatus, #[case] expected: &str) {
        assert_eq!(serde_json::to_string(&input).unwrap(), expected);
    }

    #[rstest]
    #[case("\"WANTED\"", WishlistStatus::Wanted)]
    #[case("\"ON_ORDER\"", WishlistStatus::OnOrder)]
    #[case("\"PURCHASED\"", WishlistStatus::Purchased)]
    #[case("\"IGNORED\"", WishlistStatus::Ignored)]
    fn test_serde_deserialization_tokens(#[case] input: &str, #[case] expected: WishlistStatus) {
        assert_eq!(
            serde_json::from_str::<WishlistStatus>(input).unwrap(),
            expected
        );
    }

    #[rstest]
    #[case("WANTED", WishlistStatus::Wanted)]
    #[case("wanted", WishlistStatus::Wanted)]
    #[case("WanTeD", WishlistStatus::Wanted)]
    #[case("ON_ORDER", WishlistStatus::OnOrder)]
    #[case("on_order", WishlistStatus::OnOrder)]
    #[case("On_Order", WishlistStatus::OnOrder)]
    #[case("oN_oRdEr", WishlistStatus::OnOrder)]
    fn test_fromstr_ascii_case_insensitive(#[case] input: &str, #[case] expected: WishlistStatus) {
        assert_eq!(WishlistStatus::from_str(input).unwrap(), expected);
    }

    #[rstest]
    #[case("NOT_A_STATUS")]
    #[case("ONORDER")]
    fn test_fromstr_invalid(#[case] input: &str) {
        assert!(WishlistStatus::from_str(input).is_err());
    }
}
