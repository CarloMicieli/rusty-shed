# Quickstart: Acquisition Flow (038)

**Branch**: `038-acquisition-flow`
**Prerequisites**: Rust 1.93+, pnpm 10+, `pnpm install` done.

---

## Step-by-step Implementation Order

### 1. Backend — New Use Case & Command

1. Create `src-tauri/src/collecting/application/record_acquisition.rs`
   - Implement `RecordAcquisitionInput`, `AcquisitionItemInput`, `RecordAcquisition::execute`.
   - Follow the upsert pattern: derive `RailwayModelId::new(mfr_id, product_code)`, probe `find_by_id`, create if absent.
   - Process all items, then call `collection_repo.save()` once.

2. Add `record_acquisition` to `src-tauri/src/collecting/application/mod.rs`.

3. Add `RecordAcquisitionArgs` + `AcquisitionItemArgs` to
   `src-tauri/src/collecting/interface/command_args.rs`.
   - Derive `Debug, Clone, validator::Validate, specta::Type, serde::Deserialize`.
   - Add `validate_not_future_date` custom validator for `purchase_date`.

4. Add the `record_acquisition` command handler to
   `src-tauri/src/collecting/interface/command_handlers.rs`.
   - Call `args.validate()` first; map each item to `AcquisitionItemInput`.

5. Register the command in `src-tauri/src/lib.rs` inside `collect_commands!`.

6. Verify: `cargo check && cargo clippy && cargo test`

---

### 2. Regenerate Bindings

Run `pnpm tauri dev` briefly to trigger specta export, then stop. This updates `src/lib/bindings.ts`
with the new `recordAcquisition` command and its arg/return types.

Confirm `commands.recordAcquisition` is present in `src/lib/bindings.ts`.

---

### 3. Frontend — Paraglide Keys

Add all keys from `contracts/frontend-state.md § Paraglide Keys Required` to `messages/en.json`
(and other locale files if present). Then run:

```bash
pnpm prepare     # regenerates paraglide JS bindings
pnpm check       # verify no type errors
```

---

### 4. Frontend — Feature Module

Create `src/lib/features/acquisition/`:

1. `types.ts` — `AcquisitionFormState`, `AcquisitionItemEntry`, `BatchDefaults`, validation error types.
2. `AcquisitionState.svelte.ts` — thin service wrapping `commands.recordAcquisition`; provides context.
3. `components/AcquisitionHeader.svelte` — seller select, date picker, batch defaults dropdowns.
4. `components/AcquisitionItemCard.svelte` — per-item form card with all fields + duplicate/remove actions.
5. `components/AcquisitionFooter.svelte` — "Add Another Item" + "Finalize Purchase" buttons.
6. `AcquisitionDrawer.svelte` — root: owns form state, orchestrates open/close/scroll-lock, composes sub-components.

Pattern reference: `src/lib/features/collection/components/AddModelDrawer.svelte`.

---

### 5. Dashboard Integration

In `src/routes/dashboard/+page.svelte`:

1. Add `let showAcquisitionDrawer = $state(false)`.
2. Replace the "Add Railway Model" quick-action button with `showAcquisitionDrawer = true` as the `onclick`.
3. Import and mount `<AcquisitionDrawer>` at the bottom of the template.
4. In `onSuccess`: set `showAcquisitionDrawer = false` and call `dashboard.refresh()` (or equivalent) to reload Recent Acquisitions.

---

### 6. Global Shortcut — REQUIRES APPROVAL FIRST

> **Stop here** and ask the user to approve adding `tauri-plugin-global-shortcut` to Cargo.toml.
> Do not proceed with shortcut implementation until approved.

Once approved:

1. Add `tauri-plugin-global-shortcut = "2"` to `src-tauri/Cargo.toml` `[dependencies]`.
2. Add `"global-shortcut:allow-register"` to `src-tauri/capabilities/default.json`.
3. In `src-tauri/src/lib.rs` builder chain: add `.plugin(tauri_plugin_global_shortcut::Builder::new().build())`.
4. In the `setup` closure, register `"CommandOrControl+N"` → `app.emit("open-acquisition-drawer", ())`.
5. In `src/routes/+layout.svelte`: add `listen("open-acquisition-drawer", ...)` to open the drawer via context/store.
6. Re-run `cargo check`, `pnpm tauri dev`.

---

### 7. Verification Checklist

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
pnpm lint
pnpm check
pnpm test
```

All must pass with zero errors/warnings before committing.

---

## Key File Reference

| Purpose                         | File                                                              |
| ------------------------------- | ----------------------------------------------------------------- |
| New use case                    | `src-tauri/src/collecting/application/record_acquisition.rs`      |
| New command args                | `src-tauri/src/collecting/interface/command_args.rs` (append)     |
| New command handler             | `src-tauri/src/collecting/interface/command_handlers.rs` (append) |
| Command registration            | `src-tauri/src/lib.rs` (`collect_commands!`)                      |
| Auto-generated TS bindings      | `src/lib/bindings.ts` (do not edit manually)                      |
| Paraglide messages              | `messages/en.json`                                                |
| Feature types                   | `src/lib/features/acquisition/types.ts`                           |
| Root drawer component           | `src/lib/features/acquisition/AcquisitionDrawer.svelte`           |
| Dashboard page                  | `src/routes/dashboard/+page.svelte`                               |
| Pattern reference (drawer)      | `src/lib/features/collection/components/AddModelDrawer.svelte`    |
| Pattern reference (price input) | `src/lib/features/collection/components/PurchaseSection.svelte`   |
