# Data Model: Maintenance Page Overhaul

**Branch**: `034-maintenance-overhaul`
**Date**: 2026-03-05

---

## Schema Changes

### New Migration: `0016_maintenance_card_unique_stock_id.sql`

```sql
-- Enforce 1:1 relationship: one maintenance card per owned rolling stock.
CREATE UNIQUE INDEX IF NOT EXISTS idx_maintenance_cards_owned_rolling_stock_id
    ON maintenance_cards (owned_rolling_stock_id);
```

**Rationale**: Prevents duplicate maintenance cards at the data layer. Any existing duplicates must be resolved before this migration runs. The application layer maps the resulting `UNIQUE constraint failed` error to a user-facing conflict message.

---

## View Type Changes (Rust)

### New Struct: `RollingStockDisplayInfo` (`interface/views.rs`)

```rust
/// Human-readable identity information sourced from the catalog rolling stock
/// and railway model tables. All fields are optional because a rolling stock
/// may not have a catalog entry.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RollingStockDisplayInfo {
    /// Manufacturer display name (e.g. "Bachmann", "Märklin").
    pub manufacturer_name: Option<String>,
    /// Catalog product code (e.g. "32-504").
    pub product_code: Option<String>,
    /// Series code from the rolling stock record (e.g. "Class 66").
    pub series_code: Option<String>,
    /// Road number / running number (e.g. "66001").
    pub road_number: Option<String>,
}
```

### Extended Struct: `MaintenanceCardView` (`interface/views.rs`)

Add one new field to the existing struct:

```rust
/// Human-readable identity derived from the catalog at query time.
/// None when the owned rolling stock has no catalog entry.
pub display_info: Option<RollingStockDisplayInfo>,
```

Frontend binding impact: The generated TypeScript type gains `displayInfo: RollingStockDisplayInfo | null`.

---

## Repository Query Changes

### `list_due_card_views` — enhanced SQL

Replaces the current single-table SELECT with a JOIN query that pulls display info in one pass:

```sql
SELECT
    mc.id,
    mc.owned_rolling_stock_id,
    mc.last_maintenance_date,
    mc.next_maintenance_date,
    mc.created_at,
    mc.updated_at,
    mc.version,
    mfr.name            AS manufacturer_name,
    rm.product_code     AS product_code,
    rs.series_code      AS series_code,
    rs.road_number      AS road_number
FROM maintenance_cards mc
LEFT JOIN owned_rolling_stocks ors ON mc.owned_rolling_stock_id = ors.id
LEFT JOIN rolling_stocks rs        ON ors.rolling_stock_id = rs.id
LEFT JOIN railway_models rm        ON rs.railway_model_id = rm.id
LEFT JOIN manufacturers mfr        ON rm.manufacturer_id = mfr.id
WHERE mc.next_maintenance_date <= date('now')
   OR (mc.next_maintenance_date IS NULL AND mc.last_maintenance_date IS NOT NULL
       AND mc.last_maintenance_date <= date('now'))
   OR (mc.next_maintenance_date IS NULL AND mc.last_maintenance_date IS NULL)
```

### `find_view_by_id` — same JOIN applied

The existing single-card query is updated with the same JOIN pattern for the detail page.

### Error mapping for UNIQUE constraint

In the `save` method of `SqliteMaintenanceRepository`, when handling `MaintenanceCardEvent::Created`, the SQLite error message is inspected:

```rust
.map_err(|e| {
    if e.to_string().contains("UNIQUE constraint failed") {
        DomainError::Conflict(
            "A maintenance card already exists for this rolling stock.".to_string()
        )
    } else {
        DomainError::from(e)  // existing error mapping
    }
})?;
```

---

## New Tauri Command: `get_maintenance_card`

**Location**: `src-tauri/src/maintenance/interface/command_handlers.rs`

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_maintenance_card(
    state: tauri::State<'_, AppState>,
    card_id: MaintenanceCardId,
) -> Result<Option<MaintenanceCardView>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let mut repo = unit_of_work.maintenance_repository();
    let view = repo.find_view_by_id(&card_id).await.map_err(CommandError::from)?;
    drop(repo);
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(view)
}
```

---

## New Frontend Route

### `src/routes/maintenance/[id]/+page.svelte`

- Reads `params.id` from SvelteKit page params.
- Loads card via `get_maintenance_card` command on mount.
- Provides "Back" button linking to `/maintenance`.
- Renders `MaintenanceDetailHeader`, `MaintenanceStatsRow`, `AddEventButton`, `MaintenanceEventTimeline`.

### New State: `MaintenanceDetailState.svelte.ts`

- Fields: `#card: MaintenanceCardView | null`, `#isLoading`, `#error`.
- Methods: `loadCard(id: string)`, `addEvent(args: AddMaintenanceArgs)`.
- Optimistic update: after `addEvent`, prepends new event to `#card.events` before re-fetch completes.

---

## Entity Summary

| Entity                                        | Change                                              | Location                                   |
| --------------------------------------------- | --------------------------------------------------- | ------------------------------------------ |
| `maintenance_cards` (DB table)                | Add UNIQUE index on `owned_rolling_stock_id`        | Migration 0016                             |
| `MaintenanceCardView` (Rust view)             | Add `display_info: Option<RollingStockDisplayInfo>` | `interface/views.rs`                       |
| `RollingStockDisplayInfo` (Rust view)         | New struct                                          | `interface/views.rs`                       |
| `MaintenanceRepository::find_view_by_id`      | Enhanced JOIN                                       | `infrastructure/sqlite_repository.rs`      |
| `MaintenanceRepository::list_due_card_views`  | Enhanced JOIN                                       | `infrastructure/sqlite_repository.rs`      |
| `MaintenanceRepository::save` (Created event) | Map UNIQUE error to DomainError::Conflict           | `infrastructure/sqlite_repository.rs`      |
| `get_maintenance_card` Tauri command          | New command                                         | `interface/command_handlers.rs`            |
| `MaintenanceDetailState`                      | New Svelte state class                              | `src/lib/features/maintenance/`            |
| `SidebarNavigation`                           | Fix active check: startsWith                        | `src/lib/features/navigation/components/`  |
| `/maintenance/[id]/+page.svelte`              | New detail route                                    | `src/routes/maintenance/[id]/`             |
| `MaintenanceCardItem.svelte`                  | Use `displayInfo` for title/badge                   | `src/lib/features/maintenance/components/` |
| `AddMaintenanceEventModal` (main page)        | Remove from main page header                        | `src/routes/maintenance/+page.svelte`      |
| Detail-scoped Add Event                       | Context-aware, card ID pre-filled                   | `src/lib/features/maintenance/components/` |
