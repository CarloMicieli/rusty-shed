use axum::extract::State;
use axum::{Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use axum::{extract::Request, middleware::Next, response::Response};
use rand::{Rng, distributions::Alphanumeric};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

use crate::AXUM_SERVER_PORT;
use crate::AXUM_SERVER_TOKEN;
use crate::AXUM_SHUTDOWN_SENDER;

#[derive(Clone)]
struct AuthConfig {
    token: String,
}

pub fn start_axum_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (tx, rx) = oneshot::channel();

    // Generate a secure random token
    let shared_token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    AXUM_SHUTDOWN_SENDER
        .set(Arc::new(Mutex::new(Some(shutdown_tx))))
        .ok();

    let config = AuthConfig {
        token: shared_token.clone(),
    };

    // Spawn the Axum server in a background OS thread with its own Tokio runtime
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime for axum server");

        runtime.block_on(async move {
            let app = Router::new()
                .with_state(config.clone())
                .route("/greet", get(greet_handler))
                .layer(axum::middleware::from_fn_with_state(
                    config.clone(),
                    auth_middleware,
                ));

            // Bind to port 0 to let the OS assign a random available port
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let port = listener.local_addr().unwrap().port();

            // Send the port back to the main thread
            let _ = tx.send(port);

            println!("Axum server running on http://127.0.0.1:{}", port);
            // Serve with graceful shutdown
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("Axum server error");
        });
    });

    // Wait briefly for the port to be assigned before starting the UI
    let port = tokio::runtime::Runtime::new()?
        .block_on(rx)
        .expect("Failed to get port from Axum");

    // Store the chosen port
    AXUM_SERVER_PORT.set(port).ok();
    AXUM_SERVER_TOKEN.set(shared_token).ok();

    Ok(())
}

pub fn stop_axum_server() {
    if let Some(m) = AXUM_SHUTDOWN_SENDER.get()
        && let Some(tx) = m.lock().unwrap().take()
    {
        let _ = tx.send(());
    }
}

async fn greet_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let name = params.get("name").map(|s| s.as_str()).unwrap_or("World");
    (StatusCode::OK, format!("Hello, {}", name))
}

async fn auth_middleware(
    State(config): State<AuthConfig>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = &config.token;

    match auth_header {
        Some(value) if value == format!("Bearer {}", token) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
