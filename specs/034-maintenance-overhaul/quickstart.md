# Quickstart: Maintenance Page Overhaul

**Branch**: `034-maintenance-overhaul`
**Date**: 2026-03-05

---

## Prerequisites

- Rust toolchain (edition 2024, `rust-version = 1.93.0`)
- `pnpm` (v10.27.0)
- SQLite via `sqlx` (already configured in `src-tauri/`)
- `cargo sqlx` for migration verification (optional, CI handles it)

---

## Implementation Order

Follow this order to avoid breaking the build at intermediate steps.

### Step 1 — Backend: Migration

Create `src-tauri/migrations/0016_maintenance_card_unique_stock_id.sql`:

```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_maintenance_cards_owned_rolling_stock_id
    ON maintenance_cards (owned_rolling_stock_id);
```

Verify: `pnpm run rust:test` — existing tests must still pass.

### Step 2 — Backend: Extend View Types

In `src-tauri/src/maintenance/interface/views.rs`:

1. Add `RollingStockDisplayInfo` struct (4 optional string fields, derive `Serialize`, `specta::Type`).
2. Add `display_info: Option<RollingStockDisplayInfo>` field to `MaintenanceCardView`.

### Step 3 — Backend: Update Repository Queries

In `src-tauri/src/maintenance/infrastructure/sqlite_repository.rs`:

1. Update `list_due_card_views` SQL to include LEFT JOINs and the 4 new columns. Map them into a `RollingStockDisplayInfo` when any are non-null.
2. Update `find_view_by_id` SQL with the same JOIN pattern.
3. In `save` (Created event arm), map `UNIQUE constraint failed` SQLite error to `DomainError::Conflict`.

### Step 4 — Backend: Add `get_maintenance_card` Command

In `src-tauri/src/maintenance/interface/command_handlers.rs`:

- Add the new `get_maintenance_card` handler.

In `src-tauri/src/maintenance/interface/mod.rs`:

- Export the new command.

In the Tauri builder (check `src-tauri/src/lib.rs` or equivalent):

- Register `get_maintenance_card` in the `invoke_handler` list.

### Step 5 — Sync Bindings

Run `pnpm tauri dev` (or the specta generation step) to regenerate `src/lib/bindings.ts`. Confirm `getMaintenanceCard`, `RollingStockDisplayInfo`, and the updated `MaintenanceCardView` type appear.

### Step 6 — Frontend: Sidebar Active State

In `src/lib/features/navigation/components/SidebarNavigation.svelte`:

- Change the Maintenance link check from `=== '/maintenance'` to `.startsWith('/maintenance')`.

### Step 7 — Frontend: Update `MaintenanceCardItem`

In `src/lib/features/maintenance/components/MaintenanceCardItem.svelte`:

- Replace the raw `card.ownedRollingStockId` title with `{card.displayInfo?.manufacturerName} {card.displayInfo?.productCode}`.
- Add the series code secondary label (muted gray, uppercase).
- Add the road number pill badge (top-right, monospaced font, amber-tinted).
- Guard all display fields with null-checks; show nothing when absent.

### Step 8 — Frontend: Maintenance Detail Route

Create `src/routes/maintenance/[id]/+page.svelte`:

- Load card via `getMaintenanceCard(params.id)` on mount.
- Show Back button (links to `/maintenance`).
- Render header, stats row, Add Event button, event timeline.
- Handle loading, error, and not-found states.

Create `src/lib/features/maintenance/MaintenanceDetailState.svelte.ts`:

- `loadCard(id)`: calls `getMaintenanceCard`, stores result.
- `addEvent(args)`: calls `addMaintenanceEvent`, optimistically prepends event to local card state.

### Step 9 — Frontend: Detail-Scoped Add Event Component

Create `src/lib/features/maintenance/components/AddEventModal.svelte` (or adapt existing):

- Props: `open: boolean`, `onClose: () => void`, `maintenanceCardId: string`.
- No card-selection dropdown (card ID is pre-filled from prop).
- Fields: Date Performed (default today), Maintenance Type (dropdown), Notes (textarea).
- On submit: calls `maintenanceState.addEvent(...)`.

### Step 10 — Frontend: Remove Broken Add Event from Main Page

In `src/routes/maintenance/+page.svelte`:

- Remove the "Add Event" `<Button>` and `showAddEventModal` state from the page header.
- Remove the `<AddMaintenanceEventModal>` import and component usage.

### Step 11 — Paraglide: Add Missing String Key

Add a new key `maintenance_card_already_exists` to `messages/` (and all locale files) for the duplicate card error message.

---

## Verification Checklist

```bash
# 1. Rust checks
pnpm run rust:test        # All tests pass including new integration tests
cargo clippy              # No warnings

# 2. Frontend checks
pnpm lint                 # No ESLint errors
pnpm check                # svelte-check + TypeScript clean
pnpm test                 # Vitest tests pass

# 3. Manual smoke test
# - Open app, navigate to Maintenance
# - Confirm sidebar "Maintenance" item stays active when clicking into a card detail
# - Confirm cards show manufacturer/product code instead of GUIDs
# - Confirm road number appears in pill badge
# - Open card detail → Add Event → submit → event appears without page reload
# - Try creating a second maintenance card for same rolling stock → error message shown
```
