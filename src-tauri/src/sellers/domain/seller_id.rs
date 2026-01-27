use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a seller. Format: `trn:seller:{slug}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct SellerId(pub String);

impl_identifier_traits!(SellerId);

impl AsRef<str> for SellerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for SellerId {
    const PREFIX: &'static str = "trn:seller";

    fn from_string_unchecked(s: String) -> Self {
        SellerId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_new_from_parts_creates_expected_slug() {
        let id = SellerId::new_from_parts(&["My Shop"]);
        let expected = "trn:seller:my-shop";
        assert_eq!(id.as_ref(), expected);
    }

    #[test]
    fn it_should_try_from_str_empty_fails() {
        let res = SellerId::try_from("");
        assert!(res.is_err());
    }

    #[test]
    fn it_should_try_from_str_ok() {
        let s = "trn:seller:foo";
        let id = SellerId::try_from(s).unwrap();
        assert_eq!(id.as_ref(), s);
    }

    #[test]
    fn it_should_try_from_string_ok() {
        let s = "trn:seller:bar".to_string();
        let id = SellerId::try_from(s.clone()).unwrap();
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn it_should_display_outputs_inner() {
        let id = SellerId::new_from_parts(&["baz"]);
        assert_eq!(format!("{}", id), "trn:seller:baz");
    }
}
