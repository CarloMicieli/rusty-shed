use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    specta::Type,
    sqlx::Type,
    EnumString,
    Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SellerType {
    Shop,
    Private,
    Marketplace,
    Distributor,
}
