# Research: Rolling Stock Information Grid

**Feature**: `033-rolling-stock-info-grid`
**Date**: 2026-03-05

---

## 1. Existing Backend Commands

### Decision

Use **`updateRollingStockSpecifications`** (existing) as the sole persistence command for all new fields. No new Tauri commands are needed.

### Rationale

- The command already accepts all 15 target fields: `seriesCode`, `roadNumber`, `livery`, `depot`, `flywheelFitted`, `bodyShell`, `chassis`, `interiorLights`, `lights`, `dccInterface`, `control`, `couplingSocket`, `closeCouplers`, `digitalShunting`.
- Boolean fields (`flywheelFitted`, `closeCouplers`, `digitalShunting`) are typed `boolean | null` in the generated TypeScript args — straightforward to pass.
- String enum fields (`bodyShell`, `chassis`, `interiorLights`, `lights`, `couplingSocket`) are typed `string | null` — backend parses via `FromStr`.
- The save strategy: when any single field changes, call `updateRollingStockSpecifications` with the full current local state of all fields. This mirrors what `RollingStockSpecsDrawer` already does.

### Alternatives Considered

- **New targeted commands** (e.g., `updateRollingStockFlywheel`): Rejected — creates Rust boilerplate for no UX benefit; constitution discourages unnecessary surface expansion.
- **Mix of old + new commands**: Rejected — increases cognitive complexity for the same outcome.

### Existing Specific Commands (still used for existing fields)

| Field group                                                                                               | Command                                                      |
| --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Series Code, Road Number, Livery, Depot                                                                   | `updateRollingStockIdentification`                           |
| Control Type, DCC Interface, Length                                                                       | `updateRollingStockDcc`                                      |
| Railway Company                                                                                           | `updateRollingStockRailwayCompany`                           |
| Flywheel, Body Shell, Chassis, Interior Lights, Lights, Coupling Socket, Close Couplers, Digital Shunting | `updateRollingStockSpecifications` (with full current state) |

---

## 2. Existing Frontend Components

### Decision

Reuse **`InPlaceEdit`** (text/numeric) and **`InPlaceSelectEdit`** (enum dropdowns) as-is. Create one new **`InPlaceBooleanEdit`** component for `FeatureFlag` (YES/NO/NotApplicable) fields.

### Rationale for InPlaceBooleanEdit

- `FeatureFlag` has three states: YES, NO, NotApplicable (rendered as "—")
- A two-state toggle cannot represent three states cleanly
- `InPlaceSelectEdit` could work but its `<select>` HTML element is a poor fit for a compact toggle UX
- A dedicated `InPlaceBooleanEdit` renders as a compact YES/NO chip in view mode and a 3-option inline picker (—/Yes/No) on click — consistent with the existing amber-on-dark design language
- Keeps the component API consistent: `value: string | null`, `onSave: (v: string | null) => Promise<void>`

### Existing Components Confirmed

| Component           | Path                                          | Used for                                                       |
| ------------------- | --------------------------------------------- | -------------------------------------------------------------- |
| `InPlaceEdit`       | `src/lib/components/InPlaceEdit.svelte`       | Series, Road Number, Livery, Depot, Length                     |
| `InPlaceSelectEdit` | `src/lib/components/InPlaceSelectEdit.svelte` | Body Shell, Chassis, Coupling Socket                           |
| `BadgePicker`       | `src/lib/components/BadgePicker.svelte`       | Control Type, DCC Interface (existing, unchanged)              |
| `Toggle`            | `src/lib/components/ui/toggle/Toggle.svelte`  | Available but not suitable for inline edit (no save lifecycle) |

---

## 3. Grid Layout Strategy

### Decision

Replace the current unstructured `<dl>` in `RollingStockCard` with a **CSS grid** (`grid-cols-3`) containing 5 rows of labelled field cells.

### Rationale

- The current `sm:grid-cols-2` `<dl>` only shows 8 fields and has inconsistent column spanning
- A fixed `grid-cols-3` grid gives predictable positions and makes the layout spec-compliant
- Each cell is a `<div>` with a label (`<dt>` style) above and a value/edit control below
- Empty column in Row 4, Col 3 (the "—" entry in the spec) renders as an empty `<div>` spacer

### Alternatives Considered

- **Table layout**: Rejected — semantically wrong for labelled key-value data; harder to style consistently
- **Keeping `<dl>`**: Rejected — `<dl>` semantics conflict with a rigid 3-column spec

---

## 4. Boolean Field Rendering: FeatureFlag Mapping

