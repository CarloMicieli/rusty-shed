use serde::Deserialize;
use serde::Serialize;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The control method for this railway model.
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
