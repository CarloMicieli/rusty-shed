# Tasks: Acquisition Flow

**Input**: Design documents from `/specs/038-acquisition-flow/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Organization**: Tasks grouped by user story (US1 → US4 → US2 → US3) so each story is an independently deliverable increment. No test tasks — not requested in spec.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no blocking dependencies)
- **[Story]**: User story label (US1–US4 maps to spec.md stories)

---

## Phase 1: Setup

**Purpose**: Module scaffolding and i18n keys — unblocks all frontend work.

- [ ] T001 Add all 28 `acquisition_*` and `dashboard_action_new_acquisition` Paraglide keys to `messages/en.json`; run `pnpm prepare` to regenerate `src/lib/paraglide/messages.js`
- [ ] T002 Create directory skeleton `src/lib/features/acquisition/components/` with empty placeholder files: `types.ts`, `AcquisitionState.svelte.ts`, `AcquisitionDrawer.svelte`, `components/AcquisitionHeader.svelte`, `components/AcquisitionItemCard.svelte`, `components/AcquisitionFooter.svelte`

---

## Phase 2: Foundational (Backend — Blocking)

**Purpose**: New `record_acquisition` Tauri command and regenerated TypeScript bindings. No user story work can begin until T009 completes.

**⚠️ CRITICAL**: T009 regenerates `src/lib/bindings.ts` — all frontend tasks depend on this.

- [ ] T003 [P] Add `RecordAcquisitionArgs` and `AcquisitionItemArgs` structs (with `Debug, Clone, Validate, specta::Type, Deserialize` derives and `validate_not_future_date` custom validator for `purchase_date`) to `src-tauri/src/collecting/interface/command_args.rs` — see `contracts/rust-command.md`
- [ ] T004 [P] Create `src-tauri/src/collecting/application/record_acquisition.rs` with `RecordAcquisitionInput`, `AcquisitionItemInput` structs, and the `RecordAcquisition` struct stub
- [ ] T005 Implement `RecordAcquisition::execute` in `src-tauri/src/collecting/application/record_acquisition.rs`: for each item derive `RailwayModelId::new(mfr_id, product_code)`, probe `find_by_id`, conditionally call `catalog_repo.create()`, then `collection.add_item()`, commit once — see `contracts/rust-command.md § Use Case`
- [ ] T006 Add `pub mod record_acquisition;` to `src-tauri/src/collecting/application/mod.rs`
- [ ] T007 Add `record_acquisition` async command handler to `src-tauri/src/collecting/interface/command_handlers.rs`: call `args.validate()`, map each item to `AcquisitionItemInput`, invoke `RecordAcquisition::execute`, commit UoW — see `contracts/rust-command.md § Mapping to Use Case Input`
- [ ] T008 Register `collecting::interface::command_handlers::record_acquisition` inside `collect_commands!` in `src-tauri/src/lib.rs`
- [ ] T009 Run `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`; then run `pnpm tauri dev` (start + immediately stop) to regenerate `src/lib/bindings.ts`; confirm `commands.recordAcquisition` is present in the bindings file

**Checkpoint**: `recordAcquisition` is callable from TypeScript. Backend work is complete. Frontend phases can now proceed.

---

## Phase 3: User Story 1 — Record a Single-Item Purchase (Priority: P1) 🎯 MVP

**Goal**: A collector can open the acquisition drawer from the Dashboard, fill in one item with seller and date, finalize, and see it appear in their collection.

**Independent Test**: Open the drawer, complete purchase metadata + one item (manufacturer, product code, category), click "Finalize Purchase" — verify one new collection entry is created and the drawer closes.

- [ ] T010 [P] [US1] Define `AcquisitionFormState`, `AcquisitionItemEntry`, `BatchDefaults`, `AcquisitionItemErrors`, `AcquisitionValidationErrors` interfaces and `createDefaultFormState()` / `createDefaultItem()` factory functions in `src/lib/features/acquisition/types.ts` — see `data-model.md § New Frontend Types`
- [ ] T011 [P] [US1] Implement `AcquisitionState.svelte.ts` in `src/lib/features/acquisition/AcquisitionState.svelte.ts`: thin Svelte 5 class wrapping `commands.recordAcquisition`, providing `setContext` / `getAcquisitionContext()` helpers; show success/error toasts
- [ ] T012 [P] [US1] Build `AcquisitionHeader.svelte` in `src/lib/features/acquisition/components/AcquisitionHeader.svelte`: seller `<select>` (from `sellers: SellerView[]` prop), `DatePickerField` for purchase date (max = today), Scale `<select>`, Power Method `<select>`; emit change events via callback props — see `contracts/frontend-state.md § AcquisitionHeader.svelte`
- [ ] T013 [P] [US1] Build `AcquisitionFooter.svelte` in `src/lib/features/acquisition/components/AcquisitionFooter.svelte`: "Add Another Item" (outline variant) and "Finalize Purchase" (default variant) buttons; disable Finalize when `isSubmitting || isLoadingData`; show "Saving…" text while submitting — see `contracts/frontend-state.md § AcquisitionFooter.svelte`
- [ ] T014 [US1] Build `AcquisitionItemCard.svelte` in `src/lib/features/acquisition/components/AcquisitionItemCard.svelte`: manufacturer `<select>`, product code text input, description text input, category `<select>`, scale `<select>`, epoch `<select>`, power method `<select>`, `CurrencyInput` for price with read-only currency symbol; accept `item: AcquisitionItemEntry`, `manufacturers`, `currency`, `canRemove`, `onUpdate`, `onDuplicate`, `onRemove` props — see `contracts/frontend-state.md § AcquisitionItemCard.svelte`
- [ ] T015 [US1] Build `AcquisitionDrawer.svelte` in `src/lib/features/acquisition/AcquisitionDrawer.svelte`: fixed overlay + right-panel (`max-w-2xl`, `translate-x` animation), `$effect` scroll-lock on `document.body` and `main`, `handleOpen()` loading `getSellers` + `getManufacturers` in parallel, `handleFinalize()` calling `acquisitionService.recordAcquisition()`, `handleCloseRequest()` with `hasChanges` discard-confirm guard, compose `AcquisitionHeader` + scrollable item list + `AcquisitionFooter`; single item only for this story — see `contracts/frontend-state.md § AcquisitionDrawer.svelte`
- [ ] T016 [US1] Update `src/routes/dashboard/+page.svelte`: replace "Add Railway Model" quick-action button with `showAcquisitionDrawer = true` onclick using label from `m.dashboard_action_new_acquisition()`; add `let showAcquisitionDrawer = $state(false)`; mount `<AcquisitionDrawer open={showAcquisitionDrawer} onClose={...} onSuccess={() => { showAcquisitionDrawer = false; /* reload recent acquisitions */ }}>`

**Checkpoint**: Full single-item purchase flow is functional end-to-end. ✅ MVP deliverable.

---

## Phase 4: User Story 4 — Inline Validation (Priority: P2)

**Goal**: Required field errors appear on the specific item card(s) when "Finalize" is clicked; errors clear reactively as the user types; an empty item list shows a top-level warning.

**Independent Test**: Click "Finalize Purchase" with Manufacturer missing → the specific card shows an error ring. Fix it → the error clears without re-submitting.

- [ ] T017 [US4] Add `validateForm(form: AcquisitionFormState): AcquisitionValidationErrors` function to `src/lib/features/acquisition/types.ts`; validate: items array length ≥ 1, each item's `manufacturerId` + `productCode` + `category` non-null/non-empty; return per-card `AcquisitionItemErrors[]`
- [ ] T018 [US4] Wire validation into `AcquisitionDrawer.svelte`: add `let touched = $state(false)` and `let validationErrors = $state<AcquisitionValidationErrors>({})`; add `$derived.by` that re-runs `validateForm` when `touched` is true and writes to `validationErrors`; set `touched = true` at start of `handleFinalize`, block submission if errors exist
- [ ] T019 [US4] Update `AcquisitionItemCard.svelte` in `src/lib/features/acquisition/components/AcquisitionItemCard.svelte`: accept `errors: AcquisitionItemErrors` prop; add `ring-destructive` / `border-destructive` CSS on each field that has an error; show `<p class="text-destructive text-xs">` error text beneath the field
- [ ] T020 [US4] Update `AcquisitionDrawer.svelte` in `src/lib/features/acquisition/AcquisitionDrawer.svelte`: when `validationErrors.general` is set (empty items), show an inline warning banner above the item list; pass `errors={validationErrors.items?.[index] ?? {}}` to each `AcquisitionItemCard`

**Checkpoint**: Validation UX is complete. Finalize is safely guarded. Errors surface and clear reactively.

---

## Phase 5: User Story 2 — Multi-Item Haul with Batch Defaults (Priority: P2)

**Goal**: Collector sets Scale + Power Method once; new items and cloned cards inherit those defaults; per-item override works; removing items works.

**Independent Test**: Set batch defaults, add 3 items via Clone, change scale on one card only — verify the other two retain the original batch default scale; finalize → 3 collection entries created.

- [ ] T021 [US2] Add `handleBatchDefaultChange(field: 'scale' | 'powerMethod', value: string | null)` to `AcquisitionDrawer.svelte`: propagate new value to all items whose current `field` value matches the old default (opt-in propagation); update `form.batchDefaults`; wire to `onBatchDefaultChange` prop on `AcquisitionHeader` in `src/lib/features/acquisition/AcquisitionDrawer.svelte`
- [ ] T022 [US2] Update `AcquisitionHeader.svelte` in `src/lib/features/acquisition/components/AcquisitionHeader.svelte`: accept `batchDefaults: BatchDefaults` and `onBatchDefaultChange` callback props; bind scale and power method dropdowns to the batch defaults and call `onBatchDefaultChange` on change
- [ ] T023 [US2] Add `handleDuplicate(uid: string)` to `AcquisitionDrawer.svelte`: find source entry by uid, shallow-clone all fields, assign `uid = crypto.randomUUID()`, clear `productCode = ''`, insert clone immediately after the source in `form.items` array in `src/lib/features/acquisition/AcquisitionDrawer.svelte`
- [ ] T024 [US2] Add `handleRemove(uid: string)` to `AcquisitionDrawer.svelte`: filter `form.items`; no-op if `form.items.length <= 1`; pass `canRemove={form.items.length > 1}` to each `AcquisitionItemCard` in `src/lib/features/acquisition/AcquisitionDrawer.svelte`
- [ ] T025 [US2] Update `AcquisitionItemCard.svelte` in `src/lib/features/acquisition/components/AcquisitionItemCard.svelte`: add a Copy (`Copy` lucide icon) button that calls `onDuplicate(item.uid)`; add a Trash (`Trash2` lucide icon) button that calls `onRemove(item.uid)`, hidden when `!canRemove`; position both in the top-right of the card
- [ ] T026 [US2] Add auto-scroll-to-last-card behavior in `AcquisitionDrawer.svelte`: after `handleAddItem()` or `handleDuplicate()` pushes to `form.items`, use a `$effect` watching `form.items.length` to scroll the scrollable content div to its bottom via `scrollableEl.scrollTop = scrollableEl.scrollHeight` in `src/lib/features/acquisition/AcquisitionDrawer.svelte`

**Checkpoint**: Multi-item haul with batch defaults and clone is fully functional.

---

## Phase 6: User Story 3 — Global Ctrl+N Shortcut (Priority: P3)

**Goal**: Pressing Ctrl+N from any screen opens the acquisition drawer immediately.

**Independent Test**: Navigate to the catalogue page; press Ctrl+N — acquisition drawer opens without navigating away.

- [ ] T027 [US3] Register `tauri_plugin_global_shortcut` plugin in `src-tauri/src/lib.rs`: add `.plugin(tauri_plugin_global_shortcut::Builder::new().build())` to the Tauri builder chain
- [ ] T028 [US3] Add `"global-shortcut:allow-register"` to `src-tauri/capabilities/default.json` permissions array
- [ ] T029 [US3] Register `"CommandOrControl+N"` shortcut in the `setup` closure in `src-tauri/src/lib.rs`: call `app.global_shortcut().register("CommandOrControl+N", |app, _shortcut, _event| { app.emit("open-acquisition-drawer", ()).ok(); })?`
- [ ] T030 [US3] Lift `AcquisitionDrawer` from `src/routes/dashboard/+page.svelte` to `src/routes/+layout.svelte`: manage `showAcquisitionDrawer = $state(false)` in layout, expose `openAcquisitionDrawer` via `setContext`; add `listen("open-acquisition-drawer", () => showAcquisitionDrawer = true)` in `onMount`; update dashboard page to call `getContext` for the open function instead of owning the state

**Checkpoint**: Ctrl+N opens the acquisition drawer from any route.

---

## Phase 7: Polish & Cross-Cutting

**Purpose**: Final verification pass and spec compliance confirmation.

- [ ] T031 [P] Run `cargo fmt --manifest-path src-tauri/Cargo.toml` and `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` and `cargo test --manifest-path src-tauri/Cargo.toml`; fix any warnings
- [ ] T032 [P] Run `pnpm lint`, `pnpm check`, `pnpm test`; fix any errors or warnings
- [ ] T033 Smoke-test the full acquisition flow per `quickstart.md`: single item, 5-item haul with clone + batch defaults, validation errors, discard confirm, Ctrl+N from catalogue page

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — **blocks all frontend phases**
- **Phase 3 (US1)**: Depends on Phase 2 (bindings must exist)
- **Phase 4 (US4)**: Depends on Phase 3 (edits same files; validation builds on drawer shell)
- **Phase 5 (US2)**: Depends on Phase 3 (adds multi-item behaviors to existing drawer)
- **Phase 6 (US3)**: Depends on Phase 3 (drawer component must exist to be mounted in layout)
- **Phase 7 (Polish)**: Depends on all desired phases complete

### User Story Dependencies

- **US1 (P1)**: Unblocked after Phase 2 — no dependency on other stories
- **US4 (P2)**: Depends on US1 (edits AcquisitionDrawer + AcquisitionItemCard)
- **US2 (P2)**: Depends on US1 (extends same components); can run in parallel with US4 if on different subtasks
- **US3 (P3)**: Depends on US1 (drawer component must exist); otherwise independent

### Within-Phase Parallel Opportunities

**Phase 2** — T003 and T004 are fully parallel (different files):

```
T003  RecordAcquisitionArgs in command_args.rs
T004  RecordAcquisitionInput in record_acquisition.rs
  ↓ (both complete)
