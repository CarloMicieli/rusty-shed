# Implementation Plan: The Signal Box — Error Management System

**Branch**: `036-signal-box` | **Date**: 2026-03-06 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/036-signal-box/spec.md`

---

## Summary

Transform all system error experiences in Rusty Shed from generic states into themed "Signal Failures." The implementation extends the existing `CommandError` IPC type with a new `error_id` field, introduces a reusable `SignalFailureView` Svelte component, adds a `signal()` toast method for non-fatal errors, and adds structured backend logging keyed by Error ID. No new dependencies or database migrations are required.

---

## Technical Context

**Language/Version**: Rust (edition 2024, `rust-version = 1.93.0`) + TypeScript 5.9.3 with `strict: true`
**Primary Dependencies**: Tauri 2.9.x, Svelte 5.48.2, svelte-sonner, @inlang/paraglide-js 2.7.1, tracing (via Tauri), specta (via tauri-specta)
**Storage**: N/A — no database changes; error IDs are session-scoped and log-only
**Testing**: Vitest 4.0.18 (frontend) + `cargo test` (Rust)
**Target Platform**: Desktop (Linux/macOS/Windows via Tauri)
**Project Type**: Tauri 2 desktop — dual-layer (Rust backend + SvelteKit frontend)
**Performance Goals**: Signal Failure view renders within 500ms of fault capture; no impact on happy-path performance
**Constraints**: No new npm or Cargo dependencies; all strings via Paraglide; no `unwrap()` in Rust; specta bindings must regenerate cleanly
**Scale/Scope**: Single desktop app; error handling is UI-level, not distributed-systems level

---

## Constitution Check

| Principle                                | Status | Notes                                                                                                       |
| ---------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------- |
| Modular, Library-First                   | PASS   | `SignalFailureView` is a standalone component; `error-id.ts` and `module-label.ts` are pure utility modules |
| Deterministic Interfaces & Observability | PASS   | Error IDs logged via `tracing::error!` with structured fields; IPC contract updated in specta               |
| Test-First Emphasis                      | PASS   | Unit tests required for ID generation (Rust + TS) and module label derivation                               |
| Code Quality                             | PASS   | All strings via Paraglide; Clippy/ESLint compliance required                                                |
| Testing Standards                        | PASS   | Business logic (ID generation, label mapping) isolated and unit-testable                                    |
| User Experience Consistency              | PASS   | Uses design tokens from constitution; Paraglide for all copy; consistent toast pattern                      |
| Performance Requirements                 | PASS   | Error path only; no hot-path impact                                                                         |
| Safe Rust Practices                      | PASS   | No `unwrap()`; `ErrorId::generate()` uses safe APIs; `CommandError::unknown()` factory handles logging      |
| Simplicity & Semantic Versioning         | PASS   | Minimal changes; extends existing patterns rather than replacing them                                       |
| **Architectural Laws**                   |        |                                                                                                             |
| Database (Persistence)                   | N/A    | No persistence changes                                                                                      |
| State Management                         | N/A    | No domain aggregates modified                                                                               |
| API Design / Transport Boundary          | PASS   | `Unknown` variant updated with struct fields; specta regeneration required                                  |
| Domain Logic in Rust                     | PASS   | Error ID generation and logging happen in Rust for backend errors                                           |

**Gate result: PASS — no violations. No Complexity Tracking entry required.**

---

## Project Structure

### Documentation (this feature)

```text
specs/036-signal-box/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── ipc-error-contract.md
└── tasks.md             # Phase 2 output (/speckit.tasks — not yet created)
```

### Source Code Changes

```text
src-tauri/src/core/infrastructure/
└── error.rs                          # UPDATE: Unknown → struct variant; add ErrorId; add ::unknown() factory

src/lib/services/
├── errors.ts                         # UPDATE: add errorId? to NormalizedError; update normalizeError()
├── error-id.ts                       # NEW: generateErrorId() pure utility
└── module-label.ts                   # NEW: getModuleLabel(pathname) → Paraglide string

src/lib/components/signal-failure/
└── SignalFailureView.svelte           # NEW: themed full-page error component

src/lib/toaster.ts                    # UPDATE: add signal() method

src/routes/error/
└── +page.svelte                      # REPLACE: use SignalFailureView with SvelteKit error state

src/routes/
└── +layout.svelte                    # UPDATE: replace inline startup error markup with SignalFailureView

src/app.css                           # UPDATE: add .toast-signal Amber border rule

messages/
└── en.json                           # UPDATE: add signal_* and module_label_* keys

src/__tests__/
├── error-id.test.ts                  # NEW: Vitest unit tests
└── module-label.test.ts              # NEW: Vitest unit tests

