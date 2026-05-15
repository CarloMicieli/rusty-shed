# Research: On-the-Fly Entity Quick-Add (040)

**Date**: 2026-05-15  
**Status**: Complete — all NEEDS CLARIFICATION resolved

---

## Finding 1 — Manufacturer CRUD commands are missing from backend

**Question**: Does a `create_manufacturer` Tauri command already exist?

**Finding**: No. The current backend exposes only `get_manufacturers` and `get_manufacturer_by_id` ([catalog/interface/manufacturers.rs](../../src-tauri/src/catalog/interface/manufacturers.rs)). There is no write path for manufacturers.

**Decision**: The write commands (`create_manufacturer`, `update_manufacturer`, `delete_manufacturer`) will be delivered by feature **041-entity-management** as its Rust layer. Feature 040 consumes them as an upstream dependency. No new Rust code is created in this feature for the manufacturer domain.

**Alternatives considered**: Creating a minimal `quick_create_manufacturer` command scoped to 040 only — rejected because it would duplicate domain logic that 041 must own anyway, violating the Modular Library-First principle.

---

## Finding 2 — "Buyers" is not a separate entity in the database

**Question**: Is there a `buyers` table or a `Buyer` domain type?

**Finding**: No. The `buyer_id` column in `purchase_infos` references the `sellers` table. There is no `buyers` table, no `Buyer` struct, and no `create_buyer` command. The frontend's `SellCollectionItemArgs.buyerId` is an optional reference to a seller record. The spec mentions a "Buyer" dropdown — in practice this maps to the **sellers** table filtered by context (the user picks a seller who acts as the buyer in a resale transaction).

**Decision**: The "Quick-Add Buyer" flow in 040 reuses the **Seller** creation path (`create_seller`). The Quick-Add drawer for the "Buyer" field is identical to the Quick-Add drawer for the "Seller" field. No new entity type is introduced. The spec's FR-003 (Website + Country as optional fields) is satisfied by `CreateSellerPayload`; the `seller_type` field defaults to `SellerType::Private` when entering via the Quick-Add path.

**Alternatives considered**: Creating a separate `buyers` table — deferred to 041 where the full data model for entity management is defined. 040 will not block on this.

---

## Finding 3 — Duplicate check strategy: client-side vs. backend round-trip

**Question**: How should the real-time uniqueness check (FR-004) be implemented — a debounced Tauri `invoke` call or a client-side filter of the already-loaded list?

**Finding**: All three parent forms (`AcquisitionDrawer`, `AddWishlistItemDrawer`, `AddCollectionItemDrawer`) load the full manufacturer/seller lists on open and hold them in local `$state` arrays. The collections are expected to have ≤ 500 entities each, a negligible size for an in-memory filter.

**Decision**: Duplicate detection is performed **client-side** by comparing `name.trim().toLowerCase()` against the already-loaded array. This makes the check instantaneous (no network latency, satisfies SC-003 easily) and requires zero new Tauri commands. The backend still enforces the unique constraint as a last-resort guard.

**Rationale**: A backend round-trip would add latency and complexity for a dataset that is already in memory. The backend unique index remains the authoritative safety net.

---

## Finding 4 — SellerType is a required field in CreateSellerPayload

**Question**: When creating a seller via Quick-Add, what value is used for `seller_type`?

**Finding**: `CreateSellerPayload.seller_type: SellerType` is non-optional. `SellerType` defaults to `SellerType::Shop` (via `#[default]`). The spec calls for minimal fields (Name, Website, Country) in QUICK mode.

**Decision**: The Quick-Add form for sellers/buyers sends `seller_type: "SHOP"` as a fixed default for Quick-Add entries. Users can correct the type later via the Settings Library page (feature 041). This removes a friction-adding selector from the Quick-Add UX while satisfying the backend constraint.

---

## Finding 5 — Stacked drawer pattern: how to layer the Quick-Add on top of the parent

**Question**: How should the Quick-Add drawer sit visually above the parent `DrawerShell` without closing it?

**Finding**: `DrawerShell` is a fixed right-panel (Tailwind `fixed inset-y-0 right-0`) with no explicit z-index class. All current drawers share the same visual layer. Mounting a second `DrawerShell` would place two panels side by side or on top of each other depending on DOM order, but there is no built-in "stacking" or scrim mechanism.