T005  RecordAcquisition::execute
```

**Phase 3** — T010, T011, T012, T013 are fully parallel (all different files):

```
T010  types.ts
T011  AcquisitionState.svelte.ts
T012  AcquisitionHeader.svelte
T013  AcquisitionFooter.svelte
  ↓ (all complete)
T014  AcquisitionItemCard.svelte  (needs T010)
  ↓
T015  AcquisitionDrawer.svelte    (needs T010–T014)
  ↓
T016  dashboard/+page.svelte      (needs T015)
```

**Phase 5** — T021–T026 are ordered but T025 (card icon buttons) can be done in parallel with T021–T024 (drawer handlers) since they're different files:

```
T021  handleBatchDefaultChange (drawer)   ─┐
T022  AcquisitionHeader props update      ─┤ parallel
T023  handleDuplicate (drawer)            ─┤
T024  handleRemove (drawer)              ─┘
T025  card icon buttons (card component)  ← parallel with above
  ↓ (all complete)
T026  auto-scroll behavior (drawer)
```

**Phase 6** — T027 and T028 are parallel (different files):

```
T027  Plugin registration in lib.rs    ─┐ parallel
T028  Capability in default.json       ─┘
  ↓
T029  Shortcut registration in lib.rs
  ↓
T030  Layout-level drawer + context
```

**Phase 7** — T031 and T032 are fully parallel:

```
T031  Rust checks     ─┐ parallel
T032  Frontend checks ─┘
  ↓
