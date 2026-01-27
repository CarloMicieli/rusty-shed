use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a wishlist item.
///
/// Wraps a formatted `String` of the form `trn:wishlist-item:{uuid}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct WishlistItemId(pub String);

impl_identifier_traits!(WishlistItemId);

impl WishlistItemId {
    /// Create a new `WishlistItemId` from a given `Uuid`.
    ///
    /// # Parameters
    /// - `id`: the UUID to create the WishlistItemId from
    ///
    /// # Returns
    /// A new `WishlistItemId` instance with a TRN.
    pub fn from_uuid(id: &Uuid) -> Self {
        WishlistItemId::new_from_parts(&[&id.to_string()])
    }
}

impl AsRef<str> for WishlistItemId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for WishlistItemId {
    const PREFIX: &'static str = "trn:wishlist-item";

    fn from_string_unchecked(s: String) -> Self {
        WishlistItemId(s)
    }
}

/// Generate a new `WishlistItemId` using a random `Uuid`.
///
/// The default value is a namespaced string of the form `trn:wishlist-item:{uuid}`.
impl Default for WishlistItemId {
    fn default() -> Self {
        WishlistItemId::from_uuid(&Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_try_from_str_success() {
        let trn = "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistItemId::try_from(trn).expect("expected valid id");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_try_from_str_invalid_fails() {
        let err = WishlistItemId::try_from("").expect_err("invalid uuid should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid prefix") || msg.contains("invalid UUID"));
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let trn = "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistItemId::try_from(trn).unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        let de: WishlistItemId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn it_should_try_from_string_success() {
        let trn = String::from("trn:wishlist-item:550e8400-e29b-41d4-a716-446655440000");
        let id = WishlistItemId::try_from(trn).expect("expected valid id from String");
        assert_eq!(
            id.to_string(),
            "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn it_should_default_generates_random_uuid() {
        let id = WishlistItemId::default();
        assert_ne!(id.0, format!("trn:wishlist-item:{}", uuid::Uuid::nil()));
    }

    #[test]
    fn it_should_display_outputs_uuid() {
        let trn = "trn:wishlist-item:550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistItemId::try_from(trn).unwrap();
        assert_eq!(format!("{}", id), trn);
    }
}
