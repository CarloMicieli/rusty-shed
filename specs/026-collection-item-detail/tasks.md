# Tasks: Collection Item Detail View

**Input**: Design documents from `/specs/026-collection-item-detail/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/ipc-commands.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.
No new Rust backend changes are required. All IPC commands used are existing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1–US6)

---

## Phase 1: Setup

**Purpose**: Add Paraglide i18n keys required by all sidebar sections before any component is written.

- [x] T001 Add Paraglide message keys for detail page and sidebar UI strings in `messages/en.json` (back button, not-found state, section headings: Acquisition, Condition, Operational, Personal Context, empty-state labels)

**Checkpoint**: `pnpm check` passes. Message keys resolve without type errors.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create the global collection store and extend the navigation active-state logic.
These two workstreams are independent and can be developed in parallel. Both MUST be complete
before any user story page work begins.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T002 Create `src/lib/state/collection.svelte.ts` — Svelte 5 class-based module singleton with `$state` fields `items`, `collection`, `loading`; methods `fetch()` (cache-gated), `refresh()`, `getItemById(id)`; export `collectionStore` instance
- [x] T003 [P] Add `additionalPrefixes?: string[]` field to `NavigationItem` type in `src/lib/components/navigation/types.ts`
- [x] T004 [P] Update `isActive()` in `src/lib/components/navigation/utils.ts` to check `item.additionalPrefixes?.some(p => pathname.startsWith(p))` before the href equality check
- [x] T005 Add `additionalPrefixes: ['/collection']` to the `'collection'` entry in `src/lib/components/navigation/config.ts` (depends on T003 and T004)
- [x] T006 [P] Update `src/routes/+layout.svelte` startup preload (line 79) to call `collectionStore.fetch()` from `$lib/state/collection.svelte` and remove the direct `collectionState.fetchCollection()` call for the collection; keep the existing `CollectionState` context wiring intact for `CollectionDashboard` mutations

**Checkpoint**: Foundation ready. `pnpm check` and `pnpm lint` pass. Navigation type is extended.

---

## Phase 3: User Story 1 — Route & Navigation (Priority: P1) 🎯 MVP

**Goal**: Clicking a collection card navigates to `/collection/{itemId}`. The "Collection" nav
entry stays highlighted. A back button returns to `/my-collection`. Invalid IDs show a not-found
state. Old `/models/` route is removed.

**Independent Test**: Open the app, tap any collection card → URL changes to `/collection/{id}`,
nav "Collection" item highlighted. Press back → `/my-collection`. Direct URL to
`/collection/nonexistent` → not-found message with back link.

- [x] T007 [US1] Create directory `src/routes/collection/[itemId]/` and `+page.svelte` with: loading spinner, error state, not-found state, and a working back button that calls `goto('/my-collection')` — no model card or sidebar yet, just page shell that calls `collectionStore.fetch()` and resolves the item by `itemId`
- [x] T008 [US1] Update `handleCardClick` in `src/lib/features/collection/CollectionDashboard.svelte` (line 145) to navigate to `/collection/${item.id}` instead of `/models/${item.railwayModel.railwayModelId}`
- [x] T009 [US1] Verify no other file references `/models/` via `grep -r '"/models/' src/` then remove `src/routes/models/[...modelId]/+page.svelte` (and parent directory if empty)

**Checkpoint**: US1 fully functional and independently testable. No model card or sidebar yet.

---

## Phase 4: User Story 2 — Railway Model Card (Priority: P2)

**Goal**: The existing `RailwayModelCard` renders in the left panel of the detail page, showing
all model details and image via `getRailwayModelById` + `getRailwayModelImage`.

**Independent Test**: Navigate to a collection item → model name, manufacturer, scale, epoch,
category and image (if present) are all visible in the main content area.

- [x] T010 [US2] Extend `src/routes/collection/[itemId]/+page.svelte` to: (a) parallel-fetch `getRailwayModelById(railwayModelId)` and `getRailwayModelImage(railwayModelId)` after resolving the collection item; (b) wire `$derived displayModel` via the existing `toRailwayModel()` mapper; (c) render `<RailwayModelCard>` in a two-panel flex container (`flex-col lg:flex-row`), left panel `flex-1`

**Checkpoint**: US2 functional. Model card renders. Loading and error states covered.

---

## Phase 5: User Story 3 — Acquisition Summary (Priority: P2)

**Goal**: Sidebar shows seller name (with optional hyperlink), purchase date (human-readable),
and price with currency. Missing purchase data shows a graceful "not recorded" state.

**Independent Test**: Navigate to an item with a `purchased` `purchaseInfo` → sidebar shows seller
name, "Feb 22, 2026"-style date, and formatted price. Seller with `websiteUrl` renders as `<a>`.
Item with no `purchaseInfo` → "not recorded" state, no crash.

- [x] T011 [US3] Create `src/lib/features/collection/components/CollectionItemSidebar.svelte` — shell component accepting `item: CollectionItemView` and `seller: SellerView | null` props; renders an `<aside>` right panel (`w-80 flex-shrink-0`) with a section placeholder structure
- [x] T012 [US3] Add `AcquisitionSection` inside `CollectionItemSidebar.svelte`: seller name rendered as `<a href={websiteUrl}>` when URL present, else plain text; `purchaseDate` formatted with `Intl.DateTimeFormat`; `price.amount / 100` with `price.currency` symbol; full "not recorded" empty state when `purchaseInfo` is null or not `kind: 'purchased'`; all labels via Paraglide
- [x] T013 [US3] Wire sidebar and seller fetch into `src/routes/collection/[itemId]/+page.svelte`: extract `sellerId` from `purchaseInfo.data.seller`, include `getSellerById(sellerId)` in the `Promise.all` batch, pass resolved `seller` to `<CollectionItemSidebar>` as right panel inside the two-panel layout

**Checkpoint**: US3 functional. Sidebar visible with acquisition section. Seller link works.

---

## Phase 6: User Story 4 — Condition & Grading (Priority: P3)

**Goal**: Sidebar shows model condition, box condition, and purchase condition (new vs.
second-hand) as distinct labels or badges. Missing data shows "not recorded" state.

**Independent Test**: Navigate to an item with all three condition fields set → sidebar shows
three clearly labelled condition values with distinct badge styling.

- [x] T014 [US4] Add `ConditionSection` inside `CollectionItemSidebar.svelte`: three badge/label rows for `modelCondition`, `boxCondition`, `purchaseCondition`; each renders its enum value as a human-readable label (e.g., `LIKE_NEW` → "Like New"); graceful "not recorded" when any field is null; section hidden when all three are null; labels via Paraglide

**Checkpoint**: US4 functional. Condition badges render. Section absent when no data.

---

## Phase 7: User Story 5 — Operational Snapshot (Priority: P3)

**Goal**: Sidebar shows DCC address prominently and decoded decoder identifier for each
owned rolling stock entry. Missing DCC data shows "not configured" state.

**Independent Test**: Navigate to an item whose rolling stock has `digital.dcc_address` set →
DCC number displayed prominently. `installed_decoder_id` URN parsed and displayed (e.g.,
"esu / 54621"). Item with no rolling stock or no `digital` → "not configured" state.

- [x] T015 [US5] Add `OperationalSection` inside `CollectionItemSidebar.svelte`: map over `item.rollingStocks`; for each entry, if `digital` is non-null display `dcc_address` as a prominent number and parse `installed_decoder_id` URN (`trn:decoder:{mfr}:{code}`) to display "{mfr} / {code}"; section shows "not configured" when `rollingStocks` is empty or all entries have `digital: null`; labels via Paraglide

**Checkpoint**: US5 functional. DCC address renders. Decoder URN parsed client-side.

---

## Phase 8: User Story 6 — Personal Context (Priority: P4)

**Goal**: Sidebar shows the `added_date` (human-readable) and a `line-clamp-3` truncated
preview of the `notes` field. Empty notes omits the notes section.

**Independent Test**: Navigate to an item with `addedDate` and a long `notes` value → date
formatted as "Feb 22, 2026", notes truncated at three lines. Item with no notes → no notes
section shown (or clear empty state).

- [x] T016 [US6] Add `PersonalContextSection` inside `CollectionItemSidebar.svelte`: `addedDate` formatted with `Intl.DateTimeFormat`; `notes` rendered in a `<p class="line-clamp-3">` element; notes section hidden when `notes` is null; section always visible (addedDate is always present); labels via Paraglide

**Checkpoint**: All six user stories functional. Full sidebar with four sections visible.

---

## Final Phase: Polish & Cross-Cutting Concerns

**Purpose**: Code quality verification, constitution-required tests, and manual smoke tests.

- [x] T017 [P] Add Vitest unit tests for `CollectionStore` cache behaviour in `src/__tests__/state/collection.svelte.test.ts`: (a) `fetch()` calls `getCollection` only once when called twice; (b) `refresh()` always calls `getCollection` regardless of cache; (c) `getItemById()` returns correct item or `undefined`
- [x] T018 [P] Add Vitest unit tests for the updated `isActive()` in `src/__tests__/navigation/utils.test.ts`: (a) returns true for `/collection/123` when `additionalPrefixes: ['/collection']`; (b) still returns true for exact `href` match; (c) still returns false for unrelated paths
- [x] T019 Run `pnpm check` and resolve any TypeScript / Svelte type errors across all changed files
- [x] T020 Run `pnpm lint` and resolve any ESLint warnings across all changed files
- [x] T021 Manual smoke test per `quickstart.md` verification checklist: navigation highlight, direct URL refresh, empty-state sidebar, DCC display

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all user story phases
- **US1 (Phase 3)**: Depends on Phase 2 — MVP; can demo after this phase
- **US2 (Phase 4)**: Depends on US1 (detail page shell must exist)
- **US3 (Phase 5)**: Depends on US2 (two-panel layout must exist)
- **US4 (Phase 6)**: Depends on US3 (sidebar shell must exist)
- **US5 (Phase 7)**: Depends on US3 (sidebar shell must exist) — can run in parallel with US4
- **US6 (Phase 8)**: Depends on US3 (sidebar shell must exist) — can run in parallel with US4/US5
- **Polish (Final)**: Depends on all desired user stories

### Parallel Opportunities Within Foundational (Phase 2)

```
Parallel stream A:  T002 (collection.svelte.ts)
                    T006 (layout.svelte update)

