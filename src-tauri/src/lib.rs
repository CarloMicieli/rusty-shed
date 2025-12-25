// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

pub static AXUM_SERVER_PORT: OnceCell<u16> = OnceCell::new();
pub static AXUM_SHUTDOWN_SENDER: OnceCell<Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>> =
    OnceCell::new();

mod axum_server;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_server_port() -> Result<u16, String> {
    AXUM_SERVER_PORT
        .get()
        .copied()
        .ok_or_else(|| "Axum server port not set".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // start axum server early so the UI can query the port
    if let Err(e) = axum_server::start_axum_server() {
        eprintln!("Failed to start axum server: {e}");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![greet, get_server_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // run() returns when the app exits — stop the axum server gracefully
    axum_server::stop_axum_server();
}