src-tauri/src/core/infrastructure/    # UPDATE: Rust unit tests for ErrorId
```

---

## Implementation Phases

### Phase 1 — Backend: Error ID & Logging

**Goal**: Every unhandled Rust error captures a unique ID and logs it.

**Tasks**:

1. Add `ErrorId` value type with `generate()` to `error.rs`
2. Update `Unknown` variant to struct form: `Unknown { message: String, error_id: String }`
3. Add `CommandError::unknown(msg)` factory method with integrated `tracing::error!` logging
4. Update all `From<T>` impls (anyhow, sqlx, etc.) to call `CommandError::unknown()`
5. Add Rust unit tests for ID format and uniqueness

**Verification**: `cargo check`, `cargo test`, `cargo clippy -D warnings`

---

### Phase 2 — Paraglide Messages

**Goal**: All user-facing error copy exists as Paraglide keys before any UI work.

**Tasks**:

1. Add all `signal_*` and `module_label_*` keys to `messages/en.json`
2. Run `pnpm run prepare` to regenerate Paraglide bindings
3. Verify `pnpm check` passes

---

### Phase 3 — Frontend Utilities

**Goal**: Error ID generation and module label derivation are pure, tested utilities.

**Tasks**:

1. Create `src/lib/services/error-id.ts` with `generateErrorId()`
2. Create `src/lib/services/module-label.ts` with `getModuleLabel(pathname)`
3. Update `src/lib/services/errors.ts`: add `errorId?` to `NormalizedError`; update `normalizeError()` to propagate from `Unknown.error_id`
4. Write Vitest unit tests for both utilities

**Verification**: `pnpm test`

---

### Phase 4 — SignalFailureView Component

**Goal**: Reusable, fully styled error component matching the designer spec.

**Tasks**:

1. Create `src/lib/components/signal-failure/SignalFailureView.svelte`
2. Accept props: `errorId: string`, `moduleLabel: string`, `onReset?: () => void`
3. Implement inline SVG railway signal icon (2px stroke, `#808080`)
4. Layout: centered flex column; full-viewport; `bg-[#050505]`
5. Surface panel (`bg-[#0F0F0F]`, `border border-[#1F1F1F]`, `rounded-[8px]`)
6. Three-column metadata footer (`grid grid-cols-3`)
7. "Reset Signal" primary button (Amber styling)
8. "Report to Depot" ghost button with clipboard copy + toast confirmation
9. Ensure no white-flash: root element carries background before hydration

---

### Phase 5 — Wire Error Views

**Goal**: Signal Failure view appears in both consumption points.

**Tasks**:

1. Replace `/src/routes/error/+page.svelte`: use `$page.error`, `generateErrorId()`, `getModuleLabel($page.url.pathname)`
2. Update `/src/routes/+layout.svelte`: replace inline startup error markup with `<SignalFailureView>`

---

### Phase 6 — Toast Signal Method

**Goal**: Non-fatal errors use Amber-bordered toasts.

**Tasks**:

1. Add `signal()` method to `src/lib/toaster.ts`
2. Add `.toast-signal { border: 1px solid #D48A42 !important; }` to global CSS
3. Migrate one existing non-fatal error call site to `toaster.signal()` (proof-of-concept)

---

### Phase 7 — Specta Regeneration & Final Verification

**Goal**: TypeScript bindings match updated Rust types; full workflow passes.

**Tasks**:

1. Run `pnpm tauri dev` to trigger specta regeneration
2. Verify `src/lib/bindings.ts` reflects new `Unknown` variant shape
3. Run full verification sequence:
   ```bash
   cargo fmt
   cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
   cargo test --manifest-path src-tauri/Cargo.toml
   pnpm lint
   pnpm check
   pnpm test
   ```

---

## Risk Register

| Risk                                                           | Likelihood | Impact | Mitigation                                                                      |
| -------------------------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------- |
| `Unknown` variant shape change breaks existing `From<T>` impls | High       | Medium | Fix all impls in Phase 1 before touching frontend                               |
| Specta regeneration fails due to variant change                | Medium     | Medium | Update `normalizeError()` in Phase 3 before triggering regeneration in Phase 7  |
| White flash on Signal Failure render                           | Low        | Low    | Root element carries `bg-[#050505]`; `<svelte:head>` body style as fallback     |
| Clipboard API unavailable                                      | Low        | Low    | Try/catch with fallback toast showing selectable Error ID text                  |
| Paraglide key missing during `pnpm check`                      | Medium     | Low    | Add all keys before writing any Svelte component that uses them (Phase 2 first) |

---

## Definition of Done

- [ ] `cargo test` passes with new Rust unit tests for `ErrorId`
- [ ] `pnpm test` passes with new Vitest tests for `error-id.ts` and `module-label.ts`
- [ ] Signal Failure view renders on `/error` route with all three metadata columns
- [ ] Startup error boundary shows `SignalFailureView` instead of inline markup
- [ ] "Reset Signal" reloads the page/module
- [ ] "Report to Depot" copies Error ID to clipboard with confirmation toast
- [ ] At least one non-fatal call site uses `toaster.signal()`
- [ ] Amber border visible on signal toasts
- [ ] No white flash on Signal Failure render
- [ ] No hardcoded strings — all copy via Paraglide
- [ ] `cargo clippy -D warnings` passes
- [ ] `pnpm lint` + `pnpm check` pass
- [ ] Specta bindings regenerated and TypeScript compiles cleanly
