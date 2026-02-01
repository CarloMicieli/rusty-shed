# Tasks: Track Inventory Management

**Input**: Design documents from `/specs/005-track-inventory/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/api.md ✓

**Tests**: Not explicitly requested in spec - tests excluded from task list.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and Rust backend adjustments

- [ ] T001 Create migration `src-tauri/migrations/0007_add_track_type_to_products.sql` adding track_type column
- [ ] T002 Add `track_type: TrackType` field to `TrackProduct` struct in `src-tauri/src/tracks_inventory/domain/track_product.rs`
- [ ] T003 [P] Update `TrackProductRow` in `src-tauri/src/tracks_inventory/infrastructure/entities.rs` to include track_type
- [ ] T004 [P] Update repository queries in `src-tauri/src/tracks_inventory/infrastructure/sqlite_track_product_repository.rs` for track_type
- [ ] T005 Create View structs in `src-tauri/src/tracks_inventory/application/views.rs` (TrackInventoryListItem, TrackInventoryView, TrackInventoryItemView, TrackProductView, TrackPurchaseView)
- [ ] T006 Export view structs from `src-tauri/src/tracks_inventory/application/mod.rs`
- [ ] T007 Create query use-case `src-tauri/src/tracks_inventory/application/get_track_inventories.rs`
- [ ] T008 [P] Create query use-case `src-tauri/src/tracks_inventory/application/get_track_inventory.rs`
- [ ] T009 [P] Create query use-case `src-tauri/src/tracks_inventory/application/get_track_products.rs`
- [ ] T010 Create query handlers in `src-tauri/src/tracks_inventory/interface/query_handlers.rs` (get_track_inventories, get_track_inventory, get_track_products)
- [ ] T011 Export query handlers from `src-tauri/src/tracks_inventory/interface/mod.rs`
- [ ] T011a Create delete use-case `src-tauri/src/tracks_inventory/application/delete_track_inventory.rs`
- [ ] T011b Add delete_track_inventory command handler in `src-tauri/src/tracks_inventory/interface/command_handlers.rs`
- [ ] T012 Register query and delete commands in `src-tauri/src/lib.rs` invoke_handler
- [ ] T013 Run `pnpm rust:format && pnpm rust:clippy && pnpm rust:test` to verify Rust changes
- [ ] T014 Regenerate TypeScript bindings by running `pnpm tauri build` or `pnpm dev`

---

## Phase 2: Foundational (Frontend Scaffolding)

**Purpose**: Core frontend infrastructure that MUST be complete before user story UI work

**⚠️ CRITICAL**: No UI component work can begin until this phase is complete

- [ ] T015 Add localization keys to `messages/en.json` (app*tracks, track_inventories*_, track*purchase*_, track*product*\*)
- [ ] T016 [P] Add localization keys to `messages/it.json` (Italian translations)
- [ ] T017 Create service class in `src/lib/features/track-inventory/services/TrackInventoryService.svelte.ts`
- [ ] T018 Create context provider functions (setTrackInventoryContext, getTrackInventoryContext) in service file
- [ ] T019 Update `src/lib/features/track-inventory/index.ts` to export service, context functions, and types
- [ ] T020 Create route directory structure `src/routes/my-tracks/` and `src/routes/my-tracks/[id]/`
- [ ] T021 [P] Create placeholder `src/routes/my-tracks/+page.svelte`
- [ ] T022 [P] Create placeholder `src/routes/my-tracks/[id]/+page.svelte`
- [ ] T023 Initialize TrackInventoryService in `src/routes/+layout.svelte` alongside other contexts
- [ ] T024 Run `pnpm format && pnpm lint && pnpm check` to verify frontend scaffolding

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - View Track Inventory (Priority: P1) 🎯 MVP

**Goal**: Users can navigate to "My Tracks" and view their track inventories with stock quantities

**Independent Test**: Navigate to /my-tracks, see inventory list, click one to see items with quantities

### Implementation for User Story 1

- [ ] T025 [US1] Add "My Tracks" nav item to `src/lib/features/navigation/components/SidebarNavigation.svelte` (import Tram icon, add link after My Wishlists)
- [ ] T026 [P] [US1] Add "My Tracks" nav item to `src/lib/features/navigation/components/BottomNavigation.svelte`
- [ ] T027 [US1] Implement `fetchInventories()` method in TrackInventoryService to call get_track_inventories
- [ ] T028 [US1] Implement `fetchInventory(id)` method in TrackInventoryService to call get_track_inventory
- [ ] T029 [P] [US1] Create `src/lib/features/track-inventory/components/InventoryCard.svelte` (displays name, description, total_quantity)
- [ ] T030 [P] [US1] Create `src/lib/features/track-inventory/components/InventoryList.svelte` (grid of InventoryCards)
- [ ] T031 [US1] Create `src/lib/features/track-inventory/components/EmptyState.svelte` (empty state with guidance)
- [ ] T032 [US1] Implement `src/routes/my-tracks/+page.svelte` (call fetchInventories, render InventoryList or EmptyState)
- [ ] T033 [P] [US1] Create `src/lib/features/track-inventory/components/InventoryItemRow.svelte` (track product with quantity and required)
- [ ] T034 [US1] Create `src/lib/features/track-inventory/components/InventoryDetail.svelte` (header + list of InventoryItemRows)
- [ ] T035 [US1] Implement `src/routes/my-tracks/[id]/+page.svelte` (call fetchInventory, render InventoryDetail)
- [ ] T036 [US1] Add shortage indicator styling to InventoryItemRow (visual highlight when quantity < required)
- [ ] T037 [US1] Run `pnpm format && pnpm lint && pnpm check` to verify US1

**Checkpoint**: User Story 1 complete - users can view inventories and their contents

---

## Phase 4: User Story 2 - Create and Manage Track Inventories (Priority: P1)

**Goal**: Users can create, rename, and delete track inventories

**Independent Test**: Create new inventory with name/description, rename it, delete it

### Implementation for User Story 2

- [ ] T038 [US2] Implement `createInventory(input)` method in TrackInventoryService to call create_track_inventory
- [ ] T039 [P] [US2] Implement `renameInventory(input)` method in TrackInventoryService to call rename_track_inventory
- [ ] T039a [P] [US2] Implement `deleteInventory(id)` method in TrackInventoryService to call delete_track_inventory
- [ ] T040 [US2] Create `src/lib/features/track-inventory/components/CreateInventoryDialog.svelte` (modal with name, description fields)
- [ ] T041 [US2] Add "Create Inventory" button to `src/routes/my-tracks/+page.svelte` that opens CreateInventoryDialog
- [ ] T042 [US2] Wire CreateInventoryDialog to call createInventory and refresh list on success
- [ ] T043 [P] [US2] Create `src/lib/features/track-inventory/components/RenameInventoryDialog.svelte` (modal with name field)
- [ ] T044 [US2] Add edit/rename action to InventoryCard or InventoryDetail that opens RenameInventoryDialog
- [ ] T045 [US2] Wire RenameInventoryDialog to call renameInventory and refresh on success
- [ ] T046 [US2] Add delete action with confirmation dialog to InventoryCard or InventoryDetail (calls deleteInventory)
- [ ] T047 [US2] Run `pnpm format && pnpm lint && pnpm check` to verify US2

**Checkpoint**: User Story 2 complete - users can manage their inventories

---

## Phase 5: User Story 3 - Add Track Purchase (Priority: P1)

**Goal**: Users can add purchases to update inventory quantities

**Independent Test**: Add purchase with product, quantity, price, seller - verify inventory quantity increases

### Implementation for User Story 3

- [ ] T048 [US3] Implement `fetchProducts()` method in TrackInventoryService to call get_track_products
- [ ] T049 [US3] Implement `addPurchase(input)` method in TrackInventoryService to call add_track_purchase
- [ ] T050 [US3] Create `src/lib/features/track-inventory/components/AddPurchaseDialog.svelte` (modal with product selector, quantity, price, seller, date)
- [ ] T051 [US3] Implement product dropdown/combobox in AddPurchaseDialog using fetched products
- [ ] T052 [P] [US3] Implement seller dropdown in AddPurchaseDialog using existing sellers from app context
- [ ] T053 [US3] Add "Add Purchase" button to InventoryDetail that opens AddPurchaseDialog
- [ ] T054 [US3] Wire AddPurchaseDialog to call addPurchase and refresh inventory on success
- [ ] T055 [US3] Add validation feedback for required fields in AddPurchaseDialog
- [ ] T056 [US3] Run `pnpm format && pnpm lint && pnpm check` to verify US3

**Checkpoint**: User Story 3 complete - users can add purchases and see quantities update

---

## Phase 6: User Story 4 - View Purchase History (Priority: P2)

**Goal**: Users can view purchase history for each inventory

**Independent Test**: View inventory, see list of purchases with dates, products, quantities, prices, sellers

### Implementation for User Story 4

- [ ] T057 [US4] Create `src/lib/features/track-inventory/components/PurchaseHistoryItem.svelte` (single purchase row with product, quantity, price, seller, date)
- [ ] T058 [US4] Create `src/lib/features/track-inventory/components/PurchaseHistory.svelte` (list of PurchaseHistoryItems)
- [ ] T059 [US4] Add PurchaseHistory section/tab to InventoryDetail component
- [ ] T060 [US4] Style purchase history for chronological display with clear grouping
- [ ] T061 [US4] Run `pnpm format && pnpm lint && pnpm check` to verify US4

**Checkpoint**: User Story 4 complete - users can view purchase history

---

## Phase 7: User Story 5 - Manage Track Products (Priority: P2)

**Goal**: Users can define track products for use in purchases

**Independent Test**: Create new track product with all attributes, use it in a purchase

### Implementation for User Story 5

- [ ] T062 [US5] Create Rust command `create_track_product` in `src-tauri/src/tracks_inventory/interface/command_handlers.rs`
- [ ] T063 [US5] Create use-case `src-tauri/src/tracks_inventory/application/create_track_product.rs`
- [ ] T064 [US5] Register create_track_product command in `src-tauri/src/lib.rs`
- [ ] T065 [US5] Regenerate TypeScript bindings
- [ ] T066 [US5] Implement `createProduct(input)` method in TrackInventoryService
- [ ] T067 [US5] Create `src/lib/features/track-inventory/components/CreateProductDialog.svelte` (modal with all product fields)
- [ ] T068 [US5] Add "Create Product" inline action to AddPurchaseDialog when product not found
- [ ] T069 [US5] Wire CreateProductDialog to call createProduct and update product list
- [ ] T070 [US5] Run `pnpm rust:clippy && pnpm format && pnpm lint && pnpm check` to verify US5

**Checkpoint**: User Story 5 complete - users can manage track products

---

## Phase 8: User Story 6 - Set Required Quantities (Priority: P3)

**Goal**: Users can set required quantities to plan their track needs

**Independent Test**: Set required quantity for track type, see it displayed alongside stock, see shortage indicator

### Implementation for User Story 6

- [ ] T071 [US6] Create migration `src-tauri/migrations/0008_add_required_to_inventory_items.sql` adding required column
- [ ] T072 [US6] Update `TrackInventoryItemRow` in infrastructure entities to include required field
- [ ] T073 [US6] Update repository to read/write required field
- [ ] T074 [US6] Create Rust command `set_track_item_required` in command_handlers.rs
- [ ] T075 [US6] Create use-case `src-tauri/src/tracks_inventory/application/set_item_required.rs`
- [ ] T076 [US6] Register command and regenerate bindings
- [ ] T077 [US6] Implement `setItemRequired(input)` method in TrackInventoryService
- [ ] T078 [US6] Add editable required quantity field to InventoryItemRow component
- [ ] T079 [US6] Wire required field to call setItemRequired on change
- [ ] T080 [US6] Run `pnpm rust:clippy && pnpm format && pnpm lint && pnpm check` to verify US6

**Checkpoint**: User Story 6 complete - users can plan required quantities

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T081 [P] Update `src/lib/features/track-inventory/README.md` from placeholder to actual documentation
- [ ] T082 Add loading states to all async operations in components
- [ ] T083 [P] Add error handling with toast notifications for failed operations
- [ ] T084 Ensure all user-facing strings use Paraglide messages (audit and fix any hardcoded text)
- [ ] T085 [P] Add keyboard navigation support to dialogs
- [ ] T086 Run full verification: `pnpm rust:format && pnpm rust:clippy && pnpm rust:test && pnpm format && pnpm lint && pnpm check`
- [ ] T087 Run quickstart.md checklist validation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies - start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 completion - BLOCKS all UI work
- **Phase 3-8 (User Stories)**: All depend on Phase 2 completion
  - US1, US2, US3 are all P1 - implement in order (US1 → US2 → US3)
  - US4, US5 are P2 - can start after P1 stories complete
  - US6 is P3 - implement last
- **Phase 9 (Polish)**: Depends on all desired user stories complete

### User Story Dependencies

| Story               | Priority | Can Start After | Notes                                |
| ------------------- | -------- | --------------- | ------------------------------------ |
| US1 (View)          | P1       | Phase 2         | Foundation for all other stories     |
| US2 (Create/Manage) | P1       | US1             | Needs inventory list to test against |
| US3 (Add Purchase)  | P1       | US2             | Needs inventory to add purchases to  |
| US4 (History)       | P2       | US3             | Needs purchases to display history   |
| US5 (Products)      | P2       | US3             | Enhances purchase flow               |
| US6 (Required)      | P3       | US1             | Independent of US2-5                 |

### Parallel Opportunities per Phase

**Phase 1**:

- T003, T004 can run in parallel (different infrastructure files)
- T008, T009 can run in parallel (different query files)

**Phase 2**:

- T015, T016 in parallel (en.json vs it.json)
- T021, T022 in parallel (different route files)

**Phase 3 (US1)**:

- T025, T026 in parallel (sidebar vs bottom nav)
- T029, T030 in parallel (card vs list components)

**Phase 4-8**: Similar patterns - components marked [P] can be parallelized

---

## Summary

| Phase                  | Task Count | Priority  |
| ---------------------- | ---------- | --------- |
| Setup                  | 16         | Required  |
| Foundational           | 10         | Required  |
| US1 - View             | 13         | P1 🎯 MVP |
| US2 - Create/Manage    | 11         | P1        |
| US3 - Add Purchase     | 9          | P1        |
| US4 - Purchase History | 5          | P2        |
| US5 - Track Products   | 9          | P2        |
| US6 - Required Qty     | 10         | P3        |
| Polish                 | 7          | Final     |
| **Total**              | **90**     |           |

### MVP Scope

For minimum viable product, complete:

- Phase 1 (Setup)
- Phase 2 (Foundational)
- Phase 3 (US1 - View Track Inventory)

This delivers: Navigation to "My Tracks", view inventory list, view inventory details with stock quantities.

### Independent Test Criteria

| Story | How to Test Independently                                    |
| ----- | ------------------------------------------------------------ |
| US1   | Navigate to /my-tracks, see list, click inventory, see items |
| US2   | Create inventory, rename it, delete it                       |
| US3   | Add purchase to inventory, verify quantity increases         |
| US4   | View inventory, see purchase history list                    |
| US5   | Create track product, use it in purchase                     |
| US6   | Set required quantity, see shortage indicator                |
