# Frontend Deep-Dive Audit — Refactor Plan

## Context

This plan is the result of a comprehensive automated + manual audit of the 272-file SvelteKit frontend. It identifies remaining gaps across four areas: Paraglide localization, mega-component decomposition, Tailwind 4 compliance, and test coverage. Svelte 5 Runes migration is already **100% complete** — no legacy patterns remain.

---

## Audit Findings Summary

| Area            | Status                 | Action Needed                                                            |
| --------------- | ---------------------- | ------------------------------------------------------------------------ |
| Svelte 5 Runes  | ✅ Complete            | None — no `export let`, `$:`, `createEventDispatcher`, or `<slot>` found |
| Tailwind 4      | ✅ Clean               | None — `@import 'tailwindcss'` used, no `tailwind.config.js` at root     |
| Paraglide       | ⚠️ Gaps                | ~15 files, ~30 hardcoded strings                                         |
| Mega-Components | ⚠️ 30 files >200 lines | 7 files >400 lines need decomposition                                    |
| Test Coverage   | ⚠️ Gaps                | 5 service/util files without tests                                       |

---

## Section A — Tailwind 4 (No Action Required)

The project correctly uses the Tailwind v4 approach:

- `src/routes/layout.css`: `@import 'tailwindcss'` + `@plugin '@tailwindcss/typography'`
- CSS-native `@custom-variant dark` and `:root` / `.dark` `@theme` blocks
- No `tailwind.config.js` or `postcss.config.js` at project root
- **No deprecated v3 utilities detected**

---

## Section B — Paraglide Localization Gaps

All affected files already import `* as m from '$lib/paraglide/messages'`. The work is purely adding message keys and replacing literal strings.

### B-1: `src/lib/features/track-inventory/components/PurchaseFormFields.svelte`

- **Problem:** Hardcoded field labels `"Market Price (Total)"`, `"Transaction Date"`, and placeholder text
- **Action:** Add `m.track_purchase_field_market_price()`, `m.track_purchase_field_transaction_date()` — also migrate any placeholder strings

### B-2: `src/lib/features/wishlists/components/WishlistSidebar.svelte`

- **Problem:** `"My Lists"` heading (line 18), `"Default"` badge (line 62), `title="Delete List"` (line 69)
- **Action:** Add `m.wishlists_sidebar_title()`, `m.wishlists_sidebar_default_badge()`, `m.wishlists_delete_list_title()`

### B-3: `src/lib/features/wishlists/components/WishlistItemDetail.svelte` (or similar)

- **Problem:** `"Wishlist Details"` heading, `"No rolling stocks added yet."` empty state
- **Action:** Add `m.wishlist_details_heading()`, `m.wishlist_no_rolling_stocks()`

### B-4: `src/lib/features/maintenance/components/AddMaintenanceCardModal.svelte`

- **Problem:** `aria-label="Close"` (line 94)
- **Action:** Replace with `aria-label={m.dialog_close_button()}`

### B-5: `src/lib/features/collection/components/FilterPanel.svelte`

- **Problem:** `title="Close filters"` (line 66)
- **Action:** Replace with `title={m.filter_panel_close_title()}`

### B-6: `src/lib/components/model-details/RailwayModelImagePanel.svelte`

- **Problem:** `aria-label="Railway model image"` (line 59), `"No image available"` (lines 89, 99)
- **Action:** Add `m.railway_model_image_alt()`, `m.railway_model_no_image()`

### B-7: `src/lib/components/model-details/ImageDropZone.svelte`

- **Problem:** `"Drop here to update photo"` (line 127)
- **Action:** Add `m.image_drop_zone_drag_message()`

### B-8: `src/lib/features/track-inventory/components/CreateInventoryDialog.svelte`

- **Problem:** Hardcoded placeholder text
- **Action:** Audit and migrate all `placeholder="..."` attributes to `m.*()` calls

### B-9: Bulk `aria-label` and `title` audit

- **Problem:** Multiple components use `aria-label="..."` and `title="..."` with English literals
- **Scope:** Run `grep -r 'aria-label="' src/lib --include="*.svelte"` to get full list
- **Action:** Replace each with corresponding `m.*()` key, adding new keys to `messages/en.json`

