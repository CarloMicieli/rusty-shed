# Research: The Signal Box — Error Management System

**Phase**: 0 — Research & Discovery
**Date**: 2026-03-06
**Feature**: 036-signal-box

---

## 1. Existing Error Infrastructure Audit

### Decision: Extend, don't replace

The project already has a layered error architecture that is sound. The Signal Box feature extends it rather than replacing it.

| Layer                     | What exists                                                                                                                                                                               | What changes                                                         |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| Rust domain               | `DomainError` enum in `src-tauri/src/core/domain/domain_error.rs`                                                                                                                         | No change                                                            |
| Rust IPC                  | `CommandError` enum in `src-tauri/src/core/infrastructure/error.rs` — variants: `DatabaseError`, `NotFound`, `ValidationError`, `PermissionDenied`, `Unknown`, `BusinessRule`, `Conflict` | Add `error_id: Option<String>` field to `Unknown` variant            |
| Frontend normalization    | `NormalizedError` + `safeInvoke` in `src/lib/services/errors.ts` and `tauri.ts`                                                                                                           | Add `error_id?: string` to `NormalizedError`; propagate when present |
| Frontend error page       | `/src/routes/error/+page.svelte` — minimal, no themed styling                                                                                                                             | Full replacement with Signal Failure view                            |
| Frontend startup boundary | Error state in `/src/routes/+layout.svelte` (lines 98–118)                                                                                                                                | Replace inline error markup with `<SignalFailureView>` component     |
| Toast system              | `svelte-sonner` via `src/lib/toaster.ts` + custom `sonner.svelte`                                                                                                                         | Add Amber-bordered `signal()` method; non-fatal errors use this path |

**Rationale**: Reusing the established `CommandError` → `NormalizedError` pipeline ensures type-safe propagation with minimal surface area. Adding an optional `error_id` field to `Unknown` is backward-compatible.

---

## 2. Error ID Generation Strategy

### Decision: Dual-side generation, Rust-authoritative for backend errors

**Rationale**: Backend errors have richer trace context and should generate their ID at the point of capture. Frontend-only errors (unhandled JS exceptions, Svelte boundary catches) generate their ID client-side using the same format.

**Format**: `ERR-NNNN-X` where NNNN is a 4-digit zero-padded random number (1000–9999) and X is a random uppercase letter (A–Z). This yields 234,000 possible values — sufficient for session-scoped tracing without a database sequence.

**Rust implementation**:

- Use `rand` crate (already a transitive dependency via Tauri) to generate the numeric and letter components.
- Generate ID inside the `Unknown` variant construction in `CommandError`.
- Log the Error ID with `tracing::error!` at the point of `CommandError::Unknown` creation.

**TypeScript implementation**:

- Pure function in `src/lib/services/error-id.ts`
- Uses `Math.random()` — no crypto needed for non-security IDs.
- Called by the Svelte error boundary and any unhandled promise rejections.

**Alternatives considered**:

- UUID v4: Too long for user-facing display; overkill for session tracing.
- Monotonic counter: Requires shared state; not necessary for this use case.
- Backend-only: Would require a round-trip for JS-originated errors; unnecessary coupling.

---

## 3. Module Detection Strategy

### Decision: Derive module name from SvelteKit `$page.url.pathname` at error time

The active sidebar section maps cleanly to the URL path prefix. This avoids prop-drilling or global state:

| URL prefix           | Module label     |
| -------------------- | ---------------- |
| `/dashboard`         | Yard Overview    |
| `/collection`        | Collection Depot |
| `/wishlist`          | Wishlist         |
| `/maintenance`       | Maintenance Log  |
| `/finance`           | Finance Ledger   |
| `/search`            | Global Search    |
| `/settings`          | Settings         |
| `/` (root / unknown) | Signal Box       |

A `getModuleLabel(pathname: string): string` utility derives the label at render time. It is not hardcoded in the error view — the view receives it as a prop.

**Rationale**: No additional state infrastructure needed; SvelteKit's `$page` store is always available. Labels are Paraglide message keys, satisfying the no-hardcoded-strings constraint.

---

## 4. Signal Failure View Architecture

### Decision: Reusable component + route-level page

Create `src/lib/components/signal-failure/SignalFailureView.svelte` as a standalone component that accepts props (`errorId`, `moduleLabel`, `message?`). This component is used in two places:

