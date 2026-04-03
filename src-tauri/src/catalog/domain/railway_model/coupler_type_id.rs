use crate::core::domain::identifiers::Identifier;
use crate::impl_identifier_traits;
use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a coupler type (master record).
///
/// URNs are of the form `trn:coupler:{manufacturer}:{slug}`.
#[repr(transparent)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, specta::Type, sqlx::Type)]
#[serde(transparent)]
#[sqlx(transparent)]
#[specta(transparent)]
pub struct CouplerTypeId(String);

impl_identifier_traits!(CouplerTypeId);

impl AsRef<str> for CouplerTypeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier for CouplerTypeId {
    const PREFIX: &'static str = "trn:coupler";

    fn from_string_unchecked(s: String) -> Self {
        CouplerTypeId(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_build_id_from_parts() {
        let id = CouplerTypeId::new_from_parts(&["Roco", "Roco Universal (40397)"]);
        assert_eq!(id.as_ref(), "trn:coupler:roco:roco-universal-40397");
    }

    #[test]
    fn it_should_roundtrip_display_and_try_from() {
        let id = CouplerTypeId::new_from_parts(&["Kadee", "#148"]);
        let s = id.to_string();
        let parsed = CouplerTypeId::try_from(s.as_str()).expect("parse ok");
        assert_eq!(parsed, id);
    }

    #[test]
    fn it_should_reject_empty_string() {
        assert!(CouplerTypeId::try_from("").is_err());
    }
}
