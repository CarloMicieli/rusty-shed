# Tasks: Wishlist Item Sidebar Inline Editing

**Input**: Design documents from `/specs/037-wishlist-inline-edit/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no blocking dependencies)
- **[Story]**: Which user story this task belongs to
- All file paths are relative to the repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add missing UI components required by the date-picker field before any other work begins.

- [ ] T001 Add shadcn-svelte calendar component via `pnpm dlx shadcn-svelte@latest add calendar` — creates `src/lib/components/ui/calendar/`
- [ ] T002 [P] Add shadcn-svelte popover component via `pnpm dlx shadcn-svelte@latest add popover` — creates `src/lib/components/ui/popover/`

**Checkpoint**: `src/lib/components/ui/calendar/` and `src/lib/components/ui/popover/` exist.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Backend domain event, use case, transport layer, TypeScript bindings, and shared sidebar scaffold — MUST be 100% complete before any user story UI work begins.

**⚠️ CRITICAL**: No user story implementation can start until T011 (bindings sync) is confirmed.

- [ ] T003 Add `WishlistEvent::ItemUpdated` variant (with `item_id`, `priority`, `status`, `desired_price`, `added_date` fields) to `src-tauri/src/wishlist/domain/wishlist_event.rs`
- [ ] T004 Add `Wishlist::update_item()` aggregate method and corresponding `apply_event` arm to `src-tauri/src/wishlist/domain/wishlist.rs`
- [ ] T005 [P] Add `UpdateWishlistItemInput` struct to `src-tauri/src/wishlist/application/inputs.rs`
- [ ] T006 [P] Add `UpdateWishlistItemArgs` struct (deriving `specta::Type`, `Validate`, `Deserialize`) to `src-tauri/src/wishlist/interface/command_args.rs` — implement a `deserialize_double_option` serde helper so that an absent JSON key on `desiredPriceAmount` maps to `None` (unchanged) and an explicit `null` maps to `Some(None)` (clear price); add unit tests confirming both cases (see data-model.md §Serde Implementation Note)
- [ ] T007 Create `src-tauri/src/wishlist/application/update_wishlist_item.rs` use case (loads wishlist, calls `update_item`, saves, returns updated `WishlistItem`)
- [ ] T008 [P] Handle `WishlistEvent::ItemUpdated` in `src-tauri/src/wishlist/infrastructure/repository.rs` with a targeted `UPDATE wishlist_items SET ... WHERE id = ?`
- [ ] T009 Add `update_wishlist_item` Tauri command handler to `src-tauri/src/wishlist/interface/command_handlers.rs` — call `args.validate()?` at the handler entry point before mapping to `UpdateWishlistItemInput` (ADR 8 transport-boundary validation rule)
- [ ] T010 Register `update_wishlist_item` in `collect_commands!` macro and specta export in `src-tauri/src/lib.rs`
- [ ] T011 Sync TypeScript bindings: run `pnpm tauri dev`, confirm `updateWishlistItem` and `UpdateWishlistItemArgs` appear in `src/lib/bindings.ts`
- [ ] T012 [P] Add Rust unit tests for `Wishlist::update_item()` (valid update, all-null input error, future date rejection) in `src-tauri/src/wishlist/domain/wishlist.rs` test module
- [ ] T013 [P] Add Rust integration test for `ItemUpdated` repository event processing (persists changed columns, leaves unchanged columns intact) in `src-tauri/src/wishlist/`
- [ ] T014 Add `activeField` rune, `defaultCurrency` prop, `onUpdate` callback prop, and `saveField()` optimistic-update helper (with rollback on error + `toaster.error`) to `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`

**Checkpoint**: `pnpm run rust:test` passes; `src/lib/bindings.ts` exports `updateWishlistItem`; sidebar component compiles with new props.

---

## Phase 3: User Story 1 — Edit Priority via Dropdown (Priority: P1) 🎯 MVP

**Goal**: Users can hover the Priority badge, click to open a dropdown, select a new priority, and see it persisted immediately.

**Independent Test**: Hover Priority → click → dropdown visible with LOW/NORMAL/HIGH → select → badge updates; Escape → no change.

- [ ] T015 [US1] Implement hover affordance (translucent highlight + `Pencil` icon) and Priority inline `Select` (click-to-activate, auto-focus, current value pre-selected, `onChange` triggers `saveField`, Escape cancels) in `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
- [ ] T016 [US1] Add Vitest tests for Priority inline edit in `src/__tests__/features/wishlists/WishlistItemSidebar.test.ts`:
  - activation on click; all three options (LOW/NORMAL/HIGH) present; selection calls `updateWishlistItem` and badge updates; Escape cancels without calling the command
  - **cross-field / FR-018**: while Priority is active, click the Status field → Priority closes without saving, Status activates; verify `activeField` transitions correctly (SC-006)
  - **failure path / FR-019**: mock `invoke` to reject → Priority value reverts to previous and `toaster.error` is called (SC-007)

