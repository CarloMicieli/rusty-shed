# Tauri IPC Contracts: Rolling Stock Information Grid

**Feature**: `033-rolling-stock-info-grid`
**Date**: 2026-03-05

> **No new Tauri commands are introduced by this feature.** All persistence is handled by existing commands. This document catalogues the commands consumed by the updated `RollingStockCard` component.

---

## Commands Used

### 1. `update_rolling_stock_specifications` _(primary new usage)_

Used for inline saves of all new fields: Flywheel Fitted, Body Shell, Chassis, Interior Lights, Lights, Coupling Socket, Close Couplers, Digital Shunting.

**TypeScript signature** (from generated bindings):

```typescript
commands.updateRollingStockSpecifications(
  args: UpdateRollingStockSpecificationsArgs
): Promise<Result<null, CommandError>>
```

**Args type** (generated — do not redefine):

```typescript
type UpdateRollingStockSpecificationsArgs = {
  railwayModelId: RailwayModelId;
  rollingStockId: RollingStockId;
  seriesCode: string; // Required — must be non-empty
  roadNumber: string | null;
  livery: string | null;
  depot: string | null;
  flywheelFitted: boolean | null; // true=YES, false=NO, null=NotApplicable
  bodyShell: string | null; // 'PLASTIC' | 'METAL_DIE_CAST' | null
  chassis: string | null; // 'PLASTIC' | 'METAL_DIE_CAST' | null
  interiorLights: string | null; // 'YES' | 'NO' | null
  lights: string | null; // 'YES' | 'NO' | null
  dccInterface: DccInterface | null;
  control: Control | null;
  couplingSocket: string | null; // 'NONE' | 'NEM_355' | … | 'NEM_365' | null
  closeCouplers: boolean | null; // true=YES, false=NO, null=NotApplicable
  digitalShunting: boolean | null; // true=YES, false=NO, null=NotApplicable
};
```

**Call pattern** (when saving any new field inline):

```typescript
async function saveSpecField(patch: Partial<RollingStockCardState>) {
  const result = await commands.updateRollingStockSpecifications({
    railwayModelId,
    rollingStockId: rollingStock.id,
    seriesCode: local.seriesCode, // always required
    roadNumber: local.roadNumber,
    livery: local.livery,
    depot: local.depot,
    flywheelFitted: featureFlagToBool(patch.flywheelFitted ?? local.flywheelFitted),
    bodyShell: patch.bodyShell ?? local.bodyShell,
    chassis: patch.chassis ?? local.chassis,
    interiorLights: patch.interiorLights ?? local.interiorLights,
    lights: patch.lights ?? local.lights,
    dccInterface: local.dccInterface,
    control: local.control,
    couplingSocket: patch.couplingSocket ?? local.couplingSocket,
    closeCouplers: featureFlagToBool(patch.closeCouplers ?? local.closeCouplers),
    digitalShunting: featureFlagToBool(patch.digitalShunting ?? local.digitalShunting)
  });
  // handle Result<null, CommandError>
}

// Helper: 'YES'→true, 'NO'→false, null→null
function featureFlagToBool(v: 'YES' | 'NO' | null | boolean): boolean | null {
  if (v === 'YES' || v === true) return true;
  if (v === 'NO' || v === false) return false;
  return null;
}
```

---

### 2. `update_rolling_stock_identification` _(unchanged usage)_

Still used for inline saves of Series Code, Road Number, Livery, Depot.

```typescript
commands.updateRollingStockIdentification({
  railwayModelId,
  rollingStockId,
  seriesCode,
  roadNumber,
  livery,
  depot
});
```

---

### 3. `update_rolling_stock_dcc` _(unchanged usage)_

Still used for inline saves of Control Type, DCC Interface, Length.

```typescript
commands.updateRollingStockDcc({
  railwayModelId,
  rollingStockId,
  control,
  dccInterface,
  lengthMillimeters,
  lengthInches
});
```

---

### 4. `update_rolling_stock_railway_company` _(unchanged usage)_

Still used for Railway Company selection via `BadgePicker`.

```typescript
commands.updateRollingStockRailwayCompany({
  railwayModelId,
  rollingStockId,
  railwayCompanyId
});
```

---

## Error Handling Contract

All commands return `Result<null, CommandError>`. The UI must:

1. Check `result.status === 'error'` and surface the error message inline (below the field).
2. Revert the local state to its previous value on error.
3. Keep the field in edit mode (or re-enter it) so the user can retry.

```typescript
if (result.status === 'error') {
  // revert local state
  // show inline error
} else {
  // apply optimistic update permanently
}
```

---

## Type Enum Reference (from generated bindings)

### `Control`

```
'DCC_READY' | 'DCC_FITTED' | 'DCC_SOUND' | 'NO_DCC'
```

### `DccInterface`

```
'NEM_651' | 'NEM_652' | 'NEM_654' | 'PLUX_8' | 'PLUX_12' | 'PLUX_16' | 'PLUX_22'
| 'NEXT_18' | 'NEXT_18_S' | 'MTC_21'
```

### `BodyShellType` (passed as `string` in args)

```
'PLASTIC' | 'METAL_DIE_CAST'
```

### `ChassisType` (passed as `string` in args)

```
'PLASTIC' | 'METAL_DIE_CAST'
```

### `CouplingSocket` (passed as `string` in args)

```
'NONE' | 'NEM_355' | 'NEM_356' | 'NEM_357' | 'NEM_359' | 'NEM_360' | 'NEM_362' | 'NEM_365'
```
