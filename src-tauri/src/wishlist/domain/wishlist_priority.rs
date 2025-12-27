use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

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
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_default_variant() {
        assert_eq!(WishlistPriority::default(), WishlistPriority::Normal);
    }

    #[test]
    fn test_serde_serialization_tokens() {
        assert_eq!(
            serde_json::to_string(&WishlistPriority::Low).unwrap(),
            "\"LOW\""
        );
        assert_eq!(
            serde_json::to_string(&WishlistPriority::Normal).unwrap(),
            "\"NORMAL\""
        );
        assert_eq!(
            serde_json::to_string(&WishlistPriority::High).unwrap(),
            "\"HIGH\""
        );
    }

    #[test]
    fn test_serde_deserialization_tokens() {
        assert_eq!(
            serde_json::from_str::<WishlistPriority>("\"LOW\"").unwrap(),
            WishlistPriority::Low
        );
        assert_eq!(
            serde_json::from_str::<WishlistPriority>("\"NORMAL\"").unwrap(),
            WishlistPriority::Normal
        );
        assert_eq!(
            serde_json::from_str::<WishlistPriority>("\"HIGH\"").unwrap(),
            WishlistPriority::High
        );
    }

    #[test]
    fn test_fromstr_ascii_case_insensitive() {
        assert_eq!(
            WishlistPriority::from_str("LOW").unwrap(),
            WishlistPriority::Low
        );
        assert_eq!(
            WishlistPriority::from_str("low").unwrap(),
            WishlistPriority::Low
        );
        assert_eq!(
            WishlistPriority::from_str("LoW").unwrap(),
            WishlistPriority::Low
        );

        assert_eq!(
            WishlistPriority::from_str("NORMAL").unwrap(),
            WishlistPriority::Normal
        );
        assert_eq!(
            WishlistPriority::from_str("normal").unwrap(),
            WishlistPriority::Normal
        );

        assert_eq!(
            WishlistPriority::from_str("HIGH").unwrap(),
            WishlistPriority::High
        );
        assert_eq!(
            WishlistPriority::from_str("high").unwrap(),
            WishlistPriority::High
        );
    }

    #[test]
    fn test_fromstr_invalid() {
        assert!(WishlistPriority::from_str("NOT_A_PRIORITY").is_err());
        assert!(WishlistPriority::from_str("medium").is_err());
    }
}
