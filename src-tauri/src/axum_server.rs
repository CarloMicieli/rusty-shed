use axum::{routing::get, Router, extract::Query, response::IntoResponse, http::StatusCode};
use std::collections::HashMap;
use tokio::sync::oneshot;
use std::sync::{Arc, Mutex};

use crate::AXUM_SERVER_PORT;
use crate::AXUM_SHUTDOWN_SENDER;

pub fn start_axum_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // bind to a random available port on localhost
    let listener = std::net::TcpListener::bind("127.0.0.1:9999")?;
    let port = listener.local_addr()?.port();
    
    // Store the chosen port
    AXUM_SERVER_PORT.set(port).ok();
    
    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    AXUM_SHUTDOWN_SENDER.set(Arc::new(Mutex::new(Some(shutdown_tx)))).ok();
    
    // Spawn the server on Tauri's async runtime (non-blocking)
    tauri::async_runtime::spawn(async move {
        let listener = tokio::net::TcpListener::from_std(listener).expect("Failed to convert listener");
        let app = Router::new()
            .route("/ping", get(|| async { "pong" }))
            .route("/greet", get(greet_handler));
        
        // Serve with graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("Axum server error");
    });
    
    Ok(())
}

pub fn stop_axum_server() {
    if let Some(m) = AXUM_SHUTDOWN_SENDER.get() {
        if let Some(tx) = m.lock().unwrap().take() {
            let _ = tx.send(());
        }
    }
}

async fn greet_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let name = params.get("name").map(|s| s.as_str()).unwrap_or("World");
    (StatusCode::OK, format!("Hello, {}", name))
}
