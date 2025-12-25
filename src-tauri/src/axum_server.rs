use axum::extract::State;
use axum::{Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use axum::{extract::Request, middleware::Next, response::Response};
use rand::{distr::Alphanumeric, distr::SampleString};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

use crate::AXUM_SERVER_TOKEN;
use crate::AXUM_SHUTDOWN_SENDER;

#[derive(Clone)]
struct AuthConfig {
    token: String,
}

pub fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 32)
}

pub fn start_axum_server() -> Result<tokio::sync::oneshot::Receiver<u16>, anyhow::Error> {
    let (tx, rx) = oneshot::channel();

    // Generate a secure random token
    let shared_token = generate_token();
    eprintln!("Generated Axum server token: {}", shared_token);

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    AXUM_SHUTDOWN_SENDER
        .set(Arc::new(Mutex::new(Some(shutdown_tx))))
        .ok();

    let config = AuthConfig {
        token: shared_token.clone(),
    };

    // Store the token immediately
    AXUM_SERVER_TOKEN.set(shared_token).ok();

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

    Ok(rx)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_startup_non_blocking() {
        let start = std::time::Instant::now();
        let rx = start_axum_server().expect("Failed to start server");
        let duration = start.elapsed();

        // Should return almost instantly (e.g., < 100ms), definitely not block for server init
        assert!(
            duration.as_millis() < 100,
            "Startup took too long: {:?}",
            duration
        );

        let port = rx.await.expect("Failed to receive port");
        assert!(port > 0, "Port should be non-zero");
        println!("Server started on port {}", port);
    }
}
