use crate::core::domain::domain_error::DomainError;

/// Converts a `sqlx::Error` into a `DomainError::Infrastructure` by capturing
/// the error message as a `String`. Defined here (infrastructure layer) so
/// the domain module has no compile-time dependency on sqlx.
impl From<sqlx::Error> for DomainError {
    fn from(e: sqlx::Error) -> Self {
        DomainError::Infrastructure(e.to_string())
    }
}

/// An extension trait for `sqlx::Error` results to provide domain-specific context and automated logging.
///
/// This trait follows the "Extension Trait" pattern, allowing you to enrich standard database
/// results with application-level meaning while ensuring all failures are recorded in the system logs.
pub trait WithDomainContext<T> {
    /// Enrich a database result with context, log it as an error, and convert it to a [`DomainError`].
    ///
    /// This method is intended to be used at the boundary between your infrastructure (SQL)
    /// and your domain logic.
    ///
    /// ### Side Effects
    /// - **Logging**: If the result is an `Err`, it immediately triggers an `error!` log
    ///   containing both your custom context string and the underlying database error details.
    ///
    /// ### Arguments
    /// - `context`: A string (or anything that implements `Into<String>`) describing
    ///   what the application was attempting to do (e.g., "Failed to update user balance").
    fn with_domain_context<S: Into<String>>(self, context: S) -> Result<T, DomainError>;
}

impl<T> WithDomainContext<T> for Result<T, sqlx::Error> {
    /// Implementation of the context provider for SQLx results.
    ///
    /// It maps the [`sqlx::Error`] to a [`DomainError::Infrastructure`] after
    /// logging the error with context.
    fn with_domain_context<S: Into<String>>(self, context: S) -> Result<T, DomainError> {
        self.map_err(|err| {
            let msg = context.into();
            let span = tracing::info_span!("with_domain_context_sqlx", context = %msg);
            let _enter = span.enter();
            tracing::error!(error = ?err, "infrastructure sqlx error");
            DomainError::Infrastructure(err.to_string())
        })
    }
}

impl<T> WithDomainContext<T> for Result<T, std::io::Error> {
    fn with_domain_context<S: Into<String>>(self, context: S) -> Result<T, DomainError> {
        self.map_err(|err| {
            let msg = context.into();
            let span = tracing::info_span!("with_domain_context_io", context = %msg);
            let _enter = span.enter();
            tracing::error!(error = ?err, "infrastructure io error");
            DomainError::Infrastructure(err.to_string())
        })
    }
}

impl<T> WithDomainContext<T> for Result<T, serde_json::Error> {
    fn with_domain_context<S: Into<String>>(self, context: S) -> Result<T, DomainError> {
        self.map_err(|err| {
            let msg = context.into();
            let span = tracing::info_span!("with_domain_context_json", context = %msg);
            let _enter = span.enter();
            tracing::error!(error = ?err, "infrastructure serialization error");
            DomainError::Infrastructure(err.to_string())
        })
    }
}

impl<T> WithDomainContext<T> for Result<T, anyhow::Error> {
    fn with_domain_context<S: Into<String>>(self, context: S) -> Result<T, DomainError> {
        self.map_err(|err| {
            let msg = context.into();
            let span = tracing::info_span!("with_domain_context_anyhow", context = %msg);
            let _enter = span.enter();
            tracing::error!(error = ?err, "infrastructure generic error");
            DomainError::Infrastructure(err.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WithDomainContext;
    use crate::core::domain::domain_error::DomainError;

    #[test]
    fn returns_value_when_result_is_ok() {
        let result = Ok::<_, sqlx::Error>(42).with_domain_context("read ok");

        assert!(matches!(result, Ok(42)));
    }

    #[test]
    fn maps_sqlx_error_to_infrastructure_error() {
        let result = Err::<(), _>(sqlx::Error::RowNotFound).with_domain_context("load row");

        assert!(matches!(result, Err(DomainError::Infrastructure(message)) if !message.is_empty()));
    }

    #[test]
    fn maps_io_error_to_infrastructure_error() {
        let result = Err::<(), _>(std::io::Error::other("disk unavailable"))
            .with_domain_context("read file");

        assert!(matches!(
            result,
            Err(DomainError::Infrastructure(message)) if message.contains("disk unavailable")
        ));
    }

    #[test]
    fn maps_json_error_to_infrastructure_error() {
        let result =
            serde_json::from_str::<serde_json::Value>("{").with_domain_context("parse payload");

        assert!(matches!(
            result,
            Err(DomainError::Infrastructure(message)) if message.contains("EOF while parsing an object")
        ));
    }

    #[test]
    fn maps_anyhow_error_to_infrastructure_error() {
        let result = Err::<(), _>(anyhow::anyhow!("boom")).with_domain_context("generic failure");

        assert!(matches!(result, Err(DomainError::Infrastructure(message)) if message == "boom"));
    }
}