**Decision**: Introduce a dedicated `QuickAddShell.svelte` component (derived from but not extending `DrawerShell`) that:
- Renders at a higher CSS layer (`z-[110]`) than the parent drawer
- Includes a semi-transparent backdrop (`z-[105]`) rendered behind itself but in front of the parent form
- Accepts an `onSuccess` callback and a `dismiss` callback
- Sets `pointer-events-none` + `opacity-70` on the parent drawer body via a `dimmed` prop passed down from the owning form

The parent `DrawerShell` receives a new optional boolean prop `dimmed?: boolean` (defaults `false`) that conditionally applies `opacity-70 pointer-events-none` to its scrollable content area. The header and footer remain unchanged (no interaction possible anyway due to overlay).

---

## Finding 6 — State hand-off: how the new entity enters the parent dropdown

**Question**: After Quick-Add saves, how does the new entity appear in the parent form's dropdown without a full re-fetch?

**Finding**: All three parent forms store the entity list in a local `$state` array (e.g., `let manufacturers: Manufacturer[] = $state([])`). The `Select.Root` / `SearchableSelect` components read from this array reactively.

**Decision**: The `create_manufacturer` command (from 041) returns the full `Manufacturer` object. The `create_seller` command already returns `Seller`. On quick-add success the owning form:

```typescript
manufacturers = [...manufacturers, newManufacturer]; // push into local state
onUpdate(item.uid, { manufacturerId: newManufacturer.id }); // auto-select
```

No re-fetch, no event bus, no store. Pure local state mutation matching existing form patterns.

---

## Finding 7 — Manufacturer name uniqueness index uses exact case, not LOWER()

**Question**: Does the `manufacturers` table currently enforce case-insensitive uniqueness?

**Finding**: The migration creates `CREATE UNIQUE INDEX IF NOT EXISTS idx_manufacturers_name ON manufacturers (name)`. This is a case-**sensitive** index. Similarly for `sellers`: `CREATE INDEX IF NOT EXISTS idx_sellers_name ON sellers(name)` — this is not even a unique index.

**Decision**: Feature 041 must add a migration that:
1. Replaces `idx_manufacturers_name` with a unique expression index: `CREATE UNIQUE INDEX idx_manufacturers_name_ci ON manufacturers (LOWER(name))`
2. Adds a similar unique expression index for `sellers`: `CREATE UNIQUE INDEX idx_sellers_name_ci ON sellers (LOWER(name))`

Feature 040 depends on 041 for this; the client-side duplicate check (Finding 3) uses `LOWER()` in JavaScript to be consistent with the planned database constraint.

---

## Finding 8 — Toast pattern and i18n keys

**Question**: What toast API and i18n key pattern does the project use?

**Finding**: The app uses `svelte-sonner` via `src/lib/toaster.ts`. All message strings come from Paraglide (`m.*` calls). Existing keys follow the pattern `{feature}_{action}_{element}` (e.g., `acquisition_toast_success`, `acquisition_error_finalize`).

**Decision**: New i18n keys for 040 follow the pattern `quick_add_{entity}_{state}`:
- `quick_add_manufacturer_success` — "Manufacturer '{name}' added and selected."
- `quick_add_seller_success` — "Seller '{name}' added and selected."
- `quick_add_duplicate_warning` — "A {entity} with this name already exists."
- `quick_add_save_failed` — "Could not save. Please try again."

Both `en.json` and `it.json` must be populated before the task is considered complete.

---

## Summary: Resolved Unknowns

| # | Unknown | Decision |
|---|---------|----------|
| 1 | `create_manufacturer` missing | Delivered by 041; 040 is frontend-only |
| 2 | "Buyers" entity doesn't exist | Reuse Seller domain; default `seller_type = SHOP` |
| 3 | Duplicate check strategy | Client-side LOWER() comparison; no extra Tauri call |
| 4 | `seller_type` required field | Fixed default `SHOP` in Quick-Add mode |
| 5 | Stacked drawer visual pattern | `QuickAddShell` at z-110 + scrim z-105 + `dimmed` prop |
| 6 | Post-save state hand-off | Push new entity into local `$state` array + auto-select |
| 7 | Name uniqueness uses exact case | Migration in 041 adds LOWER() expression indexes |
| 8 | Toast + i18n keys | `toaster.success(m.quick_add_*())` with new Paraglide keys |
