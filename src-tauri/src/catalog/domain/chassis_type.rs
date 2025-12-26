use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The construction type of a rolling stock's chassis.
///
/// This enum indicates the material or manufacturing technique used for the
/// chassis (the structural frame and underbody) of a model. It is part of
/// the technical specifications describing the build quality and expected
/// properties of the model's underframe.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChassisType {
    /// Plastic chassis construction (typically injection-moulded plastic).
    Plastic,

    /// Metal die-cast chassis construction (heavier, metal cast chassis parts).
    MetalDieCast,
}
