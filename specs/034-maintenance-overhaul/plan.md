# Implementation Plan: Maintenance Page Overhaul

**Branch**: `034-maintenance-overhaul` | **Date**: 2026-03-05 | **Spec**: [spec.md](spec.md)

---

## Summary

Overhaul the Maintenance module to fix three categories of issues: (1) replace opaque GUID identifiers on maintenance cards with human-readable rolling stock identity (manufacturer, product code, series, road number) via SQL JOIN enrichment; (2) enforce a 1:1 uniqueness constraint between owned rolling stock and maintenance cards at the database level; (3) replace the broken, context-free "Add Event" modal on the main page with a context-aware event form living exclusively inside a new maintenance card detail route. Navigation active-state and breadcrumb gaps are resolved as part of the routing work.

---

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)
**Primary Dependencies**: Tauri 2.9.x, Svelte 5.48.2, SvelteKit (Vite 7.3.1), sqlx (SQLite), specta (type generation), Paraglide 2.7.1
**Storage**: SQLite via sqlx with migration files; schema change in migration 0016
**Testing**: Vitest (frontend), cargo test + sqlx::test (backend)
**Target Platform**: Desktop (Linux/macOS/Windows) via Tauri
**Project Type**: Tauri 2 desktop app — Rust backend + SvelteKit frontend
**Performance Goals**: Read queries (card list, detail) must complete < 200ms on local SQLite per constitution SLO
**Constraints**: No hardcoded strings (Paraglide required). No `unwrap()` in Rust. specta type generation required for all new Tauri commands. All new commands must validate inputs and use `Result<T, CommandError>`.
**Scale/Scope**: Single-user local desktop app. Card count expected < 1000. JOIN overhead is negligible at this scale.

---

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                               | Status | Notes                                                                                                                             |
| --------------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------- |
| Database (Persistence)                  | PASS   | New constraint via migration 0016. No ad-hoc schema changes. sqlx::migrate! already in binary.                                    |
| State Management / Domain Event Pattern | PASS   | MaintenanceCard::create already emits Created event. Repository drains pending_events. UNIQUE error mapping stays in infra layer. |
| API Design & Transport Boundary         | PASS   | New get_maintenance_card command derives specta::Type. All Args derive Debug, Clone, Deserialize. Inputs validated at boundary.   |
| Domain Logic Location                   | PASS   | Conflict detection stays in Rust (DB constraint + repo error mapping). Frontend only receives the resulting error message.        |
| Code Quality                            | PASS   | No unwrap(). All new Rust code goes through clippy. New TS follows strict mode.                                                   |
| Testing Standards                       | PASS   | New integration test for uniqueness constraint. New unit tests for AddMaintenanceCard conflict path.                              |
| User Experience Consistency             | PASS   | All new strings go through Paraglide. Design follows established charcoal/amber palette.                                          |
| Performance Requirements                | PASS   | SQL JOIN on indexed columns (owned_rolling_stock_id FK + rolling_stock_id FK). No N+1.                                            |
| Safe Rust Practices                     | PASS   | No unsafe. Error propagation via ? / map_err.                                                                                     |
| Paraglide                               | PASS   | New string key maintenance_card_already_exists added to messages/.                                                                |

**No violations. No Complexity Tracking required.**

---

## Project Structure

### Documentation (this feature)

```text
specs/034-maintenance-overhaul/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── tauri-ipc.md     # Phase 1 output
├── checklists/
│   └── requirements.md  # Spec validation checklist
└── tasks.md             # Phase 2 output (/speckit.tasks — not yet created)
```

### Source Code

```text
src-tauri/
├── migrations/
│   └── 0016_maintenance_card_unique_stock_id.sql     [NEW]
└── src/maintenance/
    ├── infrastructure/
    │   └── sqlite_repository.rs                      [MODIFIED: JOIN queries, UNIQUE error map]
    └── interface/
        ├── views.rs                                  [MODIFIED: RollingStockDisplayInfo + field]
        ├── command_handlers.rs                       [MODIFIED: new get_maintenance_card]
        └── mod.rs                                    [MODIFIED: export new command]

src/
├── routes/maintenance/
│   ├── +page.svelte                                  [MODIFIED: remove Add Event button/modal]
│   └── [id]/
│       └── +page.svelte                              [NEW: detail route]
└── lib/features/maintenance/
    ├── MaintenanceDetailState.svelte.ts               [NEW]
    └── components/
        ├── MaintenanceCardItem.svelte                 [MODIFIED: display_info fields]
        ├── AddEventModal.svelte                       [NEW: context-aware, detail-scoped]
        ├── MaintenanceEventTimeline.svelte            [NEW: vertical timeline]
        └── MaintenanceDetailHeader.svelte             [NEW: amber header for detail view]

messages/en.json (+ other locale files)               [MODIFIED: maintenance_card_already_exists]
```

