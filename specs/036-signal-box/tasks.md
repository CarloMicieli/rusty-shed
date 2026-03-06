# Tasks: The Signal Box — Error Management System

**Input**: Design documents from `/specs/036-signal-box/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

---

## Phase 1: Setup

**Purpose**: Confirm starting state before any code changes.

- [ ] T001 Verify branch `036-signal-box` is active and `cargo check` + `pnpm check` both pass from clean state

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core type changes that MUST be complete before any user story view or utility can be built. Every downstream task depends on the updated `CommandError` shape and the new Paraglide keys.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T002 Add `ErrorId` value type with `generate()` method to `src-tauri/src/core/infrastructure/error.rs` — use `std::time::SystemTime` epoch millis + modulo for the numeric segment and index into a `b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"` slice for the letter; no new crate required; implement `Display` returning `"ERR-{n:04}-{c}"`
- [ ] T003 Update `CommandError::Unknown` from tuple variant `Unknown(String)` to struct variant `Unknown { message: String, error_id: String }` in `src-tauri/src/core/infrastructure/error.rs`; add `CommandError::unknown(msg: impl Into<String>) -> Self` factory that calls `ErrorId::generate()`, emits `tracing::error!(error_id = %id, "Signal Fault: {}", msg)`, and constructs the struct
- [ ] T004 Update every `From<T> for CommandError` impl in `src-tauri/src/core/infrastructure/error.rs` that currently produces `CommandError::Unknown(e.to_string())` to instead call `CommandError::unknown(e.to_string())` — covers `From<anyhow::Error>`, `From<sqlx::Error>`, `From<std::io::Error>`, and any others present in the file
- [ ] T005 Run `cargo check --manifest-path src-tauri/Cargo.toml` and fix any compilation errors from the `Unknown` variant shape change across all files that pattern-match or construct it
- [ ] T006 [P] Add all Paraglide message keys to `messages/en.json`: `signal_failure_headline`, `signal_failure_subtext`, `signal_failure_action_reset`, `signal_failure_action_report`, `signal_failure_label_error_code`, `signal_failure_label_module`, `signal_failure_label_status`, `signal_failure_status_value`, `signal_failure_report_copied`, `signal_failure_report_copy_failed`, `signal_toast_title`, `module_label_yard_overview`, `module_label_collection_depot`, `module_label_wishlist`, `module_label_maintenance_log`, `module_label_finance_ledger`, `module_label_global_search`, `module_label_settings`, `module_label_signal_box`; run `pnpm run prepare` to regenerate Paraglide bindings
- [ ] T007 Update `src/lib/services/errors.ts`: add `errorId?: string` field to the `NormalizedError` interface; update `normalizeError()` to extract `error.error_id` when `kind === 'unknown'` and set it on the returned object

**Checkpoint**: `cargo check` clean, `pnpm check` clean, all Paraglide keys available — user story implementation can now begin.

---

## Phase 3: User Story 1 — Fatal Error Signal Failure View (Priority: P1) 🎯 MVP

**Goal**: Every unhandled fatal fault renders the full-page Signal Failure view with themed copy, a unique Error ID in the metadata footer, and functional action buttons.

**Independent Test**: Navigate to `/error` in the running app — the Signal Failure view renders with headline "Signal Failure" in Amber, the railway signal icon in muted grey, a three-column footer showing ERROR CODE / MODULE / STATUS rows, and both action buttons present. Triggering `window.__TAURI__.core.invoke('nonexistent_command')` from the browser console produces a `CommandError::Unknown` with a non-empty `error_id` visible in the Tauri log output.

### Implementation for User Story 1