### Decision

Render `FeatureFlag` fields (Flywheel Fitted, Interior Lights, Lights, Close Couplers, Digital Shunting) as a **compact YES/NO chip** in view mode via `InPlaceBooleanEdit`. On click, show a 3-option inline picker: `—` (null/NotApplicable), `Yes`, `No`.

### Data Mapping

| Frontend state | Tauri args type                               | Drawer string |
| -------------- | --------------------------------------------- | ------------- |
| `'YES'`        | `string \| null` (`interiorLights`, `lights`) | "YES"         |
| `'NO'`         | same                                          | "NO"          |
| `null` / `''`  | `null`                                        | ""            |

**Exception**: `flywheelFitted`, `closeCouplers`, `digitalShunting` are typed `boolean | null` in the Tauri args — convert from `'YES'/'NO'/null` at the save boundary.

### Rationale

- Consistent three-state handling for all FeatureFlag fields
- Avoids introducing `boolean` local state for some fields and `string` for others
- "—" is a valid user choice meaning "not applicable / unknown" — not the same as "No"

---

## 5. Save Strategy for `updateRollingStockSpecifications`

### Decision

Maintain **full local state** for all 15 fields in `RollingStockCard`. On any single-field change, build the complete args object from current local state and call `updateRollingStockSpecifications`.

### Rationale

- `updateRollingStockSpecifications` requires `seriesCode` (non-null) and accepts all others as optional
- Reading full state from reactive `$state` variables has zero cost
- Avoids requiring a per-field command or any changes to the Rust backend
- Pattern already established by `RollingStockSpecsDrawer`

### Local State Additions Needed

The following are **not currently tracked** in `RollingStockCard` local state:

- `localFlywheelFitted: string | null`
- `localBodyShell: string | null`
- `localChassis: string | null`
- `localInteriorLights: string | null`
- `localLights: string | null`
- `localCouplingSocket: string | null`
- `localCloseCouplers: boolean | null`
- `localDigitalShunting: boolean | null`

---

## 6. Message Keys Audit

### Decision

Create **new `rolling_stock_field_*` keys** for fields not yet in the card context; reuse existing drawer keys where terminology matches.

### Keys Already Available (can reuse)

| Field            | Existing key                          | Value              |
| ---------------- | ------------------------------------- | ------------------ |
| DCC Interface    | `rolling_stock_field_dcc_interface`   | "DCC Interface"    |
| Series Code      | `rolling_stock_field_series_code`     | "Series Code"      |
| Road Number      | `rolling_stock_field_road_number`     | "Road Number"      |
| Livery           | `rolling_stock_field_livery`          | "Livery"           |
| Depot            | `rolling_stock_field_depot`           | "Depot"            |
| Flywheel         | `specs_drawer_field_flywheel`         | "Flywheel"         |
| Body Material    | `specs_drawer_field_body_material`    | "Body Material"    |
| Chassis Material | `specs_drawer_field_chassis_material` | "Chassis Material" |
| Coupling Socket  | `specs_drawer_field_coupling_socket`  | "Coupling Socket"  |
| Close Coupling   | `specs_drawer_field_close_coupling`   | "Close Coupling"   |
| Digital Shunting | `specs_drawer_field_digital_shunting` | "Digital Shunting" |

### Keys That Need Adding to `messages/en.json`

| Key                                   | Value             |
| ------------------------------------- | ----------------- |
| `rolling_stock_field_interior_lights` | "Interior Lights" |
| `rolling_stock_field_lights`          | "Lights"          |
| `rolling_stock_field_control_type`    | "Control Type"    |
| `rolling_stock_field_length`          | "Length"          |
| `rolling_stock_field_series`          | "Series"          |

---

## 7. Resolved Clarifications

| Question                        | Resolution                                                                   |
| ------------------------------- | ---------------------------------------------------------------------------- |
| Livery: enum vs. free text?     | **Free text** — `InPlaceEdit` consistent with current RollingStockCard usage |
| "Lights" (Row 4, Col 2): type?  | **FeatureFlag** (YES/NO/—) — maps to `lights` backend field (headlights)     |
| Depot: enum vs. free text?      | **Free text** — `InPlaceEdit`, consistent with backend `Option<String>` type |
| New Tauri commands needed?      | **No** — `updateRollingStockSpecifications` covers all new fields            |
| New primitive component needed? | **Yes** — `InPlaceBooleanEdit` for FeatureFlag fields (3-state toggle)       |
| Backend schema changes?         | **No** — all fields already in the DB schema                                 |