---

## Implementation Phases

### Phase 1A — Backend: Schema Integrity

**Goal**: Enforce the singleton rule at the data layer.

**B-1**: Create `src-tauri/migrations/0016_maintenance_card_unique_stock_id.sql`

```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_maintenance_cards_owned_rolling_stock_id
    ON maintenance_cards (owned_rolling_stock_id);
```

**B-2**: Update `sqlite_repository.rs` — `save` method (Created event arm)

After `sqlx::query(insert_card_sql).execute(...)`, intercept `UNIQUE constraint failed` SQLite errors and map to `DomainError::Conflict("A maintenance card already exists for this rolling stock.")`.

**Verification**: `cargo test` — existing `it_creates_card_and_persists` still passes. New integration test verifies the conflict path (see Test Plan).

---

### Phase 1B — Backend: Display Identity in Views

**Goal**: Populate manufacturer, product code, series code, road number in card views.

**B-3**: Add `RollingStockDisplayInfo` struct and extend `MaintenanceCardView` in `interface/views.rs`

New struct derives `Debug, Clone, Serialize, specta::Type` with `#[serde(rename_all = "camelCase")]`. All 4 fields are `Option<String>`.

Add `pub display_info: Option<RollingStockDisplayInfo>` to `MaintenanceCardView`.

**B-4**: Update `list_due_card_views` SQL in `sqlite_repository.rs`

Extend the SELECT with 4 new aliased columns from a LEFT JOIN chain:

- `maintenance_cards LEFT JOIN owned_rolling_stocks ON mc.owned_rolling_stock_id = ors.id`
- `LEFT JOIN rolling_stocks ON ors.rolling_stock_id = rs.id`
- `LEFT JOIN railway_models ON rs.railway_model_id = rm.id`
- `LEFT JOIN manufacturers ON rm.manufacturer_id = mfr.id`

Columns added: `mfr.name AS manufacturer_name`, `rm.product_code AS product_code`, `rs.series_code AS series_code`, `rs.road_number AS road_number`.

Build `display_info` as `Some(RollingStockDisplayInfo { ... })` if at least one field is non-null, otherwise `None`.

**B-5**: Apply the same JOIN to `find_view_by_id`.

**B-6**: Add `get_maintenance_card` command to `command_handlers.rs`:

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

Register in `interface/mod.rs` and in the Tauri builder's `invoke_handler!` macro.

**Verification**: `cargo clippy` clean. `cargo test` — new integration tests verify JOIN output and the new command path.

---

### Phase 1C — Sync Bindings

**B-7**: Run `pnpm tauri dev` to regenerate `src/lib/bindings.ts`.

Confirm in `bindings.ts`:

- `getMaintenanceCard` function exported
- `RollingStockDisplayInfo` interface present
- `displayInfo` field on `MaintenanceCardView`

---

### Phase 2A — Frontend: Navigation & Card Grid

**F-1**: `SidebarNavigation.svelte` — Fix active check:

```diff
- class={navLinkClasses(($page.url.pathname as string) === '/maintenance')}
+ class={navLinkClasses(($page.url.pathname as string).startsWith('/maintenance'))}
```

**F-2**: `MaintenanceCardItem.svelte` — Use `displayInfo` fields:

- Primary heading: `{card.displayInfo?.manufacturerName ?? '—'} {card.displayInfo?.productCode ?? ''}` in bold amber
- Secondary line: `{card.displayInfo?.seriesCode}` in muted gray uppercase (render only when non-null)
- Top-right pill badge: `{card.displayInfo?.roadNumber}` in monospaced font, amber-tinted pill (render only when non-null)
- Remove: the `{card.ownedRollingStockId}` raw display

**Verification**: `pnpm check` and `pnpm lint` clean.

---

### Phase 2B — Frontend: Maintenance Detail Route

**F-3**: Create `MaintenanceDetailState.svelte.ts`

Reactive class following `MaintenanceState` pattern:

