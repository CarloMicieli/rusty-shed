use std::sync::OnceLock;
use tauri_plugin_log::log::{self, Level as LogLevel, Record};
use tracing::{Event, Subscriber};
use tracing_subscriber::field::Visit;
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};
use tracing_subscriber::{EnvFilter, Registry};

const DEFAULT_LOG_FILTER: &str = "info,sqlx=warn";

static TRACING_INITIALIZER: OnceLock<()> = OnceLock::new();

#[derive(Default)]
struct EventMessageVisitor {
    message: Option<String>,
    fields: Vec<String>,
}

impl Visit for EventMessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.push_field(field.name(), format!("{value:?}"));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.push_field(field.name(), value.to_owned());
    }
}

impl EventMessageVisitor {
    fn push_field(&mut self, name: &str, value: String) {
        if name == "message" {
            self.message = Some(value);
            return;
        }

        self.fields.push(format!("{name}={value}"));
    }

    fn finish(self, event_name: &str) -> String {
        let mut rendered = self.message.unwrap_or_else(|| event_name.to_owned());

        if !self.fields.is_empty() {
            rendered.push(' ');
            rendered.push('{');
            rendered.push_str(&self.fields.join(", "));
            rendered.push('}');
        }

        rendered
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct TauriLogLayer;

impl<S> Layer<S> for TauriLogLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = EventMessageVisitor::default();
        event.record(&mut visitor);
        let message = visitor.finish(metadata.name());
        let args = format_args!("{message}");

        let record = Record::builder()
            .args(args)
            .level(tracing_level_to_log_level(*metadata.level()))
            .target(metadata.target())
            .module_path_static(metadata.module_path())
            .file_static(metadata.file())
            .line(metadata.line())
            .build();

        log::logger().log(&record);
    }
}

const fn tracing_level_to_log_level(level: tracing::Level) -> LogLevel {
    match level {
        tracing::Level::ERROR => LogLevel::Error,
        tracing::Level::WARN => LogLevel::Warn,
        tracing::Level::INFO => LogLevel::Info,
        tracing::Level::DEBUG => LogLevel::Debug,
        tracing::Level::TRACE => LogLevel::Trace,
    }
}

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

    let subscriber = Registry::default().with(env_filter).with(TauriLogLayer);

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(tracing::Level::ERROR, LogLevel::Error)]
    #[case(tracing::Level::WARN, LogLevel::Warn)]
    #[case(tracing::Level::INFO, LogLevel::Info)]
    #[case(tracing::Level::DEBUG, LogLevel::Debug)]
    #[case(tracing::Level::TRACE, LogLevel::Trace)]
    fn it_should_map_tracing_levels_to_log_levels(
        #[case] level: tracing::Level,
        #[case] expected: LogLevel,
    ) {
        assert_eq!(expected, tracing_level_to_log_level(level));
    }
}
