use tauri::Manager;

mod db;
mod state;

pub mod catalog;
pub mod collecting;
pub mod core;

use crate::state::AppState;
use db::{MIGRATOR, init_db_pool};
use log::{LevelFilter, error};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

#[tauri::command]
fn is_db_initialized(state: tauri::State<'_, AppState>) -> bool {
    state.is_initialized()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Determine log level based on build type
    let level = if cfg!(debug_assertions) {
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
        .invoke_handler(tauri::generate_handler![
            is_db_initialized,
            crate::collecting::interface::command_handlers::get_collection
        ])
        .setup(|app| {
            // 1. Initialize the pool
            let pool = tauri::async_runtime::block_on(async {
                init_db_pool().await.map_err(|e| anyhow::anyhow!(e))
            })?;

            // 2. Initial management of state
            app.manage(AppState::new(pool.clone()));

            let handle = app.handle().clone();

            // 4. Run migrations in an async task
            tauri::async_runtime::spawn(async move {
                let state_ref = handle.state::<AppState>();
                let _ = MIGRATOR
                    .run(&state_ref.db_pool())
                    .await
                    .map_err(|e| anyhow::anyhow!(e));

                state_ref.set_initialized();
            });

            // Show the main window
            let handle = app.handle().clone();
            if let Some(window) = handle.get_webview_window("main")
                && let Err(e) = window.show()
            {
                error!("Failed to show main window: {e}");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
