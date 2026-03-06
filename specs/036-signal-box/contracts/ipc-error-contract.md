# IPC Contract: CommandError (Updated)

**Feature**: 036-signal-box
**Date**: 2026-03-06
**Contract Type**: Tauri IPC — error response type (backend → frontend)

---

## Overview

This contract documents the updated `CommandError` type that all Tauri commands return on failure. The only breaking change is the promotion of the `Unknown` variant from a tuple variant to a struct variant to carry the new `error_id` field.

**Impact**: All frontend consumers that pattern-match on `CommandError::Unknown(msg)` must be updated to destructure `{ message, error_id }`.

---

## Updated Rust Type

```rust
// File: src-tauri/src/core/infrastructure/error.rs

#[derive(Debug, Clone, serde::Serialize, specta::Type, thiserror::Error)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CommandError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error")]
    ValidationError(HashMap<String, Vec<ValidationError>>),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    // UPDATED: struct variant with error_id
    #[error("Unknown error: {message}")]
    Unknown {
        message: String,
        error_id: String,
    },
}
```

---

## Serialized JSON Shape (on the wire)

### All existing variants (unchanged):

```json
{ "kind": "databaseError", "message": "..." }
{ "kind": "notFound", "message": "..." }
{ "kind": "validationError", "fields": { "name": [{ "code": "required", "message": "..." }] } }
{ "kind": "permissionDenied", "message": "..." }
{ "kind": "businessRule", "message": "..." }
{ "kind": "conflict", "message": "..." }
```

### Updated `Unknown` variant:

**Before**:
```json
{ "kind": "unknown", "message": "Something went wrong" }
```

**After**:
```json
{
  "kind": "unknown",
  "message": "Something went wrong",
  "error_id": "ERR-4421-K"
}
```

---

## Generated TypeScript Type (specta output)

```typescript
// Auto-generated — do not edit manually
export type CommandError =
  | { kind: "databaseError"; message: string }
  | { kind: "notFound"; message: string }
  | { kind: "validationError"; fields: Record<string, ValidationError[]> }
  | { kind: "permissionDenied"; message: string }
  | { kind: "businessRule"; message: string }
  | { kind: "conflict"; message: string }
  | { kind: "unknown"; message: string; error_id: string };  // UPDATED
```

---

## Frontend Normalization Contract

### `NormalizedError` (updated interface in `src/lib/services/errors.ts`):

```typescript
interface NormalizedError {
  kind: 'database' | 'not_found' | 'validation' | 'permission_denied' | 'unknown';
  message: string;
  fields?: Record<string, string>;
  errorId?: string;  // NEW: present when kind === 'unknown'
}
```

### `normalizeError()` update logic:

When the incoming `CommandError` has `kind === 'unknown'`:
- Map `error_id` → `errorId` in `NormalizedError`
- Preserve all other mapping logic unchanged

---

## New Utilities Contract

### `src/lib/services/error-id.ts`

```typescript
/**
 * Generates a unique Error ID for session-scoped tracing.
 * Format: ERR-NNNN-X where NNNN is 1000–9999 and X is A–Z.
 */
export function generateErrorId(): string;

/**
 * Validates whether a string matches the Error ID format.
 */
export function isErrorId(value: unknown): value is string;
```

### `src/lib/services/module-label.ts`

```typescript
/**
 * Derives the active module label from the current URL pathname.
 * Returns a Paraglide-translated string.
 */
export function getModuleLabel(pathname: string): string;
```

---

## New Toaster Method Contract

### `src/lib/toaster.ts` — addition

```typescript
interface Toaster {
  // ... existing methods ...

  /**
   * Shows an Amber-bordered Signal toast for non-fatal yard faults.
   * Uses the 'toast-signal' CSS class for Amber border styling.
   */
  signal(title: string, options?: { description?: string }): string | number;
}
```

---

## Rust Logging Contract

Every `CommandError::Unknown` construction MUST emit a structured log entry:

```rust
tracing::error!(
    error_id = %error.error_id,
    "Signal Fault: {}", error.message
);
```

This is enforced by having `ErrorId::generate()` + the log call co-located in a factory method:

```rust
impl CommandError {
    pub fn unknown(message: impl Into<String>) -> Self {
        let id = ErrorId::generate();
        let msg = message.into();
        tracing::error!(error_id = %id, "Signal Fault: {}", msg);
        Self::Unknown { message: msg, error_id: id.to_string() }
    }
}
```

All call sites that currently produce `CommandError::Unknown(msg)` migrate to `CommandError::unknown(msg)`.

---

## Migration Notes

1. **Specta regeneration required** after changing `Unknown` to a struct variant. Run `pnpm tauri dev` once to rebuild bindings.
2. **Frontend `normalizeError()`** must be updated before specta regeneration to avoid TypeScript compile errors.
3. **Existing `From<anyhow::Error>` impl** for `CommandError` must be updated to call `CommandError::unknown(e.to_string())` instead of `CommandError::Unknown(e.to_string())`.
4. **No database migrations** required — this feature involves no schema changes.
