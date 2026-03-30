# Tauri IPC Contracts: Train Formations

**Branch**: `039-train-formations` | **Phase**: 1 | **Date**: 2026-03-29

All commands are exposed via Tauri `invoke` and typed by `specta`. Naming follows ADR 8 conventions:

- `Args` = write payloads (derive `Debug, Clone, garde::Validate, specta::Type, serde::Deserialize`)
- Command handlers return `Result<T, CommandError>` — defined in `src-tauri/src/core/infrastructure/error.rs`. **Never** `Result<T, String>`.
- Command handlers validate `Args` at the boundary before invoking use cases.
- Query handlers return read-optimized view models.
- Domain errors propagate via `From<DomainError> for CommandError` using `?` — no manual `.map_err()` needed unless converting from a non-domain type.

---

## Formation CRUD

### `create_train_formation`

Creates a new `TrainFormation` record.

**Input (`CreateTrainFormationArgs`)**:

```rust
pub struct CreateTrainFormationArgs {
    pub name: String,           // required, non-empty, unique
    pub category_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
}
```

**Output**: `TrainFormationView` (see view models)

**Errors**: `DUPLICATE_NAME`, `INVALID_YEAR_RANGE`, `VALIDATION_ERROR`

---

### `update_train_formation`

Updates metadata of an existing `TrainFormation`.

**Input (`UpdateTrainFormationArgs`)**:

```rust
pub struct UpdateTrainFormationArgs {
    pub id: String,
    pub name: String,
    pub category_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
}
```

**Output**: `TrainFormationView`

**Errors**: `NOT_FOUND`, `DUPLICATE_NAME`, `INVALID_YEAR_RANGE`

---

### `delete_train_formation`

Deletes a `TrainFormation` and all its `FormationElement` rows (cascade).

**Input**: `formation_id: String`

**Output**: `()` (unit)

**Errors**: `NOT_FOUND`

---

### `get_train_formation`

Fetches a single formation with its full composition + ownership status.

**Input**: `formation_id: String`

**Output**: `TrainFormationDetail`

**Errors**: `NOT_FOUND`

---

### `get_train_formations`

Lists all formations (summary cards, no composition detail).

**Input**: none

**Output**: `Vec<TrainFormationSummary>`

---

## Composition CRUD

### `add_formation_element`

Appends a Prototype slot to a formation's composition.

**Input (`AddFormationElementArgs`)**:

```rust
pub struct AddFormationElementArgs {
    pub formation_id: String,
    pub prototype_id: String,
    pub owned_rolling_stock_id: Option<String>,  // optional quick-assign
}
```

**Output**: `FormationElementView`

**Errors**: `FORMATION_NOT_FOUND`, `PROTOTYPE_NOT_FOUND`, `INVALID_ROLLING_STOCK_ID`

---

### `remove_formation_element`

Removes a slot from the composition and shifts `position_order` of subsequent elements.

**Input**: `element_id: String`

**Output**: `()` (unit)

**Errors**: `NOT_FOUND`

---

### `reorder_formation_elements`

Bulk-updates `position_order` for all elements in a formation (called on drag-and-drop finalize).

**Input (`ReorderFormationElementsArgs`)**:

```rust
pub struct ReorderFormationElementsArgs {
    pub formation_id: String,
    pub ordered_element_ids: Vec<String>,  // complete ordered list of all element IDs
}
```

**Output**: `()` (unit)

