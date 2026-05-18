use crate::buyers::application::merge_buyer::MergeBuyer;
use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::interface::command_handlers as sellers_command_handlers;
use crate::sellers::interface::{CreateSellerPayload, Seller, UpdateSellerPayload};
use crate::state::AppState;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MergeBuyerArgs {
    pub source_id: SellerId,
    pub target_id: SellerId,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BuyerMergeResult {
    pub source_id: String,
    pub target_id: String,
    pub relinked_count: i64,
}

pub async fn merge_buyers_inner(
    state: &AppState,
    args: MergeBuyerArgs,
) -> Result<BuyerMergeResult, CommandError> {
    let mut tx = state.db_pool().begin().await.map_err(CommandError::from)?;

    let relinked_count = MergeBuyer::execute(&mut tx, &args.source_id, &args.target_id)
        .await
        .map_err(CommandError::from)?;

    tx.commit().await.map_err(CommandError::from)?;

    Ok(BuyerMergeResult {
        source_id: args.source_id.to_string(),
        target_id: args.target_id.to_string(),
        relinked_count,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn merge_buyers(
    state: tauri::State<'_, AppState>,
    args: MergeBuyerArgs,
) -> Result<BuyerMergeResult, CommandError> {
    merge_buyers_inner(&state, args).await
}
