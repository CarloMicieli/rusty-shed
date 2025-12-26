use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The construction type of rolling stock's body shell.
///
/// This enum describes the material / manufacturing technique used for the outer
/// body shell of a model (for example, a plastic injection-moulded shell vs a
/// metal die-cast shell). It is used in technical specifications to describe
/// the build of the vehicle's exterior.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BodyShellType {
    /// Plastic body shell construction (typically injection moulded plastic).
    Plastic,

    /// Metal die-cast body shell construction (heavier, metal cast components).
    MetalDieCast,
}