**Checkpoint**: Priority field is fully editable; all US1 acceptance scenarios pass in Vitest.

---

## Phase 4: User Story 2 — Edit Status via Dropdown (Priority: P1)

**Goal**: Users can click the Status badge, select from WANTED / ON_ORDER / PURCHASED / IGNORED, and have the change persisted.

**Independent Test**: Click Status → dropdown shows all four values → select → badge updates; Escape → no change.

- [ ] T017 [US2] Implement hover affordance and Status inline `Select` (same pattern as Priority: click-activate, auto-focus, pre-select, `onChange` saves, Escape cancels) in `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
- [ ] T018 [P] [US2] Add Vitest tests for Status inline edit in `src/__tests__/features/wishlists/WishlistItemSidebar.test.ts`:
  - activation; all four options (WANTED/ON_ORDER/PURCHASED/IGNORED) present; selection saves; Escape cancels
  - **failure path / FR-019**: mock `invoke` to reject → Status value reverts and `toaster.error` is called

**Checkpoint**: Status field is fully editable; all US2 acceptance scenarios pass in Vitest.

---

## Phase 5: User Story 3 — Edit Desired Price via Input (Priority: P2)

**Goal**: Users can click the Desired Price field (or "Not set" placeholder), enter a decimal amount, confirm to persist, or clear to remove the price.

**Independent Test**: Click "Not set" → numeric input appears with currency label (from settings) → enter valid amount → confirm → formatted price shown; enter non-numeric → inline error; clear → confirm → "Not set"; Escape → no change.

- [ ] T019 [US3] Implement hover affordance and numeric `Input` component with settings-currency adornment for the Desired Price row (click-activate, existing amount pre-selected, auto-focus) in `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
- [ ] T020 [US3] Add price validation logic: reject non-numeric and negative values with inline error message; allow clear (empty string = remove price); on Enter/blur trigger `saveField` converting to cents; on Escape cancel in `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
- [ ] T021 [P] [US3] Add Vitest tests for Desired Price inline edit in `src/__tests__/features/wishlists/WishlistItemSidebar.test.ts`:
  - valid entry saves; non-numeric input rejected inline; negative value rejected inline; clearing input confirms as null ("Not set"); Escape cancels
  - **failure path / FR-019**: mock `invoke` to reject → price field reverts to previous value and `toaster.error` is called

**Checkpoint**: Desired Price field is fully editable with validation; all US3 acceptance scenarios pass.

---

## Phase 6: User Story 4 — Edit Added Date via Calendar (Priority: P2)

**Goal**: Users can click the Added date, select any past or current date from a calendar picker, and have the change persisted. Future dates are unselectable.

**Independent Test**: Click date → popover calendar opens with current date highlighted → future dates disabled → select past date → field updates → Escape → no change.

- [ ] T022 [US4] Implement hover affordance and `Popover` + `Calendar` date-picker for the Added date row with `maxDate = today` constraint, pre-selecting the stored date, auto-closing on selection and triggering `saveField`, Escape closes popover without saving in `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
- [ ] T023 [P] [US4] Add Vitest tests for Added Date calendar in `src/__tests__/features/wishlists/WishlistItemSidebar.test.ts`:
  - popover opens on click; future dates disabled; past date selection saves; Escape cancels
  - **failure path / FR-019**: mock `invoke` to reject → date field reverts to previous value and `toaster.error` is called

**Checkpoint**: Added date field is fully editable with past-date constraint; all US4 acceptance scenarios pass.

---

## Phase 7: User Story 5 — List Field Remains Read-Only (Priority: P3)

**Goal**: The List (wishlist name) field has no hover affordance, no `activeField` assignment, and no click handler — it is permanently read-only.

**Independent Test**: Hover List field → no highlight, no pencil icon; click → `activeField` remains `null`.

