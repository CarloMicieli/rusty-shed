# Tasks: Maintenance Page Overhaul

**Input**: Design documents from `specs/034-maintenance-overhaul/`
**Branch**: `034-maintenance-overhaul`
**Total Tasks**: 24

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no shared dependencies)
- **[Story]**: Which user story this task belongs to (US1–US4)
- Exact file paths included in every task description

---

## Phase 1: Setup (Shared Backend Infrastructure)

**Purpose**: Create new Rust artifacts that all subsequent phases depend on. These 4 tasks touch different files and can all run in parallel.

- [X] T001 [P] Create migration file `src-tauri/migrations/0016_maintenance_card_unique_stock_id.sql` with `CREATE UNIQUE INDEX IF NOT EXISTS idx_maintenance_cards_owned_rolling_stock_id ON maintenance_cards (owned_rolling_stock_id);`
- [X] T002 [P] Add `RollingStockDisplayInfo` struct (4 `Option<String>` fields: `manufacturer_name`, `product_code`, `series_code`, `road_number`) and add `pub display_info: Option<RollingStockDisplayInfo>` field to `MaintenanceCardView` in `src-tauri/src/maintenance/interface/views.rs` — both derive `Debug, Clone, Serialize, specta::Type` with `#[serde(rename_all = "camelCase")]`
- [X] T003 [P] Add `get_maintenance_card` command handler to `src-tauri/src/maintenance/interface/command_handlers.rs` (accepts `card_id: MaintenanceCardId`, calls `repo.find_view_by_id`, returns `Result<Option<MaintenanceCardView>, CommandError>`); export it from `src-tauri/src/maintenance/interface/mod.rs`; register it in `collect_commands!` macro in `src-tauri/src/lib.rs` alongside the other maintenance commands
- [X] T004 [P] Add Paraglide message key `maintenance_card_already_exists` with value `"A maintenance card already exists for this locomotive."` to `messages/en.json` and `messages/it.json` (use same pattern as existing `maintenance_create_card_success` key at line ~554 in en.json)

---

## Phase 2: Foundational (Sync TypeScript Bindings)

**Purpose**: Run specta code generation so all frontend tasks can use the correct generated types. **BLOCKS all frontend work.**

**⚠️ CRITICAL**: Tasks T011, T016, T017, T018, T019, T020 cannot start until this phase is complete.

- [X] T005 Run `pnpm tauri dev` to regenerate `src/lib/bindings.ts` from the updated Rust types (T002, T003 must be complete first); confirm `getMaintenanceCard`, `RollingStockDisplayInfo`, and the `displayInfo` field on `MaintenanceCardView` appear in `src/lib/bindings.ts`

**Checkpoint**: Bindings regenerated — frontend tasks can now begin

---

## Phase 3: User Story 1 — Singleton Enforcement (Priority: P1) 🎯 MVP

**Goal**: Prevent creation of duplicate maintenance cards for the same rolling stock at the database level. Surface a clear error in the UI when a duplicate is attempted.

**Independent Test**: Navigate to Maintenance → open Add Card modal → select a rolling stock that already has a card → submit → confirm an error message is shown and no second card is created.

> Note: T006, T007, and T008 all modify `sqlite_repository.rs` or its test module — sequence them to avoid conflicts.

- [X] T006 [US1] In `src-tauri/src/maintenance/infrastructure/sqlite_repository.rs` `save()` method (inside the `MaintenanceCardEvent::Created` arm), wrap the `insert_card_sql` execute call to detect `UNIQUE constraint failed` in the SQLite error string and return `DomainError::Conflict("A maintenance card already exists for this rolling stock.".to_string())` instead of a generic infra error
- [X] T007 [US1] Add `sqlx::test` integration test `repo_prevents_duplicate_card_for_same_stock` to the test module in `src-tauri/src/maintenance/infrastructure/sqlite_repository.rs`: insert a card for a rolling stock via fixture, then call `repo.save()` with a `Created` event for the same `owned_rolling_stock_id`, assert the result is `Err(DomainError::Conflict(...))`
- [X] T008 [US1] Update `src/lib/features/maintenance/components/AddMaintenanceCardModal.svelte`: in the `catch` block of the submit handler, check if the error message contains `"already exists"` (or matches the conflict pattern) and display `m.maintenance_card_already_exists()` as the form error instead of the raw error message

