use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Default, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WishlistStatus {
    #[default]
    Wanted,
    OnOrder,
    Purchased,
    Ignored,
}

// Tests for WishlistStatus
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_default_variant() {
        assert_eq!(WishlistStatus::default(), WishlistStatus::Wanted);
    }

    #[test]
    fn test_serde_serialization_tokens() {
        assert_eq!(
            serde_json::to_string(&WishlistStatus::Wanted).unwrap(),
            "\"WANTED\""
        );
        assert_eq!(
            serde_json::to_string(&WishlistStatus::OnOrder).unwrap(),
            "\"ON_ORDER\""
        );
        assert_eq!(
            serde_json::to_string(&WishlistStatus::Purchased).unwrap(),
            "\"PURCHASED\""
        );
        assert_eq!(
            serde_json::to_string(&WishlistStatus::Ignored).unwrap(),
            "\"IGNORED\""
        );
    }

    #[test]
    fn test_serde_deserialization_tokens() {
        assert_eq!(
            serde_json::from_str::<WishlistStatus>("\"WANTED\"").unwrap(),
            WishlistStatus::Wanted
        );
        assert_eq!(
            serde_json::from_str::<WishlistStatus>("\"ON_ORDER\"").unwrap(),
            WishlistStatus::OnOrder
        );
        assert_eq!(
            serde_json::from_str::<WishlistStatus>("\"PURCHASED\"").unwrap(),
            WishlistStatus::Purchased
        );
        assert_eq!(
            serde_json::from_str::<WishlistStatus>("\"IGNORED\"").unwrap(),
            WishlistStatus::Ignored
        );
    }

    #[test]
    fn test_fromstr_ascii_case_insensitive() {
        // various casings should parse
        assert_eq!(
            WishlistStatus::from_str("WANTED").unwrap(),
            WishlistStatus::Wanted
        );
        assert_eq!(
            WishlistStatus::from_str("wanted").unwrap(),
            WishlistStatus::Wanted
        );
        assert_eq!(
            WishlistStatus::from_str("WanTeD").unwrap(),
            WishlistStatus::Wanted
        );

        assert_eq!(
            WishlistStatus::from_str("ON_ORDER").unwrap(),
            WishlistStatus::OnOrder
        );
        assert_eq!(
            WishlistStatus::from_str("on_order").unwrap(),
            WishlistStatus::OnOrder
        );
        assert_eq!(
            WishlistStatus::from_str("On_Order").unwrap(),
            WishlistStatus::OnOrder
        );
        assert_eq!(
            WishlistStatus::from_str("oN_oRdEr").unwrap(),
            WishlistStatus::OnOrder
        );
    }

    #[test]
    fn test_fromstr_invalid() {
        assert!(WishlistStatus::from_str("NOT_A_STATUS").is_err());
        // missing underscore should not match
        assert!(WishlistStatus::from_str("ONORDER").is_err());
    }
}