---

## Section C — Mega-Component Decomposition

All sub-components must be placed in the **same feature directory** as the parent.

### C-1 (CRITICAL): `src/lib/components/model-details/RailwayModelTabsContainer.svelte` — 675 lines

- **Problem:** Giant component handling Details tab (rich-text editor + language fallback) AND Rolling Stock tab (single-unit view, multi-unit accordion, all save handlers). Contains 4+ async save functions and a complex `RsFormState` mapping.
- **Extract:**
  - `src/lib/components/model-details/components/ModelDetailsTab.svelte` — Details tab content (rich text editor + language badge UI)
  - `src/lib/components/model-details/components/RollingStockSingleUnit.svelte` — Single-unit hero card + 3-column spec grid
  - `src/lib/components/model-details/components/RollingStockMultiUnit.svelte` — Multi-unit accordion view
  - Keep save handlers in parent or extract to a `useRollingStockEditor.svelte.ts` Runes-based controller
- **Target:** Parent reduced to ~150 lines (tab shell + state orchestration)

### C-2 (HIGH): `src/lib/features/wishlists/components/AddRailwayModelDrawer.svelte` — 474 lines

- **Problem:** Combines search/filter UI, result list, form state, and drawer shell
- **Extract:**
  - `src/lib/features/wishlists/components/RailwayModelSearchBar.svelte` — search input + filter
  - `src/lib/features/wishlists/components/RailwayModelSearchResults.svelte` — results list
  - `src/lib/features/wishlists/components/AddRailwayModelForm.svelte` — form fields
- **Target:** Parent reduced to ~150 lines

### C-3 (HIGH): `src/lib/features/collection/components/AddModelDrawer.svelte` — 432 lines

- **Problem:** Combines model search, selection UI, category selectors, and form fields in one component
- **Extract:**
  - `src/lib/features/collection/components/ModelSearchSection.svelte` — search input + results
  - `src/lib/features/collection/components/ModelSelectionCard.svelte` — selected model preview
- **Target:** Parent reduced to ~200 lines

### C-4 (MEDIUM): `src/lib/features/catalogue/CreateRailwayModel.svelte` — 385 lines

- **Note:** Previously flagged as "won't do" due to superform binding complexity. Re-evaluate if the superform bindings can be cleanly isolated.
- **Decision:** Keep deferred unless regression risk is acceptable.

### C-5 (MEDIUM): `src/lib/components/RailwayModelPreviewCard.svelte` — 331 lines

- **Problem:** Contains multiple conditional rendering blocks for preview vs. edit modes
- **Extract:** `src/lib/components/model-details/components/PreviewCardActions.svelte` — action buttons section

### C-6 (MEDIUM): `src/lib/features/catalogue/components/RollingStockCategoryFields.svelte` — 328 lines

- **Problem:** Long component with many repeated field groups
- **Extract:** `src/lib/features/catalogue/components/RollingStockScaleFields.svelte` — scale/gauge selector group

### C-7 (LOW): `src/lib/components/model-details/components/RollingStockTechnicalSpecs.svelte` — 300 lines

- **Problem:** Renders many spec rows in a grid; could use a data-driven row component
- **Extract:** `src/lib/components/model-details/components/SpecRow.svelte` — single label+value row

---

## Section D — Test Coverage Gaps

### D-1: `src/lib/features/maintenance/utils/urgency.ts`

- **Problem:** Pure date-math logic with zero tests
- **Action:** Create `src/__tests__/lib/features/maintenance/urgency.test.ts`
- **Cases:** `overdue` (past date), `warning` (≤7 days), `normal` (>7 days), `null` input

### D-2: `src/lib/features/maintenance/services/MaintenanceService.ts` — 57 lines

- **Problem:** Service with Tauri command calls, no direct unit tests
- **Action:** Create `src/__tests__/lib/features/maintenance/MaintenanceService.test.ts`
- **Pattern:** Mock `commands` from `$lib/bindings` (see `src/__tests__/lib/features/collection/service.test.ts` as reference)

