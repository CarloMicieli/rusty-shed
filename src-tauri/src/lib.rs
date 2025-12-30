pub mod catalog;
pub mod collecting;
pub mod collection;
pub mod core;
pub mod maintenance;
pub mod state;
pub mod wishlist;

#[cfg(test)]
pub mod test_utils;

use crate::catalog::interface::command_handlers as catalog_command_handlers;
use crate::collecting::interface::command_handlers as collecting_command_handlers;
use crate::core::infrastructure::db::Database;
use crate::maintenance::interface::command_handlers as maintenance_command_handlers;
use crate::state::AppState;
use crate::wishlist::interface::command_handlers as wishlist_command_handlers;
use log::{LevelFilter, error};
use specta_typescript::{BigIntExportBehavior, Typescript};
use tauri::Manager;
use tauri::path::BaseDirectory;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};
use tauri_specta::{Builder, collect_commands};

#[tauri::command]
#[specta::specta]
fn is_db_initialized(state: tauri::State<'_, AppState>) -> bool {
    state.is_initialized()
}

#[tauri::command]
#[specta::specta]
fn get_app_version() -> String {
    // Use the crate package version set at compile time
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let is_dev_build = cfg!(debug_assertions);

    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        is_db_initialized,
        catalog_command_handlers::get_manufacturer_by_id,
        catalog_command_handlers::get_railway_model_by_id,
        catalog_command_handlers::get_railway_models_by_ids,
        catalog_command_handlers::get_railway_company_by_id,
        catalog_command_handlers::create_railway_model,
        collecting_command_handlers::get_collection,
        collecting_command_handlers::get_depot,
        collection::list_collection_items,
        collection::create_collection_item,
        collection::update_collection_item,
        collection::delete_collection_item,
        wishlist_command_handlers::get_wishlists,
        wishlist_command_handlers::get_wishlist_by_id,
        wishlist_command_handlers::create_wishlist,
        wishlist_command_handlers::rename_wishlist,
        wishlist_command_handlers::delete_wishlist,
        wishlist_command_handlers::set_default_wishlist,
        wishlist_command_handlers::add_to_wishlist,
        wishlist_command_handlers::remove_from_wishlist,
        wishlist_command_handlers::move_item_to_list,
        maintenance_command_handlers::get_maintenance_dashboard,
        maintenance_command_handlers::add_maintenance_record,
        get_app_version
    ]);

    let ts_config = Typescript::default().bigint(BigIntExportBehavior::BigInt);

    // 2. Export the bindings (This creates the TS file)
    #[cfg(debug_assertions)] // Only export during development
    builder
        .export(ts_config, "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");

    let level = if is_dev_build {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(level)
                .max_file_size(50000)
                .rotation_strategy(RotationStrategy::KeepOne)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                ])
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            // Compute DB path using tauri path helpers and init the pool
            let pool = tauri::async_runtime::block_on(async {
                let handle = app.handle();
                let db_path = handle
                    .path()
                    .resolve("database.sqlite", BaseDirectory::AppData)?;

                Database::new_sqlite_pool(&db_path)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            })?;

            // Initial management of state
            app.manage(AppState::new(pool.clone()));

            // Show the main window IMMEDIATELY to avoid blank screen
            // The UI can handle the "not initialized" state gracefully
            if let Some(window) = app.get_webview_window("main")
                && let Err(e) = window.show()
            {
                error!("Failed to show main window: {e}");
            }

            let handle = app.handle().clone();

            // Run migrations in an async task (non-blocking)
            tauri::async_runtime::spawn(async move {
                let state_ref = handle.state::<AppState>();
                let _ = Database::run_migrations(&state_ref.db_pool())
                    .await
                    .map_err(|e| anyhow::anyhow!(e));

                let _ = Database::run_initial_seed(&state_ref.db_pool())
                    .await
                    .map_err(|e| anyhow::anyhow!(e));

                state_ref.set_initialized();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
