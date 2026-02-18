# Data Model: Rolling Stock Progressive Editing

**Branch**: `024-rolling-stock-edit` | **Date**: 2026-02-18

---

## Entities

### RailwayModel (existing aggregate — extended)

| Field               | Type                         | Nullable | Notes                                      |
| ------------------- | ---------------------------- | -------- | ------------------------------------------ |
| id                  | `RailwayModelId` (UUID)      | No       | Primary key                                |
| manufacturer_id     | `ManufacturerId`             | No       | FK to manufacturers                        |
| product_code        | `ProductCode`                | No       | Unique within manufacturer                 |
| description         | `String`                     | No       | Free-text; in-place editable (FR-001)      |
| details             | `Option<String>`             | Yes      | Free-text; in-place editable (FR-002)      |
| power_method        | `PowerMethod`                | No       | AC / DC / TrixExpress                      |
| scale               | `Scale`                      | No       | Badge-click constrained selection (FR-014) |
| epoch               | `Epoch`                      | No       | Badge-click constrained selection (FR-015) |
| category            | `Category`                   | No       | Read-only in this feature                  |
| delivery_date       | `Option<DeliveryDate>`       | Yes      | Read-only in this feature                  |
| availability_status | `Option<AvailabilityStatus>` | Yes      | Read-only in this feature                  |
| rolling_stocks      | `Vec<RollingStock>`          | No       | Child entities                             |
| pending_events      | `Vec<RailwayModelEvent>`     | —        | Aggregate event buffer (not persisted)     |

**New domain methods added in this feature:**

- `update_scale(scale: Scale)` → emits `RailwayModelUpdated { changed: {"scale": "..."} }`
- `update_epoch(epoch: Epoch)` → emits `RailwayModelUpdated { changed: {"epoch": "..."} }`
- `update_rolling_stock_identification(id, patches)` → emits `RollingStockUpdated { changed: {...} }`
- `update_rolling_stock_railway_company(id, company_id)` → emits `RollingStockUpdated { changed: {"railway_company_id": "..."} }`
- `update_rolling_stock_specifications(id, specs)` → emits `RollingStockUpdated { changed: {...} }`

---

### RollingStock (existing enum — methods added)

The `RollingStock` enum discriminates across 5 variants (Locomotive, ElectricMultipleUnit, FreightCar, PassengerCar, Railcar). Fields relevant to this feature are present in all variants:

| Field                    | Type                              | Nullable                   | Editable Via                           |
| ------------------------ | --------------------------------- | -------------------------- | -------------------------------------- |
| id                       | `RollingStockId` (UUID)           | No                         | —                                      |
| railway_id               | `RailwayCompanyId`                | No                         | BadgePicker on card (FR-020)           |
| series_code              | `String`                          | No                         | In-place on card (FR-008)              |
| road_number              | `Option<String>`                  | Yes                        | In-place on card (FR-009)              |
| livery                   | `Option<String>`                  | Yes                        | In-place on card (FR-010)              |
| depot                    | `Option<String>`                  | Yes (Locomotive/EMU only)  | In-place on card (FR-011)              |
| technical_specifications | `Option<TechnicalSpecifications>` | Yes                        | Drawer — Technical + Coupling sections |
| dcc_interface            | `Option<DccInterface>`            | Yes (motorised types only) | Drawer — Control section               |
| control                  | `Option<Control>`                 | Yes (motorised types only) | Drawer — Control section               |

**New methods added to RollingStock enum:**

- `apply_identification_patch(series_code, road_number, livery, depot)` — returns `changed: serde_json::Value`
- `apply_railway_company(company_id: RailwayCompanyId)` — returns `changed: serde_json::Value`
- `apply_specifications(specs: RollingStockSpecPatch)` — returns `changed: serde_json::Value`

---

### TechnicalSpecifications (existing value object — used for drawer sections)

| Field            | Type             | DB Column                                  | Drawer Section |
| ---------------- | ---------------- | ------------------------------------------ | -------------- |
| flywheel_fitted  | `Option<bool>`   | `technical_flywheel_fitted TEXT`           | Technical      |
| body_shell       | `Option<String>` | `technical_body_shell TEXT`                | Technical      |
| chassis          | `Option<String>` | `technical_chassis TEXT`                   | Technical      |
| interior_lights  | `Option<String>` | `technical_interior_lights TEXT`           | Technical      |
| lights           | `Option<String>` | `technical_lights TEXT`                    | Technical      |
| coupling_socket  | `Option<String>` | `technical_coupling_socket TEXT`           | Coupling       |
| close_couplers   | `Option<bool>`   | `technical_coupling_close_couplers TEXT`   | Coupling       |
| digital_shunting | `Option<bool>`   | `technical_coupling_digital_shunting TEXT` | Coupling       |

