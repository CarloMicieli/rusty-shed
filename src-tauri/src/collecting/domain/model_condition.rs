use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// This represents the physical and mechanical state of the locomotive or rolling stock.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumString,
    Display,
    Default,
    Serialize,
    Deserialize,
    sqlx::Type,
    specta::Type,
)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelCondition {
    /// Brand new, no signs of use, factory fresh.
    #[default]
    Mint,
    /// Almost like new, with only very minor signs of handling.
    NearMint,
    /// Clean, very light use, no missing detail parts.
    Excellent,
    /// Minor wear from use; paint is still strong.
    VeryGood,
    /// Visible wear, small scratches, but functional.
    Good,
    /// Significant wear, missing small parts (couplers, buffers).
    Fair,
    /// Heavy damage, non-functional, or heavily modified.
    Poor,
    /// Not a runner; only useful for salvaging components.
    ForParts,
}
