use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use slug::slugify;

/// Strongly-typed identifier for a seller. Format: `trn:seller:{slug}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
#[specta(transparent)]
pub struct SellerId(pub String);

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
