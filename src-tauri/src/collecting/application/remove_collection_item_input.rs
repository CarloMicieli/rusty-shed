use crate::{catalog::domain::railway_model::Category, collecting::domain::CollectionItemId};

/// Input structure for removing an item from the collection.
#[derive(Debug, Clone)]
pub struct RemoveCollectionItemInput {
    /// The ID of the collection item to remove.
    pub collection_item_id: CollectionItemId,
    /// The category of the item.
    pub category: Category,
    /// The date the item was removed from the collection.
    pub removed_date: chrono::NaiveDate,
}
