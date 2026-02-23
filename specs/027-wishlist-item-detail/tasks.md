# Tasks: Wishlist Item Detail View

**Input**: Design documents from `/specs/027-wishlist-item-detail/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Included in Polish phase per constitution ("Tests are required for new features").

**Scope**: Frontend-only. No Rust/backend changes. 2 new files, 4 modified files.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1–US4)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the i18n foundation required by all user story phases.

- [ ] T001 Add `wishlist_item_*` message keys to `messages/en.json` — keys: `wishlist_item_back`, `wishlist_item_not_found`, `wishlist_item_not_found_message`, `wishlist_item_loading`, `wishlist_item_error`, `wishlist_item_section_details`, `wishlist_item_wishlist_name`, `wishlist_item_status`, `wishlist_item_purchased_price`, `wishlist_item_price_not_set`, `wishlist_item_section_personal_context`, `wishlist_item_added_date`, `wishlist_item_notes`, `wishlist_item_status_wanted`, `wishlist_item_status_on_order`, `wishlist_item_status_purchased`, `wishlist_item_status_ignored`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: One-line navigation config change that must land before US1 can be fully verified.

**⚠️ CRITICAL**: The navigation active state gate must pass before US1 is considered done.

- [ ] T002 Add `additionalPrefixes: ['/wishlists']` to the wishlists entry in `src/lib/components/navigation/config.ts` so the "Wishlists" nav item stays highlighted on all `/wishlists/*` paths

**Checkpoint**: Foundation ready — user story implementation can begin.

---

## Phase 3: User Story 1 — Navigate to a Wishlist Item (Priority: P1) 🎯 MVP

**Goal**: Item card click navigates to `/wishlists/{wishlistId}/items/{itemId}`; "Wishlists" nav stays highlighted; back button returns to `/my-wishlists`.

**Independent Test**: Open `/my-wishlists`, select a wishlist, click any item card. Browser navigates to the correct URL. "Wishlists" nav item is highlighted. Pressing back returns to `/my-wishlists`. Navigating to a non-existent item ID shows the "item not found" state.

### Implementation for User Story 1

- [ ] T003 [US1] Add `wishlistId: string` prop to `WishlistItems.svelte` and pass it to each `<WishlistItemCard>` in `src/lib/features/wishlists/components/WishlistItems.svelte` — also update any parent components (`WishlistsDashboard.svelte`) that render `<WishlistItems>` to supply the active wishlist ID
- [ ] T004 [US1] Add `wishlistId: string` to the `Props` interface and `onclick={() => goto('/wishlists/${wishlistId}/items/${item.id}')}` on the card container in `src/lib/features/wishlists/components/WishlistItemCard.svelte`; add `onclick={(e) => e.stopPropagation()}` on the Remove/Move/Purchase action buttons to prevent bubbling (depends on T003)
- [ ] T005 [P] [US1] Create `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte` — extract route params (`wishlistId`, `itemId`), call `commands.getWishlistById(wishlistId)`, look up item by id from `wishlistView.items`, implement loading/error/notFound page states, and add back button calling `goto('/my-wishlists')` using `m.wishlist_item_back()` label (depends on T001)

**Checkpoint**: Clicking a wishlist item card navigates to the correct URL; "Wishlists" nav remains highlighted; back button works; error states display without crashes.

---

## Phase 4: User Story 2 — View Railway Model Card (Priority: P2)

**Goal**: The railway model card renders in the primary content area of the detail page with model name, manufacturer, scale, image (or placeholder), and all catalogue fields.

**Independent Test**: Navigate to a detail page. Verify the railway model card appears in the main (left) area with all model fields populated. An item whose model has an image shows it; one without shows a placeholder.

### Implementation for User Story 2

- [ ] T006 [US2] Extend the detail page to fetch model details and image in parallel after the wishlist item is found — `const [model, imageResponse] = await Promise.all([commands.getRailwayModelById(...), commands.getRailwayModelImage(...)])` — map via `toRailwayModel(model, null, imageResponse)` and render `<RailwayModelCard>` in the main content area in `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte` (depends on T005)

**Checkpoint**: The model card renders with all catalogue data. Two-panel layout scaffold is in place.

---

## Phase 5: User Story 3 — View Wishlist Context (Priority: P2)

**Goal**: Sidebar shows: parent wishlist name, priority badge (Low/Normal/High), status badge (Wanted/On Order/Purchased/Ignored), desired price (or "Not set"), and purchased price row (only when non-null).

**Independent Test**: Navigate to an item with priority=HIGH and desired_price set. Sidebar shows the wishlist name, a "High" badge, the correct desired price with currency. Navigate to an item with no desired_price — sidebar shows "Not set" without errors. Navigate to a PURCHASED item — sidebar shows both desired and purchased price.

### Implementation for User Story 3

- [ ] T007 [P] [US3] Create `src/lib/features/wishlists/components/WishlistItemSidebar.svelte` — props: `item: WishlistItem`, `wishlistName: string`; render a "Wish List Details" section (`<section class="rounded-lg border border-white/10 bg-black/20 p-4">`) with definition-list rows for: List name, Priority (badge with `wishlist_priority_*` labels), Status (badge with `wishlist_item_status_*` labels), Desired Price (formatted via `Intl.NumberFormat` with currency, or `m.wishlist_item_price_not_set()` if null), Purchased Price (row visible only when `item.purchasedPrice !== null`) (depends on T001)
- [ ] T008 [US3] Integrate `<WishlistItemSidebar>` into the detail page — pass `item={wishlistItem}` and `wishlistName={wishlistView.name}` — use two-column layout (model card left, sidebar right) matching the collection item detail page layout in `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte` (depends on T006, T007)

**Checkpoint**: Sidebar "Wish List Details" section renders correctly for items with and without prices.

---

## Phase 6: User Story 4 — View Item Notes & Dates (Priority: P3)

**Goal**: Sidebar shows a "Personal Context" section with the item's added date (human-readable) and notes text (section hidden when notes is null).

**Independent Test**: Navigate to an item with both `added_date` and `notes` set. Sidebar shows "Personal Context" section with the formatted date (e.g., "Feb 23, 2026") and the notes text. Navigate to an item with no notes — "Personal Context" section is either absent or shows a clean empty state.

### Implementation for User Story 4

- [ ] T009 [US4] Add "Personal Context" section to `src/lib/features/wishlists/components/WishlistItemSidebar.svelte` — display `item.addedDate` formatted via `Intl.DateTimeFormat` with `{ dateStyle: 'medium' }`, and `item.notes` text (truncated if very long); hide the section or show empty state gracefully when `item.notes === null` (depends on T007)

**Checkpoint**: All four user stories are complete and independently functional.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Code quality, tests, and final validation.

- [ ] T010 [P] Run `pnpm lint` and fix any ESLint errors across modified files (`config.ts`, `WishlistItemCard.svelte`, `WishlistItems.svelte`, `WishlistItemSidebar.svelte`, `+page.svelte`)
- [ ] T011 [P] Run `pnpm check` and resolve all `svelte-check` and TypeScript errors across the same files
- [ ] T012 [P] Write Vitest unit tests for `WishlistItemSidebar` in `src/__tests__/features/wishlists/WishlistItemSidebar.test.ts` — cover: renders wishlist name, renders correct priority label, renders desired price, renders "Not set" for null price, hides purchased price row when null, shows purchased price when non-null
- [ ] T013 [P] Write Vitest unit tests for updated navigation config in `src/__tests__/components/navigation/config.test.ts` — cover: `isActive()` returns `true` for `/wishlists/abc/items/xyz` with `additionalPrefixes: ['/wishlists']`, returns `false` for unrelated paths
- [ ] T014 Validate all items in quickstart.md manual QA checklist — navigate to detail page, verify model card, sidebar, back nav, not-found state

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: No dependencies — can run in parallel with Phase 1
- **US1 (Phase 3)**: Depends on Phase 1 (T001 for i18n) and Phase 2 (T002 for nav config)
- **US2 (Phase 4)**: Depends on US1 (Phase 3) being complete (page route must exist)
- **US3 (Phase 5)**: T007 [sidebar creation] can start in parallel with Phase 4 (different file); T008 [sidebar wiring] depends on T006 and T007
- **US4 (Phase 6)**: Depends on T007 (sidebar component) being complete
- **Polish (Phase 7)**: T010–T013 can all run in parallel; T014 depends on T010–T013

### User Story Dependencies

```
Phase 1 (T001) ──┐
Phase 2 (T002) ──┼──► US1 (T003→T004, T005) ──► US2 (T006) ──► US3 T008
                 │                                                    ↑
                 └──────────────────────────────── US3 T007 [P] ─────┘
                                                        │
                                                   US4 (T009)
```

### Within Each User Story

- **US1**: T003 → T004 (sequential; T005 in parallel with T003/T004)
- **US2**: T006 (single task, depends on T005)
- **US3**: T007 [P] (can start in parallel with T006); T008 (depends on T006 + T007)
- **US4**: T009 (depends on T007)

### Parallel Opportunities

- T001 and T002 (Phase 1 + 2) can run simultaneously
- T003/T004 and T005 can run simultaneously (different files)
- T007 (sidebar creation) and T006 (model data fetching) can run simultaneously
- T010, T011, T012, T013 (Polish) can all run simultaneously

---

## Parallel Execution Examples

### US1 — Navigate to a Wishlist Item

```
# Run simultaneously (different files):
Task A: "Update WishlistItems.svelte and WishlistsDashboard.svelte to propagate wishlistId (T003→T004)"
Task B: "Create the route skeleton +page.svelte (T005)"
```

### US2 + US3 start in parallel after US1

```
# Run simultaneously (different files):
Task A: "Add model data fetching + RailwayModelCard to +page.svelte (T006)"
Task B: "Create WishlistItemSidebar.svelte component (T007)"
# Then:
Task C: "Wire sidebar into detail page (T008)" — after both A and B complete
```

### Polish Phase

```
# Run simultaneously:
Task A: "pnpm lint check (T010)"
Task B: "pnpm check (T011)"
Task C: "Write WishlistItemSidebar tests (T012)"
Task D: "Write navigation config tests (T013)"
```

---

## Implementation Strategy

### MVP (User Story 1 Only)

1. Complete Phase 1 (T001) + Phase 2 (T002)
2. Complete Phase 3: US1 (T003, T004, T005)
3. **STOP and VALIDATE**: Item cards navigate correctly, nav highlighted, back works
4. Deliverable: navigation fix is live, detail URL is accessible

### Incremental Delivery

1. Complete Phase 1 + 2 → Foundation ready
2. Add US1 (Phase 3) → Navigation works → **Demo MVP**
3. Add US2 (Phase 4) → Model card visible → **Demo with model info**
4. Add US3 (Phase 5) → Sidebar with wishlist details → **Demo complete view**
5. Add US4 (Phase 6) → Sidebar with notes/dates → **Full feature complete**
6. Polish (Phase 7) → Tests pass, linting clean → **Ready to merge**

---

## Notes

- [P] tasks have no file conflicts and no inter-dependencies — run in parallel
- `wishlist_field_priority` and `wishlist_field_desired_price` keys already exist in `en.json` — reuse them for sidebar field labels
- `collection_item_not_recorded` already exists — reuse as fallback empty state
- Price formatting: `new Intl.NumberFormat(undefined, { style: 'currency', currency: item.desiredPrice.currency }).format(Number(item.desiredPrice.amount) / 100)` — same pattern as `CollectionItemSidebar.svelte`
- Reference files: `src/routes/collection/[itemId]/+page.svelte` and `src/lib/features/collection/components/CollectionItemSidebar.svelte` are the direct templates
