use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

/// Strongly-typed identifier for a track inventory record.
///
/// `TrackInventoryId` is a transparent newtype wrapping a `String` that stores
/// the canonical TRN for inventory aggregates. The expected form is:
///
/// `trn:track-inventory:{UUID}`
///
/// Construct instances via `From<Uuid>`, `Default` (generates a new UUID) or
/// the fallible `TryFrom<&str>`/`TryFrom<String>` implementations to validate
/// external input. The type serializes as a plain string and is `sqlx::transparent`
/// for convenient persistence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[specta(transparent)]
#[sqlx(transparent)]
pub struct TrackInventoryId(String);

impl_identifier_traits!(TrackInventoryId);

impl AsRef<str> for TrackInventoryId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for TrackInventoryId {
    // Use a normalized prefix without a trailing colon. The shared
    // `Identifier` helper expects `PREFIX` to not include the colon
    // so it can append one during validation/formatting.
    const PREFIX: &'static str = "trn:track-inventory";

    fn from_string_unchecked(s: String) -> Self {
        TrackInventoryId(s)
    }
}

impl TrackInventoryId {
    pub fn new_from_uuid(u: Uuid) -> Self {
        TrackInventoryId(format!("{}:{}", Self::PREFIX, u))
    }
}

impl Deref for TrackInventoryId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for TrackInventoryId {
    fn from(u: Uuid) -> Self {
        TrackInventoryId::new_from_uuid(u)
    }
}

impl Default for TrackInventoryId {
    fn default() -> Self {
        let u = Uuid::new_v4();
        TrackInventoryId::from(u)
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
        let id = TrackInventoryId::from(u);
        assert_eq!(
            id.to_string(),
            format!("{}:{}", TrackInventoryId::PREFIX, u)
        );
    }
}
