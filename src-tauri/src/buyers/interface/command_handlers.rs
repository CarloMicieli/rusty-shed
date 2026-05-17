use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::interface::command_handlers as sellers_command_handlers;
use crate::sellers::interface::{CreateSellerPayload, Seller, UpdateSellerPayload};
use crate::state::AppState;

#[tauri::command]
#[specta::specta]
pub async fn get_buyers(
	state: tauri::State<'_, AppState>,
) -> Result<Vec<SellerView>, CommandError> {
	sellers_command_handlers::get_sellers(state).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_buyer_by_id(
	state: tauri::State<'_, AppState>,
	id: SellerId,
) -> Result<Option<SellerView>, CommandError> {
	sellers_command_handlers::get_seller_by_id(state, id).await
}

#[tauri::command]
#[specta::specta]
pub async fn create_buyer(
	state: tauri::State<'_, AppState>,
	payload: CreateSellerPayload,
) -> Result<Seller, CommandError> {
	sellers_command_handlers::create_seller(state, payload).await
}

#[tauri::command]
#[specta::specta]
pub async fn update_buyer(
	state: tauri::State<'_, AppState>,
	payload: UpdateSellerPayload,
) -> Result<Seller, CommandError> {
	sellers_command_handlers::update_seller(state, payload).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_buyer(
	state: tauri::State<'_, AppState>,
	id: SellerId,
) -> Result<(), CommandError> {
	sellers_command_handlers::delete_seller(state, id).await
}
