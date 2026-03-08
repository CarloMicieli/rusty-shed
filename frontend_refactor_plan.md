# Frontend Refactor Plan

## Context

A deep-dive audit of the SvelteKit frontend (233 .svelte files, ~24,800 LOC) identified
five categories of technical debt: Svelte 4 legacy syntax, missing Paraglide localisation,
mega-components requiring decomposition, three deprecated Tailwind v3 classes, and untested
logic-heavy files. The goal is to bring the codebase to full Svelte 5 Runes compliance,
enforce i18n, improve maintainability through component splitting, and raise test coverage.

---

## CATEGORY A — Svelte 4 Legacy Syntax Migration

### Task A-1 — FormField.svelte: Deduplicate & Migrate `export let` → `$props()` [ ]

**The Problem:** Two byte-for-byte identical files use `export let` and `<slot />` (Svelte 4).

**Location:**
- `src/lib/shared/ui/FormField.svelte` — `export let` ×4, `<slot />`
- `src/lib/components/ui/FormField.svelte` — exact duplicate of above

**Refactor Action:**
1. Delete `src/lib/components/ui/FormField.svelte` (the legacy path).
2. Rewrite `src/lib/shared/ui/FormField.svelte` using Svelte 5 Runes:

```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    label: string;
    error?: string;
    required?: boolean;
    fieldId: string;
    children: Snippet;
  }
  const { label, error, required = false, fieldId, children }: Props = $props();
</script>

<div class="field">
  <label for={fieldId}>{label}{#if required}<span>*</span>{/if}</label>
  {@render children()}
  {#if error}<p class="error">{error}</p>{/if}
</div>
```

3. Update all import sites (`grep -r "components/ui/FormField"`) to point to the shared path.

---

### Task A-2 — Export Feature Module: Migrate `export let` + `on:click` → `$props()` + `onclick` [ ]

**The Problem:** The entire export feature is a scaffolded stub using Svelte 4 syntax. All four
components use `export let`. `ExportDialog` additionally uses `on:click`. All UI text is hardcoded
(resolved together with Task B-4).

**Location:**
- `src/lib/features/export/components/ExportDialog.svelte` — `export let` ×3, `on:click` ×2
- `src/lib/features/export/components/ExportProgress.svelte` — `export let` ×4
- `src/lib/features/export/components/ExportPreview.svelte` — `export let` ×2
- `src/lib/features/export/components/ExportReport.svelte` — `export let` ×1

**Refactor Action:** Convert all four files to `$props()` and `onclick={...}`.
Example for `ExportDialog.svelte`:

```svelte
<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  interface Props {
    isOpen: boolean;
    isLoading: boolean;
    error: string | null;
    onclose: () => void;
    onexport: () => void;
  }
  const { isOpen, isLoading, error, onclose, onexport }: Props = $props();
</script>

{#if isOpen}
  <dialog>
    <h2>{m.export_dialog_title()}</h2>
    <button onclick={onclose}>{m.export_button_cancel()}</button>
    <button onclick={onexport} disabled={isLoading}>
      {isLoading ? m.app_loading() : m.export_button_export()}
    </button>
  </dialog>
{/if}
```

Note: all required translation keys already exist in `src/paraglide/messages/en.js`.

---

## CATEGORY B — Paraglide Localisation

> **Standard import:** `import * as m from '$lib/paraglide/messages.js';`
>
> **Month/day names** must use `Intl.DateTimeFormat` (not new message keys) for automatic locale support.

### Task B-1 — DepotTable.svelte: Wire Paraglide (Critical — 15+ hardcoded strings) [ ]

**The Problem:** No Paraglide import. Table headers, status values, action button labels, and the
detail sheet panel content are entirely hardcoded in English. All matching `depot_*` keys exist.

**Location:** `src/lib/features/depot/components/DepotTable.svelte`

**Refactor Action:** Add `import * as m from '$lib/paraglide/messages.js'` and replace:
- `headers` array (`STATUS`, `VISUAL`, `ROAD NUMBER`, etc.) → `depot_*` keys
- `'On Track'` / `'In Storage'` → `m.depot_status_in_service()` and equivalent key
- Action buttons (`Update ADDR`, `Close`, `Decommission`) → `depot_*` keys
- `Displaying_Limit_Reached` and `Load Complete Buffer` → `m.depot_overflow_note()` / `m.depot_view_all()`

---

