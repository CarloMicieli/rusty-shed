pub mod budget;
pub mod catalog;
pub mod cloud_backup;
pub mod collecting;
pub mod commands;
pub mod core;
pub mod dashboard;
pub mod database_backup;
pub mod dcc_inventory;
pub mod export;
pub mod import;
pub mod maintenance;
pub mod media;
pub mod search;
pub mod sellers;
pub mod settings;
pub mod state;
pub mod tracks_inventory;
pub mod viewport;
pub mod wishlist;

#[cfg(test)]
pub mod test_utils;

use crate::budget::interface::command_handlers as budget_command_handlers;
use crate::catalog::interface::command_handlers as catalog_command_handlers;
use crate::catalog::interface::manufacturers as manufacturers_command_handlers;
use crate::catalog::interface::railway_companies as railway_companies_command_handlers;
use crate::cloud_backup::infrastructure::start_connectivity_monitor;
use crate::collecting::interface::command_handlers as collecting_command_handlers;
use crate::commands::cloud_backup as cloud_backup_command_handlers;
use crate::commands::database_backup as database_backup_command_handlers;
use crate::core::infrastructure::db::Database;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::logging;
use crate::core::interface::command_handlers as core_command_handlers;
use crate::dashboard::interface::command_handlers as dashboard_command_handlers;
use crate::dcc_inventory::interface::command_handlers as dcc_inventory_command_handlers;
use crate::import::interface::command_handlers as import_command_handlers;
use crate::maintenance::interface::command_handlers as maintenance_command_handlers;
use crate::media::interface::command_handlers as media_command_handlers;
use crate::search::interface::command_handlers as search_command_handlers;
use crate::sellers::interface::command_handlers as sellers_command_handlers;
use crate::settings::ensure_default_settings;
use crate::settings::interface::commands::{get_settings, initialize_settings, update_settings};
use crate::state::AppState;
use crate::tracks_inventory::interface::command_handlers as tracks_inventory_command_handlers;
use crate::tracks_inventory::interface::query_handlers as tracks_inventory_query_handlers;
use crate::wishlist::interface::command_handlers as wishlist_command_handlers;
use specta_typescript::{BigIntExportBehavior, Typescript};
use std::fs;
use std::path::{Component, Path};
use tauri::Manager;
use tauri::path::BaseDirectory;
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
            CommandError::DatabaseError(e.to_string())
        })?;

    log::info!("init_database: starting seeding");
    Database::run_initial_seed(&state.db_pool())
        .await
        .map_err(|e| {
            log::error!("init_database: seeding failed: {}", e);
            CommandError::DatabaseError(e.to_string())
        })?;

    log::info!("init_database: ensuring default settings");
    ensure_default_settings(&state.db_pool())
        .await
        .map_err(|e| {
            log::error!("init_database: settings failed: {}", e);
            CommandError::DatabaseError(e.to_string())
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
        catalog_command_handlers::update_railway_model_text,
        catalog_command_handlers::update_rolling_stock_identification,
        catalog_command_handlers::update_railway_model_classification,
        catalog_command_handlers::update_rolling_stock_railway_company,
        catalog_command_handlers::update_rolling_stock_specifications,
        catalog_command_handlers::get_railway_model_translations,
        catalog_command_handlers::upsert_railway_model_translation,
        catalog_command_handlers::search_railway_models,
        collecting_command_handlers::add_railway_model_to_collection,
        collecting_command_handlers::update_collection_item,
        collecting_command_handlers::remove_collection_item,
        wishlist_command_handlers::add_railway_model_to_wish_list,
        collecting_command_handlers::get_collection,
        collecting_command_handlers::get_depot,
        dashboard_command_handlers::get_dashboard_summary,
        wishlist_command_handlers::get_wishlists,
        wishlist_command_handlers::get_wishlist_by_id,
        wishlist_command_handlers::create_wishlist,
        wishlist_command_handlers::rename_wishlist,
        wishlist_command_handlers::delete_wishlist,
        wishlist_command_handlers::set_default_wishlist,
        wishlist_command_handlers::add_to_wishlist,
        wishlist_command_handlers::remove_from_wishlist,
        wishlist_command_handlers::move_item_to_list,
        wishlist_command_handlers::purchase_wishlist_item,
        maintenance_command_handlers::get_maintenance_dashboard,
        maintenance_command_handlers::add_maintenance_event,
        maintenance_command_handlers::add_maintenance_card,
        sellers_command_handlers::get_sellers,
        sellers_command_handlers::get_seller_by_id,
        sellers_command_handlers::create_seller,
        sellers_command_handlers::update_seller,
        sellers_command_handlers::delete_seller,
        tracks_inventory_command_handlers::create_track_inventory,
        tracks_inventory_command_handlers::rename_track_inventory,
        tracks_inventory_command_handlers::add_track_purchase,
        tracks_inventory_command_handlers::set_track_item_quantity,
        tracks_inventory_command_handlers::set_item_required,
        tracks_inventory_command_handlers::delete_track_inventory,
        tracks_inventory_command_handlers::create_track_product,
        tracks_inventory_query_handlers::get_track_inventories,
        tracks_inventory_query_handlers::get_track_inventory,
        tracks_inventory_query_handlers::get_track_products,
        dcc_inventory_command_handlers::new_digital_rolling_stock,
        dcc_inventory_command_handlers::change_dcc_address,
        dcc_inventory_command_handlers::change_decoder,
        dcc_inventory_command_handlers::get_digital_rolling_stocks,
        dcc_inventory_command_handlers::get_digital_summary,
        dcc_inventory_command_handlers::get_decoders,
        dcc_inventory_command_handlers::check_dcc_address_duplicate,
        dcc_inventory_command_handlers::get_installable_rolling_stocks,
        import_command_handlers::analyze_import_package,
        import_command_handlers::get_import_preview,
        import_command_handlers::execute_import,
        import_command_handlers::cancel_import_session,
        budget_command_handlers::get_budget_config,
        budget_command_handlers::set_budget_config,
        budget_command_handlers::get_monthly_budget_records,
        budget_command_handlers::get_budget_dashboard,
        budget_command_handlers::add_extra_budget,
        budget_command_handlers::remove_extra_budget,
        budget_command_handlers::get_extra_budgets,
        budget_command_handlers::get_quarterly_summaries,
        cloud_backup_command_handlers::cloud_backup_get_connection_status,
        cloud_backup_command_handlers::cloud_backup_connect_google,
        cloud_backup_command_handlers::cloud_backup_disconnect_google,
        cloud_backup_command_handlers::cloud_backup_check_connectivity,
        cloud_backup_command_handlers::cloud_backup_sync_now,
        cloud_backup_command_handlers::cloud_backup_list_backups,
        cloud_backup_command_handlers::cloud_backup_restore,
        cloud_backup_command_handlers::cloud_backup_get_sync_status,
        media_command_handlers::get_railway_model_image,
        media_command_handlers::upload_model_image,
        media_command_handlers::upload_model_image_bytes,
        media_command_handlers::delete_model_image,
        get_image_path,
        initialize_settings,
        get_settings,
        update_settings,
        database_backup_command_handlers::export_database,
        database_backup_command_handlers::import_database,
        search_command_handlers::global_search
    ]);

    #[allow(unused_variables)]
    let ts_config = Typescript::default().bigint(BigIntExportBehavior::BigInt);

    // 2. Export the bindings (This creates the TS file)
    #[cfg(debug_assertions)] // Only export during development
    builder
        .export(ts_config, "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            logging::init_logger(app)?;

            let version = env!("CARGO_PKG_VERSION");

            log::info!("{}", LOGO);
            println!("  Crate v{}", version);

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

            log::info!("Models directory: {}", models_dir.display());

            // Initial management of state
            app.manage(AppState::new(pool.clone(), models_dir));

            start_connectivity_monitor(app.handle().clone());

            // Setup viewport for main window
            if let Some(window) = app.get_webview_window("main")
                && let Err(e) = crate::viewport::setup_viewport(&window)
            {
                log::warn!("Failed to setup viewport: {}", e);
            }

            Ok(())
        });

    // Only enable the bridge in debug mode for safety
    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

const LOGO: &str = r#"
      _____             _              _____ _             _ 
     |  __ \           | |            / ____| |           | |
     | |__) |   _  ___ | |_ _   _    | (___ | |__   ___ __| |
     |  _  / | | |/ __|| __| | | |    \___ \| '_ \ / _ \ _` |
     | | \ \ |_| |\__ \| |_| |_| |    ____) | | | |  __/(_| |
     |_|  \_\__,_||___/ \__|\__, |   |_____/|_| |_|\___|__,_|
      ____  ____  ____  ____ __/ |_____________________________
     \____\\____\\____\\____\___/ \____\\____\\____\\____\\___/
"#;
