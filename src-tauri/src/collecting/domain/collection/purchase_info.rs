use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseInfo {
    pub id: String,
    pub item_id: String,
    pub date: Option<String>,
    pub price_amount: i64, // in cents
    pub price_currency: String,
    pub seller: String,
}
