pub mod app_uow;
pub mod budget;
pub mod catalog;
pub mod cloud_backup;
pub mod collecting;
pub mod core;
pub mod dashboard;
pub mod data_management;

pub mod dcc_inventory;
pub mod maintenance;
pub mod media;
pub mod search;
pub mod sellers;
pub mod settings;
pub mod state;
pub mod tracks_inventory;
pub mod trains;
pub mod viewport;
pub mod wishlist;

#[cfg(test)]
pub mod test_utils;

use crate::budget::interface::command_handlers as budget_command_handlers;
use crate::catalog::interface::command_handlers as catalog_command_handlers;
use crate::catalog::interface::manufacturers as manufacturers_command_handlers;
use crate::catalog::interface::railway_companies as railway_companies_command_handlers;
use crate::cloud_backup::commands as cloud_backup_command_handlers;
use crate::cloud_backup::infrastructure::start_connectivity_monitor;
use crate::collecting::interface::command_handlers as collecting_command_handlers;
use crate::core::infrastructure::db::Database;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::logging::init_tracing;
use crate::core::interface::command_handlers as core_command_handlers;
use crate::dashboard::interface::command_handlers as dashboard_command_handlers;
use crate::data_management::interface::backup_handlers as database_backup_command_handlers;
use crate::data_management::interface::export_handlers as export_command_handlers;
use crate::data_management::interface::import_handlers as import_command_handlers;
use crate::dcc_inventory::interface::command_handlers as dcc_inventory_command_handlers;
use crate::maintenance::interface::command_handlers as maintenance_command_handlers;
use crate::media::interface::command_handlers as media_command_handlers;
use crate::search::interface::command_handlers as search_command_handlers;
use crate::sellers::interface::command_handlers as sellers_command_handlers;
use crate::settings::ensure_default_settings;
use crate::settings::interface::commands::{
    get_locale, get_settings, initialize_settings, update_settings,
};
use crate::state::AppState;
use crate::tracks_inventory::interface::command_handlers as tracks_inventory_command_handlers;
use crate::tracks_inventory::interface::query_handlers as tracks_inventory_query_handlers;
use crate::trains::interface::command_handlers as trains_command_handlers;
use crate::wishlist::interface::command_handlers as wishlist_command_handlers;
use specta_typescript::Typescript;
use std::fs;
use std::path::{Component, Path};
use tauri::path::BaseDirectory;
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};
use tauri_specta::{Builder, collect_commands};
use tracing::Instrument;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

/// Inner implementation for [`get_image_path`].
pub async fn get_image_path_inner(
    state: &AppState,
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

            let mut full_path = state.models_dir().to_path_buf();
            full_path.push(id_path);

            // Ensure the file exists before returning the absolute path
            match tokio::fs::metadata(&full_path).await {
                Ok(meta) if meta.is_file() => Ok(full_path
                    .to_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| CommandError::unknown("Non-Unicode path"))?),
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
async fn get_image_path(
    state: tauri::State<'_, AppState>,
    id: String,
    category: String,
) -> Result<String, CommandError> {
    get_image_path_inner(&state, id, category).await
}

/// Inner implementation for [`init_database`].
pub async fn init_database_inner(state: &AppState) -> Result<bool, CommandError> {
    let span = tracing::info_span!("init_database");
    let _enter = span.enter();

    Database::run_migrations(&state.db_pool())
        .instrument(tracing::info_span!("migrations"))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "database migrations failed");
            CommandError::DatabaseError(e.to_string())
        })?;

    Database::run_initial_seed(&state.db_pool())
        .instrument(tracing::info_span!("initial_seed"))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "database seeding failed");
            CommandError::DatabaseError(e.to_string())
        })?;

    ensure_default_settings(&state.db_pool())
        .instrument(tracing::info_span!("ensure_default_settings"))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "ensuring default settings failed");
            CommandError::DatabaseError(e.to_string())
        })?;

    tracing::info!("database initialized");
    state.set_initialized();
    Ok(true)
}

#[tauri::command]
#[specta::specta]
async fn init_database(state: tauri::State<'_, AppState>) -> Result<(), CommandError> {
    let span = tracing::info_span!("init_database_command");
    init_database_inner(&state)
        .instrument(span)
        .await
        .map(|_| ())
}