### D-3: `src/lib/features/collection/utils/modelViewMapper.ts` — 289 lines

- **Problem:** Critical data transformation utility with no tests
- **Action:** Create `src/__tests__/lib/features/collection/modelViewMapper.test.ts`
- **Cases:** Full model → view mapping, null/optional fields, all status variants

### D-4: `src/lib/schemas/seller.ts` + `src/lib/schemas/settings.ts`

- **Problem:** Zod validation schemas without coverage
- **Action:** Create `src/__tests__/lib/schemas/seller.test.ts` and `settings.test.ts`
- **Pattern:** Follow `src/__tests__/lib/schemas/railway-model.test.ts` (valid/invalid inputs per field)

### D-5: `src/lib/features/collection/domain/FilterState.ts`

- **Problem:** Domain state class (likely a Svelte Rune-based state object) without tests
- **Action:** Create `src/__tests__/lib/features/collection/FilterState.test.ts`
- **Pattern:** Follow `src/__tests__/state/CollectionState.svelte.test.ts`

### D-6 (LOWER PRIORITY): Additional untested services

The following services have no unit tests and should be addressed after D-1 through D-5:

- `src/lib/services/settings.ts` — settings management service
- `src/lib/services/database-backup.ts` — backup logic (data integrity)
- `src/lib/features/depot/services/DepotService.svelte.ts` — inventory management
- `src/lib/features/budget/services/BudgetService.svelte.ts` — financial calculations
- `src/lib/services/sellerService.ts` — seller domain service (adapter IS tested, service is not)

**Note:** `shadow-xs` and `shadow-sm` in `src/lib/components/ui/` are **shadcn-svelte auto-generated** files — do not edit manually. These utilities are valid in Tailwind v4 (`shadow-xs` was added in v4). No Tailwind changes needed.

---

## Critical Files

| File                                                                      | Area        | Lines |
| ------------------------------------------------------------------------- | ----------- | ----- |
| `src/lib/components/model-details/RailwayModelTabsContainer.svelte`       | C-1         | 675   |
| `src/lib/features/wishlists/components/AddRailwayModelDrawer.svelte`      | C-2         | 474   |
| `src/lib/features/collection/components/AddModelDrawer.svelte`            | C-3         | 432   |
| `src/lib/features/catalogue/CreateRailwayModel.svelte`                    | C-4 (defer) | 385   |
| `src/lib/components/RailwayModelPreviewCard.svelte`                       | C-5         | 331   |
| `src/lib/features/catalogue/components/RollingStockCategoryFields.svelte` | C-6         | 328   |
| `src/lib/features/track-inventory/components/PurchaseFormFields.svelte`   | B-1         | 193   |
| `src/lib/features/collection/utils/modelViewMapper.ts`                    | D-3         | 289   |
| `src/lib/features/maintenance/utils/urgency.ts`                           | D-1         | ~35   |
| `src/paraglide/messages/en.json`                                          | B-\*        | —     |

## Reference Patterns to Reuse

- **Service test mocking pattern:** `src/__tests__/lib/features/collection/service.test.ts`
- **Schema test pattern:** `src/__tests__/lib/schemas/railway-model.test.ts`
- **State test pattern:** `src/__tests__/state/CollectionState.svelte.test.ts`
- **Component decomposition pattern:** `src/lib/features/import/` (ImportPreview → 5 sub-components)
- **Paraglide message key format:** snake*case, feature-prefixed (e.g., `track_purchase_field*\*`)

---

## Recommended Task Order

1. **B tasks first** — Small, high-impact, low-risk. Each file is ~5-15 min.
2. **D tasks** — Pure logic tests, no UI risk.
3. **C-1** — Highest impact decomposition (675 lines).
4. **C-2, C-3** — Next largest.
5. **C-5, C-6, C-7** — Lower priority.
6. **C-4 (CreateRailwayModel)** — Keep deferred.

---

## Verification

After each task:

1. `pnpm check` — zero TypeScript errors
2. `pnpm lint` — zero ESLint/Svelte warnings
3. `pnpm test` — all existing tests pass (currently 929 passing)
4. Visual smoke-test in `pnpm tauri dev` for any refactored component
