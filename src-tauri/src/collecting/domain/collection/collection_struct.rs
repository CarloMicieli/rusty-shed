use crate::collecting::domain::collection::{CollectionItem, CollectionSummary};
use crate::core::domain::MonetaryAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,

    // Summary counts
    pub summary: CollectionSummary,

    // Total Value
    pub total_value: Option<MonetaryAmount>,

    pub items: Vec<CollectionItem>,
}
