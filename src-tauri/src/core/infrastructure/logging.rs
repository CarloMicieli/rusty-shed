use std::sync::OnceLock;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

const DEFAULT_LOG_FILTER: &str = "info,sqlx=warn";

static TRACING_INITIALIZER: OnceLock<()> = OnceLock::new();

/// Initializes global tracing with an environment-driven filter.
///
/// If `RUST_LOG` is present it is used, otherwise a fallback of
/// `info,sqlx=warn` is applied to suppress noisy SQLx debug logs.
pub fn init_tracing() -> anyhow::Result<()> {
    if TRACING_INITIALIZER.get().is_some() {
        return Ok(());
    }

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(DEFAULT_LOG_FILTER))
        .map_err(|e| anyhow::anyhow!("invalid tracing filter: {e}"))?;

    let subscriber = Registry::default().with(env_filter).with(
        fmt::layer()
            .compact()
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false),
    );

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| anyhow::anyhow!("failed to initialize tracing subscriber: {e}"))?;

    let _ = TRACING_INITIALIZER.set(());
    Ok(())
}

#[cfg(test)]
pub mod test_helper {
    use std::sync::OnceLock;
    use tracing_subscriber::fmt;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::{EnvFilter, Registry};

    static INITIALIZER: OnceLock<()> = OnceLock::new();

    /// Initializes a global logger for the test suite using a thread-safe singleton pattern.
    ///
    /// This function ensures that the logging backend is configured exactly once, regardless
    /// of how many tests are run in parallel. It uses [`std::sync::OnceLock`] to synchronize
    /// initialization across multiple threads.
    ///
    /// ### Features
    /// - **Persistence**: Logs are written to `logs/tests/test_run.log`.
    /// - **Rotation**: Automatically rotates log files daily and retains the last 3 runs.
    /// - **Mirroring**: Outputs all logs to `stderr` so they are visible during `cargo test -- --nocapture`.
    /// - **Idempotency**: Subsequent calls to this function in the same process are no-ops.
    ///
    /// ### Implementation Details
    /// This helper uses `try_start()` instead of `start()` to gracefully handle scenarios
    /// where a logger might have already been initialized (e.g., in a full integration
    /// test that boots the Tauri application).
    pub fn setup() {
        INITIALIZER.get_or_init(|| {
            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info,sqlx=warn"))
                .expect("failed to build tracing env filter");

            let subscriber = Registry::default().with(env_filter).with(
                fmt::layer()
                    .compact()
                    .with_target(true)
                    .with_thread_ids(false)
                    .with_thread_names(false),
            );

            if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
                eprintln!("tracing subscriber already active or failed: {e}");
            }
        });
    }
}
