use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, specta::Type, sqlx::Type,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum TrackType {
    Straight,
    Curve,
    Turnout,
    FlexTrack,
}