#[tauri::command]
#[specta::specta]
fn show_main_window(window: tauri::Window) -> Result<(), CommandError> {
    let span = tracing::info_span!("show_main_window");
    let _enter = span.enter();

    window
        .show()
        .map_err(|e| CommandError::unknown(e.to_string()))?;
    window
        .set_focus()
        .map_err(|e| CommandError::unknown(e.to_string()))?;
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
        catalog_command_handlers::update_railway_model_delivery_date,
        catalog_command_handlers::update_rolling_stock_railway_company,
        catalog_command_handlers::update_rolling_stock_category,
        catalog_command_handlers::update_rolling_stock_subcategory,
        catalog_command_handlers::update_rolling_stock_service_level,
        catalog_command_handlers::update_rolling_stock_dcc,
        catalog_command_handlers::update_rolling_stock_specifications,
        catalog_command_handlers::get_railway_model_translations,
        catalog_command_handlers::upsert_railway_model_translation,
        catalog_command_handlers::search_railway_models,
        catalog_command_handlers::add_rolling_stock_to_model,
        catalog_command_handlers::delete_rolling_stock,
        catalog_command_handlers::get_coupler_types,
        catalog_command_handlers::set_rolling_stock_coupler,
        collecting_command_handlers::add_railway_model_to_collection,
        collecting_command_handlers::record_acquisition,
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
        wishlist_command_handlers::update_wishlist_item,
        maintenance_command_handlers::get_maintenance_dashboard,
        maintenance_command_handlers::get_maintenance_card,
        maintenance_command_handlers::add_maintenance_event,
        maintenance_command_handlers::add_maintenance_card,
        maintenance_command_handlers::delete_maintenance_event,
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
        export_command_handlers::get_export_preview,
        export_command_handlers::open_export_file_dialog,
        export_command_handlers::execute_export,
        import_command_handlers::analyze_import_package,
        import_command_handlers::get_import_preview,
        import_command_handlers::execute_import,
        import_command_handlers::cancel_import_session,
        budget_command_handlers::get_budget_config,
        budget_command_handlers::set_budget_config,
        budget_command_handlers::get_monthly_budget_records,
        budget_command_handlers::get_budget_dashboard,
        budget_command_handlers::get_budget_bootstrap,
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
        get_locale,
        database_backup_command_handlers::export_database,
        database_backup_command_handlers::import_database,
        search_command_handlers::global_search,
        trains_command_handlers::create_train_formation,
        trains_command_handlers::update_train_formation,
        trains_command_handlers::delete_train_formation,
        trains_command_handlers::get_train_formation,
        trains_command_handlers::get_train_formations,
        trains_command_handlers::add_formation_element,
        trains_command_handlers::remove_formation_element,
        trains_command_handlers::reorder_formation_elements,
        trains_command_handlers::assign_rolling_stock_to_element,
        trains_command_handlers::set_traction_override,
        trains_command_handlers::get_prototypes,
        trains_command_handlers::create_custom_prototype,
        trains_command_handlers::get_formation_categories,
        trains_command_handlers::create_formation_category
    ]);

    #[allow(unused_variables)]
    let ts_config = Typescript::default();

    // 2. Export the bindings (This creates the TS file)
    #[cfg(debug_assertions)] // Only export during development
    builder
        .export(ts_config, "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");

    let mut builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview), // Required for attachConsole() to work
                ])
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_oauth::init())
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            init_tracing().map_err(|e| anyhow::anyhow!("failed to initialize tracing: {e}"))?;
            let setup_span = tracing::info_span!("app_setup");
            let _setup_guard = setup_span.enter();

            let version = env!("CARGO_PKG_VERSION");

            tracing::info!("{}", LOGO);
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

            tracing::info!(models_dir = %models_dir.display(), "models directory resolved");

            let db_path = app
                .handle()
                .path()
                .resolve("database.sqlite", BaseDirectory::AppData)
                .map_err(|e| anyhow::anyhow!("Failed to resolve db path: {e}"))?;

            // Initial management of state
            app.manage(AppState::new(pool.clone(), models_dir, db_path));

            start_connectivity_monitor(app.handle().clone());

            // Setup viewport and icon for main window
            if let Some(window) = app.get_webview_window("main") {
                // Set window icon explicitly so GNOME shows it during `tauri dev`
                // (no .desktop file exists in dev mode, so we push the pixel buffer directly)
                let icon_path = app
                    .path()
                    .resolve("icons/128x128.png", BaseDirectory::Resource)
                    .map_err(|e| anyhow::anyhow!("Failed to resolve icon path: {e}"))?;

                match tauri::image::Image::from_path(&icon_path) {
                    Ok(icon) => {
                        if let Err(e) = window.set_icon(icon) {
                            tracing::warn!(error = %e, "failed to set window icon");
                        }
                    }
                    Err(e) => tracing::warn!(
                        "Failed to load window icon from {}: {}",
                        icon_path.display(),
                        e
                    ),
                }

                if let Err(e) = crate::viewport::setup_viewport(&window) {
                    tracing::warn!(error = %e, "failed to setup viewport");
                }
            }

            // Register Ctrl+N global shortcut to open the acquisition drawer
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
            app.global_shortcut()
                .on_shortcut("CommandOrControl+N", |app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        app.emit("open-acquisition-drawer", ()).ok();
                    }
                })
                .map_err(|e| anyhow::anyhow!("Failed to register global shortcut: {e}"))?;

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
