use serde::{Deserialize, Serialize};

use crate::core::domain::Language;

/// A text value resolved to a specific language code.
///
/// The `lang` field records which language was actually resolved,
/// enabling the UI to show a fallback indicator when `lang` differs
/// from the user's requested language.
#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct LocalizedField {
    /// The language code that was resolved.
    #[cfg_attr(test, dummy(expr = "Language::English"))]
    pub lang: Language,
    /// The text content in the resolved language.
    #[cfg_attr(test, dummy(expr = "\"Test\".to_string()"))]
    pub value: String,
}
