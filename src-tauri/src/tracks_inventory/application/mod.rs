#![allow(dead_code)]
#![allow(unused_imports)]

mod add_purchase;
mod create_track_inventory;
mod create_track_product;
mod delete_track_inventory;
mod get_track_inventories;
mod get_track_inventory;
mod get_track_products;
mod inputs;
mod rename_track_inventory;
mod set_item_quantity;

pub use add_purchase::AddTrackPurchaseUseCase;
pub use create_track_inventory::CreateTrackInventoryUseCase;
pub use create_track_product::CreateTrackProductUseCase;
pub use delete_track_inventory::DeleteTrackInventoryUseCase;
pub use get_track_inventories::GetTrackInventoriesQuery;
pub use get_track_inventory::GetTrackInventoryQuery;
pub use get_track_products::GetTrackProductsQuery;
pub use inputs::AddTrackPurchaseInput;
pub use inputs::CreateTrackProductInput;
pub use inputs::NewTrackInventoryInput;
pub use inputs::RenameTrackInventoryInput;
pub use inputs::SetTrackItemQuantityInput;
pub use rename_track_inventory::RenameTrackInventoryUseCase;
pub use set_item_quantity::SetTrackItemQuantityUseCase;
// View types live in the domain layer and are re-exported here for convenience.
pub use crate::tracks_inventory::domain::{
    TrackInventoryItemView, TrackInventoryListItem, TrackInventoryView, TrackProductView,
    TrackPurchaseView,
};