---

### Domain Events (RailwayModelEvent — extended)

```
RailwayModelEvent
├── RailwayModelCreated      (existing — unchanged)
├── RailwayModelUpdated      (existing — used for description, details, scale, epoch patches)
├── RollingStockAdded        (existing — unchanged)
├── RollingStockRemoved      (existing — unchanged)
└── RollingStockUpdated      (NEW)
    ├── event_id: Uuid
    ├── railway_model_id: RailwayModelId
    ├── rolling_stock_id: RollingStockId
    ├── timestamp: NaiveDateTime
    └── changed: serde_json::Value   ← minimal patch of changed fields
```

**Repository event handling added:**

```
match event {
    RollingStockUpdated { rolling_stock_id, changed, .. } =>
        self.update_rolling_stock(&rolling_stock_id, &changed).await?
}
```

---

## State Transitions

### In-Place Text Field

```
DISPLAY ──[click / focus]──► EDITING
  │                              │
  │                              ├─[blur]──► SAVING ──[ok]──► DISPLAY (updated)
  │                              │               └──[err]──► EDITING (error shown, value preserved)
  │                              └─[Escape]──► DISPLAY (original)
```

### Constrained Badge Picker

```
CLOSED ──[badge click]──► OPEN
  │                          │
  │                          ├─[select option]──► SAVING ──[ok]──► CLOSED (value updated)
  │                          │                       └──[err]──► CLOSED (toast error, value reverted)
  │                          └─[Escape / outside click]──► CLOSED (original preserved)
```

### Technical Specs Drawer

```
CLOSED ──[Edit Specs click]──► FETCHING ──[ok]──► OPEN (form populated)
  │                                  └──[err]──► CLOSED (toast error)
  │
OPEN (no changes) ──[close]──► CLOSED
OPEN (dirty) ──[close]──► CONFIRM DIALOG ──[Discard]──► CLOSED
                                          └──[Cancel]──► OPEN (continue editing)
OPEN ──[Save]──► SAVING ──[ok]──► CLOSED (card updated)
                     └──[err]──► OPEN (inline error, values preserved)
```

---

## Validation Rules

### RailwayModel Field Updates

| Field       | Rule                                          |
| ----------- | --------------------------------------------- |
| description | Non-empty string; trimmed; max 500 characters |
| details     | Nullable; max 2000 characters when present    |
| scale       | Must be a valid `Scale` enum variant          |
| epoch       | Must be a valid `Epoch` enum variant          |

### RollingStock Field Updates

| Field              | Rule                                                       |
| ------------------ | ---------------------------------------------------------- |
| series_code        | Non-empty string; trimmed                                  |
| road_number        | Nullable; trimmed when present                             |
| livery             | Nullable; trimmed when present                             |
| depot              | Nullable; trimmed when present                             |
| railway_company_id | Must reference a valid record in `railway_companies` table |
| dcc_interface      | Must be a valid `DccInterface` enum variant when provided  |
| control            | Must be a valid `Control` enum variant when provided       |

All validation enforced in Rust (domain layer or application use case). Frontend may show inline hints but backend is authoritative.

---

## Database Impact

**No new migration required.** All mutations in this feature generate SQL UPDATEs against existing columns.

### UPDATE patterns generated by RollingStockUpdated event

```sql
-- Identification patch (subset of fields):
UPDATE rolling_stocks
SET series_code = ?,
    road_number = ?,
    livery      = ?,
    depot       = ?
WHERE id = ?;

-- Railway company patch:
UPDATE rolling_stocks
SET railway_company_id = ?
WHERE id = ?;

-- Full specifications patch (drawer save):
UPDATE rolling_stocks
SET series_code                       = ?,
    road_number                       = ?,
    livery                            = ?,
    depot                             = ?,
    technical_flywheel_fitted         = ?,
    technical_body_shell              = ?,
    technical_chassis                 = ?,
    technical_interior_lights         = ?,
    technical_lights                  = ?,
    dcc_interface                     = ?,
    control                           = ?,
    technical_coupling_socket         = ?,
    technical_coupling_close_couplers = ?,
    technical_coupling_digital_shunting = ?
WHERE id = ?;
```

### UPDATE pattern for RailwayModelUpdated event (scale/epoch)

```sql
UPDATE railway_models
SET scale = ?,
    epoch = ?
WHERE id = ?;
```