T033  Smoke test
```

---

## Implementation Strategy

### MVP (User Story 1 Only)

1. Complete Phase 1 (Setup) + Phase 2 (Foundational)
2. Complete Phase 3 (US1: single-item purchase)
3. **STOP and VALIDATE**: open drawer from Dashboard, record one item, confirm it appears in collection
4. Demo-ready: the most common use case works end-to-end

### Incremental Delivery

1. Phase 1 + 2 → backend command live, bindings regenerated
2. - Phase 3 (US1) → MVP: single-item purchase from Dashboard ✅
3. - Phase 4 (US4) → validation UX hardened ✅
4. - Phase 5 (US2) → multi-item haul + batch defaults + clone ✅
5. - Phase 6 (US3) → Ctrl+N from anywhere ✅
6. - Phase 7 → production-ready ✅

---

## Summary

| Phase        | Story                | Tasks  | Parallel            |
| ------------ | -------------------- | ------ | ------------------- |
| 1 Setup      | —                    | 2      | —                   |
| 2 Foundation | —                    | 7      | T003/T004           |
| 3 US1 (P1)   | Single-item purchase | 7      | T010/T011/T012/T013 |
| 4 US4 (P2)   | Inline validation    | 4      | —                   |
| 5 US2 (P2)   | Multi-item + clone   | 6      | T021–T025           |
| 6 US3 (P3)   | Ctrl+N shortcut      | 4      | T027/T028           |
| 7 Polish     | —                    | 3      | T031/T032           |
| **Total**    |                      | **33** |                     |

**MVP scope**: Phases 1–3 (16 tasks) — US1 fully functional.
