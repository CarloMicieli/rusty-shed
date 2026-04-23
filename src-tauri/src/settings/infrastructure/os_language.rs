use crate::core::domain::Language;

/// Detect OS language using tauri-plugin-os
pub fn detect_os_language() -> Language {
    match tauri_plugin_os::locale() {
        Some(locale) => parse_language_code(&locale),
        None => Language::English, // Fallback to English if detection fails
    }
}

/// Parse language code from OS locale string
/// Examples: "en-US" → English, "it-IT" → Italian, "es-ES" → English (fallback)
pub fn parse_language_code(locale: &str) -> Language {
    // Extract language prefix (e.g., "it" from "it-IT")
    let lang_prefix = locale.split('-').next().unwrap_or("").to_lowercase();

    match lang_prefix.as_str() {
        "en" => Language::English,
        "it" => Language::Italian,
        _ => Language::English, // Fallback for unsupported languages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_language_code_english() {
        assert_eq!(parse_language_code("en"), Language::English);
        assert_eq!(parse_language_code("en-US"), Language::English);
        assert_eq!(parse_language_code("en-GB"), Language::English);
    }

    #[test]
    fn test_parse_language_code_italian() {
        assert_eq!(parse_language_code("it"), Language::Italian);
        assert_eq!(parse_language_code("it-IT"), Language::Italian);
    }

    #[test]
    fn test_parse_language_code_fallback() {
        assert_eq!(parse_language_code("es"), Language::English);
        assert_eq!(parse_language_code("es-ES"), Language::English);
        assert_eq!(parse_language_code("fr-FR"), Language::English);
        assert_eq!(parse_language_code("de-DE"), Language::English);
        assert_eq!(parse_language_code(""), Language::English);
    }
}
