use serde::Deserialize;
use serde::Serialize;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The control method for this railway model.
///
/// The `Control` enum captures whether a model is DCC-ready, has a decoder
/// fitted, has a sound-equipped decoder, or has no DCC support at all.
///
/// Variants:
/// - `DccReady`: The model is prepared for a DCC decoder (e.g. a standard
///   decoder plug is present) but no decoder is installed.
/// - `DccFitted`: A DCC decoder has been installed.
/// - `DccSound`: A DCC decoder with a sound module is installed.
/// - `NoDcc`: The model does not support DCC (no standard interface present);
///   installation may require model-specific wiring or a hardwired decoder.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Control {
    /// The model can be fitted with a dcc decoder.
    DccReady,

    /// The model has a dcc decoder installed.
    DccFitted,

    /// The model has a dcc decoder installed with the sound module.
    DccSound,

    /// The model has no dcc support (like no standard decoder plug)
    NoDcc,
}

impl Control {
    /// Returns true if this `Control` value represents a fitted decoder.
    ///
    /// Specifically, this method returns `true` for `Control::DccFitted` and
    /// `Control::DccSound`, and `false` for other variants such as
    /// `Control::DccReady` and `Control::NoDcc`.
    pub fn has_decoder(&self) -> bool {
        *self == Control::DccFitted || *self == Control::DccSound
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::ParseError;

    #[rstest]
    #[case(Control::DccFitted, true)]
    #[case(Control::DccSound, true)]
    #[case(Control::DccReady, false)]
    #[case(Control::NoDcc, false)]
    fn has_decoder_cases(#[case] input: Control, #[case] expected: bool) {
        assert_eq!(expected, input.has_decoder());
    }

    #[rstest]
    #[case("DCC_READY", Ok(Control::DccReady))]
    #[case("DCC_FITTED", Ok(Control::DccFitted))]
    #[case("DCC_SOUND", Ok(Control::DccSound))]
    #[case("NO_DCC", Ok(Control::NoDcc))]
    // verify ascii case-insensitive parsing
    #[case("dcc_sound", Ok(Control::DccSound))]
    fn parse_control(#[case] input: &str, #[case] expected: Result<Control, ParseError>) {
        let result = input.parse::<Control>();
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(Control::DccReady, "DCC_READY")]
    #[case(Control::DccFitted, "DCC_FITTED")]
    #[case(Control::DccSound, "DCC_SOUND")]
    #[case(Control::NoDcc, "NO_DCC")]
    fn display_control(#[case] input: Control, #[case] expected: &str) {
        assert_eq!(expected, input.to_string());
    }
}
