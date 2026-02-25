use serde::{Deserialize, Serialize};

/// A text value resolved to a specific language code.
///
/// The `lang` field records which language was actually resolved,
/// enabling the UI to show a fallback indicator when `lang` differs
/// from the user's requested language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct LocalizedField {
    /// The language code that was resolved (e.g. "en" or "it").
    pub lang: String,
    /// The text content in the resolved language.
    pub value: String,
}
