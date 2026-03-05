# Tauri IPC Contracts: Maintenance Page Overhaul

**Branch**: `034-maintenance-overhaul`
**Date**: 2026-03-05

All commands use Tauri `invoke` with specta-generated TypeScript types.

---

## Existing Commands (Unchanged)

### `get_maintenance_dashboard`

- **Input**: none
- **Output**: `MaintenanceCardView[]`
- **Change**: Repository query enhanced with LEFT JOIN to populate `display_info` field. No signature change.

### `add_maintenance_card`

- **Input**: `OwnedRollingStockId`
- **Output**: `MaintenanceCardId`
- **Change**: Repository save now returns `DomainError::Conflict` if a card already exists for the given rolling stock. The `CommandError` mapping converts this to an error the frontend can handle. No signature change.

### `add_maintenance_event`

- **Input**: `AddMaintenanceArgs { id, maintenanceCardId, datePerformed, maintenanceType?, notes? }`
- **Output**: `void`
- **Change**: No change to command itself. Frontend usage changes: `maintenanceCardId` is now pre-filled from the detail page context rather than selected by the user.

---

## New Command

### `get_maintenance_card`

**Purpose**: Retrieve a single maintenance card with all events and display info for the detail page.

**Rust signature**:

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_maintenance_card(
    state: tauri::State<'_, AppState>,
    card_id: MaintenanceCardId,
) -> Result<Option<MaintenanceCardView>, CommandError>
```

**TypeScript usage**:

```typescript
import { commands } from '$lib/bindings';

const card = await commands.getMaintenanceCard(cardId);
// card: MaintenanceCardView | null
```

**Input**: `MaintenanceCardId` (TRN string, e.g. `trn:maintenance-card:<uuid>`)

**Output**: `MaintenanceCardView | null`

- `null` when no card exists for the given ID (404-equivalent)
- On success: `MaintenanceCardView` with `displayInfo` populated if catalog data is available

**Error cases**:

- `CommandError::NotFound` equivalent if card does not exist (returns `Ok(None)` at application level)
- `CommandError` on infrastructure failure (DB error)

---

## Updated View Types (specta-generated)

### `RollingStockDisplayInfo` (new)

```typescript
interface RollingStockDisplayInfo {
  manufacturerName: string | null;
  productCode: string | null;
  seriesCode: string | null;
  roadNumber: string | null;
}
```

### `MaintenanceCardView` (extended)

```typescript
interface MaintenanceCardView {
  id: MaintenanceCardId;
  ownedRollingStockId: OwnedRollingStockId;
  lastMaintenanceDate: string | null; // NaiveDate → ISO string
  nextMaintenanceDate: string | null; // NaiveDate → ISO string
  events: MaintenanceCardEventView[];
  displayInfo: RollingStockDisplayInfo | null; // NEW
}
```

---

## Error Contract

When `add_maintenance_card` is called for a rolling stock that already has a card, the frontend receives a `CommandError` with a message matching `"A maintenance card already exists"`. The `AddMaintenanceCardModal` must catch this error and display a user-facing message (via Paraglide key `maintenance_card_already_exists` — new key to be added).
