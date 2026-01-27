use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a track purchase record.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct TrackPurchaseId(String);

impl_identifier_traits!(TrackPurchaseId);

impl TrackPurchaseId {
    /// Create a new `TrackPurchaseId` from a UUID.
    pub fn from_uuid(u: &Uuid) -> Self {
        TrackPurchaseId::new_from_parts(&[&u.to_string()])
    }
}

impl AsRef<str> for TrackPurchaseId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for TrackPurchaseId {
    const PREFIX: &'static str = "trn:track-purchase";

    fn from_string_unchecked(s: String) -> Self {
        TrackPurchaseId(s)
    }
}

impl From<Uuid> for TrackPurchaseId {
    fn from(u: Uuid) -> Self {
        TrackPurchaseId::from_uuid(&u)
    }
}

impl Default for TrackPurchaseId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        TrackPurchaseId::from(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[test]
    fn it_should_from_uuid_and_display() {
        let u = Uuid::new_v4();
        let id = TrackPurchaseId::from(u);
        assert_eq!(id.to_string(), format!("trn:track-purchase:{}", u));
    }

    #[test]
    fn it_should_try_from_invalid_trn() {
        let err = TrackPurchaseId::try_from("bad").unwrap_err();
        assert!(format!("{}", err).contains("Invalid prefix"));
    }
}
