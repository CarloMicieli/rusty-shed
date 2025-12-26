//! Control metadata for a railway model.
//!
//! This module defines the `Control` enum which describes the presence and
//! state of DCC (Digital Command Control) fittings on a model locomotive.
//!
//! Serialization and parsing
//! - The enum uses `serde` with `rename_all = "SCREAMING_SNAKE_CASE"`, so
//!   JSON serialization will produce values like `"DCC_READY"` or
//!   `"NO_DCC"`.
//! - `strum_macros::EnumString` is also derived and configured to use
//!   screaming snake case and ASCII case-insensitive parsing. This makes
//!   `Control::try_from("dcc_ready")` and `Control::try_from("DCC_READY")`
//!   both succeed.

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
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
    use rstest::rstest;

    #[rstest]
    #[case(Control::DccFitted, true)]
    #[case(Control::DccSound, true)]
    #[case(Control::DccReady, false)]
    #[case(Control::NoDcc, false)]
    fn has_decoder_cases(#[case] input: Control, #[case] expected: bool) {
        assert_eq!(expected, input.has_decoder());
    }
}
