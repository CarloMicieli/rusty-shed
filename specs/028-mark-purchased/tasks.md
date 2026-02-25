# Tasks: Mark Wishlist Item as Purchased

**Input**: Design documents from `/specs/028-mark-purchased/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to
- Exact file paths are included in each description

---

## Phase 1: Setup

**Purpose**: Add i18n message keys before any UI work — required by all frontend tasks.

- [x] T001 Add all Paraglide message keys for the purchase dialog to `messages/en.json` (see full key list in `specs/028-mark-purchased/plan.md` → "Paraglide i18n Keys Required")

---

## Phase 2: Foundational — Backend (Blocking Prerequisites)

**Purpose**: Backend changes that must ALL be complete before the frontend can call `purchase_wishlist_item` with condition data.

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete and `pnpm rust:build` succeeds.

- [x] T002 [P] Add `ItemPurchased { item_id: WishlistItemId, purchased_price: MonetaryAmount }` variant to the `WishlistEvent` enum in `src-tauri/src/wishlist/domain/wishlist_event.rs`
- [x] T003 [P] Add `PurchaseWishlistItemCommand` struct (extends `MoveWishlistItemId` with `purchase_condition: Option<PurchaseCondition>` and `model_condition: Option<ModelCondition>` fields) in `src-tauri/src/wishlist/application/purchase_wishlist_item.rs`
- [x] T004 Add `purchase_item(&mut self, item_id: &WishlistItemId, purchased_price: MonetaryAmount) -> Result<(), DomainError>` method to the `Wishlist` aggregate in `src-tauri/src/wishlist/domain/wishlist.rs` — validates item status is `Wanted` or `OnOrder`, updates state, emits `WishlistEvent::ItemPurchased` (depends on T002)
- [x] T005 Add `ItemPurchased` event handler in the `SqliteWishlistRepository::save_wishlist()` infrastructure implementation in `src-tauri/src/wishlist/infrastructure/` — runs `UPDATE wishlist_items SET status='PURCHASED', purchased_price_amount=?, purchased_price_currency=? WHERE id=?` (depends on T002, T004)
- [x] T006 Add `condition: Option<String>` field to `PurchaseWishlistArgs` in `src-tauri/src/wishlist/interface/` (wherever `PurchaseWishlistArgs` is defined), add `TryFrom<PurchaseWishlistArgs>` decomposition that maps `"New"` → `(PurchaseCondition::New, None)`, `"PreOwnedLikeNew"` → `(PurchaseCondition::PreOwned, ModelCondition::NearMint)`, `"PreOwnedVeryGood"` → `(PurchaseCondition::PreOwned, ModelCondition::VeryGood)`, `"PreOwnedGood"` → `(PurchaseCondition::PreOwned, ModelCondition::Good)`, `"PreOwnedAcceptable"` → `(PurchaseCondition::PreOwned, ModelCondition::Fair)` (depends on T003)
- [x] T007 Refactor `PurchaseWishlistItemService::move_wishlist_item()` (or introduce `execute()`) in `src-tauri/src/wishlist/application/purchase_wishlist_item.rs` to: (1) call `wishlist.purchase_item(item_id, price)` instead of directly mutating item fields, (2) forward `purchase_condition` and `model_condition` into `AddCollectionItemInput` (depends on T003, T004, T005)
- [x] T008 Update `purchase_wishlist_item` Tauri command handler in `src-tauri/src/wishlist/interface/command_handlers.rs` to accept the updated `PurchaseWishlistArgs`, construct `PurchaseWishlistItemCommand`, and call the refactored service (depends on T006, T007)
- [x] T009 Update existing Rust unit tests in `src-tauri/src/wishlist/application/purchase_wishlist_item.rs` for the refactored service; add new test cases for: (1) purchase attempt on an already-Purchased item returns `DomainError`, (2) purchase with condition fields correctly propagates condition to `AddCollectionItemInput`, (3) `purchase_item()` on `Wishlist` aggregate emits `ItemPurchased` event (depends on T004, T007)
- [x] T010 Run `pnpm rust:build` and `pnpm rust:test` — fix all compilation errors and test failures before proceeding (depends on T002–T009)

**Checkpoint**: `pnpm rust:build` succeeds, all Rust tests pass, specta regenerates `src/lib/bindings.ts` with the updated `PurchaseWishlistArgs` type including `condition`.

---

## Phase 3: User Story 1 — Purchase from Detail Page (Priority: P1) 🎯 MVP

**Goal**: A collector can open a wishlist item's detail page, click "Purchase", fill in the purchase dialog, and have the item atomically transferred to their collection.

**Independent Test**: Open a wishlist item detail page with status Wanted → click "Purchase" → complete the dialog → verify item disappears from wishlist and appears in collection with correct price, date, seller, and condition. Verify the Purchase button is absent when the item is re-opened (now Purchased status).

### Implementation for User Story 1

- [x] T011 [P] [US1] Create `PurchaseDialog.svelte` in `src/lib/features/wishlist/components/PurchaseDialog.svelte` — dialog component using shadcn-svelte `Dialog`, superforms + Zod (`PurchaseFormSchema`) for validation; fields: price (number, required), purchase date (date picker, default today, no future dates), seller (select populated from `get_sellers()` result), condition (select with options from `PURCHASE_CONDITION_OPTIONS`); on mount loads sellers via `get_sellers()` and default currency via `get_settings()`; on submit invokes `purchase_wishlist_item` with correct payload; on success emits close event; on error keeps dialog open and displays inline error using `purchaseDialog.error.saveFailed` message key (depends on T001, T010)
- [x] T012 [P] [US1] Add `openPurchaseDialog(item: WishlistItem)`, `closePurchaseDialog()`, `handlePurchaseSuccess()`, and reactive `purchaseDialogState` to `src/lib/features/wishlist/WishlistController.svelte.ts` — `handlePurchaseSuccess()` must invalidate/reload the wishlist state so the purchased item is removed from the view (depends on T010)
- [x] T013 [US1] Add "Purchase" button to the wishlist item detail page component in `src/lib/features/wishlist/` — button visible only when item status is `Wanted` or `OnOrder`; clicking it calls `openPurchaseDialog(item)`; mount `PurchaseDialog` on the page bound to `purchaseDialogState`; on success navigate or refresh as appropriate (depends on T011, T012)

**Checkpoint**: User Story 1 fully functional — purchase from detail page works end-to-end. Verify: price required validation fires, future date validation fires, successful purchase removes item from wishlist and adds to collection with all entered details.

---

## Phase 4: User Story 2 — Purchase from Wishlist Preview Card (Priority: P2)

**Goal**: A collector can click "Purchase" directly on a wishlist item card in the list view, without navigating to the detail page.

**Independent Test**: From the wishlist list view, click "Purchase" on a preview card → complete dialog → card disappears from list, item appears in collection. The same dialog and backend as US1 are reused.

### Implementation for User Story 2

- [x] T014 [US2] Add "Purchase" button to `src/lib/features/wishlist/components/WishlistItemCard.svelte` (or the equivalent preview card component) — button shown only when item status is `Wanted` or `OnOrder`; clicking it calls `openPurchaseDialog(item)` from the shared `WishlistController` state; the `PurchaseDialog` is already mounted at the wishlist page level and reuses the same state from T012 (depends on T011, T012)

**Checkpoint**: User Story 2 fully functional — purchase from preview card triggers the same dialog as the detail page, card disappears from list on success.

---

## Phase 5: User Story 3 — Cancel Purchase Flow (Priority: P3)

**Goal**: Dismissing the purchase dialog (cancel button, Escape key, or clicking outside) leaves the wishlist item completely unchanged.

**Independent Test**: Open the purchase dialog, partially fill in fields, then dismiss via each method (cancel button / Escape / outside click) — wishlist item status unchanged, no collection entry created.

### Implementation for User Story 3

- [x] T015 [US3] Verify and harden dismiss behavior in `src/lib/features/wishlist/components/PurchaseDialog.svelte` — confirm shadcn-svelte `Dialog` handles Escape and outside-click dismissal without side effects; ensure `closePurchaseDialog()` resets `isSubmitting` and `error` state so the dialog is clean if reopened; no `purchase_wishlist_item` IPC call is made unless the user explicitly submits the form (depends on T011, T012)

**Checkpoint**: All three user stories independently functional. Any dismissal method leaves wishlist state intact.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates and final verification.

- [x] T016 [P] Run `pnpm lint && pnpm check && pnpm test` — resolve all TypeScript, ESLint, and Vitest failures in frontend files changed by this feature
- [x] T017 [P] Run `pnpm rust:fmt && pnpm rust:clippy` — resolve all formatting and clippy warnings (clippy runs with `-D warnings`)
- [ ] T018 Verify `src/lib/bindings.ts` has been regenerated and the `PurchaseWishlistArgs` TypeScript type includes the `condition` field — if stale, run `pnpm rust:build` to trigger specta regeneration
- [ ] T019 Manual end-to-end test following all scenarios in `specs/028-mark-purchased/quickstart.md` — purchase from detail page, purchase from card, cancel flow, validation errors, zero price, status guard (no Purchase button on already-Purchased items)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 completion — **BLOCKS all user stories**
- **User Story 1 (Phase 3)**: Depends on Phase 2 — MVP, deliver first
- **User Story 2 (Phase 4)**: Depends on Phase 3 (reuses dialog + controller) — can start as soon as T011 and T012 are done
- **User Story 3 (Phase 5)**: Depends on Phase 3 (dialog must exist) — can start as soon as T011 is done
- **Polish (Phase 6)**: Depends on all user story phases complete

### User Story Dependencies

- **US1 (P1)**: Requires Foundational complete. No dependency on US2 or US3.
- **US2 (P2)**: Requires T011 + T012 from US1 (reuses dialog and controller state).
- **US3 (P3)**: Requires T011 from US1 (dialog must exist to test dismissal).

### Within Phase 2 (Foundational)

```
T002 (WishlistEvent) ──────────────┐
T003 (Command struct)  ────────────┼──► T007 (Service refactor) ──► T008 (Handler) ──► T009 (Tests) ──► T010 (Build)
T004 (Aggregate method) ◄── T002  ─┤
T005 (Infrastructure) ◄── T002,T004┘
T006 (Args + TryFrom) ◄── T003
```

T002 and T003 have no upstream dependencies within this phase — they can start in parallel immediately.

### Parallel Opportunities

```bash
# Phase 2: Start these two in parallel
T002  # WishlistEvent::ItemPurchased variant
T003  # PurchaseWishlistItemCommand struct

