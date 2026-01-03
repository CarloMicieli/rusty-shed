use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Constant prefix used for namespaced wishlist identifiers.
pub const WISHLIST_PREFIX: &str = "trn:wishlist:";

/// Strongly-typed identifier for a wishlist.
///
/// Wraps a formatted `String` of the form `trn:wishlist:{uuid}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct WishlistId(pub String);

/// Generate a new `WishlistId` using a random `Uuid`.
///
/// The default value is a namespaced string of the form `trn:wishlist:{uuid}`.
impl Default for WishlistId {
    fn default() -> Self {
        WishlistId(format!("{}{}", WISHLIST_PREFIX, Uuid::new_v4()))
    }
}

/// Parse a `WishlistId` from a `&str`.
///
/// Accepts either a raw UUID string (e.g. `550e8400-e29b-41d4-a716-446655440000`)
/// or a namespaced form `trn:wishlist:{uuid}`. Returns an error when the UUID
/// portion fails to parse.
impl TryFrom<&str> for WishlistId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid_str = if let Some(s) = value.strip_prefix(WISHLIST_PREFIX) {
            s
        } else {
            value
        };
        let parsed = Uuid::parse_str(uuid_str).map_err(|e| anyhow!("invalid uuid: {}", e))?;
        Ok(WishlistId(format!("{}{}", WISHLIST_PREFIX, parsed)))
    }
}

/// Parse a `WishlistId` from an owned `String`.
///
/// Delegates to the `&str` implementation.
impl TryFrom<String> for WishlistId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        WishlistId::try_from(value.as_str())
    }
}

/// Display the namespaced identifier as a string.
///
/// This writes the inner formatted string, e.g. `trn:wishlist:{uuid}`.
impl std::fmt::Display for WishlistId {
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
        let id = WishlistId::try_from(uuid).expect("expected valid id");
        assert_eq!(id.to_string(), format!("{}{}", WISHLIST_PREFIX, uuid));
    }

    #[test]
    fn try_from_str_invalid_fails() {
        let err = WishlistId::try_from("not-a-uuid").expect_err("invalid uuid should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("invalid uuid"));
    }

    #[test]
    fn serde_roundtrip() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistId::try_from(uuid).unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        // compact serde may serialize as string
        let de: WishlistId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn try_from_string_success() {
        let uuid = String::from("550e8400-e29b-41d4-a716-446655440000");
        let id = WishlistId::try_from(uuid).expect("expected valid id from String");
        assert_eq!(
            id.to_string(),
            format!(
                "{}{}",
                WISHLIST_PREFIX, "550e8400-e29b-41d4-a716-446655440000"
            )
        );
    }

    #[test]
    fn default_generates_random_uuid() {
        let id = WishlistId::default();
        // default should not produce the nil UUID
        assert_ne!(id.0, format!("{}{}", WISHLIST_PREFIX, uuid::Uuid::nil()));
    }

    #[test]
    fn display_outputs_uuid() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistId::try_from(uuid).unwrap();
        assert_eq!(format!("{}", id), format!("{}{}", WISHLIST_PREFIX, uuid));
    }
}