**Checkpoint**: US1 complete — duplicate card creation is blocked end-to-end. Run `pnpm run rust:test` to confirm.

---

## Phase 4: User Story 2 — Human-Readable Card Identification (Priority: P2)

**Goal**: Replace GUID strings on maintenance cards with structured identity: manufacturer + product code as the title, series code as a secondary label, road number as a pill badge.

**Independent Test**: Open Maintenance page → confirm each card shows readable manufacturer and product code (not a raw TRN), series code below it (when present), and road number pill badge in top-right (when present).

> Note: T009 and T010 modify `sqlite_repository.rs` — do T009 first, then T010.

- [X] T009 [US2] In `src-tauri/src/maintenance/infrastructure/sqlite_repository.rs` `list_due_card_views()`: replace the current single-table SELECT with a query that LEFT JOINs `owned_rolling_stocks`, `rolling_stocks`, `railway_models`, and `manufacturers` to pull `mfr.name AS manufacturer_name`, `rm.product_code AS product_code`, `rs.series_code AS series_code`, `rs.road_number AS road_number`; map these 4 columns into a `RollingStockDisplayInfo` and set `display_info: Some(info)` on the view (or `None` if all 4 are null)
- [X] T010 [P] [US2] Add `sqlx::test` integration test `repo_list_due_card_views_includes_display_info` to `sqlite_repository.rs` test module: using a fixture that links a maintenance card to a rolling stock with catalog data, call `list_due_card_views()` and assert the returned view has a non-null `display_info` with the expected `manufacturer_name` and `product_code` values
- [X] T011 [US2] Update `src/lib/features/maintenance/components/MaintenanceCardItem.svelte`: replace `{card.ownedRollingStockId}` in the `<h3>` with `{card.displayInfo?.manufacturerName ?? '—'} {card.displayInfo?.productCode ?? ''}`; add a secondary `<div>` below showing `{card.displayInfo?.seriesCode}` in muted gray uppercase (render only when non-null); add a top-right pill badge showing `{card.displayInfo?.roadNumber}` in font-mono with amber-tinted styling (render only when non-null, positioned absolute top-right or in the existing badge slot)

**Checkpoint**: US2 complete — maintenance card grid shows readable identity. Run `pnpm check` to confirm type safety.

---

## Phase 5: User Story 3 — Functional Add Event in Detail View (Priority: P2)

**Goal**: Create the maintenance card detail page with a working, context-aware Add Event flow. Events appear in the timeline immediately after saving.

**Independent Test**: Click any maintenance card → detail page loads with card header, event timeline, and "Add Event" button; click Add Event → fill in date + type + notes → save → new event appears at top of timeline without page reload; empty-state wrench icon shows for cards with no events.

> T012 modifies `sqlite_repository.rs` — do after T009 to avoid file conflicts. T013 adds to the test module — do after T010. T014 and T015 are independent of each other and can run in parallel. T016 depends on T005 (bindings). T017 depends on T014, T015, T016.