- [ ] T008 [P] [US1] Create `src/lib/services/error-id.ts` exporting `generateErrorId(): string` — pure function, no imports required; format: `"ERR-" + String(Math.floor(Math.random() * 9000) + 1000) + "-" + String.fromCharCode(65 + Math.floor(Math.random() * 26))` ; also export `isErrorId(value: unknown): value is string` regex guard using `/^ERR-\d{4}-[A-Z]$/`
- [ ] T009 [P] [US1] Create `src/lib/services/module-label.ts` exporting `getModuleLabel(pathname: string): string` — use `startsWith` checks against `/dashboard`, `/collection`, `/wishlist`, `/maintenance`, `/finance`, `/search`, `/settings` to return the corresponding Paraglide function call (`m.module_label_yard_overview()` etc.); default fallback returns `m.module_label_signal_box()`
- [ ] T010 [US1] Create `src/lib/components/signal-failure/SignalFailureView.svelte` — Svelte 5 Runes component accepting props `{ errorId: string, moduleLabel: string, onReset?: () => void }`; root element: `<div class="min-h-screen bg-[#050505] flex items-center justify-center p-8">`; inner surface panel: `bg-[#0F0F0F] border border-[#1F1F1F] rounded-[8px] p-10 max-w-xl w-full flex flex-col items-center gap-8`; include inline SVG railway signal icon (upright post, horizontal stop arm, circle lamp — all paths use `stroke="#808080" stroke-width="2" fill="none"`); headline `<h1>` using `m.signal_failure_headline()` in `text-[#D48A42]`; subtext `<p>` using `m.signal_failure_subtext()` in `text-[#808080]`; three-column footer `<div class="grid grid-cols-3 gap-4 w-full border-t border-[#1F1F1F] pt-6">`; each column: label in uppercase text-xs `text-[#808080]` and value below; ERROR CODE value uses `font-mono text-sm text-white`; MODULE value: `moduleLabel`; STATUS value: `m.signal_failure_status_value()`; action row: primary Amber button `bg-[#D48A42] hover:bg-[#D48A42]/90 text-black` calling `onReset ?? (() => window.location.reload())`; ghost button `border border-[#1F1F1F] text-[#808080] hover:text-white` calling clipboard copy with try/catch — on success show `toaster.success(m.signal_failure_report_copied())`, on failure show `toaster.warning(m.signal_failure_report_copy_failed())`
- [ ] T011 [US1] Replace contents of `src/routes/error/+page.svelte` — import `SignalFailureView`, `generateErrorId`, `getModuleLabel` from their respective modules and `page` from `$app/stores`; derive `errorId` via `generateErrorId()` (called once via `$state`); derive `moduleLabel` via `getModuleLabel($page.url.pathname)`; render `<SignalFailureView {errorId} {moduleLabel} />`; remove all previous content
- [ ] T012 [US1] Update `src/routes/+layout.svelte` — in the startup error boundary block (the `{#if startupError}` branch), replace the inline error markup with `<SignalFailureView errorId={generateErrorId()} moduleLabel={m.module_label_signal_box()} onReset={() => window.location.reload()} />`; import `SignalFailureView` and `generateErrorId` at the top of the script block
- [ ] T013 [US1] Run `pnpm tauri dev` once to trigger specta type regeneration after the `CommandError::Unknown` struct change; verify the generated bindings file (typically `src/lib/bindings.ts` or equivalent) reflects `{ kind: "unknown"; message: string; error_id: string }` for the Unknown variant; fix any resulting TypeScript errors in `normalizeError()` or consuming code

**Checkpoint**: User Story 1 fully functional — Signal Failure view renders at `/error` and in the startup boundary with correct themed styling, Error ID in footer, and working action buttons.

---

## Phase 4: User Story 2 — Non-Fatal Error Toast Notification (Priority: P2)

**Goal**: Non-fatal background faults surface as Amber-bordered toast notifications that do not interrupt the current view, use domain language only, and auto-dismiss.

**Independent Test**: In a running session, trigger any non-fatal error path (e.g., call the migrated error call site). An Amber-bordered toast appears at the configured position, contains domain-language copy (no "server," "HTTP," "error 500"), and auto-dismisses after ~4 seconds. Clicking the × dismisses it immediately. No full-page takeover occurs.

### Implementation for User Story 2

