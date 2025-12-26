use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub static AXUM_SERVER_PORT: OnceCell<u16> = OnceCell::new();
pub static AXUM_SERVER_TOKEN: OnceCell<String> = OnceCell::new();
pub static AXUM_SHUTDOWN_SENDER: OnceCell<Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>> =
    OnceCell::new();

mod axum_server;
mod db;

pub mod catalog;
pub mod collecting;
pub mod core;

use db::{DB_POOL, MIGRATOR, init_db_pool};
use log::{error, LevelFilter};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

#[tauri::command]
async fn get_server_config() -> Result<(u16, String), String> {
    let port = AXUM_SERVER_PORT
        .get()
        .copied()
        .ok_or_else(|| "Axum server port not set".to_string())?;
    let token = AXUM_SERVER_TOKEN
        .get()
        .cloned()
        .ok_or_else(|| "Axum server token not set".to_string())?;
    Ok((port, token))
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
        .invoke_handler(tauri::generate_handler![get_server_config])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // start axum server early (non-blocking)
                // start axum server early (non-blocking)
                match axum_server::start_axum_server() {
                    Ok(rx) => match rx.await {
                        Ok(port) => {
                            AXUM_SERVER_PORT.set(port).ok();
                        }
                        Err(e) => {
                            error!("Failed to receive port from axum server: {e}");
                            return;
                        }
                    },
                    Err(e) => {
                        error!("Failed to start axum server: {e}");
                        return;
                    }
                }

                // Initialize DB and run migrations
                if let Err(e) = async {
                    // create pool
                    let pool = init_db_pool().await.map_err(|e| anyhow::anyhow!(e))?;
                    // run migrations
                    MIGRATOR.run(&pool).await.map_err(|e| anyhow::anyhow!(e))?;
                    // store pool globally
                    DB_POOL
                        .set(pool)
                        .map_err(|_| anyhow::anyhow!("Failed to set DB_POOL"))?;
                    Ok::<(), anyhow::Error>(())
                }
                .await
                {
                    error!("Database initialization failed: {e}");
                    // Abort startup or show error
                    std::process::exit(1);
                }

                // Show the main window
                if let Some(window) = handle.get_webview_window("main")
                    && let Err(e) = window.show()
                {
                    error!("Failed to show main window: {e}");
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // run() returns when the app exits — stop the axum server gracefully
    axum_server::stop_axum_server();
}
