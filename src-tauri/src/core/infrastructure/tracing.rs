//! Shared tracing primitives for ADR-12 instrumentation.

use crate::core::domain::ErrorId;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::error::CommandError;
use tracing::Span;

/// Maps error values to a standardized tracing error type.
pub trait AsTraceKind {
    /// Returns the canonical error kind used in span fields.
    fn trace_kind(&self) -> &'static str;
}

/// Identifies errors with a stable backend error reference.
pub trait AsErrorId {
    /// Return the identifier associated with this error.
    fn error_id(&self) -> ErrorId;
}

impl AsErrorId for CommandError {
    fn error_id(&self) -> ErrorId {
        CommandError::error_id(self)
    }
}

impl AsErrorId for DomainError {
    fn error_id(&self) -> ErrorId {
        DomainError::error_id(self)
    }
}

impl AsTraceKind for CommandError {
    fn trace_kind(&self) -> &'static str {
        match self {
            CommandError::ValidationError { .. } => "validation",
            CommandError::NotFound { .. } => "not_found",
            CommandError::PermissionDenied { .. } => "unauthorized",
            CommandError::BusinessRule { .. } | CommandError::Conflict { .. } => "domain",
            CommandError::DatabaseError { .. } => "infrastructure",
            CommandError::Unknown { .. } => "internal",
        }
    }
}

impl AsTraceKind for DomainError {
    fn trace_kind(&self) -> &'static str {
        match self {
            DomainError::Validation { .. } | DomainError::ValidationError { .. } => "validation",
            DomainError::NotFound { .. } => "not_found",
            DomainError::BusinessRule { .. } | DomainError::Conflict { .. } => "domain",
            DomainError::Infrastructure { .. } => "infrastructure",
            DomainError::InvalidIdentifier { .. } => "validation",
        }
    }
}

/// Extension trait to enrich the current tracing span with error metadata.
pub trait RecordTraceErrExt<T, E> {
    /// Records `error.type` and `error.message` on the current span when `self` is `Err`.
    fn record_trace_err(self) -> Self;
}

impl<T, E> RecordTraceErrExt<T, E> for Result<T, E>
where
    E: AsTraceKind + AsErrorId + std::fmt::Display,
{
    fn record_trace_err(self) -> Self {
        if let Err(ref err) = self {
            let current = Span::current();
            current.record("error.type", err.trace_kind());
            current.record("error.message", tracing::field::display(err));
            current.record("error.id", tracing::field::display(err.error_id()));
        }
        self
    }
}

/// Returns the current tracing span identifier, or `"unindexed"` when unavailable.
pub fn current_trace_id() -> String {
    Span::current()
        .id()
        .map(|id| format!("{:x}", id.into_u64()))
        .unwrap_or_else(|| "unindexed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_error_trace_kind_maps_correctly() {
        assert_eq!(
            CommandError::validation_field("field", "invalid").trace_kind(),
            "validation"
        );
        assert_eq!(CommandError::not_found("x").trace_kind(), "not_found");
        assert_eq!(
            CommandError::permission_denied("x").trace_kind(),
            "unauthorized"
        );
        assert_eq!(CommandError::database("x").trace_kind(), "infrastructure");
    }

    #[test]
    fn current_trace_id_without_span_is_unindexed() {
        assert_eq!(current_trace_id(), "unindexed");
    }
}
