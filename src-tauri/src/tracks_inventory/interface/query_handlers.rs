use crate::core::domain::Language;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    GetTrackInventoriesQuery, GetTrackInventoryQuery, GetTrackProductsQuery,
    TrackInventoryListItem, TrackInventoryView, TrackProductView,
};
use crate::tracks_inventory::domain::TrackInventoryId;
use tracing::info;

pub async fn get_track_inventories_inner(
    state: &AppState,
) -> Result<Vec<TrackInventoryListItem>, CommandError> {
    info!("Fetching all track inventories");

    let pool = state.db_pool();
    let mut uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let inventories = GetTrackInventoriesQuery::execute(&mut uow).await?;

    Ok(inventories)
}

/// Query handler to fetch all track inventories with summary information.
///
/// # Arguments
/// - `state`: The application state.
///
/// # Returns
/// A list of track inventory summaries.
#[tauri::command]
#[specta::specta]
pub async fn get_track_inventories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<TrackInventoryListItem>, CommandError> {
    get_track_inventories_inner(&state).await
}

pub async fn get_track_inventory_inner(
    state: &AppState,
    id: TrackInventoryId,
) -> Result<TrackInventoryView, CommandError> {
    info!("Fetching track inventory: {:?}", id);

    let pool = state.db_pool();
    let mut uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let inventory = GetTrackInventoryQuery::execute(&mut uow, &id).await?;

    Ok(inventory)
}

/// Query handler to fetch a single track inventory with full details.
///
/// # Arguments
/// - `state`: The application state.
/// - `id`: The ID of the inventory to fetch.
///
/// # Returns
/// The complete inventory view with items and purchases.
#[tauri::command]
#[specta::specta]
pub async fn get_track_inventory(
    state: tauri::State<'_, AppState>,
    id: TrackInventoryId,
) -> Result<TrackInventoryView, CommandError> {
    get_track_inventory_inner(&state, id).await
}

pub async fn get_track_products_inner(
    state: &AppState,
    lang: Language,
) -> Result<Vec<TrackProductView>, CommandError> {
    info!("Fetching all track products");

    let pool = state.db_pool();
    let mut uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let products = GetTrackProductsQuery::execute(&mut uow, lang).await?;

    Ok(products)
}

/// Query handler to fetch all track products.
///
/// # Arguments
/// - `state`: The application state.
///
/// # Returns
/// A list of all track products available.
#[tauri::command]
#[specta::specta]
pub async fn get_track_products(
    state: tauri::State<'_, AppState>,
    lang: Language,
) -> Result<Vec<TrackProductView>, CommandError> {
    get_track_products_inner(&state, lang).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_track_inventories_inner_returns_empty_list_when_no_rows(pool: SqlitePool) {
        let state = app_state(pool);

        let result = get_track_inventories_inner(&state)
            .await
            .expect("query should succeed");

        assert!(result.is_empty());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn get_track_inventories_inner_returns_summary_rows(pool: SqlitePool) {
        let state = app_state(pool);

        let result = get_track_inventories_inner(&state)
            .await
            .expect("query should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Test Inventory");
        assert_eq!(result[0].total_items, 1);
        assert_eq!(result[0].total_quantity, 1);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn get_track_inventory_inner_returns_not_found_for_missing_inventory(pool: SqlitePool) {
        let state = app_state(pool);
        let missing_id = TrackInventoryId::try_from(
            "trn:track-inventory:00000000-0000-0000-0000-999999999999",
        )
        .expect("valid inventory id");

        let result = get_track_inventory_inner(&state, missing_id).await;

        assert!(matches!(result, Err(CommandError::NotFound(_))), "{result:?}");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn get_track_products_inner_returns_localized_products(pool: SqlitePool) {
        let state = app_state(pool);

        let result = get_track_products_inner(&state, Language::English)
            .await
            .expect("query should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].product_code, "60100");
        assert_eq!(result[0].description, "Straight track");
        assert_eq!(result[0].manufacturer_name, "ACME");
    }
}