- [ ] T024 [P] [US5] Confirm List field `<dd>` has no group-hover, no cursor-pointer, no `onclick`, and no `activeField` mutation in `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
- [ ] T025 [P] [US5] Add Vitest assertions: hovering/clicking the List field does not set `activeField`, does not render any input, in `src/__tests__/features/wishlists/WishlistItemSidebar.test.ts`

**Checkpoint**: List field has zero editable behaviour confirmed by tests.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: i18n, parent prop wiring, and full verification pass.

- [ ] T026 [P] Add inline-edit i18n message keys to `messages/en.json` and `messages/it.json`; run `pnpm prepare` to regenerate `src/lib/paraglide/`
  - Required keys (follow existing `snake_case` convention in `messages/en.json`):
    - `wishlist_item_price_invalid_format` — `"Price must be a number"` (inline error for non-numeric input)
    - `wishlist_item_price_negative` — `"Price must be zero or greater"` (inline error for negative input)
    - `wishlist_item_price_not_set` — `"Not set"` (placeholder when desired price is null)
    - `wishlist_item_edit_field_label` — `"Edit {field}"` (parameterised `aria-label` for each edit trigger; `field: string`)
    - `wishlist_item_edit_cancel_label` — `"Cancel editing"` (a11y label for Escape/close affordance)
- [ ] T027 [P] Thread `defaultCurrency` prop from `SettingsState.settings.currency` down to `WishlistItemSidebar` in `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`
- [ ] T028 Run full verification sequence: `pnpm format && pnpm lint && pnpm check && pnpm test && pnpm run rust:clippy && pnpm run rust:test` — resolve every error and warning to zero before committing

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately; T001 and T002 are parallel.
- **Foundational (Phase 2)**: Depends on Phase 1. T003 must precede T004, T007, T008. T005 and T006 are parallel with T003/T004. T007 depends on T003+T004+T005. T008 parallel with T005/T006 after T003. T009 depends on T007. T010 depends on T009. T011 depends on T010. T012 and T013 are parallel after T004 and T008 respectively. T014 depends on T011.
- **User Stories (Phases 3–7)**: All depend on T014 (Foundational complete). Each story modifies the single component file so they must be implemented sequentially to avoid file conflicts.
- **Polish (Phase 8)**: T026 and T027 can start any time after T011; T028 must be last.

### User Story Dependencies

- **US1 (P1)**: Foundational complete → start immediately — no dependency on other stories.
- **US2 (P1)**: Depends on US1 complete (same component file); can be tested independently.
- **US3 (P2)**: Depends on US2 complete (same component file); independently testable.
- **US4 (P2)**: Depends on US3 complete (same component file); independently testable.
- **US5 (P3)**: Depends on US4 complete (same component file); introduces no new UI.

### Parallel Opportunities Within Phases

- **Phase 1**: T001 ‖ T002
- **Phase 2**: { T003 → T004 } ‖ T005 ‖ T006; T008 ‖ T005/T006 (after T003); T012 ‖ T013
- **Phase 3**: T016 (test file) can begin while T015 (component) is being reviewed
- **Phase 4**: T018 (test file) ‖ T017 review
- **Phase 5**: T021 (test file) ‖ T020 review
- **Phase 6**: T023 (test file) ‖ T022 review
- **Phase 7**: T024 ‖ T025
- **Phase 8**: T026 ‖ T027

---

## Parallel Example: Phase 2 Backend

```bash
# Stream 1 — Domain core
T003  # Add WishlistEvent::ItemUpdated in wishlist_event.rs
T004  # Add Wishlist::update_item() in wishlist.rs

# Stream 2 — Application / transport layer (in parallel with Stream 1)
T005  # Add UpdateWishlistItemInput in inputs.rs
T006  # Add UpdateWishlistItemArgs in command_args.rs

# Stream 3 — Repository (in parallel after T003)
T008  # Handle ItemUpdated event in repository.rs

# After all streams converge:
T007  # Create update_wishlist_item.rs use case
T009  # Add command handler in command_handlers.rs
T010  # Register in lib.rs
T011  # Sync bindings
```

---

## Implementation Strategy

### MVP Scope (User Stories 1 + 2 only)

1. Complete Phase 1: Setup (T001–T002)
2. Complete Phase 2: Foundational (T003–T014)
3. Complete Phase 3: US1 Priority dropdown (T015–T016)
4. **VALIDATE**: Priority inline edit is fully functional
5. Complete Phase 4: US2 Status dropdown (T017–T018)
6. **VALIDATE + DEMO**: Both P1 stories working — ship as MVP
7. Continue with US3, US4, US5, Polish

### Full Delivery Order

Phase 1 → Phase 2 → US1 → US2 → US3 → US4 → US5 → Polish

### Task Summary

| Phase              | Tasks         | Notes                         |
| ------------------ | ------------- | ----------------------------- |
| Setup              | T001–T002     | 2 tasks                       |
| Foundational       | T003–T014     | 12 tasks — blocks all stories |
| US1 Priority       | T015–T016     | 2 tasks (P1)                  |
| US2 Status         | T017–T018     | 2 tasks (P1)                  |
| US3 Desired Price  | T019–T021     | 3 tasks (P2)                  |
| US4 Added Date     | T022–T023     | 2 tasks (P2)                  |
| US5 List Read-Only | T024–T025     | 2 tasks (P3)                  |
| Polish             | T026–T028     | 3 tasks                       |
| **Total**          | **T001–T028** | **28 tasks**                  |
