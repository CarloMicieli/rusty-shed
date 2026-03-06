# Data Model: The Signal Box — Error Management System

**Phase**: 1 — Design
**Date**: 2026-03-06
**Feature**: 036-signal-box

---

## Overview

This feature introduces no new persistent entities. All data lives in-memory for the duration of a session (error state is transient). The key data structures are:

1. **SignalFault** — the unified representation of a captured error, shared between the Rust IPC layer and the frontend.
2. **ErrorId** — a value type for the unique short identifier.
3. **ModuleLabel** — a display-only string derived from the active route.

---

## Rust Types

### `ErrorId` (value type)

A newtype wrapper around `String` that enforces the `ERR-NNNN-X` format on construction.

```
ErrorId
├── value: String           // e.g. "ERR-8821-X"
└── fn generate() -> Self   // creates a new unique ID
```

**Validation rules**:
- Format: `ERR-[1000–9999]-[A–Z]`
- Generated at fault time; never persisted.
- `Display` impl returns the raw string for logging and UI.

---

### `CommandError` (updated IPC error type)

The existing `CommandError` enum gains an optional `error_id` on the `Unknown` variant. All other variants remain unchanged to preserve backward compatibility.

```
CommandError (enum)
├── DatabaseError(String)
├── NotFound(String)
├── ValidationError(HashMap<String, Vec<ValidationError>>)
├── PermissionDenied(String)
├── BusinessRule(String)
├── Conflict(String)
└── Unknown { message: String, error_id: String }   ← UPDATED: was Unknown(String)
```

**State transitions**:
```
Rust error occurs
  └─→ DomainError / sqlx::Error / anyhow::Error
        └─→ CommandError::Unknown { message, error_id: ErrorId::generate() }
              └─→ tracing::error!(error_id, message)
                    └─→ Serialized to frontend via Tauri IPC
```

**Derive traits**: `Debug, Clone, serde::Serialize, specta::Type, thiserror::Error`

---

## TypeScript Types

### `ErrorId` (value type)

```typescript
type ErrorId = string;  // format: "ERR-NNNN-X"

// Generator utility (src/lib/services/error-id.ts)
function generateErrorId(): ErrorId
```

### `NormalizedError` (updated)

```typescript
interface NormalizedError {
  kind: 'database' | 'not_found' | 'validation' | 'permission_denied' | 'unknown';
  message: string;
  fields?: Record<string, string>;  // validation errors only
  errorId?: ErrorId;                // ← NEW: present for unknown/fatal errors
}
```

### `SignalFaultContext` (new, frontend-only)

Used to pass context into `SignalFailureView`:

```typescript
interface SignalFaultContext {
  errorId: ErrorId;
  moduleLabel: string;   // derived from URL pathname via getModuleLabel()
  message?: string;      // optional additional context; not shown to user in production
}
```

**State transitions**:
```
Tauri IPC error (Unknown variant)
  └─→ normalizeError() → NormalizedError { kind: 'unknown', errorId }
        └─→ SignalFaultContext { errorId, moduleLabel: getModuleLabel($page.url.pathname) }
              └─→ <SignalFailureView context={...} />

JS unhandled exception (Svelte boundary)
  └─→ generateErrorId() → ErrorId
        └─→ SignalFaultContext { errorId, moduleLabel: getModuleLabel($page.url.pathname) }
              └─→ <SignalFailureView context={...} />
```

---

## Module Label Map

This is a pure derivation (no storage) used by `getModuleLabel()`:

| URL Prefix | Paraglide Key | Display Value (en) |
|------------|---------------|-------------------|
| `/dashboard` | `module_label_yard_overview` | Yard Overview |
| `/collection` | `module_label_collection_depot` | Collection Depot |
| `/wishlist` | `module_label_wishlist` | Wishlist |
| `/maintenance` | `module_label_maintenance_log` | Maintenance Log |
| `/finance` | `module_label_finance_ledger` | Finance Ledger |
| `/search` | `module_label_global_search` | Global Search |
| `/settings` | `module_label_settings` | Settings |
| `*` (fallback) | `module_label_signal_box` | Signal Box |

---

## Non-Fatal Toast Data

Toast notifications for non-fatal errors carry minimal data — no new type is introduced. The existing `toaster.signal(message)` call passes a Paraglide message string.

```typescript
// Usage at call site:
toaster.signal(m.signal_toast_title(), { description: m.some_specific_fault_message() });
```

---

## What is NOT modelled

- **Persistence**: Error IDs are never written to the database. Logs are the only durable record.
- **Error history / list**: No error inbox or history view is in scope.
- **User accounts / auth**: Not applicable.
- **Severity levels beyond fatal/non-fatal**: The two-tier model (Signal Failure view vs. toast) is the complete classification scheme.
