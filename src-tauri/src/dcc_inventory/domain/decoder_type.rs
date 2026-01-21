use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Represents the type of a DCC decoder.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Display,
    Default,
    EnumString,
    specta::Type,
    sqlx::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecoderType {
    /// Plain decoder without sound or function capabilities.
    #[default]
    Plain,
    /// Decoder with sound capabilities.
    Sound,
    /// Decoder with function capabilities.
    Function,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(DecoderType::Plain, "PLAIN")]
    #[case(DecoderType::Sound, "SOUND")]
    #[case(DecoderType::Function, "FUNCTION")]
    fn display_and_parse_case(#[case] variant: DecoderType, #[case] expected: &str) {
        // Display -> expected SCREAMING_SNAKE_CASE
        let s = variant.to_string();
        assert_eq!(s, expected);

        // FromStr should parse case-insensitively
        let parsed = expected.parse::<DecoderType>().expect("parse ok");
        assert_eq!(parsed, variant);

        let lower = expected.to_lowercase();
        let parsed_lower = lower.parse::<DecoderType>().expect("parse ok lower");
        assert_eq!(parsed_lower, variant);
    }
}
