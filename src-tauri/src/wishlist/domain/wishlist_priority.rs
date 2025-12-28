use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Priority assigned to a wishlist item.
///
/// Indicates how important or urgent an item is for the user. The enum is
/// serialized as SCREAMING_SNAKE_CASE (e.g. `"LOW"`, `"NORMAL"`, `"HIGH"`)
/// and supports case-insensitive parsing via `FromStr` thanks to
/// `strum_macros::EnumString`.
///
/// The default variant is `Normal`.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Default, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WishlistPriority {
    Low,
    #[default]
    Normal,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_default_variant() {
        assert_eq!(WishlistPriority::default(), WishlistPriority::Normal);
    }

    #[rstest]
    #[case(WishlistPriority::Low, "\"LOW\"")]
    #[case(WishlistPriority::Normal, "\"NORMAL\"")]
    #[case(WishlistPriority::High, "\"HIGH\"")]
    fn test_serde_serialization_tokens(#[case] input: WishlistPriority, #[case] expected: &str) {
        assert_eq!(serde_json::to_string(&input).unwrap(), expected);
    }

    #[rstest]
    #[case("\"LOW\"", WishlistPriority::Low)]
    #[case("\"NORMAL\"", WishlistPriority::Normal)]
    #[case("\"HIGH\"", WishlistPriority::High)]
    fn test_serde_deserialization_tokens(#[case] input: &str, #[case] expected: WishlistPriority) {
        assert_eq!(
            serde_json::from_str::<WishlistPriority>(input).unwrap(),
            expected
        );
    }

    #[rstest]
    #[case("LOW", WishlistPriority::Low)]
    #[case("low", WishlistPriority::Low)]
    #[case("LoW", WishlistPriority::Low)]
    #[case("NORMAL", WishlistPriority::Normal)]
    #[case("normal", WishlistPriority::Normal)]
    #[case("HIGH", WishlistPriority::High)]
    #[case("high", WishlistPriority::High)]
    fn test_fromstr_ascii_case_insensitive(
        #[case] input: &str,
        #[case] expected: WishlistPriority,
    ) {
        assert_eq!(WishlistPriority::from_str(input).unwrap(), expected);
    }

    #[rstest]
    #[case("NOT_A_PRIORITY")]
    #[case("medium")]
    fn test_fromstr_invalid(#[case] input: &str) {
        assert!(WishlistPriority::from_str(input).is_err());
    }
}
