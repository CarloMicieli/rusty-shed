# Research: Rolling Stock Progressive Editing

**Branch**: `024-rolling-stock-edit` | **Date**: 2026-02-18
**Phase**: 0 — Resolves all NEEDS CLARIFICATION from plan Technical Context

---

## 1. Existing Domain & Schema State

### Decision

No new SQLite migration is required. All columns needed for the four drawer sections already exist in `rolling_stocks` (migration `0001_create_railway_models_and_rolling_stocks.sql`).

**Relevant columns in `rolling_stocks`:**

| Section        | Spec Field       | DB Column                                                           |
| -------------- | ---------------- | ------------------------------------------------------------------- |
| Identification | Series Code      | `series_code TEXT NOT NULL`                                         |
| Identification | Road Number      | `road_number TEXT`                                                  |
| Identification | Livery           | `livery TEXT`                                                       |
| Identification | Depot            | `depot TEXT`                                                        |
| Identification | Railway Company  | `railway_company_id TEXT NOT NULL REFERENCES railway_companies(id)` |
| Technical      | Flywheel         | `technical_flywheel_fitted TEXT`                                    |
| Technical      | Body Material    | `technical_body_shell TEXT`                                         |
| Technical      | Chassis Material | `technical_chassis TEXT`                                            |
| Technical      | Lighting         | `technical_interior_lights TEXT`, `technical_lights TEXT`           |
| Control        | DCC Interface    | `dcc_interface TEXT`                                                |
| Control        | Control Type     | `control TEXT`                                                      |
| Coupling       | Socket Type      | `technical_coupling_socket TEXT`                                    |
| Coupling       | Close Coupling   | `technical_coupling_close_couplers TEXT`                            |
| Coupling       | Digital Shunting | `technical_coupling_digital_shunting TEXT`                          |

**Rationale**: Avoids migration risk; existing repo mapping infrastructure handles these columns already.
**Alternatives considered**: Adding a new `rolling_stock_specs` table — rejected as unnecessary normalization; all spec data belongs to the rolling stock row.

---

## 2. Domain Event Strategy for Rolling Stock Updates

### Decision

Add a new `RollingStockUpdated` variant to the existing `RailwayModelEvent` enum.

```rust
RollingStockUpdated {
    event_id: Uuid,
    railway_model_id: RailwayModelId,
    rolling_stock_id: RollingStockId,
    timestamp: NaiveDateTime,
    /// Minimal patch describing changed fields (same convention as RailwayModelUpdated.changed).
    changed: serde_json::Value,
},
```

**Rationale**: Follows the existing patch-based `RailwayModelUpdated` pattern exactly. The `changed` JSON object records only the fields that were mutated, enabling the repository to build a minimal SQL UPDATE. Reusing the established event pattern keeps domain code consistent and the repository's event-dispatch loop easy to reason about.
**Alternatives considered**:

- Reusing `RailwayModelUpdated` with nested rolling stock data — rejected; conflates aggregate-level and child-level mutations, harder to audit.
- Full-snapshot event — rejected; over-stores data and makes minimal SQL updates harder to derive.

---

## 3. Command Design: Avoiding `Option<Option<T>>` Serialization Pitfalls

### Decision

Use 5 focused Tauri commands, each scoped to a single user interaction type. This sidesteps `Option<Option<T>>` entirely.

| Command                                | Trigger                                                                           |
| -------------------------------------- | --------------------------------------------------------------------------------- |
| `update_railway_model_text`            | Blur-to-save on description or details in-place field                             |
| `update_railway_model_classification`  | Badge-click → scale or epoch picker selection                                     |
| `update_rolling_stock_identification`  | Blur-to-save on any card in-place field (series code, road number, livery, depot) |
| `update_rolling_stock_railway_company` | Railway company picker selection on card                                          |
| `update_rolling_stock_specifications`  | Save button in technical spec drawer                                              |

**Rationale**: Each command maps 1:1 to a user interaction. For text fields, `None` means "no change" (distinct from clearing). For nullable fields (`road_number`, `livery`, `depot`), the command receives an `Option<String>` where the frontend always sends the current field value — `None` from JS `null` means "clear". The `series_code` field on the identification command is always `String` (required, never nullable).
**Alternatives considered**:

- Two mega-commands (`update_railway_model`, `update_rolling_stock`) — rejected because `Option<Option<T>>` for clearing nullable fields is not cleanly expressible via serde/specta without custom types.
- Single command per field (8+ commands) — rejected as over-engineering; batch per interaction is the right granularity.

---

## 4. Frontend In-Place Edit Pattern

### Decision

Implement a reusable `InPlaceEdit.svelte` Svelte 5 component using `$state` runes. The component handles:

- Click (or focus) → activates edit mode with cursor placed at click point
- Blur → triggers `onSave` callback (async); shows spinner during save; on error restores editable state with unsaved value
- Escape → cancels and restores original value
- Hover → applies `dashed border + background tint` affordance via CSS

**Rationale**: The same interaction behaviour applies identically across all 6 in-place text fields (description, details, series code, road number, livery, depot) per the spec (FR-003 to FR-007, FR-012). A single reusable primitive avoids duplicating this logic in every parent component.
**Alternatives considered**:

- Inline edit logic per component — rejected; would duplicate the blur/escape/error state machine six times.
- Using a third-party inline-edit library — rejected; no existing shadcn-svelte or Skeleton UI primitive matches; simple `$state` machine is sufficient and stays on-stack.

---

## 5. Frontend Constrained Selection Pattern

### Decision

Implement a reusable `BadgePicker.svelte` component using a Svelte popover/dropdown anchored to the trigger element. Uses the existing shadcn-svelte `Popover` or a lightweight custom overlay.

- Click on badge/value → opens picker positioned near trigger
- Click option → triggers `onSelect` callback; closes picker
- Escape or outside click → closes picker; original value preserved
- Error on save → shows toast error; reverts displayed value

**Rationale**: Scale, Era, and Railway Company all use the same constrained-selection interaction (FR-014 through FR-020). A single reusable component parameterised by option list avoids duplication.
**Alternatives considered**:

- shadcn-svelte `Select` component — considered; however it renders differently (full-width dropdown) and doesn't anchor near a badge. Custom popover better matches "positioned in close proximity to triggering badge" (FR-016).
- Inline `<select>` — rejected; doesn't match visual design requirements.

---

## 6. Drawer Implementation

### Decision

Use the existing manual drawer pattern from `ItemDrawer.svelte` (fixed overlay + slide-in panel) rather than the shadcn-svelte `Sheet` component. The drawer is a `RollingStockSpecsDrawer.svelte` component accepting `rollingStockId` + `railwayModelId` as props, fetching its own data on open.

- On open: fetches rolling stock data via the existing `get_railway_model_by_id` command
- Unsaved changes guard: `$derived.by` dirty-check comparison against original fetched data; shows Dialog confirmation on close attempt if dirty
- Four form sections: Identification, Technical, Control, Coupling
- Constrained fields (Control Type, DCC Interface, Coupling Socket) use `BadgePicker` or `<select>` within the drawer

**Rationale**: The existing `ItemDrawer.svelte` pattern (fixed overlay with `bg-black/40` + slide-in panel) is already established and matches the visual design. The `Sheet` component is available but the manual implementation is more flexible for the wide drawer needed for 4-section forms.
**Alternatives considered**:

- Sheet component — available at `src/lib/components/ui/sheet/`; rejected for this feature because the custom drawer provides a wider panel and more control over the transition without requiring component customisation.
- Modal dialog — rejected; spec explicitly calls for a "side drawer" that slides in without obscuring the full page.

---

## 7. State Management for Edit Operations

### Decision

A lightweight `RollingStockEditState.svelte.ts` class per rolling stock card context (not global state). It tracks:

- `activeField: string | null` — which field is currently in edit mode
- `pendingValue: string` — value in the in-place input
- `isSaving: boolean` — save in progress
- `lastError: string | null` — error message to display

The drawer state (form values, dirty flag, saving state) is managed internally in `RollingStockSpecsDrawer.svelte` via `$state` runes.

**Rationale**: Edit state is ephemeral and scoped to the interaction; it does not need to be shared globally. The service layer (`CollectionService`, `DepotState`) already handles the canonical data; after a successful save, the relevant list is re-fetched.
**Alternatives considered**:

- Global store for all edit states — rejected; over-engineered for ephemeral per-card UI state.
- Lifting state to parent feature — rejected; the parent doesn't need to know which field is being edited.

---

## 8. Paraglide i18n Keys Required

New message keys to add to `messages/` (English values shown):