- `loadCard(id: string)`: calls `commands.getMaintenanceCard(id as MaintenanceCardId)`, stores in `#card`
- `addEvent(args: AddMaintenanceArgs)`: calls `commands.addMaintenanceEvent(args)`, then optimistically prepends constructed `MaintenanceCardEventView` to `#card.events` (using `args.id` as the uuid, `args.datePerformed`, `args.maintenanceType`, `args.notes`)
- Context helpers: `setMaintenanceDetailState`, `getMaintenanceDetailState`

**F-4**: Create `MaintenanceEventTimeline.svelte`

Props: `events: MaintenanceCardEventView[]`

- When `events.length === 0`: centered `<Wrench>` icon (monochromatic) + "No events logged yet." text
- When non-empty: vertical list of charcoal event cards with `1px solid #1F1F1F` border
- Each card shows: date in monospaced font, maintenance type as a muted badge, notes in small text

**F-5**: Create `AddEventModal.svelte` (detail-scoped)

Props: `open: boolean`, `onClose: () => void`, `maintenanceCardId: string`

- No card-selection dropdown
- Date Performed: `<input type="date">` defaulting to today's ISO date string
- Maintenance Type: `<select>` using same Paraglide keys as existing modal
- Notes: `<textarea rows={3}>`
- On submit: `maintenanceDetailState.addEvent({ id: crypto.randomUUID(), maintenanceCardId, datePerformed, maintenanceType, notes: notes.trim() || null })`
- Inline validation: date is required; show error if empty on submit attempt

**F-6**: Create `src/routes/maintenance/[id]/+page.svelte`

Structure:

```svelte
<script lang="ts">
  import { page } from '$app/stores';
  // initialize MaintenanceDetailState, load on mount
  // id = $page.params.id
</script>

<!-- Back button top-left -->
<!-- Header: amber manufacturerName + productCode, muted seriesCode, road number pill -->
<!-- Stats row: 3 columns — Last Serviced, Next Due, Total Events -->
<!-- Add Event amber button -->
<!-- MaintenanceEventTimeline -->
<!-- AddEventModal (conditionally rendered) -->
```

Handle states: loading skeleton, error banner, not-found → navigate to `/maintenance`.

**F-7**: `src/routes/maintenance/+page.svelte` — Remove Add Event from header:

- Delete `showAddEventModal` state
- Delete `handleAddEvent()` function
- Remove Add Event `<Button>` from `{#snippet actions()}`
- Remove `<AddMaintenanceEventModal>` import and render block

**F-8**: Add Paraglide key `maintenance_card_already_exists` to `messages/en.json`:

```json
"maintenance_card_already_exists": "A maintenance card already exists for this locomotive."
```

Add to all locale files that exist. Update `AddMaintenanceCardModal.svelte` to display this key when it catches a conflict error from the backend.

---

### Phase 3 — Verification

**V-1**: Backend

```bash
pnpm run rust:test   # All tests pass
cargo clippy         # No warnings (-D warnings)
```

**V-2**: Frontend

```bash
pnpm lint            # ESLint clean
pnpm check           # svelte-check + TypeScript clean
pnpm test            # Vitest clean
```

**V-3**: Manual smoke test (per `quickstart.md` checklist)

---

## Test Plan

### New Backend Tests (sqlx::test integration)

**`repo_prevents_duplicate_card_for_same_stock`**

- Setup: insert one card for a rolling stock via fixture
- Action: call `repo.save(card)` with `Created` event for the same `owned_rolling_stock_id`
- Assert: returns `Err(DomainError::Conflict(...))`

**`repo_list_due_card_views_includes_display_info`**

- Setup: fixture with rolling stock linked to catalog data (manufacturer, rolling stock, product code)
- Action: `repo.list_due_card_views()`
- Assert: returned view has non-null `display_info` with correct manufacturer/product/series/road values

**`repo_find_view_by_id_includes_display_info`**

- Same as above but via `find_view_by_id`

### New Frontend Tests (Vitest)

**`MaintenanceDetailState: addEvent optimistically prepends event`**

- Mock `commands.addMaintenanceEvent` to resolve after 100ms
- Call `state.addEvent(...)`
- Assert: `state.card.events.length` increments immediately (before mock resolves)

**`MaintenanceCardItem: renders displayInfo when present`** (if component test infra exists)

- Render with a card that has non-null `displayInfo`
- Assert: manufacturer+product code visible in heading, road number badge visible, no raw TRN strings
