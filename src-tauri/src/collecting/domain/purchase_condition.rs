use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The condition under which a collectible item was purchased.
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
pub enum PurchaseCondition {
    /// The item was purchased brand new from a retailer.
    #[default]
    New,
    /// The item was purchased second-hand from another collector or seller.
    PreOwned,
}
