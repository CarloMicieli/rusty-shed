# Contract: update_rolling_stock_specifications

**Command**: `update_rolling_stock_specifications`
**Trigger**: Save button in the technical specification drawer (FR-021–FR-029, FR-027, FR-028)
**Layer path**: `Args → UpdateRollingStockSpecificationsInput → UpdateRollingStockSpecifications::execute → RailwayModelRepository::save`

---

## Rust Args Type

```rust
/// Full technical specification payload for a RollingStock unit.
/// Saves all four drawer sections (Identification, Technical, Control, Coupling) atomically.
#[derive(Debug, Clone, serde::Deserialize, validator::Validate, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockSpecificationsArgs {
    pub railway_model_id: RailwayModelId,
    pub rolling_stock_id: RollingStockId,

    // ── Identification ─────────────────────────────────────────────────────────
    /// Required — must be non-empty.
    pub series_code: String,
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub depot: Option<String>,

    // ── Technical ──────────────────────────────────────────────────────────────
    pub flywheel_fitted: Option<bool>,
    pub body_shell: Option<String>,
    pub chassis: Option<String>,
    pub interior_lights: Option<String>,
    pub lights: Option<String>,

    // ── Control ────────────────────────────────────────────────────────────────
    /// Only relevant for motorised rolling stock (Locomotive, EMU, Railcar).
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,

    // ── Coupling ───────────────────────────────────────────────────────────────
    pub coupling_socket: Option<String>,
    pub close_couplers: Option<bool>,
    pub digital_shunting: Option<bool>,
}
```

## Rust Command Handler

```rust
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_specifications(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockSpecificationsArgs,
) -> Result<(), CommandError> {
    args.validate()?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockSpecifications::execute(&mut uow, args.try_into()?).await?;
    uow.commit().await.map_err(CommandError::from)?;
    Ok(())
}
```

## Use Case Input

```rust
pub struct UpdateRollingStockSpecificationsInput {
    pub railway_model_id: RailwayModelId,
    pub rolling_stock_id: RollingStockId,
    pub series_code: String,
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub depot: Option<String>,
    pub flywheel_fitted: Option<bool>,
    pub body_shell: Option<String>,
    pub chassis: Option<String>,
    pub interior_lights: Option<String>,
    pub lights: Option<String>,
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,
    pub coupling_socket: Option<String>,
    pub close_couplers: Option<bool>,
    pub digital_shunting: Option<bool>,
}
```

## Use Case Logic (`UpdateRollingStockSpecifications::execute`)

1. Load `RailwayModel` by `railway_model_id` (return `DomainError::NotFound` if absent)
2. Locate rolling stock with `rolling_stock_id` inside the aggregate (return `DomainError::NotFound` if absent)
3. Validate: `series_code` is non-empty → `DomainError::Validation` if blank
4. Call `model.update_rolling_stock_specifications(rolling_stock_id, input_spec)`:
   - Mutates all fields on the matching `RollingStock` variant
   - Builds `changed` JSON patch covering all mutated fields
   - Emits `RollingStockUpdated { changed }`
5. `repository.save(&mut model).await?` → drains event → SQL:
   ```sql
   UPDATE rolling_stocks
   SET series_code = ?,
       road_number = ?,
       livery      = ?,
       depot       = ?,
       technical_flywheel_fitted           = ?,
       technical_body_shell                = ?,
       technical_chassis                   = ?,
       technical_interior_lights           = ?,
       technical_lights                    = ?,
       dcc_interface                       = ?,
       control                             = ?,
       technical_coupling_socket           = ?,
       technical_coupling_close_couplers   = ?,
       technical_coupling_digital_shunting = ?
   WHERE id = ?
   ```

## New Domain Method on RailwayModel