### In-Place Edit

- `edit_field_save` → "Save"
- `edit_field_cancel` → "Cancel"
- `edit_field_placeholder_empty` → "Click to add..."
- `edit_save_error` → "Failed to save. Your changes are preserved."

### Badge Picker

- `badge_picker_close` → "Close"

### Railway Model Fields

- `railway_model_field_description` → "Description"
- `railway_model_field_details` → "Details"
- `railway_model_field_scale` → "Scale"
- `railway_model_field_era` → "Era"

### Rolling Stock Card Fields

- `rolling_stock_field_series_code` → "Series Code"
- `rolling_stock_field_road_number` → "Road Number"
- `rolling_stock_field_livery` → "Livery"
- `rolling_stock_field_depot` → "Depot"
- `rolling_stock_field_railway_company` → "Railway Company"
- `rolling_stock_edit_specs_button` → "Edit Specs"

### Specifications Drawer

- `specs_drawer_title` → "Rolling Stock Specifications"
- `specs_drawer_section_identification` → "Identification"
- `specs_drawer_section_technical` → "Technical"
- `specs_drawer_section_control` → "Control"
- `specs_drawer_section_coupling` → "Coupling"
- `specs_drawer_field_flywheel` → "Flywheel"
- `specs_drawer_field_body_material` → "Body Material"
- `specs_drawer_field_chassis_material` → "Chassis Material"
- `specs_drawer_field_lighting` → "Lighting"
- `specs_drawer_field_dcc_interface` → "DCC Interface"
- `specs_drawer_field_control_type` → "Control Type"
- `specs_drawer_field_coupling_socket` → "Coupling Socket"
- `specs_drawer_field_close_coupling` → "Close Coupling"
- `specs_drawer_field_digital_shunting` → "Digital Shunting"
- `specs_drawer_save` → "Save"
- `specs_drawer_cancel` → "Cancel"
- `specs_drawer_unsaved_title` → "Discard changes?"
- `specs_drawer_unsaved_message` → "You have unsaved changes. Discard them?"
- `specs_drawer_unsaved_confirm` → "Discard"
- `specs_drawer_save_success` → "Specifications saved"
- `specs_drawer_save_error` → "Failed to save specifications. Please try again."

---

## 9. Existing Commands Reused (No Change)

- `get_railway_model_by_id` — used by drawer to load current values on open
- `get_railway_companies` — used by `BadgePicker` for Railway Company picker options
- `get_collection` / `get_depot` — re-fetched after successful mutations to refresh displayed data

---

## 10. Enum Values for Constrained Fields

Values sourced from existing Rust domain enums (already in TypeScript bindings):

**Scale**: `H0`, `N`, `TT`, `O`, `Z`, `S`, `G`, `H0e`, `H0m`, `Tm`
**Epoch (Era)**: `I`, `II`, `IIa`, `IIb`, `IIc`, `III`, `IIIa`, `IIIb`, `IV`, `IVa`, `IVb`, `V`, `Va`, `Vb`, `Vc`, `VI`
**Control Type**: `DccReady`, `DccFitted`, `DccSound`, `NoDcc` (maps to `Analogue` in UI)
**DCC Interface**: `NEM_651`, `NEM_652`, `NEM_654`, `PLUX_8`, `PLUX_12`, `PLUX_16`, `PLUX_22`, `NEXT_18`, `NEXT_18_S`, `MTC_21`
**Railway Companies**: Fetched at runtime from `railway_companies` table via `get_railway_companies` command

---

## Resolved Clarifications Summary

| NEEDS CLARIFICATION                                                       | Resolution                                                                |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Whether new migration is needed                                           | No — all columns present in migration 0001                                |
| How to handle `Option<Option<T>>` for nullable fields                     | 5 focused commands with clear `Option<T>` semantics per field             |
| Whether to add `RollingStockUpdated` event or reuse `RailwayModelUpdated` | New `RollingStockUpdated` variant added to `RailwayModelEvent`            |
| Which drawer implementation to use (Sheet vs custom)                      | Custom manual drawer following existing `ItemDrawer.svelte` pattern       |
| Whether global state is needed for edit operations                        | No — lightweight per-card `RollingStockEditState` + drawer-internal state |
| Which enum values to show in constrained pickers                          | Sourced from existing Rust enums already in TypeScript bindings           |
