use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a wishlist.
///
/// Wraps a formatted `String` of the form `trn:wishlist:{uuid}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct WishlistId(pub String);

impl_identifier_traits!(WishlistId);

impl WishlistId {
    /// Create a new `WishlistId` from a `Uuid`.
    ///
    /// # Parameters
    /// - `uuid`: the UUID to use in the wishlist ID
    ///
    /// # Returns
    /// A new `WishlistId` instance.
    pub fn from_uuid(uuid: &Uuid) -> Self {
        WishlistId::new_from_parts(&[&uuid.to_string()])
    }
}

impl AsRef<str> for WishlistId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for WishlistId {
    const PREFIX: &'static str = "trn:wishlist";

    fn from_string_unchecked(s: String) -> Self {
        WishlistId(s)
    }
}

/// Garde validator: rejects a `&str` that cannot be parsed as a `WishlistId`.
pub fn validate_wishlist_id(value: &str, _: &()) -> garde::Result {
    WishlistId::try_from(value)
        .map(|_| ())
        .map_err(|_| garde::Error::new("error_invalid_wishlist_id"))
}

/// Generate a new `WishlistId` using a random `Uuid`.
///
/// The default value is a namespaced string of the form `trn:wishlist:{uuid}`.
impl Default for WishlistId {
    fn default() -> Self {
        WishlistId::from_uuid(&Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_try_from_str_success() {
        let trn = "trn:wishlist:550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistId::try_from(trn).expect("expected valid id");
        assert_eq!(id.to_string(), trn);
    }

    #[test]
    fn it_should_try_from_str_invalid_fails() {
        let err = WishlistId::try_from("not-a-uuid").expect_err("invalid uuid should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid prefix") || msg.contains("invalid UUID"));
    }

    #[test]
    fn it_should_serde_roundtrip() {
        let trn = "trn:wishlist:550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistId::try_from(trn).unwrap();
        let s = serde_json::to_string(&id).expect("serialize");
        let de: WishlistId = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de, id);
    }

    #[test]
    fn it_should_try_from_string_success() {
        let trn = String::from("trn:wishlist:550e8400-e29b-41d4-a716-446655440000");
        let id = WishlistId::try_from(trn).expect("expected valid id from String");
        assert_eq!(
            id.to_string(),
            "trn:wishlist:550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn it_should_default_generates_random_uuid() {
        let id = WishlistId::default();
        // default should not produce the nil UUID
        assert_ne!(id.0, format!("trn:wishlist:{}", uuid::Uuid::nil()));
    }

    #[test]
    fn it_should_display_outputs_uuid() {
        let trn = "trn:wishlist:550e8400-e29b-41d4-a716-446655440000";
        let id = WishlistId::try_from(trn).unwrap();
        assert_eq!(format!("{}", id), trn);
    }
}
