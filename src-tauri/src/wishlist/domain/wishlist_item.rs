use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::core::domain::MonetaryAmount;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct WishlistItem {
    pub id: String,
    pub railway_model_id: RailwayModelId,
    pub priority: WishlistPriority,
    pub status: WishlistStatus,
    pub added_date: NaiveDate,
    pub removed_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub desired_price: Option<MonetaryAmount>,
    pub purchased_price: Option<MonetaryAmount>,
}