# Phase 3: Start these two in parallel (after T010)
T011  # PurchaseDialog.svelte
T012  # WishlistController state additions

# Phase 6: Run these two in parallel
T016  # Frontend quality checks
T017  # Rust quality checks
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 2: Foundational (T002–T010) — CRITICAL blocker
3. Complete Phase 3: User Story 1 (T011–T013)
4. **STOP and VALIDATE**: Purchase from detail page works end-to-end
5. Run Phase 6 checks for deliverable quality

### Incremental Delivery

1. Phase 1 + Phase 2 → Backend ready, bindings regenerated
2. Phase 3 → MVP: purchase from detail page ✅
3. Phase 4 → Purchase from card ✅
4. Phase 5 → Cancel safety verified ✅
5. Phase 6 → All quality gates pass ✅

---

## Notes

- `[P]` tasks touch different files — safe to implement in parallel
- `[Story]` label maps each task to the user story it completes
- Each user story phase is independently testable after completion
- Commit after each phase (or after each logical task group)
- The `PurchaseDialog.svelte` is the shared component for both US1 and US2 — implement it once in US1's phase
- All user-facing strings MUST use Paraglide message keys added in T001 — no hardcoded text
- If `PurchaseWishlistArgs` is in `command_handlers.rs` rather than a separate args file, adapt T006 accordingly