1. `/src/routes/error/+page.svelte` — full-page error route (used by SvelteKit's built-in error handling and programmatic navigation).
2. `/src/routes/+layout.svelte` — replaces the inline startup error state (lines 98–118).

**Rationale**: A single component prevents visual drift between the two surfaces. Both show identical UI; only the data source differs (route state vs. layout reactive state).

**Icon**: An SVG Railway Signal (danger position — arm horizontal, lamp red circle) rendered inline. Thin-stroke (2px) monochromatic in `text-muted-foreground` (`#808080`). No external icon library required — reduces bundle size and avoids dependency risk.

---

## 5. Toast Notification Strategy for Non-Fatal Errors

### Decision: Add `signal()` method to existing toaster; style via CSS custom properties

The existing `svelte-sonner` integration uses CSS custom properties (`--normal-bg`, `--normal-border`, etc.) already exposed in `sonner.svelte`. Non-fatal signal errors get a dedicated `signal()` toast method in `toaster.ts` that:

- Passes `class: 'toast-signal'` to Sonner
- A CSS rule in `app.css` maps `.toast-signal` to Amber border (`border: 1px solid #D48A42`)
- Message content comes from Paraglide — no hardcoded strings

**Auto-dismiss**: Default Sonner duration (4000ms) is used. The `signal()` method does not override duration, keeping it consistent with other toasts.

**Alternatives considered**:

- Custom toast component: More control but duplicates the Sonner infrastructure already in place.
- New toast library: Unjustified dependency addition for this scope.

---

## 6. Backend Logging Strategy

### Decision: Use `tracing::error!` with structured fields

Tauri 2 includes `tracing` as a dependency. All `CommandError::Unknown` constructions will log:

```
tracing::error!(
    error_id = %id,
    module = %"backend",
    message = %msg,
    "Signal Fault captured"
);
```

**Rationale**: `tracing` is already the project's logging primitive via Tauri. No new dependency. Structured fields allow log aggregation tools to filter by `error_id`.

**Alternatives considered**:

- `log` crate: Less structured; no field-level filtering.
- External error tracker (Sentry etc.): Overkill for a local desktop app; introduces network dependency.

---

## 7. Paraglide Message Keys Required

New keys to add to `messages/en.json`:

| Key                                 | Value                                                    |
| ----------------------------------- | -------------------------------------------------------- |
| `signal_failure_headline`           | `Signal Failure`                                         |
| `signal_failure_subtext`            | `The yard master encountered an unexpected obstruction.` |
| `signal_failure_action_reset`       | `Reset Signal`                                           |
| `signal_failure_action_report`      | `Report to Depot`                                        |
| `signal_failure_label_error_code`   | `ERROR CODE`                                             |
| `signal_failure_label_module`       | `MODULE`                                                 |
| `signal_failure_label_status`       | `STATUS`                                                 |
| `signal_failure_status_value`       | `CRITICAL`                                               |
| `signal_failure_report_copied`      | `Error ID copied to clipboard`                           |
| `signal_failure_report_copy_failed` | `Select and copy the Error ID manually`                  |
| `module_label_yard_overview`        | `Yard Overview`                                          |
| `module_label_collection_depot`     | `Collection Depot`                                       |
| `module_label_wishlist`             | `Wishlist`                                               |
| `module_label_maintenance_log`      | `Maintenance Log`                                        |
| `module_label_finance_ledger`       | `Finance Ledger`                                         |
| `module_label_global_search`        | `Global Search`                                          |
| `module_label_settings`             | `Settings`                                               |
| `module_label_signal_box`           | `Signal Box`                                             |
| `signal_toast_title`                | `Yard Fault`                                             |

---

## 8. Dependencies Assessment

No new npm or Cargo dependencies required. All functionality uses:

- `svelte-sonner` (existing)
- `@inlang/paraglide-js` (existing)
- `tracing` (existing, via Tauri)
- `rand` (existing transitive, or use timestamp-based generation if not available as direct dep)

**Rand availability**: Check `src-tauri/Cargo.toml` — if `rand` is not a direct dependency, use a timestamp-seeded approach (`std::time::SystemTime`) for ID generation rather than adding a dependency. This satisfies the no-new-dependencies constraint from CLAUDE.md.

---

## 9. Constitution Check — Pre-Design

| Principle                                | Compliance                                                                                  |
| ---------------------------------------- | ------------------------------------------------------------------------------------------- |
| Modular, Library-First                   | `SignalFailureView` is a self-contained component; `error-id.ts` is a pure utility module.  |
| Deterministic Interfaces & Observability | Error IDs are logged with structured fields via `tracing`. IPC changes are specta-typed.    |
| Test-First Emphasis                      | `error-id.ts` logic is pure and unit-testable; Rust ID generation is testable in isolation. |
| Code Quality                             | No new linting exceptions introduced; all strings via Paraglide.                            |
| User Experience Consistency              | Paraglide for all strings; design tokens for colors; consistent toast pattern.              |
| Performance                              | Error views are rendered only on failure; no impact on happy-path performance.              |
| Safe Rust                                | No `unwrap()`. Error ID generation uses safe APIs only.                                     |
| No Hardcoded Strings                     | All UI copy is Paraglide message keys (see table above).                                    |
| Tauri IPC / specta                       | `error_id` field addition to `CommandError::Unknown` requires specta type regeneration.     |
| Domain Logic in Rust                     | Error ID assignment for backend errors happens in Rust.                                     |

No architectural violations detected. No complexity tracking entry required.
