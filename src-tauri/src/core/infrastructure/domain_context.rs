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
            // Perform the logging side effect before moving the error
            log::error!("{}: {:?}", msg, err);
            DomainError::Infrastructure(err.to_string())
        })
    }
}