```rust
pub fn update_rolling_stock_specifications(
    &mut self,
    rolling_stock_id: &RollingStockId,
    spec: RollingStockSpecPatch,
) -> Result<(), DomainError> {
    let rs = self.rolling_stocks.iter_mut()
        .find(|rs| rs.id_as_ref() == rolling_stock_id)
        .ok_or_else(|| DomainError::NotFound { ... })?;

    if spec.series_code.trim().is_empty() {
        return Err(DomainError::Validation("series_code must not be empty".to_string()));
    }

    let changed = rs.apply_specifications(spec);  // Returns serde_json::Value patch

    self.push_event(RailwayModelEvent::RollingStockUpdated {
        event_id: Uuid::new_v4(),
        railway_model_id: self.id.clone(),
        rolling_stock_id: rolling_stock_id.clone(),
        timestamp: Utc::now().naive_utc(),
        changed,
    });
    Ok(())
}
```

## TypeScript Binding (generated by specta)

```typescript
export interface UpdateRollingStockSpecificationsArgs {
  railwayModelId: RailwayModelId;
  rollingStockId: RollingStockId;
  // Identification
  seriesCode: string;
  roadNumber: string | null;
  livery: string | null;
  depot: string | null;
  // Technical
  flywheelFitted: boolean | null;
  bodyShell: string | null;
  chassis: string | null;
  interiorLights: string | null;
  lights: string | null;
  // Control
  dccInterface: DccInterface | null;
  control: Control | null;
  // Coupling
  couplingSocket: string | null;
  closeCouplers: boolean | null;
  digitalShunting: boolean | null;
}

// In commands:
async updateRollingStockSpecifications(
  args: UpdateRollingStockSpecificationsArgs
): Promise<Result<null, CommandError>>
```

## Drawer Open: Data Loading

Before the drawer is displayed, the frontend fetches current values:

```typescript
// On drawer open — reuse existing command:
const result = await commands.getRailwayModelById(railwayModelId);
// Locate the specific rolling stock within result.data.rollingStock
// Populate form state with current values
```

## Frontend Usage — Save

```typescript
// Save button in drawer
const result = await commands.updateRollingStockSpecifications({
  railwayModelId: props.railwayModelId,
  rollingStockId: props.rollingStockId,
  seriesCode: form.seriesCode,
  roadNumber: form.roadNumber || null,
  livery: form.livery || null,
  depot: form.depot || null,
  flywheelFitted: form.flywheelFitted,
  bodyShell: form.bodyShell || null,
  chassis: form.chassis || null,
  interiorLights: form.interiorLights || null,
  lights: form.lights || null,
  dccInterface: form.dccInterface,
  control: form.control,
  couplingSocket: form.couplingSocket || null,
  closeCouplers: form.closeCouplers,
  digitalShunting: form.digitalShunting
});

if (result.status === 'ok') {
  toaster.success(m.specs_drawer_save_success());
  drawerOpen = false;
  // Re-fetch parent model or emit refresh event
} else {
  // Drawer remains open; show inline error (FR-028)
  inlineError = m.specs_drawer_save_error();
}
```

## Unsaved Changes Guard

```typescript
// Dirty-check using $derived.by in drawer component
const isDirty = $derived.by(() => {
  return JSON.stringify(form) !== JSON.stringify(originalValues);
});

// On close attempt when dirty → show confirmation Dialog (FR-027)
```

## Error Cases

| Scenario                                 | Error                                                                 |
| ---------------------------------------- | --------------------------------------------------------------------- |
| `railway_model_id` not found             | `CommandError::NotFound`                                              |
| `rolling_stock_id` not in model          | `CommandError::NotFound`                                              |
| `series_code` is empty                   | `CommandError::ValidationError { seriesCode: ["must not be empty"] }` |
| Invalid `DccInterface` or `Control` enum | `CommandError::ValidationError`                                       |
| Database failure                         | `CommandError::DatabaseError`                                         |

## Test Requirements

- **Unit (Rust)**: Full spec save emits `RollingStockUpdated` with all section fields in `changed`
- **Unit (Rust)**: Empty `series_code` on drawer save returns `DomainError::Validation`
- **Unit (Rust)**: Empty record (all optional fields null) accepted without error
- **Integration (Rust)**: All 14 spec columns updated correctly; re-fetch confirms values
- **Component (Vitest)**: Drawer populates from fetched data; dirty-check triggers confirmation dialog on close; inline error shown when save fails with values preserved (FR-028)
- **Acceptance (Manual)**: SC-003 — open drawer, complete all 4 sections, save — should complete in <4 minutes
