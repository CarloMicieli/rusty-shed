# Quickstart: The Signal Box — Error Management System

**Feature**: 036-signal-box
**Date**: 2026-03-06

---

## Prerequisites

- Branch `036-signal-box` is checked out (done by `/speckit.specify`)
- Project builds cleanly: `pnpm tauri dev` runs without errors
- Familiarity with: Svelte 5 Runes, Tauri IPC, `tracing` crate

---

## Key Files to Touch

| File | Change type |
|------|-------------|
| `src-tauri/src/core/infrastructure/error.rs` | Update `Unknown` variant; add `ErrorId::generate()` factory |
| `src/lib/services/errors.ts` | Add `errorId?` to `NormalizedError`; update `normalizeError()` |
| `src/lib/services/error-id.ts` | **New** — pure TS Error ID generator |
| `src/lib/services/module-label.ts` | **New** — pathname → module label utility |
| `src/lib/components/signal-failure/SignalFailureView.svelte` | **New** — themed error component |
| `src/routes/error/+page.svelte` | Replace with `<SignalFailureView>` |
| `src/routes/+layout.svelte` | Replace inline startup error markup with `<SignalFailureView>` |
| `src/lib/toaster.ts` | Add `signal()` method |
| `src/app.css` (or equivalent global CSS) | Add `.toast-signal` Amber border rule |
| `messages/en.json` | Add all `signal_*` and `module_label_*` keys |

---

## Step-by-Step Implementation Order

### Step 1 — Backend: Error ID + Updated `CommandError`

1. In `src-tauri/src/core/infrastructure/error.rs`:
   - Add `ErrorId` struct with `generate()` method (no new crate — use `SystemTime` + `rand` if available, else timestamp-based).
   - Change `Unknown(String)` to `Unknown { message: String, error_id: String }`.
   - Add `CommandError::unknown(msg)` factory that generates ID + logs with `tracing::error!`.
2. Update all `From<T> for CommandError` impls that produce `Unknown` to use `CommandError::unknown(...)`.
3. Run `cargo check` — fix any compilation errors from the variant shape change.

### Step 2 — Frontend: Paraglide Messages

1. Add all keys from the research.md message table to `messages/en.json`.
2. Run `pnpm run prepare` (or let dev server pick up changes) to regenerate Paraglide bindings.

### Step 3 — Frontend: Utilities

1. Create `src/lib/services/error-id.ts` with `generateErrorId()`.
2. Create `src/lib/services/module-label.ts` with `getModuleLabel(pathname)` using the URL prefix map.
3. Update `src/lib/services/errors.ts`: add `errorId?: string` to `NormalizedError`; propagate from `CommandError.Unknown.error_id`.

### Step 4 — Frontend: `SignalFailureView` Component

1. Create `src/lib/components/signal-failure/SignalFailureView.svelte`.
2. Component props: `{ errorId: string, moduleLabel: string, onReset?: () => void }`.
3. Layout: centered flex column, full viewport height.
4. Include the inline SVG railway signal icon (monochromatic, 2px stroke, `#808080`).
5. Use strict color tokens: `#050505` bg, `#0F0F0F` surface, `#D48A42` Amber, `#1F1F1F` borders, `8px` radius.
6. Three-column metadata footer (CSS grid, `grid-cols-3`).
7. "Reset Signal" button (Amber/primary style).
8. "Report to Depot" button (ghost style): copies `errorId` to clipboard via `navigator.clipboard.writeText`; shows success/fallback toast.

### Step 5 — Wire Up Error Views

1. Replace `/src/routes/error/+page.svelte` contents: use `$page.error` (SvelteKit's error object), derive `errorId` via `generateErrorId()`, derive `moduleLabel` from `$page.url.pathname`.
2. In `/src/routes/+layout.svelte`, replace the inline startup error markup (lines 98–118 of the current file) with `<SignalFailureView errorId={...} moduleLabel={...} />`.

### Step 6 — Toast: Signal Method

1. In `src/lib/toaster.ts`, add `signal(title, options?)` that calls `toast(title, { ...options, class: 'toast-signal' })`.
2. In global CSS, add:
   ```css
   .toast-signal {
     border: 1px solid #D48A42 !important;
   }
   ```
3. Migrate one existing non-fatal error call site (e.g., the DCC sync failure) to use `toaster.signal()` as a proof-of-concept.

### Step 7 — Rebuild Specta Bindings

Run `pnpm tauri dev` once to trigger specta type regeneration after the `CommandError` struct change. Verify the generated TypeScript bindings in `src/lib/bindings.ts` reflect the new `unknown` variant shape.

### Step 8 — Tests

**Rust tests** (in `src-tauri/src/core/infrastructure/error.rs` or adjacent `tests/` module):
- `test_error_id_format`: verify `ErrorId::generate()` matches `ERR-[1000-9999]-[A-Z]`.
- `test_error_id_uniqueness`: generate 1000 IDs; assert no duplicates.
- `test_unknown_factory_sets_error_id`: `CommandError::unknown("msg").error_id` is non-empty.

**Vitest tests** (`src/__tests__/error-id.test.ts`):
- `generateErrorId()` returns string matching `/^ERR-\d{4}-[A-Z]$/`.
- Two consecutive calls return different IDs.

**Vitest tests** (`src/__tests__/module-label.test.ts`):
- Each URL prefix returns the correct Paraglide key.
- Unknown pathname returns the fallback label.

### Step 9 — Verification Workflow

```bash
cargo fmt
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
pnpm lint
pnpm check
pnpm test
```

---

## How to Trigger the Signal Failure View Manually (Dev Testing)

During development, trigger the error view by:

1. **Route-level**: Navigate to `/error` directly. The `+page.svelte` will render with a generated Error ID and the module label for the current session.
2. **Layout-level**: In `+layout.svelte`, temporarily set the startup error state to a non-null error object to test the startup boundary path.
3. **Backend error**: From the browser dev console, call `window.__TAURI__.core.invoke('nonexistent_command')` — this will return a `CommandError::Unknown` with an `error_id`.

---

## Design Token Reference (for the component)

| Token | Value | Usage |
|-------|-------|-------|
| Base background | `#050505` | Page/window background |
| Surface | `#0F0F0F` | Cards, metadata panel |
| Amber | `#D48A42` | Headline text, primary button |
| Border | `#1F1F1F` | 1px container borders |
| Border radius | `8px` | All containers |
| Muted | `#808080` | SVG icon, secondary text |
| Monospace font | `font-mono` (Tailwind) | Error Code value |

---

## Common Pitfalls

- **White flash on render**: Ensure the Signal Failure view's root element has `bg-[#050505]` applied before any child hydration. Use `<svelte:head>` to set `<style>body { background: #050505 }</style>` if needed.
- **Specta type mismatch**: After changing `Unknown` to a struct variant, the old TypeScript discriminated union check `error.kind === 'unknown' && typeof error === 'string'` pattern breaks. Update `normalizeError()` first, then regenerate bindings.
- **Clipboard fallback**: `navigator.clipboard.writeText` may throw in non-secure contexts. Always wrap in try/catch and show the fallback toast.
- **Paraglide key not found**: If a key is missing from `messages/en.json`, Paraglide will throw at compile time. Add all keys before running `pnpm check`.
