use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a digital rolling stock entry.
///
/// URNs are of the form `trn:digital-rolling-stock:{UUID}` where the UUID
/// portion is a valid RFC4122 UUID (v4 typically).
#[repr(transparent)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[sqlx(transparent)]
#[specta(transparent)]
pub struct DigitalRollingStockId(String);

impl_identifier_traits!(DigitalRollingStockId);

impl AsRef<str> for DigitalRollingStockId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for DigitalRollingStockId {
    const PREFIX: &'static str = "trn:digital-rolling-stock";

    fn from_string_unchecked(s: String) -> Self {
        DigitalRollingStockId(s)
    }
}

impl DigitalRollingStockId {
    /// Create a new `DigitalRollingStockId` from a `Uuid`.
    pub fn from_uuid(u: Uuid) -> Self {
        let s = format!("{}:{}", Self::PREFIX, u);
        Self::from_string_unchecked(s)
    }

    /// Try to extract the UUID portion of the identifier.
    pub fn uuid(&self) -> Option<Uuid> {
        let prefix = format!("{}:", Self::PREFIX);
        if self.0.starts_with(&prefix) {
            let tail = &self.0[prefix.len()..];
            Uuid::parse_str(tail).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[test]
    fn digital_rolling_stock_id_from_uuid_and_parse() {
        let u = Uuid::new_v4();
        let id = DigitalRollingStockId::from_uuid(u);
        assert_eq!(
            id.as_ref(),
            format!("{}:{}", DigitalRollingStockId::PREFIX, u)
        );

        // Try parsing via TryFrom<&str>
        let parsed = DigitalRollingStockId::try_from(id.as_ref()).expect("should parse");
        assert_eq!(parsed, id);

        // uuid() extracts the uuid
        assert_eq!(parsed.uuid().unwrap(), u);
    }
}
