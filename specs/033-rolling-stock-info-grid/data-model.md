# Data Model: Rolling Stock Information Grid

**Feature**: `033-rolling-stock-info-grid`
**Date**: 2026-03-05

> **Note**: No changes to the Rust domain model or database schema are required. All 15 fields already exist in the backend. This document describes the **frontend data shapes** and **view model** used by `RollingStockCard`.

---

## View Model: `RollingStockCardState`

The component maintains a single reactive state object containing the local (optimistic) values of all editable fields. Initial values are extracted from `OwnedRollingStockView` when the component mounts and when the parent prop changes.

```typescript
// Frontend local state (Svelte 5 $state)
interface RollingStockCardState {
  // ── Identification ──────────────────────────────────────────────
  seriesCode: string; // Required; never null
  roadNumber: string | null;
  livery: string | null; // Free text
  depot: string | null; // Free text

  // ── Control ─────────────────────────────────────────────────────
  control: Control | null; // DCC_READY | DCC_FITTED | DCC_SOUND | NO_DCC
  dccInterface: DccInterface | null;
  lengthMillimeters: number | null;
  lengthInches: number | null;

  // ── Technical ───────────────────────────────────────────────────
  flywheelFitted: 'YES' | 'NO' | null; // FeatureFlag
  bodyShell: 'PLASTIC' | 'METAL_DIE_CAST' | null;
  chassis: 'PLASTIC' | 'METAL_DIE_CAST' | null;
  interiorLights: 'YES' | 'NO' | null; // FeatureFlag
  lights: 'YES' | 'NO' | null; // FeatureFlag (headlights)

  // ── Coupling ────────────────────────────────────────────────────
  couplingSocket: CouplingSocket | null; // NONE | NEM_355 | … | NEM_365
  closeCouplers: boolean | null;
  digitalShunting: boolean | null;

  // ── Railway Company ─────────────────────────────────────────────
  railwayCompanyId: string | null;
  railwayCompanyName: string;
}
```

---

## Field Registry (Information Grid)

Maps each grid position to its frontend state key, label message key, edit component, and option source.

| Row | Col | Field            | State Key                            | Label Key                             | Edit Component         | Options / Notes           |
| --- | --- | ---------------- | ------------------------------------ | ------------------------------------- | ---------------------- | ------------------------- |
| 1   | 1   | Series           | `seriesCode`                         | `rolling_stock_field_series`          | `InPlaceEdit` (text)   | Required                  |
| 1   | 2   | Depot            | `depot`                              | `rolling_stock_field_depot`           | `InPlaceEdit` (text)   | Optional                  |
| 1   | 3   | Livery           | `livery`                             | `rolling_stock_field_livery`          | `InPlaceEdit` (text)   | Optional                  |
| 2   | 1   | Control Type     | `control`                            | `rolling_stock_field_control_type`    | `BadgePicker`          | `CONTROL_OPTIONS`         |
| 2   | 2   | DCC Interface    | `dccInterface`                       | `rolling_stock_field_dcc_interface`   | `BadgePicker`          | `DCC_INTERFACE_OPTIONS`   |
| 2   | 3   | Length           | `lengthMillimeters` / `lengthInches` | `rolling_stock_field_length`          | `InPlaceEdit` (number) | Metric/imperial toggle    |
| 3   | 1   | Flywheel Fitted  | `flywheelFitted`                     | `specs_drawer_field_flywheel`         | `InPlaceBooleanEdit`   | YES / NO / —              |
| 3   | 2   | Body Shell       | `bodyShell`                          | `specs_drawer_field_body_material`    | `InPlaceSelectEdit`    | `BODY_SHELL_OPTIONS`      |
| 3   | 3   | Chassis          | `chassis`                            | `specs_drawer_field_chassis_material` | `InPlaceSelectEdit`    | `CHASSIS_OPTIONS`         |
| 4   | 1   | Interior Lights  | `interiorLights`                     | `rolling_stock_field_interior_lights` | `InPlaceBooleanEdit`   | YES / NO / —              |
| 4   | 2   | Lights           | `lights`                             | `rolling_stock_field_lights`          | `InPlaceBooleanEdit`   | YES / NO / —              |
| 4   | 3   | _(empty)_        | —                                    | —                                     | —                      | Spacer cell               |
| 5   | 1   | Coupling Socket  | `couplingSocket`                     | `specs_drawer_field_coupling_socket`  | `InPlaceSelectEdit`    | `COUPLING_SOCKET_OPTIONS` |
| 5   | 2   | Close Couplers   | `closeCouplers`                      | `specs_drawer_field_close_coupling`   | `InPlaceBooleanEdit`   | YES / NO / —              |
| 5   | 3   | Digital Shunting | `digitalShunting`                    | `specs_drawer_field_digital_shunting` | `InPlaceBooleanEdit`   | YES / NO / —              |

