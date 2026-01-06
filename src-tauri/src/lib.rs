pub mod catalog;
pub mod collecting;
pub mod core;
pub mod dashboard;
pub mod dcc_inventory;
pub mod maintenance;
pub mod sellers;
pub mod settings;
pub mod state;
pub mod wishlist;

#[cfg(test)]
pub mod test_utils;

use crate::catalog::interface::command_handlers as catalog_command_handlers;
use crate::catalog::interface::manufacturers as manufacturers_command_handlers;
use crate::catalog::interface::railway_companies as railway_companies_command_handlers;
use crate::collecting::interface::command_handlers as collecting_command_handlers;
use crate::core::infrastructure::db::Database;
use crate::core::infrastructure::error::CommandError;
use crate::core::interface::command_handlers as core_command_handlers;
use crate::dashboard::dashboard_summary;
use crate::maintenance::interface::command_handlers as maintenance_command_handlers;
use crate::sellers::interface::command_handlers as sellers_command_handlers;
use crate::settings::{ensure_default_settings, get_settings, update_settings};
use crate::state::AppState;
use crate::wishlist::interface::command_handlers as wishlist_command_handlers;
use log::LevelFilter;
use specta_typescript::{BigIntExportBehavior, Typescript};
use std::fs;
use std::path::{Component, Path};
use tauri::Manager;
use tauri::path::BaseDirectory;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};
use tauri_specta::{Builder, collect_commands};

#[tauri::command]
#[specta::specta]
async fn get_image_path(
    state: tauri::State<'_, AppState>,
    id: String,
    category: String,
) -> Result<String, CommandError> {
    match category.as_str() {
        "static" => Ok(id),
        "railway_model" => {
            // Prevent path traversal by rejecting any component that isn't normal
            let id_path = Path::new(&id);
            let valid = id_path
                .components()
                .all(|c| matches!(c, Component::Normal(_)));

            if !valid {
                return Err(CommandError::validation_field(
                    "id",
                    "Invalid image id; must be a file name",
                ));
            }

            let mut full_path = state.models_dir();
            full_path.push(id_path);

            // Ensure the file exists before returning the absolute path
            match tokio::fs::metadata(&full_path).await {
                Ok(meta) if meta.is_file() => Ok(full_path
                    .to_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| CommandError::Unknown("Non-Unicode path".into()))?),
                _ => Err(CommandError::NotFound(format!(
                    "No image found for railway model {id}"
                ))),
            }
        }
        other => Err(CommandError::validation_field(
            "category",
            format!("Unsupported category '{other}'"),
        )),
    }
}

#[tauri::command]
#[specta::specta]
async fn init_database(state: tauri::State<'_, AppState>) -> Result<(), CommandError> {
    log::info!("init_database: starting migrations");
    Database::run_migrations(&state.db_pool())
        .await
        .map_err(|e| {
            log::error!("init_database: migrations failed: {}", e);
            CommandError::Unknown(e.to_string())
        })?;

    log::info!("init_database: starting seeding");
    Database::run_initial_seed(&state.db_pool())
        .await
        .map_err(|e| {
            log::error!("init_database: seeding failed: {}", e);
            CommandError::Unknown(e.to_string())
        })?;

    log::info!("init_database: ensuring default settings");
    ensure_default_settings(&state.db_pool())
        .await
        .map_err(|e| {
            log::error!("init_database: settings failed: {}", e);
            CommandError::Unknown(e.to_string())
        })?;

    log::info!("init_database: initialization complete");
    state.set_initialized();
    Ok(())
}

#[tauri::command]
#[specta::specta]
fn show_main_window(window: tauri::Window) -> Result<(), CommandError> {
    log::info!("show_main_window: calling window.show()");
    window
        .show()
        .map_err(|e| CommandError::Unknown(e.to_string()))?;
    window
        .set_focus()
        .map_err(|e| CommandError::Unknown(e.to_string()))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let is_dev_build = cfg!(debug_assertions);

    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        core_command_handlers::is_db_initialized,
        core_command_handlers::get_app_version,
        init_database,
        show_main_window,
        manufacturers_command_handlers::get_manufacturers,
        manufacturers_command_handlers::get_manufacturer_by_id,
        catalog_command_handlers::get_railway_model_by_id,
        railway_companies_command_handlers::get_railway_companies,
        railway_companies_command_handlers::get_railway_company_by_id,
        catalog_command_handlers::create_railway_model,
        collecting_command_handlers::get_collection,
        collecting_command_handlers::get_depot,
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
        sellers_command_handlers::get_sellers,
        sellers_command_handlers::get_seller_by_id,
        sellers_command_handlers::create_seller,
        sellers_command_handlers::update_seller,
        sellers_command_handlers::delete_seller,
        dashboard_summary,
        get_image_path,
        get_settings,
        update_settings
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
        .plugin(tauri_plugin_fs::init())
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

            // Ensure the models directory exists under AppLocalData
            let models_dir = app.path().resolve("models", BaseDirectory::AppLocalData)?;

            if let Err(e) = fs::create_dir_all(&models_dir) {
                return Err(anyhow::anyhow!(format!(
                    "failed to create models directory {}: {e}",
                    models_dir.display()
                ))
                .into());
            }

            // Initial management of state
            app.manage(AppState::new(pool.clone(), models_dir));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
