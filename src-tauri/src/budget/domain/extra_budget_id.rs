use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for an extra budget entry.
///
/// Wraps a formatted `String` of the form `trn:extra-budget:{uuid}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct ExtraBudgetId(String);

impl_identifier_traits!(ExtraBudgetId);

impl ExtraBudgetId {
    /// Create a new `ExtraBudgetId` from a `Uuid`.
    pub fn from_uuid(uuid: &Uuid) -> Self {
        ExtraBudgetId::new_from_parts(&[&uuid.to_string()])
    }
}

impl AsRef<str> for ExtraBudgetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for ExtraBudgetId {
    const PREFIX: &'static str = "trn:extra-budget";

    fn from_string_unchecked(s: String) -> Self {
        ExtraBudgetId(s)
    }
}

/// Garde validator: rejects a `&str` that cannot be parsed as an `ExtraBudgetId`.
pub fn validate_extra_budget_id(value: &str, _: &()) -> garde::Result {
    ExtraBudgetId::try_from(value)
        .map(|_| ())
        .map_err(|_| garde::Error::new("error_invalid_extra_budget_id"))
}

/// Generate a new `ExtraBudgetId` using a random `Uuid`.
impl Default for ExtraBudgetId {
    fn default() -> Self {
        ExtraBudgetId::from_uuid(&Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_from_uuid() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let id = ExtraBudgetId::from_uuid(&uuid);
        assert_eq!(
            id.as_ref(),
            "trn:extra-budget:550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn it_should_create_default() {
        let id = ExtraBudgetId::default();
        assert!(id.as_ref().starts_with("trn:extra-budget:"));
    }

    #[test]
    fn it_should_validate_extra_budget_id() {
        let id = ExtraBudgetId::default();
        let result = validate_extra_budget_id(id.as_ref(), &());
        assert!(result.is_ok());
    }

    #[test]
    fn it_should_reject_invalid_extra_budget_id() {
        let result = validate_extra_budget_id("not-a-valid-id", &());
        assert!(result.is_err());
    }
}
