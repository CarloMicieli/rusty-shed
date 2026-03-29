# Research: Train Formations

**Branch**: `039-train-formations` | **Phase**: 0 | **Date**: 2026-03-29

---

## Resolved Unknowns

### 1. FormationEntry Dual-Reference Model

**Decision**: `FormationEntry` carries a **mandatory** `prototype_id` (master catalog) and an **optional** `rolling_stock_id` (user-assigned physical model for that slot).

**Rationale**: This unlocks the full "gap analysis" use case ("I need 5 Gran Comfort coaches, I own 3") while also allowing advanced users to assign specific physical models (e.g., Roco vs Lima) to individual slots. Ownership is fast-derived: `SELECT COUNT(*) FROM owned_rolling_stocks WHERE prototype_id = ?`.

**Alternatives considered**:

- `FormationEntry → RollingStock` only: loses planning capability, can't add unowned units to a formation.
- `FormationEntry → Prototype` only: can't record "I've assigned my Roco model to slot 3."

---

### 2. Prototype Library Sourcing

**Decision**: **Seeded dataset**, same pattern as `RailwayCompany`. App ships with a curated list of common European prototype records; users can add custom ones (`is_custom = true`).

**Rationale**: Fully offline (Tauri app constraint). No external network dependency. Consistent with how `railway_companies` seed data is already handled in the codebase. Custom entries use the same `is_custom` flag pattern seen in `FormationCategory`.

**Alternatives considered**:

- User-maintained only: requires users to create every prototype before building formations — high friction.
- External API: violates offline-first constraint.

---

### 3. Ownership Badge + Assignment Picker

**Decision**: **Auto-detect, manual assignment**. The `FormationCell` automatically shows an "N owned" badge derived from `$derived` reactive lookup. If exactly 1 match, a "Quick Assign" shortcut appears. Users can open an assignment picker to explicitly assign a specific `RollingStock` item.

**Rationale**: Delivers immediate value (gap analysis visible without user action) while giving power users control over model assignment. `$derived` in Svelte 5 makes the badge computation lightweight and reactive.

**Alternatives considered**:

- Auto-assign first match: hides which model is assigned, loses control.
- Manual only: requires user action to see ownership — eliminates the automatic gap-analysis value proposition.

---

### 4. Custom Prototype Creation

**Decision**: **Inline from the drawer**. A "+ Add Prototype" action appears at the bottom of search results (or on zero results). Opens a nested creation form for: railway company, series code, car type, service level. No separate management screen in this feature.

**Rationale**: Same UX as custom `FormationCategory` creation already specified in the feature. Minimizes context switches during formation building. Avoids scope creep of a full Prototype Library admin screen.

**Alternatives considered**:

- Dedicated management screen: adds scope and a new route; deferred to a future feature.
- Both: adds both screen and inline; over-scoped for this feature.

---

### 5. Traction Evaluation Source of Truth

**Decision**: **Prototype `car_type` drives traction**. A slot counts as motorized if `Prototype.car_type IN ('Locomotive', 'PowerCar')`, regardless of whether a physical model is assigned to that slot.

**Rationale**: Traction is a prototypical property (a `Re 4/4` is always motorized; a `UIC-Z1` is always a coach). This is consistent with using Prototype as the master data. The per-entry `traction_override` flag (FR-018) still allows power users to override edge cases (dummy pushed by a hidden motorized car).

**Alternatives considered**:

- Assignment-driven: a slot without a physical model never counts as traction; penalizes "planning" mode formations.
- Both Prototype type + assignment: too restrictive for planning workflows.

---

## Technology Notes

### Drag-and-Drop: `svelte-dnd-action`

**Decision**: Use `svelte-dnd-action` for horizontal DnD in the track view.

**Rationale**:

- Treats DnD as a standard HTML action (`use:dndzone`) — clean Svelte 5 integration with `$state` and `$derived`.
- Automatically provides ghost placeholder for FR-014.
- Supports horizontal lists out of the box.
- Svelte's `animate:flip` handles gap-fill animation (FR-015) naturally.