- [X] T012 [US3] In `src-tauri/src/maintenance/infrastructure/sqlite_repository.rs` `find_view_by_id()`: apply the same 4-table LEFT JOIN (owned_rolling_stocks → rolling_stocks → railway_models → manufacturers) as T009 to enrich the returned `MaintenanceCardView` with `display_info`
- [X] T013 [P] [US3] Add `sqlx::test` integration test `repo_find_view_by_id_includes_display_info` to `sqlite_repository.rs` test module: using a fixture with catalog-linked rolling stock, call `find_view_by_id()` for a known card ID and assert the view's `display_info` has the expected values
- [X] T014 [P] [US3] Create `src/lib/features/maintenance/MaintenanceDetailState.svelte.ts`: Svelte 5 Runes class with `#card: MaintenanceCardView | null`, `#isLoading: boolean`, `#error: string | null` reactive fields; `loadCard(id: string): Promise<void>` calls `commands.getMaintenanceCard`; `addEvent(args: AddMaintenanceArgs): Promise<void>` calls `commands.addMaintenanceEvent` then optimistically prepends a new `MaintenanceCardEventView` (constructed from `args`) to `#card.events` before awaiting completion; `setMaintenanceDetailState` / `getMaintenanceDetailState` context helpers
- [X] T015 [P] [US3] Create `src/lib/features/maintenance/components/MaintenanceEventTimeline.svelte`: props `events: MaintenanceCardEventView[]`; when `events.length === 0` render centered `<Wrench>` icon (monochromatic, lucide-svelte) + `"No events logged yet."` text; when non-empty render a vertical list of event cards (charcoal `bg-[#0c0c0c]`, `border border-[#1F1F1F]`) each showing `date_performed` in `font-mono`, `maintenance_type` as a small muted badge (when present), and `notes` in small text (when present)
- [X] T016 [US3] Create `src/lib/features/maintenance/components/AddEventModal.svelte`: props `open: boolean`, `onClose: () => void`, `maintenanceCardId: string`; no card-selection field; form fields: Date Performed (`<input type="date">` defaulting to today's ISO date), Maintenance Type (`<select>` reusing the same `maintenanceTypes` array from the existing `AddMaintenanceEventModal`), Notes (`<textarea rows={3}>`); date field is required — show inline error if empty on submit; on successful submit call `maintenanceDetailState.addEvent({ id: crypto.randomUUID(), maintenanceCardId, datePerformed, maintenanceType, notes: notes.trim() || null })`; use same visual style (amber focus rings, zinc-950 inputs) as existing modals
- [X] T017 [US3] Create `src/routes/maintenance/[id]/+page.svelte`: initialize `MaintenanceDetailState` via `setMaintenanceDetailState`; on mount call `maintenanceDetailState.loadCard($page.params.id)`; render: (1) top-left Back `<a href="/maintenance">` with left-arrow icon, (2) amber header showing `{card.displayInfo?.manufacturerName} {card.displayInfo?.productCode}` bold + `{card.displayInfo?.seriesCode}` muted uppercase + road-number pill badge, (3) 3-column stats row (Last Serviced, Next Due, Total Events — all with muted labels + mono values), (4) amber "Add Event" `<Button>` that sets `showAddEventModal = true`, (5) `<MaintenanceEventTimeline events={card.events} />`, (6) `<AddEventModal bind:open={showAddEventModal} maintenanceCardId={card.id} onClose={...} />`; handle loading skeleton, error banner (with retry), and `null` card (redirect to `/maintenance`)
- [X] T018 [P] [US3] Add Vitest test `MaintenanceDetailState: addEvent optimistically prepends event` in `src/__tests__/state/MaintenanceDetailState.svelte.test.ts`: mock `commands.addMaintenanceEvent` to return a pending promise; call `state.addEvent(...)` without awaiting; assert `state.card.events.length` has already incremented by 1 before the mock resolves
- [X] T019 [US3] In `src/routes/maintenance/+page.svelte`: remove `showAddEventModal` state variable, `handleAddEvent()` function, the "Add Event" `<Button>` from the `{#snippet actions()}` block, the `<AddMaintenanceEventModal>` conditional render, and the `AddMaintenanceEventModal` import

**Checkpoint**: US3 complete — detail page works end-to-end with context-aware event logging. Run `pnpm check && pnpm test` to confirm.

---

## Phase 6: User Story 4 — Active Navigation & Breadcrumb (Priority: P3)

**Goal**: The Maintenance sidebar item stays highlighted when inside a card detail page.

**Independent Test**: Click any maintenance card → confirm the "Maintenance" sidebar nav item shows amber background + left border highlight. Click Back button → confirm return to maintenance grid.

> Note: The Back button is already implemented in T017. This phase is just the sidebar fix.

- [X] T020 [US4] In `src/lib/features/navigation/components/SidebarNavigation.svelte` line ~113: change the Maintenance link active check from `($page.url.pathname as string) === '/maintenance'` to `($page.url.pathname as string).startsWith('/maintenance')` — same pattern already used by the `/railway-tracks` link in the same file

**Checkpoint**: US4 complete — sidebar active state works on all maintenance sub-routes.

---

## Phase 7: Polish & Verification

**Purpose**: Final quality checks across all user stories.

- [X] T021 [P] Run `pnpm run rust:test` and confirm all backend tests pass (including T007, T010, T013 new tests)
- [X] T022 [P] Run `cargo clippy --manifest-path src-tauri/Cargo.toml` and fix any warnings (`-D warnings` in CI)
- [X] T023 [P] Run `pnpm lint && pnpm check && pnpm test` and confirm all frontend checks pass clean
- [X] T024 Execute the manual smoke test checklist from `specs/034-maintenance-overhaul/quickstart.md`: verify card grid shows manufacturer names, detail page loads, Add Event works, duplicate card blocked with error, sidebar stays active in detail view

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (T001–T004): No dependencies — start immediately, all parallel
         ↓
Phase 2 (T005): Depends on T002 + T003 — BLOCKS all frontend tasks
         ↓
Phase 3 (T006–T008): T001 can unblock at any time; T008 requires T004+T005
Phase 4 (T009–T011): T009 must follow T006 (same file); T011 requires T005
Phase 5 (T012–T019): T012 must follow T009 (same file); frontend requires T005
Phase 6 (T020): Independent of T005 — can run any time after Phase 1
Phase 7 (T021–T024): All story phases complete
```

### User Story Dependencies

- **US1 (P1)**: Depends on Phase 1 (T001, T004). Can start immediately.
- **US2 (P2)**: Depends on Phase 1 (T002) + Phase 2 (T005) for frontend.
- **US3 (P2)**: Depends on Phase 1 (T002, T003) + Phase 2 (T005). T012 must follow T009.
- **US4 (P3)**: Depends on nothing — can start after T001.

### Sequencing Within sqlite_repository.rs (same file)

To avoid edit conflicts, apply changes to `sqlite_repository.rs` in this order:

1. T006 — `save()` UNIQUE error mapping
2. T009 — `list_due_card_views()` JOIN
3. T012 — `find_view_by_id()` JOIN

And integration tests in the same file:

1. T007 — duplicate prevention test
2. T010 — list_due_card_views display_info test
3. T013 — find_view_by_id display_info test

---

## Parallel Opportunities

### Phase 1 (all 4 parallel)

```
T001: Create migration file
T002: Extend view types in views.rs
T003: Add get_maintenance_card command
T004: Add Paraglide keys
```

### After Phase 2 (bindings synced), US3 internal parallelism

```
T014: Create MaintenanceDetailState.svelte.ts
T015: Create MaintenanceEventTimeline.svelte
→ Then T016 (AddEventModal) + T018 (Vitest test for detail state)
→ Then T017 (detail page, depends on T014+T015+T016)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 (T001, T004) → migration + error key ready
2. Complete T006 → UNIQUE constraint enforced in Rust
3. Complete T007 → test proves it works
4. Complete T008 → frontend surfaces the error
5. **STOP and VALIDATE**: Duplicate card creation is blocked — this is independently shippable

### Incremental Delivery

| Step          | Tasks     | Story Delivered            |
| ------------- | --------- | -------------------------- |
| Foundation    | T001–T005 | Infrastructure ready       |
| MVP           | T006–T008 | US1: Singleton enforcement |
| Grid identity | T009–T011 | US2: Readable card names   |
| Detail view   | T012–T019 | US3: Working Add Event     |
| Nav polish    | T020      | US4: Sidebar active state  |
| QA            | T021–T024 | All stories verified       |
