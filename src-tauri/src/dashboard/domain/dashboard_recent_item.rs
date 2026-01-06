use crate::catalog::domain::railway_model::RailwayModelId;
use chrono::NaiveDateTime;
use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
pub struct DashboardRecentItem {
    /// Unique identifier for the recent item.
    pub id: RailwayModelId,
    /// Title of the recent item.
    pub title: String,
    /// Optional subtitle of the recent item.
    pub subtitle: Option<String>,
    /// Source of the recent item (e.g., Collection or Wishlist).
    pub source: Source,
    /// Timestamp when the recent item was created.
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, EnumString, Display, sqlx::Type, specta::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    Collection,
    Wishlist,
}