---

## Option Sets (Frontend Constants)

### `BODY_SHELL_OPTIONS`

```typescript
const BODY_SHELL_OPTIONS = [
  { value: '', label: '—' },
  { value: 'PLASTIC', label: 'Plastic' },
  { value: 'METAL_DIE_CAST', label: 'Metal Die-Cast' }
] as const;
```

### `CHASSIS_OPTIONS`

```typescript
const CHASSIS_OPTIONS = [
  { value: '', label: '—' },
  { value: 'PLASTIC', label: 'Plastic' },
  { value: 'METAL_DIE_CAST', label: 'Metal Die-Cast' }
] as const;
```

### `COUPLING_SOCKET_OPTIONS`

```typescript
const COUPLING_SOCKET_OPTIONS = [
  { value: '', label: '—' },
  { value: 'NONE', label: 'None' },
  { value: 'NEM_355', label: 'NEM 355' },
  { value: 'NEM_356', label: 'NEM 356' },
  { value: 'NEM_357', label: 'NEM 357' },
  { value: 'NEM_359', label: 'NEM 359' },
  { value: 'NEM_360', label: 'NEM 360' },
  { value: 'NEM_362', label: 'NEM 362' },
  { value: 'NEM_365', label: 'NEM 365' }
] as const;
```

---

## `InPlaceBooleanEdit` Component Contract

New primitive component to be created at `src/lib/components/InPlaceBooleanEdit.svelte`.

### Props

```typescript
interface InPlaceBooleanEditProps {
  /** Current value: 'YES', 'NO', or null (not applicable / unknown). */
  value: 'YES' | 'NO' | null;
  /** Placeholder shown when value is null. */
  placeholder?: string;
  /** Called with 'YES', 'NO', or null when user picks an option. */
  onSave: (value: 'YES' | 'NO' | null) => Promise<void>;
}
```

### Behavior

- **View mode**: Shows a compact chip — green check + "Yes" for YES, muted "No" for NO, italic "—" for null.
- **Edit mode**: Inline 3-button row (`—` / `Yes` / `No`). Clicking any button immediately calls `onSave` and returns to view mode.
- **Saving state**: Buttons disabled, spinner shown.
- **Error state**: Error text below, value reverts to previous.
- **Keyboard**: `Escape` cancels without saving.

### State Transitions

```
view ──click──▶ editing ──select──▶ saving ──success──▶ view
                         ──escape──▶ view
                                    ──error──▶ editing (with error)
```

---

## Extraction from `OwnedRollingStockView`

The view type from the bindings includes a nested `technical_specifications` object:

```typescript
// Pseudo-extraction (src/lib/bindings.ts shape)
function extractCardState(rs: OwnedRollingStockView): RollingStockCardState {
  const ts = rs.technical_specifications;
  const coupling = ts?.coupling;
  return {
    seriesCode: rs.series_code,
    roadNumber: rs.road_number ?? null,
    livery: rs.livery ?? null,
    depot: rs.depot ?? null,
    control: rs.control ?? null,
    dccInterface: rs.dcc_interface ?? null,
    lengthMillimeters: rs.length_over_buffers?.millimeters ?? null,
    lengthInches: rs.length_over_buffers?.inches ?? null,
    flywheelFitted:
      ts?.flywheel_fitted === 'YES' ? 'YES' : ts?.flywheel_fitted === 'NO' ? 'NO' : null,
    bodyShell: ts?.body_shell ?? null,
    chassis: ts?.chassis ?? null,
    interiorLights:
      ts?.interior_lights === 'YES' ? 'YES' : ts?.interior_lights === 'NO' ? 'NO' : null,
    lights: ts?.lights === 'YES' ? 'YES' : ts?.lights === 'NO' ? 'NO' : null,
    couplingSocket: coupling?.socket ?? null,
    closeCouplers:
      coupling?.close_couplers === 'YES' ? true : coupling?.close_couplers === 'NO' ? false : null,
    digitalShunting:
      coupling?.digital_shunting === 'YES'
        ? true
        : coupling?.digital_shunting === 'NO'
          ? false
          : null,
    railwayCompanyId: rs.railway_company?.id ?? null,
    railwayCompanyName: rs.railway_company?.name ?? ''
  };
}
```

---

## Validation Rules

| Field                  | Rule                                                         |
| ---------------------- | ------------------------------------------------------------ |
| `seriesCode`           | Required; must not be empty string                           |
| `lengthMillimeters`    | Optional; if provided must be a positive number              |
| All enum fields        | Must match one of the defined values or be null/empty string |
| All FeatureFlag fields | Must be `'YES'`, `'NO'`, or `null`                           |

Validation is enforced at the backend boundary via the existing `validator::Validate` on `UpdateRollingStockSpecificationsArgs`. Frontend only performs empty-string guard for `seriesCode`.
