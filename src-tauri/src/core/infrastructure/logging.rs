use anyhow::Context;
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, LoggerHandle, Naming};
use tauri::{App, Manager, Wry};

/// Initializes the global logger with a daily rolling file strategy and terminal mirroring.
///
/// This function configures `flexi_logger` to handle application-wide logging for both
/// the Rust backend and specific crate filtering (e.g., `tauri` and `sqlx`).
///
/// ### Logging Strategy
/// - **Storage**: Logs are stored in the OS-specific application log directory
///   (e.g., `AppData/Local` on Windows or `~/Library/Logs` on macOS).
/// - **Rotation**: A new log file is created every 24 hours.
/// - **Retention**: Only the last 7 days of logs are preserved; older files are automatically deleted.
/// - **Mirroring**:
///   - In **Debug** mode (`cargo tauri dev`), logs are printed to both the file and `stderr`.
///   - In **Release** mode, logs are written *only* to the file to keep the user's terminal clean.
///
/// ### Filtering
/// - Default level is `info`.
/// - `tauri` and `sqlx` are throttled to `warn` to reduce noise from internal framework events.
///
/// ### Errors
/// Returns an [`anyhow::Error`] if:
/// - The system cannot resolve the application's log directory path.
/// - The logger has already been initialized (only one global logger is permitted).
/// - The application lacks write permissions for the log directory.
pub fn init_logger(app: &mut App<Wry>) -> anyhow::Result<LoggerHandle> {
    let is_dev_build = cfg!(debug_assertions);
    let log_spec = if is_dev_build {
        "debug, tauri=info, sqlx=debug" // Verbose for development
    } else {
        "info, tauri=warn, sqlx=warn" // Quiet for production
    };

    Logger::try_with_str(log_spec)?
        .log_to_file(
            FileSpec::default()
                .directory(app.path().app_log_dir()?) // Use Tauri's recommended log path
                .basename("rust_shed"), // Base name for log files
        )
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7), // Keep a week of logs
        )
        // In dev mode, show logs in terminal; in release, only to file
        .duplicate_to_stderr(if cfg!(debug_assertions) {
            Duplicate::All
        } else {
            Duplicate::None
        })
        .start()
        .with_context(|| "failed to initialize logging")
}

#[cfg(test)]
pub mod test_helper {
    use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
    use std::sync::OnceLock;

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
        // .get_or_init will execute the closure only once.
        // Subsequent calls will simply return the existing value and skip the closure.
        INITIALIZER.get_or_init(|| {
            let result = Logger::try_with_str("info, sqlx=warn")
                .expect("Invalid Log Spec")
                .log_to_file(
                    FileSpec::default()
                        .directory("logs/tests")
                        .basename("test_run"),
                )
                .rotate(
                    Criterion::Age(Age::Day),
                    Naming::Timestamps,
                    Cleanup::KeepLogFiles(3),
                )
                .duplicate_to_stderr(Duplicate::All)
                .start();

            match result {
                Ok(_) => println!("✅ Standard library OnceLock initialized logger."),
                Err(e) => {
                    // This handles cases where another logger is already active
                    eprintln!("⚠️ Logger already active or failed: {e}");
                }
            }
        });
    }
}
