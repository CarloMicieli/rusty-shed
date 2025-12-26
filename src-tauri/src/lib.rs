use tauri::Manager;

mod db;
mod state;

pub mod catalog;
pub mod collecting;
pub mod core;

use crate::state::AppState;
use db::{MIGRATOR, init_db_pool};
use log::{LevelFilter, error};
use specta_typescript::{BigIntExportBehavior, Typescript};
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
        crate::collecting::interface::command_handlers::get_collection,
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
            // 1. Initialize the pool
            let pool = tauri::async_runtime::block_on(async {
                init_db_pool().await.map_err(|e| anyhow::anyhow!(e))
            })?;

            // 2. Initial management of state
            app.manage(AppState::new(pool.clone()));

            // 3. Show the main window IMMEDIATELY to avoid blank screen
            // The UI can handle the "not initialized" state gracefully
            if let Some(window) = app.get_webview_window("main")
                && let Err(e) = window.show()
            {
                error!("Failed to show main window: {e}");
            }

            let handle = app.handle().clone();

            // 4. Run migrations in an async task (non-blocking)
            tauri::async_runtime::spawn(async move {
                let state_ref = handle.state::<AppState>();
                let _ = MIGRATOR
                    .run(&state_ref.db_pool())
                    .await
                    .map_err(|e| anyhow::anyhow!(e));

                state_ref.set_initialized();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