- [ ] T014 [US2] Add `.toast-signal { border: 1px solid #D48A42 !important; border-radius: 8px; }` rule to `src/app.css` (or the equivalent global stylesheet); place it near other Sonner customization rules if present
- [ ] T015 [US2] Add `signal(title: string, options?: { description?: string }): string | number` method to the toaster object in `src/lib/toaster.ts` — implementation: call `toast(title, { ...options, className: 'toast-signal' })` (check whether the existing toaster uses `class` or `className` prop and match it); export the updated toaster
- [ ] T016 [US2] Identify one existing non-fatal error call site in the codebase (e.g., the cloud backup sync failure in `src/` that currently calls `toaster.error()` or similar) and migrate it to `toaster.signal(m.signal_toast_title(), { description: <existing domain message> })`; verify the call uses an existing Paraglide message key for the description (do not hardcode strings)

**Checkpoint**: User Story 2 functional — at least one non-fatal error path shows an Amber-bordered toast without full-page takeover.

---

## Phase 5: User Story 3 — Traceable Error Identification (Priority: P3)

**Goal**: Error IDs are provably unique, match the specified format, and every ID shown in the UI has a corresponding backend log entry — verified through automated tests.

**Independent Test**: Run `cargo test` and confirm `test_error_id_format` and `test_error_id_uniqueness` pass. Run `pnpm test` and confirm `error-id.test.ts` and `module-label.test.ts` pass. Trigger a backend fault and confirm the Tauri log output contains a `Signal Fault` entry with a `error_id` field whose value matches what was displayed in the Signal Failure view.

### Implementation for User Story 3

- [ ] T017 [P] [US3] Add Rust unit tests to `src-tauri/src/core/infrastructure/error.rs` inside a `#[cfg(test)] mod tests` block: `test_error_id_format` — assert `ErrorId::generate().to_string()` matches regex `r"^ERR-\d{4}-[A-Z]$"`; `test_error_id_numeric_range` — generate 100 IDs and assert each numeric segment parses to a value in `1000..=9999`; `test_error_id_uniqueness` — generate 500 IDs, collect into a `HashSet<String>`, assert `set.len() == 500`; `test_unknown_factory_sets_error_id` — call `CommandError::unknown("test")`, match on `Unknown { error_id, .. }`, assert `!error_id.is_empty()`
- [ ] T018 [P] [US3] Create `src/__tests__/error-id.test.ts` with Vitest tests: `generateErrorId() returns string matching ERR-NNNN-X format` — assert `/^ERR-\d{4}-[A-Z]$/.test(generateErrorId())`; `generateErrorId() numeric segment is in range 1000–9999` — parse segment from 100 calls and assert all in range; `two consecutive calls return different IDs` — assert `generateErrorId() !== generateErrorId()` (run 10 times to reduce flakiness); `isErrorId() returns true for valid IDs` and `false for invalid strings`
- [ ] T019 [P] [US3] Create `src/__tests__/module-label.test.ts` with Vitest tests covering all URL prefix mappings: `/dashboard` → yard overview key, `/collection` → collection depot key, `/wishlist` → wishlist key, `/maintenance` → maintenance log key, `/finance` → finance ledger key, `/search` → global search key, `/settings` → settings key, `/unknown-path` → signal box fallback key, empty string → signal box fallback key; mock the Paraglide `m.*` functions to return their key names for assertion

**Checkpoint**: All user stories have automated test coverage for their core logic. US1 + US2 + US3 all independently functional and tested.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final quality pass, formatting, and full verification pipeline.

