# Research: Maintenance Page Overhaul

**Branch**: `034-maintenance-overhaul`
**Date**: 2026-03-05

---

## Decision 1: Enriching MaintenanceCardView with Display Identity

**Decision**: Extend `MaintenanceCardView` with an optional `display_info` sub-struct (`RollingStockDisplayInfo`) populated via a multi-table LEFT JOIN in all repository view queries.

**Rationale**: The display fields (manufacturer name, product code, series code, road number) live in catalog tables (`manufacturers`, `railway_models`, `rolling_stocks`) and are reachable from `maintenance_cards` via a 4-step LEFT JOIN chain. A JOIN at read-query time is the simplest approach and avoids denormalization.

The SQL path:

```
maintenance_cards
  LEFT JOIN owned_rolling_stocks  ON mc.owned_rolling_stock_id = ors.id
  LEFT JOIN rolling_stocks        ON ors.rolling_stock_id = rs.id
  LEFT JOIN railway_models        ON rs.railway_model_id = rm.id
  LEFT JOIN manufacturers         ON rm.manufacturer_id = mfr.id
```

Fields pulled from this join:

- `mfr.name` → `manufacturer_name`
- `rm.product_code` → `product_code`
- `rs.series_code` → `series_code`
- `rs.road_number` → `road_number`

All fields are `Option<String>` since the rolling stock may not have a catalog entry (`rolling_stock_id` on `owned_rolling_stocks` can be NULL).

**Alternatives considered**:

- Separate N+1 queries per card: rejected (performance, complexity).
- Materialized/denormalized view table: rejected (over-engineered for current scale).

---

## Decision 2: Singleton Enforcement — Unique Constraint at DB Layer

**Decision**: Add `UNIQUE (owned_rolling_stock_id)` constraint to `maintenance_cards` via a new migration (`0016_maintenance_card_unique_stock_id.sql`). Map the resulting SQLite `UNIQUE constraint failed` error in the repository to `DomainError::Conflict` with a descriptive message.

**Rationale**: A database-level unique index is the definitive guard against duplicate cards regardless of how the application layer is called. SQLite surfaces this as `UNIQUE constraint failed` in the error message, which can be pattern-matched in the repository's error handler. No business logic changes are needed in the domain or application layers beyond this infrastructure-level guard.

**Alternatives considered**:

- Application-level check-then-insert: rejected (TOCTOU race condition, weaker guarantee).
- UI-only prevention (hide already-used rolling stock): insufficient (doesn't protect integrity if called programmatically).

---

## Decision 3: Maintenance Card Detail Route

**Decision**: Create `src/routes/maintenance/[id]/+page.svelte` as a dedicated SvelteKit dynamic route. The `[id]` segment is the `MaintenanceCardId` TRN (URL-encoded). The page loads full card data via a new `get_maintenance_card` Tauri command and manages its own state.

**Rationale**: This matches the SvelteKit routing convention used throughout the project (collection item detail, wishlist item detail both use `[id]` routes). A dynamic segment isolates the detail page cleanly and allows direct linking.

**Alternatives considered**:

- Side drawer/sheet on the grid page: rejected (harder to navigate to directly, insufficient space for timeline view).
- Modal overlay on grid: rejected (poor UX for browsing event history, no direct URL).

---

## Decision 4: New Backend Command `get_maintenance_card`

**Decision**: Add a new `#[tauri::command]` handler `get_maintenance_card(card_id: MaintenanceCardId) -> Result<MaintenanceCardView, CommandError>` that calls the existing `find_view_by_id` repository method (enhanced with the JOIN described in Decision 1).

**Rationale**: The repository already implements `find_view_by_id`. It only needs to be enhanced with the JOIN. The command handler is minimal: validate input, call use-case/repo, commit, return view.

**Alternatives considered**:

- Reuse `get_maintenance_dashboard` filtered by ID: rejected (semantically incorrect; dashboard returns due/overdue cards only).

---

## Decision 5: Optimistic UI for Add Event (Local State Patch)

**Decision**: After `add_maintenance_event` succeeds, patch the detail page's reactive state by prepending the new event to the card's `events` array rather than re-fetching the full card from the backend.

**Rationale**: Meets SC-003 (event visible in < 5 seconds). The event data needed for display (date, type, notes, a client-generated UUID) is fully available from the form submission payload. The backend does not return the persisted event, but the frontend can construct a display-compatible `MaintenanceCardEventView` object immediately.

**Alternatives considered**:

- Full re-fetch after save: correct but slower; acceptable fallback if optimistic patch is complex.
- WebSocket/SSE push: over-engineered for a local desktop app.

---

## Decision 6: Sidebar Active State Fix

**Decision**: Change the Maintenance nav link's active check from `=== '/maintenance'` to `($page.url.pathname as string).startsWith('/maintenance')` in `SidebarNavigation.svelte`.

**Rationale**: The same `startsWith` pattern is already in use for the `/railway-tracks` route in the same file. It is the established convention for routes that have sub-pages.

---

## Decision 7: Context-Aware Add Event — Remove Page-Level Modal

**Decision**: Remove the `AddMaintenanceEventModal` button and import from the main maintenance page header. Create a new detail-scoped variant (or repurpose the same modal with a `maintenanceCardId` prop) used only within the detail page. The `maintenanceCardId` is passed directly from the detail page's loaded card data — no card-selection UI needed.

**Rationale**: The current modal requires the user to manually select a card from a dropdown. In the detail view, the card ID is already known from the URL. The card-select dropdown becomes obsolete and should be removed.

---

## Existing Infrastructure Confirmed

- `find_view_by_id` repository method: exists, loads card + events via two queries. Will be enhanced with JOIN.
- `find_by_rolling_stock_id` repository method: exists. Confirms existing lookup path for uniqueness check.
- `AddMaintenanceArgs` / `add_maintenance_event` command: exists and functional. The `maintenance_card_id` field is already a string (TRN). Context-aware usage from the detail page simply pre-fills this field.
- `MaintenanceType` enum: existing domain type with Paraglide-backed UI labels.
- `MaintenanceCardEventView`: existing view type — can be constructed client-side for optimistic update.
