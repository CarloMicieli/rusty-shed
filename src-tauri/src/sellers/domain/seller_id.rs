use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::ops::Deref;

/// Strongly-typed identifier for a seller. Format: `trn:seller:{slug}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct SellerId(pub String);

impl Deref for SellerId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SellerId {
    pub fn new_from_name(name: &str) -> Self {
        let slug = slugify(name);
        SellerId(format!("trn:seller:{slug}"))
    }
}

impl TryFrom<&str> for SellerId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(anyhow!("seller id must not be empty"));
        }
        Ok(SellerId(value.to_owned()))
    }
}

impl TryFrom<String> for SellerId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SellerId::try_from(value.as_str())
    }
}

impl std::fmt::Display for SellerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use slug::slugify;

    #[test]
    fn new_from_name_creates_expected_slug() {
        let id = SellerId::new_from_name("My Shop");
        let expected = format!("trn:seller:{}", slugify("My Shop"));
        assert_eq!(id.0, expected);
    }

    #[test]
    fn try_from_str_empty_fails() {
        let res = SellerId::try_from("");
        assert!(res.is_err());
    }

    #[test]
    fn try_from_str_ok() {
        let s = "trn:seller:foo";
        let id = SellerId::try_from(s).unwrap();
        assert_eq!(id.0, s);
    }

    #[test]
    fn try_from_string_ok() {
        let s = "trn:seller:bar".to_string();
        let id = SellerId::try_from(s.clone()).unwrap();
        assert_eq!(id.to_string(), s);
    }

    #[test]
    fn display_outputs_inner() {
        let id = SellerId("trn:seller:baz".to_string());
        assert_eq!(format!("{}", id), "trn:seller:baz");
    }
}