- [ ] T020 Run `cargo fmt --manifest-path src-tauri/Cargo.toml` and commit any formatting changes separately
- [ ] T021 Run `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` and fix all warnings — common issues: unused imports from old `Unknown(String)` pattern, missing `#[allow(...)]` annotations if needed
- [ ] T022 [P] Run `pnpm lint` and fix any ESLint issues in the new Svelte component and TS utility files
- [ ] T023 [P] Run `pnpm check` (svelte-check) and resolve any TypeScript strict-mode errors in `SignalFailureView.svelte`, `error/+page.svelte`, and `+layout.svelte`
- [ ] T024 Run `cargo test --manifest-path src-tauri/Cargo.toml` and confirm all Rust tests (including new T017 tests) pass
- [ ] T025 Run `pnpm test` and confirm all Vitest tests (including T018, T019) pass with no failures
- [ ] T026 Manual smoke test per quickstart.md: navigate to `/error` route, verify Signal Failure view appearance matches design spec (Amber headline, muted icon, three-column footer, Amber primary button, ghost secondary button); confirm "Reset Signal" reloads and "Report to Depot" copies the Error ID to clipboard with confirmation toast; verify no white flash during render

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all user stories**
- **US1 (Phase 3)**: Depends on Phase 2 — T008 and T009 are parallelizable with each other; T010 depends on T008+T009; T011, T012 depend on T010; T013 depends on T003+T007
- **US2 (Phase 4)**: Depends on Phase 2 only — fully independent of US1
- **US3 (Phase 5)**: Depends on Phase 2 + US1 (T008 must exist for T018; T002–T003 must exist for T017) — T017, T018, T019 all parallelizable
- **Polish (Phase 6)**: Depends on all phases complete

### User Story Dependencies

- **US1 (P1)**: Foundational (Phase 2) complete → no other story dependency
- **US2 (P2)**: Foundational (Phase 2) complete → no other story dependency; can run in parallel with US1
- **US3 (P3)**: Foundational (Phase 2) + US1 utilities (T008) complete → test-only additions, no new production code

### Within Each User Story

- T008 and T009 (US1 utilities) are independent — parallel
- T010 depends on T008 + T009 (uses both utilities in template)
- T011 and T012 depend on T010 (import `SignalFailureView`)
- T013 depends on T003 (Rust change) + T007 (TS normalization) being complete
- T014, T015, T016 (US2) are sequential within the story but independent of US1
- T017, T018, T019 (US3 tests) are all parallelizable

### Parallel Opportunities

```bash
# Phase 2 — run after T002/T003/T004 sequence:
T006 (Paraglide)    ← parallel with T005 (NormalizedError update) once T004 is done

# Phase 3 — US1:
T008 (error-id.ts)  ← parallel with T009 (module-label.ts)
# then:
T010 (SignalFailureView) ← depends on T008 + T009
# then:
T011 (+page.svelte) ← parallel with T012 (+layout.svelte) once T010 done

# Phase 4 — US2 (fully parallel with Phase 3):
T014 (CSS) ← parallel with T015 (toaster method)
# then:
T016 (migrate call site) ← depends on T014 + T015

# Phase 5 — US3:
T017 (Rust tests) ← parallel with T018 (TS error-id tests) and T019 (module-label tests)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 2: Foundational (T002–T007) — required, blocks everything
3. Complete Phase 3: User Story 1 (T008–T013)
4. **STOP and VALIDATE**: Navigate to `/error`; verify Signal Failure view renders correctly
5. Demonstrate: themed error view with unique Error ID and working action buttons

### Incremental Delivery

1. **Foundation** (Phase 2) → shared types and messages ready
2. **+US1** (Phase 3) → full-page Signal Failure view with Error IDs
3. **+US2** (Phase 4) → non-fatal toasts with Amber border (add value, zero regression risk)
4. **+US3** (Phase 5) → automated test coverage confirming traceability
5. **Polish** (Phase 6) → final verification suite passes

---

## Notes

- [P] = different files, no dependencies on incomplete tasks in same phase — safe to parallelize
- [USn] label maps each task to its user story for traceability and independent delivery
- T013 (specta regeneration) is a one-time step after changing the Rust `Unknown` variant — it cannot be skipped or deferred
- All Paraglide keys MUST be added (T006) before any Svelte component references them, or `pnpm check` will fail
- The `CommandError::unknown()` factory (T003) must be the only path that constructs `Unknown { ... }` — direct struct construction in non-factory code should not exist after T004
- Clipboard API availability: `SignalFailureView` must use try/catch around `navigator.clipboard.writeText` (handled in T010)
