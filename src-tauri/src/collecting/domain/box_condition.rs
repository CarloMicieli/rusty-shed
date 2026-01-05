use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The condition of the original packaging box for a collectible item.
///
/// ### Notes
/// In the railway hobby, the box can sometimes represent 30%–50% of the total value, especially for vintage brands like Hornby Dublo or Lionel.
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
pub enum BoxCondition {
    /// Box is crisp, no tears, no "shelf wear."
    #[default]
    OriginalMint,
    /// Some corner scuffing or minor creases.
    OriginalGood,
    /// Significant tears, tape repairs, or faded colors.
    OriginalWorn,
    /// Not the original box, but a suitable storage box.
    ReplacementBox,
    /// "Loose" model with no packaging at all.
    NoBox,
}
