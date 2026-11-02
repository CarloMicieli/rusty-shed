//! Shared tracing primitives for ADR-12 instrumentation.

use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::error::CommandError;
use tracing::Span;

/// Maps error values to a standardized tracing error type.
pub trait AsTraceKind {
    /// Returns the canonical error kind used in span fields.
    fn trace_kind(&self) -> &'static str;
}

impl AsTraceKind for CommandError {
    fn trace_kind(&self) -> &'static str {
        match self {
            CommandError::ValidationError(_) => "validation",
            CommandError::NotFound(_) => "not_found",
            CommandError::PermissionDenied(_) => "unauthorized",
            CommandError::BusinessRule(_) | CommandError::Conflict(_) => "domain",
            CommandError::DatabaseError(_) => "infrastructure",
            CommandError::Unknown { .. } => "internal",
        }
    }
}

impl AsTraceKind for DomainError {
    fn trace_kind(&self) -> &'static str {
        match self {
            DomainError::Validation(_) | DomainError::ValidationError(_) => "validation",
            DomainError::NotFound { .. } => "not_found",
            DomainError::BusinessRule(_) | DomainError::Conflict(_) => "domain",
            DomainError::Infrastructure(_) => "infrastructure",
            DomainError::InvalidIdentifier(_) => "validation",
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
    E: AsTraceKind + std::fmt::Display,
{
    fn record_trace_err(self) -> Self {
        if let Err(ref err) = self {
            let current = Span::current();
            current.record("error.type", err.trace_kind());
            current.record("error.message", tracing::field::display(err));
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
        assert_eq!(
            CommandError::NotFound("x".to_string()).trace_kind(),
            "not_found"
        );
        assert_eq!(
            CommandError::PermissionDenied("x".to_string()).trace_kind(),
            "unauthorized"
        );
        assert_eq!(
            CommandError::DatabaseError("x".to_string()).trace_kind(),
            "infrastructure"
        );
    }

    #[test]
    fn current_trace_id_without_span_is_unindexed() {
        assert_eq!(current_trace_id(), "unindexed");
    }
}