### Task B-2 — BudgetTable.svelte: Wire Paraglide (Critical — 13+ hardcoded strings) [ ]

**The Problem:** No Paraglide import despite complete `budget_table_*` and `budget_status_*` key
coverage already existing in the messages file.

**Location:** `src/lib/features/budget/components/BudgetTable.svelte`

**Refactor Action:**
- Add import and map every column header to its `budget_table_*` key.
- Replace static month name array with locale-aware formatting:
  ```ts
  import { getLocale } from '$lib/paraglide/runtime.js';
  const monthName = (month: number) =>
    new Intl.DateTimeFormat(getLocale(), { month: 'long' }).format(new Date(2000, month));
  ```
- Replace `getStatusLabel()` string returns with `m.budget_status_projected()` / `m.budget_status_completed()` / etc.

---

### Task B-3 — ImportPreview.svelte & ImportReport.svelte: Wire Paraglide (Critical) [ ]

**The Problem:** Both import feature components have 10–15 hardcoded strings each. Most matching
`import_*` keys exist; a few new ones need to be added.

**Location:**
- `src/lib/features/import/components/ImportPreview.svelte`
- `src/lib/features/import/components/ImportReport.svelte`

**Refactor Action:** Add `import * as m from '$lib/paraglide/messages.js'` to both files. Map all
visible text to `import_*` keys. Add missing keys to `messages/en.json` for any text without a
corresponding key (e.g. `import_total_records`, `import_new_records`, `import_duplicate_records`,
`import_image_failures`).

---

### Task B-4 — Export Module: Wire Paraglide (handled alongside A-2) [ ]

**The Problem:** All four export stub components contain fully hardcoded UI text.

**Location:** `src/lib/features/export/components/` (all 4 files)

**Refactor Action:** Handled as part of Task A-2. All required keys already exist:
`export_dialog_title`, `export_button_export`, `export_button_cancel`, `export_preview_title`, etc.
No new keys needed.

---

### Task B-5 — CollectionSummary.svelte & DeleteModal.svelte: Wire Paraglide [ ]

**The Problem:** No Paraglide import. Stat card labels and confirmation button text are hardcoded.

**Location:**
- `src/lib/features/collection/components/CollectionSummary.svelte`
- `src/lib/features/collection/components/DeleteModal.svelte`

**Refactor Action:** Add import. Replace:
- Stat labels (`Collection value`, `Total units`, `Locomotives`, …) → `stats_total_collection_value`,
  `constants_categories_*` keys
- `Cancel` / `Confirm` buttons → `common_cancel` / `common_delete` (both already exist)

---

### Task B-6 — Budget Charts: Wire Paraglide (BudgetDonutChart, YearlySpendingChart, BudgetMonthRow) [ ]

**The Problem:** Donut chart center labels, spending chart month abbreviations, and `[Active]` badge
are hardcoded.

**Location:**
- `src/lib/features/budget/components/BudgetDonutChart.svelte`
- `src/lib/features/budget/components/YearlySpendingChart.svelte`
- `src/lib/features/budget/components/BudgetMonthRow.svelte`

**Refactor Action:** Add import. Use existing `budget_*` keys for `Remaining` / `Available`.
Replace month abbreviations in `YearlySpendingChart` with `Intl.DateTimeFormat(locale, { month: 'short' })`.
Add one new key `budget_month_active` for the `[Active]` badge.

---

### Task B-7 — DigitalSummary.svelte & WishlistItemCard.svelte: Wire Paraglide [ ]

**The Problem:** Summary stat labels and wishlist card field labels (`Price Target`, `Product Code`,
`Move`, `Purchase`, `High Priority`) are hardcoded.

**Location:**
- `src/lib/features/digital-roster/components/DigitalSummary.svelte`
- `src/lib/features/wishlists/components/WishlistItemCard.svelte`

**Refactor Action:** Add import to both. Map `DigitalSummary` to existing `digital_roster_*` keys.
For `WishlistItemCard`, add missing keys `wishlists_price_target`, `wishlists_move`,
`wishlists_high_priority` to `messages/en.json`.

---

### Task B-8 — CreateRailwayModel.svelte: Sidebar Labels & Error Messages [ ]

**The Problem:** The catalogue creation sidebar field labels and form validation error messages are
hardcoded English strings.

**Location:** `src/lib/features/catalogue/CreateRailwayModel.svelte`