**Errors**: `FORMATION_NOT_FOUND`, `ELEMENT_IDS_MISMATCH` (if sent IDs don't match DB set)

---

### `assign_rolling_stock_to_element`

Assigns (or clears) an owned physical model to a specific formation element slot.

**Input (`AssignRollingStockToElementArgs`)**:

```rust
pub struct AssignRollingStockToElementArgs {
    pub element_id: String,
    pub owned_rolling_stock_id: Option<String>,  // None = unassign
}
```

**Output**: `FormationElementView`

**Errors**: `ELEMENT_NOT_FOUND`, `ROLLING_STOCK_NOT_FOUND`

---

### `set_traction_override`

Sets the per-slot traction override flag.

**Input (`SetTractionOverrideArgs`)**:

```rust
pub struct SetTractionOverrideArgs {
    pub element_id: String,
    pub traction_override: i32,  // 0 = use prototype default, 1 = force count, -1 = force exclude
}
```

**Output**: `FormationElementView`

**Errors**: `ELEMENT_NOT_FOUND`

---

## Prototype Library

### `get_prototypes`

Returns all prototypes (seeded + custom), grouped by railway company.

**Input**: `search_query: Option<String>` (real-time filter; debounced on frontend)

**Output**: `Vec<PrototypeGroupView>` (grouped by `railway_company_id`)

---

### `create_custom_prototype`

Creates a user-defined prototype record (`is_custom = true`).

**Input (`CreateCustomPrototypeArgs`)**:

```rust
pub struct CreateCustomPrototypeArgs {
    pub railway_company_id: String,
    pub series_code: String,
    pub car_type: String,       // validated against CarType enum
    pub service_level: Option<String>,
    pub category: String,
    pub is_motorized: bool,
    pub default_is_dummy: bool,
    pub notes: Option<String>,
}
```

**Output**: `PrototypeView`

**Errors**: `COMPANY_NOT_FOUND`, `DUPLICATE_SERIES_CODE`, `INVALID_CAR_TYPE`

---

## Formation Categories

### `get_formation_categories`

Returns all formation categories (seeded + custom).

**Input**: none

**Output**: `Vec<FormationCategoryView>`

---

### `create_formation_category`

Creates a user-defined category (`is_custom = true`).

**Input (`CreateFormationCategoryArgs`)**:

```rust
pub struct CreateFormationCategoryArgs {
    pub name: String,  // non-empty, unique
}
```

**Output**: `FormationCategoryView`

**Errors**: `DUPLICATE_NAME`

---

## View Models (Specta-typed Read DTOs)

```rust
// Post-write response for create_train_formation and update_train_formation
pub struct TrainFormationView {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    pub element_count: i64,
    pub has_traction: bool,
}

// Summary card for formation list
pub struct TrainFormationSummary {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub epoch: Option<String>,
    pub element_count: i64,
    pub has_traction: bool,        // derived
    pub owned_count: i64,          // elements with owned_rolling_stock_id
    pub planned_count: i64,        // elements without owned_rolling_stock_id
}

// Full detail for formation builder screen
pub struct TrainFormationDetail {
    pub id: String,
    pub name: String,
    pub category: Option<FormationCategoryView>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    pub elements: Vec<FormationElementView>,  // ordered by position_order
    pub has_traction: bool,                   // derived
}

// Individual element / slot
pub struct FormationElementView {
    pub id: String,
    pub position_order: i32,
    pub prototype: PrototypeView,
    pub owned_rolling_stock_id: Option<String>,
    pub snapshot_series_code: Option<String>,    // populated on assign; retained on model delete
    pub snapshot_company_name: Option<String>,   // populated on assign; retained on model delete
    pub stock_not_found: bool,                   // true when snapshot_series_code is set but owned_rolling_stock_id is None
    pub owned_count_for_prototype: i64,          // # of owned_rolling_stocks with same prototype_id
    pub traction_override: i32,
    pub is_traction_slot: bool,                  // derived from prototype + override
}

// Prototype in search results
pub struct PrototypeView {
    pub id: String,
    pub railway_company_id: String,
    pub company_name: String,
    pub series_code: String,
    pub car_type: String,
    pub service_level: Option<String>,
    pub category: String,
    pub is_motorized: bool,
    pub default_is_dummy: bool,
    pub is_custom: bool,
}

// Grouped for search results drawer
pub struct PrototypeGroupView {
    pub railway_company_id: String,
    pub company_name: String,
    pub prototypes: Vec<PrototypeView>,
}

pub struct FormationCategoryView {
    pub id: String,
    pub name: String,
    pub is_custom: bool,
}
```

---

## `garde` Validation Pattern

All `*Args` structs use `garde` (v0.22.1, already in `Cargo.toml`) — not `validator`. Apply constraints declaratively on the struct, implement cross-field logic in a custom function, and invoke at the IPC boundary.

### Step 1 — Declare constraints on `Args`

```rust
use garde::Validate;

#[derive(Debug, Clone, serde::Deserialize, specta::Type, garde::Validate)]
pub struct CreateTrainFormationArgs {
    #[garde(length(min = 1, max = 100))]
    pub name: String,

    pub category_id: Option<String>,

    #[garde(range(min = 1800, max = 2100))]
    pub start_year: Option<i32>,

    // Cross-field check: end_year must be ≥ start_year
    #[garde(custom(validate_year_range))]
    pub end_year: Option<i32>,

    #[garde(skip)]
    pub epoch: Option<String>,

    #[garde(skip)]
    pub notes: Option<String>,
}
```

Use `#[garde(skip)]` for fields with no built-in constraint so `garde` ignores them.

### Step 2 — Custom cross-field validator

```rust
fn validate_year_range(end: &Option<i32>, args: &CreateTrainFormationArgs) -> garde::Result {
    if let (Some(s), Some(e)) = (args.start_year, *end) {
        if s > e {
            return Err(garde::Error::new("start_year cannot exceed end_year"));
        }
    }
    Ok(())
}
```

A garde custom validator is monomorphised to its specific struct type — the **same function cannot be shared** across two different `Args` structs. Extract the shared logic into a private helper and declare a separate function for `UpdateTrainFormationArgs`:

```rust
fn check_year_range(start: Option<i32>, end: Option<i32>) -> garde::Result {
    if let (Some(s), Some(e)) = (start, end) {
        if s > e {
            return Err(garde::Error::new("start_year cannot exceed end_year"));
        }
    }
    Ok(())
}

fn validate_year_range(end: &Option<i32>, args: &CreateTrainFormationArgs) -> garde::Result {
    check_year_range(args.start_year, *end)
}

fn validate_year_range_update(end: &Option<i32>, args: &UpdateTrainFormationArgs) -> garde::Result {
    check_year_range(args.start_year, *end)
}
```

Then apply `#[garde(custom(validate_year_range_update))]` on `UpdateTrainFormationArgs.end_year`.

### Step 3 — Invoke at IPC boundary

```rust
#[tauri::command]
#[specta::specta]
pub async fn create_train_formation(
    args: CreateTrainFormationArgs,
    state: tauri::State<'_, AppState>,
) -> Result<TrainFormationView, CommandError> {
    args.validate(&())
        .map_err(|e| CommandError::BusinessRule(e.to_string()))?;

    let mut uow = state.unit_of_work().await?;
    let result = CreateTrainFormation::execute(&mut uow, args).await?;
    uow.commit().await.map_err(CommandError::from)?;
    Ok(result)
}
```

- `args.validate(&())` — the `&()` context is required by `garde`; pass `&()` for all `Args` structs with no external context.
- Garde validation errors are mapped to `CommandError::BusinessRule` (the garde error message is already human-readable). Do **not** attempt to construct `CommandError::ValidationError` from garde output — its `HashMap` shape is for `validator`-crate field errors produced by the domain layer.
- All subsequent `?` calls propagate through `From<DomainError> for CommandError` automatically.

### Validation rules per command

| Command                      | Field                 | Rule                                  |
| ---------------------------- | --------------------- | ------------------------------------- |
| `create_train_formation`     | `name`                | `length(min=1, max=100)`              |
| `create_train_formation`     | `start_year`          | `range(min=1800, max=2100)` or `skip` |
| `create_train_formation`     | `end_year`            | `custom(validate_year_range)`         |
| `update_train_formation`     | `name`                | `length(min=1, max=100)`              |
| `update_train_formation`     | `start_year`          | `range(min=1800, max=2100)` or `skip` |
| `update_train_formation`     | `end_year`            | `custom(validate_year_range_update)`  |
| `create_custom_prototype`    | `series_code`         | `length(min=1, max=50)`               |
| `create_custom_prototype`    | `car_type`            | `custom(validate_car_type_enum)`      |
| `create_formation_category`  | `name`                | `length(min=1, max=80)`               |
| `reorder_formation_elements` | `ordered_element_ids` | `length(min=1)`                       |

> `validate_car_type_enum` checks the string is one of the 9 known `car_type` values defined in `data-model.md` before the repo is called.

---

## Error Variant Mapping

The string error codes used in each command section above map to these `CommandError` variants:

| Error code (spec shorthand) | `CommandError` variant             | Trigger                                                   |
| --------------------------- | ---------------------------------- | --------------------------------------------------------- |
| `DUPLICATE_NAME`            | `CommandError::Conflict(msg)`      | Unique constraint on `name` violated                      |
| `NOT_FOUND`                 | `CommandError::NotFound(msg)`      | Query returns 0 rows for a given ID                       |
| `FORMATION_NOT_FOUND`       | `CommandError::NotFound(msg)`      | `formation_id` not in `train_formations`                  |
| `PROTOTYPE_NOT_FOUND`       | `CommandError::NotFound(msg)`      | `prototype_id` not in `prototypes`                        |
| `ELEMENT_NOT_FOUND`         | `CommandError::NotFound(msg)`      | `element_id` not in `formation_elements`                  |
| `ROLLING_STOCK_NOT_FOUND`   | `CommandError::NotFound(msg)`      | `owned_rolling_stock_id` not in `owned_rolling_stocks`    |
| `COMPANY_NOT_FOUND`         | `CommandError::NotFound(msg)`      | `railway_company_id` not in `railway_companies`           |
| `INVALID_YEAR_RANGE`        | `CommandError::BusinessRule(msg)`  | `start_year > end_year`; emitted by garde or domain layer |
| `VALIDATION_ERROR`          | `CommandError::BusinessRule(msg)`  | Any garde field constraint failure                        |
| `INVALID_CAR_TYPE`          | `CommandError::BusinessRule(msg)`  | `car_type` string not in the 9-value enum                 |
| `ELEMENT_IDS_MISMATCH`      | `CommandError::BusinessRule(msg)`  | Sent element IDs don't match DB set for the formation     |
| `DUPLICATE_SERIES_CODE`     | `CommandError::Conflict(msg)`      | `(railway_company_id, series_code)` already exists        |
| `DATABASE_ERROR`            | `CommandError::DatabaseError(msg)` | sqlx error propagated via `From<sqlx::Error>`             |

The `From<DomainError> for CommandError` impl handles `NotFound`, `BusinessRule`, `Conflict`, and `ValidationError` variants automatically via `?`.

> **Domain Events** are defined in `quickstart.md` Phase A2a (`TrainFormationEvent` enum, 10 variants). That file is the single authoritative reference — do not redefine events here.

---

## TypeScript Bindings (specta-generated)

All Rust types above auto-generate into `src/lib/bindings.ts` via `tauri-specta`. No manual type redefinition on the frontend.

Key command signatures (auto-generated):

```ts
// From tauri-specta bindings:
createTrainFormation(args: CreateTrainFormationArgs): Promise<TrainFormationView>
updateTrainFormation(args: UpdateTrainFormationArgs): Promise<TrainFormationView>
deleteTrainFormation(formationId: string): Promise<void>
getTrainFormation(formationId: string): Promise<TrainFormationDetail>
getTrainFormations(): Promise<TrainFormationSummary[]>
addFormationElement(args: AddFormationElementArgs): Promise<FormationElementView>
removeFormationElement(elementId: string): Promise<void>
reorderFormationElements(args: ReorderFormationElementsArgs): Promise<void>
assignRollingStockToElement(args: AssignRollingStockToElementArgs): Promise<FormationElementView>
setTractionOverride(args: SetTractionOverrideArgs): Promise<FormationElementView>
getPrototypes(searchQuery?: string): Promise<PrototypeGroupView[]>
createCustomPrototype(args: CreateCustomPrototypeArgs): Promise<PrototypeView>
getFormationCategories(): Promise<FormationCategoryView[]>
createFormationCategory(args: CreateFormationCategoryArgs): Promise<FormationCategoryView>
```
