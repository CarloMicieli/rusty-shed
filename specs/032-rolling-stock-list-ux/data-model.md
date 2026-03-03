# Data Model: Rolling Stock List UX (032)

**Date**: 2026-03-03

---

## Existing entities (unchanged)

### RollingStock (catalog, discriminated union)

Unchanged domain entity in `src-tauri/src/catalog/domain/railway_model/rolling_stock.rs`. Five variants: Locomotive, ElectricMultipleUnit, FreightCar, PassengerCar, Railcar. Each has `series_code: String` (required), `road_number: Option<String>`, `livery: Option<String>`, `depot: Option<String>` (Locomotive, EMU, Railcar only), `control: Option<Control>`, `dcc_interface: Option<DccInterface>`.

### OwnedRollingStockView (modified)

**File**: `src-tauri/src/collecting/domain/owned_rolling_stock_view.rs`

**Change**: Add `depot: Option<String>` field.

**Before** (abridged):

```rust
pub struct OwnedRollingStockView {
    pub id: OwnedRollingStockId,
    pub rolling_stock_id: RollingStockId,
    pub notes: Option<String>,
    pub series: Option<String>,
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub control: Option<Control>,
    pub railway_company_name: Option<String>,
    pub digital: Option<DigitalSetup>,
}
```

**After** (abridged):

```rust
pub struct OwnedRollingStockView {
    pub id: OwnedRollingStockId,
    pub rolling_stock_id: RollingStockId,
    pub notes: Option<String>,
    pub series: Option<String>,
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub control: Option<Control>,
    pub railway_company_name: Option<String>,
    pub digital: Option<DigitalSetup>,
    pub depot: Option<String>,      // ← NEW
}
```

**SQL change** (in `sqlite_railway_model_repository.rs`): Add `rs.depot` to the SELECT that builds `OwnedRollingStockView`.

---

## New transport type

### AddRollingStockToModelArgs

**File**: `src-tauri/src/catalog/infrastructure/tauri/catalogue_commands.rs` (or equivalent commands file)

```rust
/// Transport Args for adding a new rolling stock variant to an existing Railway Model.
/// Follows ADR-8: Args suffix, derives Debug/Clone/Validate/Type/Deserialize.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate, specta::Type)]
pub struct AddRollingStockToModelArgs {
    /// The parent railway model identifier (TRN string).
    pub railway_model_id: String,

    /// The railway company that operated this rolling stock (TRN string).
    pub railway_company_id: String,

    /// Rolling stock category. One of: LOCOMOTIVE, ELECTRIC_MULTIPLE_UNIT,
    /// FREIGHT_CAR, PASSENGER_CAR, RAILCAR.
    pub category: String,

    /// Series code identifying this variant (required, non-empty).
    #[validate(length(min = 1, message = "series_code must not be empty"))]
    pub series_code: String,

    /// Optional road/fleet number.
    pub road_number: Option<String>,

    /// Optional livery description.
    pub livery: Option<String>,

    /// Optional depot name.
    pub depot: Option<String>,

    /// Optional control type (Control enum serialized as string, e.g. "DCC_READY").
    pub control: Option<String>,
}
```

---

## New application use case

### AddRollingStockToModel

**File**: `src-tauri/src/catalog/application/add_rolling_stock_to_model.rs`

**Input** (mapped from `AddRollingStockToModelArgs` after boundary validation):

```rust
pub struct AddRollingStockToModelInput {
    pub railway_model_id: RailwayModelId,
    pub railway_company_id: RailwayCompanyId,
    pub category: RollingStockCategory,
    pub series_code: String,
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub depot: Option<String>,
    pub control: Option<Control>,
}
```

**Flow**:

1. Load `RailwayModel` by `railway_model_id` from repository → `NotFound` error if absent
2. Verify `railway_company_id` is valid → `ValidationError` if parse fails
3. Build `RollingStockParams` from category + fields (type-specific discriminant uses a per-category default)
4. Call `railway_model.add_rolling_stock(params)` → emits `RollingStockAdded` domain event
5. Save aggregate via repository (drains events, writes to `rolling_stocks` table atomically)
6. Return the generated `RollingStockId`

**Category → default discriminant mapping**:

| Category               | Default sub-type       |
| ---------------------- | ---------------------- |
| LOCOMOTIVE             | ElectricLocomotive     |
| ELECTRIC_MULTIPLE_UNIT | MotorCar               |
| RAILCAR                | MotorCar               |
| PASSENGER_CAR          | (no sub-type required) |
| FREIGHT_CAR            | (no sub-type required) |

---

## State transitions

```
RailwayModel (existing)
  └─ [add_rolling_stock_to_model invoked]
       ↓
  RollingStockAdded event emitted
       ↓
  Repository drains event → INSERT INTO rolling_stocks
       ↓
  RollingStockId returned to frontend
       ↓
  Frontend triggers parent refresh (onRollingStockAdded callback)
       ↓
  RollingStockList re-renders with new entry
```

---

## Validation rules

| Field                | Rule                                                                   |
| -------------------- | ---------------------------------------------------------------------- |
| `railway_model_id`   | Must parse to a valid `RailwayModelId`; corresponding model must exist |
| `railway_company_id` | Must parse to a valid `RailwayCompanyId`                               |
| `category`           | Must be one of the 5 `RollingStockCategory` values                     |
| `series_code`        | Non-empty (length ≥ 1) after trimming                                  |
| `road_number`        | Optional; if present, non-empty recommended but not enforced           |
| `control`            | Optional; if present, must parse to a valid `Control` enum variant     |
| `depot`, `livery`    | Optional; no format constraint                                         |