**Refactor Action:** Add `import * as m from '$lib/paraglide/messages.js'`. Map sidebar labels
(`Product Code`, `Scale`, `Power`, `Rolling Stock`) to existing `form_new_model_*` keys. Add new
keys for validation error messages:
- `form_error_required_fields` — "Please fill in all required fields (including English description)"
- `form_error_no_rolling_stock` — "At least one rolling stock is required"
- `form_error_unexpected` — "An unexpected error occurred"

---

### Task B-9 — Low-Priority Remaining Files: Wire Paraglide [ ]

**The Problem:** Minor hardcoded strings scattered across smaller components.

**Locations and fixes:**

| File | Hardcoded Strings | Action |
|---|---|---|
| `src/lib/features/import/components/ImportDropZone.svelte` | 3 | Use existing `import_dropzone_*` keys |
| `src/lib/features/maintenance/components/MaintenanceEventTimeline.svelte` | 1 | Add `maintenance_no_events` key |
| `src/lib/features/track-inventory/components/PurchaseHistoryItem.svelte` | 2 (`Qty`, `Total`) | Add `track_purchase_qty`, `track_purchase_total` keys |
| `src/lib/features/depot/components/DepotCategory.svelte` | 1 (`UNITS` badge) | Use `depot_*` key |
| `src/routes/railway-tracks/[id]/+page.svelte` | 2 (error / not found) | Add `track_inventories_not_found` key |
| `src/lib/components/model-details/ImageDropZone.svelte` | 3 (format error messages) | Use `upload_error_unsupported_format` with format param |
| `src/lib/components/model-details/ImageUpload.svelte` | 1 (`Cancel` in delete dialog) | Use existing `common_cancel` |
| `src/lib/components/RichTextToolbar.svelte` | 4 (`aria-label` attributes) | Add `toolbar_bold`, `toolbar_italic`, `toolbar_bullet_list`, `toolbar_ordered_list` keys |
| `src/routes/+error.svelte` | 1 (`<title>`) | Use `m.app_name()` |
| `src/lib/features/depot/components/DepotStatusFooter.svelte` | 5 (terminal-style labels) | Evaluate if intentional design; document as exception if so |

---

## CATEGORY C — Mega-Component Decomposition

> **Architecture rule:** Sub-components must live in a `components/` subfolder within the same
> feature directory as the parent (e.g. `features/depot/components/DepotDetailSheet.svelte`).

### Task C-1 — RailwayModelCard.svelte: Extract 4 Sub-Components (1,127 lines — CRITICAL) [ ]

**The Problem:** A single component handles image upload/drag-drop/crop, inline description
editing, scale/era inline editing, tab navigation, and rolling stock unit management.

**Location:** `src/lib/components/RailwayModelCard.svelte`

**Refactor Action:** Extract into `src/lib/components/model-details/`:
- `RailwayModelCardHeader.svelte` — manufacturer, product code, scale, era, power metadata display
- `RailwayModelImagePanel.svelte` — drag-and-drop area, upload button, `ImageCropDialog` trigger
- `RailwayModelDescriptionEditor.svelte` — inline description field with save/cancel
- `RailwayModelTabsContainer.svelte` — thin tab switcher (details / rolling stock)

`RailwayModelCard.svelte` becomes a thin orchestrator ~150 lines composing the four above.

---

### Task C-2 — RollingStockCard.svelte: Extract 3 Sub-Components (787 lines — CRITICAL) [ ]

**The Problem:** Collapsed/expanded modes, identification fields, and full technical spec section
are all in one file.

**Location:** `src/lib/components/model-details/RollingStockCard.svelte`

**Refactor Action:** Extract into `src/lib/components/model-details/components/`:
- `RollingStockCardHeader.svelte` — collapsed row (series, road number, livery, expand toggle)
- `RollingStockIdentificationFields.svelte` — inline-editable identification fields group
- `RollingStockTechnicalSpecs.svelte` — flywheel, body shell, chassis, lighting (expanded section)

---

### Task C-3 — RollingStockSpecsDrawer.svelte: Extract 3 Sub-Components (648 lines — CRITICAL) [ ]

**The Problem:** Full-page drawer with ~14 form fields managed in one monolithic component.

**Location:** `src/lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte`

