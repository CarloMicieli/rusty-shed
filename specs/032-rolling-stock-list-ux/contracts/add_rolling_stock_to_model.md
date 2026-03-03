# IPC Contract: `add_rolling_stock_to_model`

**Transport**: Tauri IPC (`invoke`)
**Generated types**: via `tauri-specta` — types live in `src/lib/bindings.ts` after `pnpm tauri dev`

---

## Command signature (Rust)

```rust
#[tauri::command]
#[specta::specta]
pub async fn add_rolling_stock_to_model(
    state: tauri::State<'_, AppState>,
    args: AddRollingStockToModelArgs,
) -> Result<RollingStockId, CommandError>
```

## TypeScript generated binding

```typescript
async addRollingStockToModel(
  args: AddRollingStockToModelArgs
): Promise<Result<RollingStockId, CommandError>>
```

---

## Request payload: `AddRollingStockToModelArgs`

| Field                | Type                   | Required | Constraint                                       |
| -------------------- | ---------------------- | -------- | ------------------------------------------------ |
| `railway_model_id`   | `string`               | ✅       | Valid TRN: `trn:railway-model:*`                 |
| `railway_company_id` | `string`               | ✅       | Valid TRN: `trn:railway-company:*`               |
| `category`           | `RollingStockCategory` | ✅       | One of 5 enum values                             |
| `series_code`        | `string`               | ✅       | Non-empty after trim                             |
| `road_number`        | `string \| null`       | ❌       | —                                                |
| `livery`             | `string \| null`       | ❌       | —                                                |
| `depot`              | `string \| null`       | ❌       | —                                                |
| `control`            | `Control \| null`      | ❌       | Must be a valid `Control` enum string if present |

## Response

**Success**: `{ status: "ok", data: RollingStockId }` — the newly assigned `RollingStockId` (TRN string)

**Error cases**:

| Code               | Condition                                                                                          |
| ------------------ | -------------------------------------------------------------------------------------------------- |
| `NOT_FOUND`        | `railway_model_id` does not map to an existing model                                               |
| `VALIDATION_ERROR` | `series_code` empty, `railway_company_id` invalid, `category` unrecognised, `control` unrecognised |
| `DATABASE_ERROR`   | SQLite write failure                                                                               |

---

## Existing commands touched by this feature

### `update_rolling_stock_identification` (unchanged signature)

Existing command. Now also updates `depot`. The `RollingStockCard` will call this for Depot inline edits once `depot` is wired to an `InPlaceEdit`.

```typescript
async updateRollingStockIdentification(
  args: UpdateRollingStockIdentificationArgs // already includes depot: string | null
): Promise<Result<null, CommandError>>
```

No signature change needed — `depot` is already present in `UpdateRollingStockIdentificationArgs`.

---

## Enum values reference

### `RollingStockCategory` (existing)

```
"LOCOMOTIVE" | "ELECTRIC_MULTIPLE_UNIT" | "FREIGHT_CAR" | "PASSENGER_CAR" | "RAILCAR"
```

### `Control` (existing)

```
"DCC_READY" | "DCC_FITTED" | "DCC_SOUND" | "NO_DCC" | "MFX" | "SELECTRIX"
```

(exact variants from bindings; creation drawer exposes the same `controlOptions` array as `RollingStockSpecsDrawer`)

---

## Frontend usage pattern

```typescript
// In RollingStockCreateDrawer.svelte
const result = await commands.addRollingStockToModel({
  railway_model_id: railwayModelId,
  railway_company_id: selectedCompanyId,
  category: selectedCategory,
  series_code: form.seriesCode.trim(),
  road_number: form.roadNumber || null,
  livery: form.livery || null,
  depot: form.depot || null,
  control: (form.control || null) as Control | null
});

if (result.status === 'error') {
  inlineError = m.rolling_stock_create_error();
  return;
}

toaster.success(m.rolling_stock_create_success());
onCreated(result.data); // passes new RollingStockId to parent for refresh
onClose();
```