Parallel stream B:  T003 (types.ts)
                    T004 (utils.ts)
                    → T005 (config.ts, after T003+T004)
```

### Sidebar Section Parallelism (Phases 6–8)

Once the sidebar shell (T011–T013) is complete, the remaining three sections can be
developed in parallel if desired:

```
T014 [US4] ConditionSection
T015 [US5] OperationalSection    ← all three can start after T013
T016 [US6] PersonalContextSection
```

---

## Implementation Strategy

### MVP (User Story 1 Only)

1. Phase 1: Add message keys (T001)
2. Phase 2: Global store + nav extension (T002–T006)
3. Phase 3: Route + nav + old route removal (T007–T009)
4. **STOP and VALIDATE**: Collection cards navigate to new URL. Nav stays highlighted. Back works.

### Incremental Delivery

1. Phases 1–2 → Foundation ready
2. Phase 3 (US1) → MVP: working route + nav. Demo.
3. Phase 4 (US2) → Model card renders. Demo.
4. Phase 5 (US3) → Acquisition sidebar. Demo.
5. Phases 6–8 (US4–US6) → Full sidebar. Final demo.
6. Polish phase → Ship.

---

## Task Summary

| Phase           | User Story           | Tasks        | Parallel [P] tasks     |
| --------------- | -------------------- | ------------ | ---------------------- |
| 1: Setup        | —                    | T001         | 0                      |
| 2: Foundational | —                    | T002–T006    | T002, T003, T004, T006 |
| 3: US1 (P1) 🎯  | Route & Navigation   | T007–T009    | 0                      |
| 4: US2 (P2)     | Railway Model Card   | T010         | 0                      |
| 5: US3 (P2)     | Acquisition Summary  | T011–T013    | 0                      |
| 6: US4 (P3)     | Condition & Grading  | T014         | 0                      |
| 7: US5 (P3)     | Operational Snapshot | T015         | T015 (vs T014/T016)    |
| 8: US6 (P4)     | Personal Context     | T016         | T016 (vs T014/T015)    |
| Final: Polish   | —                    | T017–T021    | T017, T018             |
| **Total**       |                      | **21 tasks** | **9 parallelizable**   |

---

## Notes

- [P] tasks touch different files and have no dependency on incomplete sibling tasks
- US4, US5, US6 all evolve the same `CollectionItemSidebar.svelte` file — they cannot run
  in parallel within that file, but the sidebar shell (US3) must be done first
- All user-facing strings MUST use `m.*` Paraglide functions — never hardcoded
- Run `pnpm check` and `pnpm lint` after each phase before moving to the next
- The `collectionStore` module singleton approach means NO Svelte context is required on
  the detail page — just `import { collectionStore } from '$lib/state/collection.svelte'`
