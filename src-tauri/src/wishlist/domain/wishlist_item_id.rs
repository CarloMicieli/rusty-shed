use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a wishlist item.
///
/// Wraps a formatted `String` of the form `trn:wishlist-item:{uuid}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct WishlistItemId(pub String);

impl Default for WishlistItemId {
    fn default() -> Self {
        WishlistItemId(format!("trn:wishlist-item:{}", Uuid::new_v4()))
    }
}

impl TryFrom<&str> for WishlistItemId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid_str = if let Some(s) = value.strip_prefix("trn:wishlist-item:") {
            s
        } else {
            value
        };
        let parsed = Uuid::parse_str(uuid_str).map_err(|e| anyhow!("invalid uuid: {}", e))?;
        Ok(WishlistItemId(format!("trn:wishlist-item:{}", parsed)))
    }
}

impl TryFrom<String> for WishlistItemId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        WishlistItemId::try_from(value.as_str())
    }
}

impl std::fmt::Display for WishlistItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn try_from_str_success() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistItemId::try_from(uuid).expect("expected valid id");
        assert_eq!(id.to_string(), format!("trn:wishlist-item:{}", uuid));
    }

    #[test]
    fn try_from_str_invalid_fails() {
        let err = WishlistItemId::try_from("").expect_err("invalid uuid should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("invalid uuid") || msg.contains("parse"));
    }

    #[test]
    fn serde_roundtrip() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistItemId::try_from(uuid).unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        let de: WishlistItemId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn try_from_string_success() {
        let uuid = String::from("550e8400-e29b-41d4-a716-446655440000");
        let id = WishlistItemId::try_from(uuid).expect("expected valid id from String");
        assert_eq!(
            id.to_string(),
            "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn default_generates_random_uuid() {
        let id = WishlistItemId::default();
        assert_ne!(id.0, format!("trn:wishlist-item:{}", uuid::Uuid::nil()));
    }

    #[test]
    fn display_outputs_uuid() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistItemId::try_from(uuid).unwrap();
        assert_eq!(format!("{}", id), format!("trn:wishlist-item:{}", uuid));
    }
}
