use std::fmt;

use serde::{Deserialize, Serialize};
use specta::Type;

/// Application display language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[serde(rename = "en")]
    #[default]
    English,
    #[serde(rename = "it")]
    Italian,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::Italian => write!(f, "it"),
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "en" => Ok(Language::English),
            "it" => Ok(Language::Italian),
            other => Err(format!("unknown language code: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_default() {
        assert_eq!(Language::default(), Language::English);
    }

    #[test]
    fn test_language_from_str_valid() {
        assert_eq!(Language::try_from("en").unwrap(), Language::English);
        assert_eq!(Language::try_from("it").unwrap(), Language::Italian);
    }

    #[test]
    fn test_language_from_str_invalid() {
        let result = Language::try_from("fr");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown language code"));
    }

    #[test]
    fn test_language_serialization() {
        let lang = Language::English;
        let json = serde_json::to_string(&lang).unwrap();
        assert_eq!(json, "\"en\"");

        let lang = Language::Italian;
        let json = serde_json::to_string(&lang).unwrap();
        assert_eq!(json, "\"it\"");
    }

    #[test]
    fn test_language_deserialization() {
        let lang: Language = serde_json::from_str("\"en\"").unwrap();
        assert_eq!(lang, Language::English);

        let lang: Language = serde_json::from_str("\"it\"").unwrap();
        assert_eq!(lang, Language::Italian);
    }
}
