#![allow(dead_code)]
#![allow(unused_imports)]

mod add_purchase;
mod create_track_inventory;
mod create_track_product;
mod delete_track_product;
mod delete_track_product_translation;
mod delete_track_inventory;
mod get_track_inventories;
mod get_track_inventory;
mod get_track_products;
mod inputs;
mod rename_track_inventory;
mod set_item_quantity;
mod update_track_product;
mod upsert_track_product_translation;

pub use add_purchase::AddTrackPurchaseUseCase;
pub use create_track_inventory::CreateTrackInventoryUseCase;
pub use create_track_product::CreateTrackProductUseCase;
pub use delete_track_product::DeleteTrackProductUseCase;
pub use delete_track_product_translation::DeleteTrackProductTranslationUseCase;
pub use delete_track_inventory::DeleteTrackInventoryUseCase;
pub use get_track_inventories::GetTrackInventoriesQuery;
pub use get_track_inventory::GetTrackInventoryQuery;
pub use get_track_products::GetTrackProductsQuery;
pub use inputs::AddTrackPurchaseInput;
pub use inputs::CreateTrackProductInput;
pub use inputs::DeleteTrackProductInput;
pub use inputs::DeleteTrackProductTranslationInput;
pub use inputs::NewTrackInventoryInput;
pub use inputs::RenameTrackInventoryInput;
pub use inputs::SetTrackItemQuantityInput;
pub use inputs::UpdateTrackProductInput;
pub use inputs::UpsertTrackProductTranslationInput;
pub use rename_track_inventory::RenameTrackInventoryUseCase;
pub use set_item_quantity::SetTrackItemQuantityUseCase;
pub use update_track_product::UpdateTrackProductUseCase;
pub use upsert_track_product_translation::UpsertTrackProductTranslationUseCase;
// View types live in the domain layer and are re-exported here for convenience.
pub use crate::tracks_inventory::domain::{
    TrackInventoryItemView, TrackInventoryListItem, TrackInventoryView, TrackProductView,
    TrackPurchaseView,
};