**Integration pattern** (Optimistic UI):

1. `consider` event: update local `$state` array → instant visual snap, no DB call.
2. `finalize` event: persist final order via a single bulk Tauri command (`reorder_formation_elements`).
3. Rust side: SQLx transaction updates all affected `position_order` values atomically.

**Watch-out**: Never call `invoke` inside the `consider` (dragging) event — only in `finalize` (drop). Otherwise disk writes cause stutter at 60fps.

---

### Component Architecture for Track View

Three-level split for Svelte 5 performance:

| Component               | Responsibility                                                                                      |
| ----------------------- | --------------------------------------------------------------------------------------------------- |
| `FormationTrack.svelte` | Scrollable container, `use:dndzone` action, `{#each items as item (item.id)}` with `animate:flip`   |
| `FormationCell.svelte`  | Individual slot — `$derived` for ownership badge, `$derived` for SVG icon from `Prototype.car_type` |
| `IdentityCard.svelte`   | Sticky first column via `sticky left-0 z-10`, shows name, category, traction warning                |

---

### Traction Warning Reactivity

The "No Traction" computation is `$derived`:

```ts
const hasTraction = $derived(
  entries.some(
    (e) =>
      ((e.prototype.car_type === 'Locomotive' || e.prototype.car_type === 'PowerCar') &&
        !e.traction_override_disabled) ||
      e.traction_override_enabled
  )
);
```

This updates within one Svelte tick after any composition change (satisfies SC-003: <500ms).

---

### Bulk Reorder Pattern (SQLx)

Sending 50 individual `UPDATE position_order = ? WHERE id = ?` queries is avoided. Instead:

- Frontend sends: `reorder_formation_elements(formation_id, ordered_ids: Vec<String>)`
- Rust uses a SQLx transaction iterating `enumerate()` over the ordered list.
- Single round-trip; DB write completes in microseconds.

---

### Navigation Integration

**Sidebar**: Add a new entry to `NAVIGATION_ITEMS` in `src/lib/components/navigation/config.ts` with `isPrimary: false` (appears in the "More" menu on mobile via `SECONDARY_ITEMS`).

**Icon**: `TrainFront` or `Combine` from `lucide-svelte` (TBD in design).

**Route**: `/train-formations` (new SvelteKit route directory `src/routes/train-formations/`).

---

### i18n Key Namespace

All Paraglide message keys for this feature follow the prefix `formations_`. Examples:

- `formations_page_title`
- `formations_add_stock`
- `formations_no_traction_warning`
- `formations_owned_badge` (e.g., `"{n} owned"`)
- `formations_planned_status`
- `formations_prototype_series_code`

Italian translations required for all keys alongside English.

---

### Prototype `default_is_motorized` Flag

Following the suggestion from research: the `prototypes` table includes `is_motorized BOOLEAN DEFAULT 0`. This means:

- At seed time, `Locomotive` and `PowerCar` types have `is_motorized = 1`.
- When a user adds a custom Prototype of type `Locomotive`, `is_motorized` defaults to `1` based on UI logic.
- The `traction_override` column on `formation_elements` overrides this per-slot.

---

## Schema Decisions

The proposed SQL schema (from research notes) is adopted with the following refinements:

1. `prototypes.is_motorized` added (derived from `car_type` at seed time; editable for custom prototypes).
2. Foreign key `owned_rolling_stocks` in `formation_elements` references the correct table name — to be confirmed against existing schema (likely `owned_rolling_stocks` or `collecting_items`).
3. All tables include `created_at`, `updated_at`, `version` columns (consistent with existing schema conventions in the codebase).
4. `prototypes` table includes `default_is_dummy` flag for "display-only" prototypes.

---

## Open Items (Resolved — No Blockers)

All Phase 0 unknowns are resolved. No NEEDS CLARIFICATION items remain.