**Refactor Action:** Extract into `src/lib/features/rolling-stock-edit/components/`:
- `RollingStockBasicFields.svelte` — series code, road number, livery, depot
- `RollingStockTechnicalFields.svelte` — flywheel, body shell, chassis, lights, DCC interface, control, coupling
- `DrawerActionFooter.svelte` — Save/Cancel/Discard buttons (reusable across all drawers)

---

### Task C-4 — AddRailwayModelDrawer & AddModelDrawer: Share a Common Base Form (627 lines each — CRITICAL) [ ]

**The Problem:** Both drawers share near-identical form structure but are maintained as separate
627-line copies.

**Location:**
- `src/lib/features/wishlists/components/AddRailwayModelDrawer.svelte`
- `src/lib/features/collection/components/AddModelDrawer.svelte`

**Refactor Action:** Extract a shared `RailwayModelBaseForm.svelte` into `src/lib/shared/components/`
containing manufacturer, product code, description, category, scale, power method, and epoch fields.
Each drawer composes this base form and adds its own context-specific section:
- Collection drawer → purchase info fields
- Wishlists drawer → priority and price target fields

---

### Task C-5 — DecoderInstallDrawer.svelte: Extract 3 Sub-Components (496 lines — CRITICAL) [ ]

**The Problem:** Rolling stock selection, decoder selection, DCC address input, and two confirmation
dialogs live in a single component.

**Location:** `src/lib/features/digital-roster/components/DecoderInstallDrawer.svelte`

**Refactor Action:** Extract into `src/lib/features/digital-roster/components/`:
- `DecoderRollingStockPicker.svelte` — searchable list of installable rolling stocks
- `DecoderPicker.svelte` — decoder search (manufacturer filter + decoder list)
- `DecoderInstallConfirmDialog.svelte` — "are you sure" confirmation overlay

---

### Task C-6 — CreateRailwayModel.svelte & RollingStockSection.svelte (452 + 379 lines — HIGH) [ ]

**The Problem:** The catalogue creation form embeds sidebar orchestration logic inline, and its
rolling stock accordion section is itself a mega-component with 14+ conditional category-specific
fields.

**Location:**
- `src/lib/features/catalogue/CreateRailwayModel.svelte`
- `src/lib/features/catalogue/components/RollingStockSection.svelte`

**Refactor Action:**
- From `CreateRailwayModel`: extract `CatalogueFormSidebar.svelte` into
  `src/lib/features/catalogue/components/`
- From `RollingStockSection`: extract `RollingStockCategoryFields.svelte` (category-specific
  subtype selects) and `RollingStockControlFields.svelte` (control, DCC interface, service level)

---

### Task C-7 — High-Priority Drawers & Sidebars (200–413 lines) [ ]

**The Problem:** Multiple feature-module components exceed the 200-line readability threshold.

**Locations (in priority order):**

| File | Lines | Suggested Extraction |
|---|---|---|
| `src/lib/features/collection/components/CollectionItemSidebar.svelte` | 413 | `CollectionItemDetails.svelte`, `CollectionPurchaseInfo.svelte` |
| `src/lib/features/track-inventory/components/AddPurchaseDialog.svelte` | 393 | `PurchaseFormFields.svelte` |
| `src/lib/features/track-inventory/components/CreateProductDialog.svelte` | 358 | `ProductFormFields.svelte` |
| `src/lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte` | 348 | Reuse `RollingStockBasicFields` from C-3 |
| `src/lib/features/wishlists/components/PurchaseDialog.svelte` | 309 | `PurchasePaymentFields.svelte` |
| `src/lib/features/depot/components/DepotTable.svelte` | 307 | `DepotDetailSheet.svelte` (detail action sheet) |
| `src/lib/components/SettingsForm.svelte` | 295 | Relocate imports to canonical `src/lib/features/settings/components/SettingsForm.svelte` (146 lines) and delete the `src/lib/components/` copy |
| `src/routes/dashboard/+page.svelte` | 240 | `DashboardCharts.svelte` |
| `src/lib/features/wishlists/WishlistsDashboard.svelte` | 242 | `WishlistFilterBar.svelte` |

---

## CATEGORY D — Tailwind v4 Enforcement

### Task D-1 — InventoryItemRow.svelte: Remove Deprecated Opacity Modifier Classes [ ]

**The Problem:** Three Tailwind v3 standalone opacity modifier classes (`border-opacity-*`,
`bg-opacity-*`) are present. These are removed in Tailwind v4 and have no effect.

**Location:** `src/lib/features/track-inventory/components/InventoryItemRow.svelte` (lines 72, 74, 76)

**Refactor Action:** Replace with v4 slash-opacity syntax on the colour utility class:

```svelte
<!-- BEFORE (v3 — removed in v4) -->
class:border-opacity-5={!hasShortage}
class:bg-opacity-10={hasShortage}
class:border-opacity-20={hasShortage}

<!-- AFTER (v4 correct) -->
class:border-white/5={!hasShortage}
class:bg-red-950/10={hasShortage}
class:border-red-500/20={hasShortage}
```

> **Overall Tailwind v4 status: GOOD.** `src/routes/layout.css` correctly uses
> `@import "tailwindcss"`, an `@theme inline` block, and no `tailwind.config.js`. No further
> structural changes needed. Optionally verify that `@tailwindcss/forms` (loaded via `@plugin`) is
> still required; remove if unused.

---

## CATEGORY E — Test Coverage Gaps

### Task E-1 — Import Controller: Add Unit Tests [ ]

**The Problem:** Zero test coverage for the import orchestration logic.

**Location:** `src/lib/features/import/import.controller.svelte.ts` (145 lines)

**Refactor Action:** Create `src/__tests__/lib/features/import/import.controller.test.ts`.
Cover: file parsing validation, progress state transitions (idle → running → complete/error),
cancellation handling.

---

### Task E-2 — TrackInventoryService: Add Unit Tests [ ]

**The Problem:** No tests for inventory management business logic.

**Location:** `src/lib/features/track-inventory/services/TrackInventoryService.svelte.ts` (105 lines)

**Refactor Action:** Create `src/__tests__/lib/features/track-inventory/service.test.ts`.
Cover: CRUD operations, shortage calculation, purchase aggregation logic.

---

### Task E-3 — SearchService: Add Unit Tests [ ]

**The Problem:** No tests for search filtering and ranking logic.

**Location:** `src/lib/features/search/SearchService.svelte.ts` (86 lines)

**Refactor Action:** Create `src/__tests__/lib/features/search/SearchService.test.ts`.
Cover: empty query, single match, multi-token match, category filtering, empty result set.

---

### Task E-4 — Zod Schemas: Add Unit Tests (railway-model.ts) [ ]

**The Problem:** The core domain schema (299 lines of Zod validators) has zero test coverage.
Schema regressions have high blast radius across the entire form layer.

**Location:** `src/lib/schemas/railway-model.ts`

**Refactor Action:** Create `src/__tests__/lib/schemas/railway-model.test.ts`.
Cover: valid payloads passing, missing required fields, enum boundary values, nested rolling stock
validation, optional fields defaulting correctly.

---

### Task E-5 — Catalogue Utils & Export Controller: Add Unit Tests [ ]

**The Problem:** Data transformation utilities and export orchestration logic are untested.

**Location:**
- `src/lib/features/catalogue/utils.ts` (145 lines)
- `src/lib/features/export/export.controller.svelte.ts` (60 lines)

**Refactor Action:**
- Create `src/__tests__/lib/features/catalogue/utils.test.ts` — cover `resolveLabel`, category
  mapping, and data-shaping functions.
- Create `src/__tests__/lib/features/export/export.controller.test.ts` — cover state transitions
  and error handling paths.

---

## Execution Order

| Phase | Tasks | Rationale |
|---|---|---|
| 1 — Blockers | A-1, A-2, D-1 | Syntax violations and deprecated classes — small and surgical |
| 2 — i18n Critical | B-1, B-2, B-3, B-4 | Files with the most violations; existing keys cover most |
| 3 — i18n Remaining | B-5 through B-9 | Remaining localisation, some new keys needed |
| 4 — Decomposition | C-1 through C-6 | Largest components first |
| 5 — Decomposition | C-7 | Medium-sized components |
| 6 — Tests | E-1 through E-5 | Coverage added after structure is stable |

---

## Verification Checklist

Run after completing each phase:

- [ ] `pnpm check` — `svelte-check` passes with **zero** errors
- [ ] `pnpm lint` — ESLint passes with **zero** errors/warnings
- [ ] `pnpm test` — all Vitest tests pass
- [ ] Manual smoke test — Collection, Depot, Budget, Import, and Wishlists routes render correctly
- [ ] After any component restructure — `pnpm tauri dev` to confirm IPC bindings still resolve
