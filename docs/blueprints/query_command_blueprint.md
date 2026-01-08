# Blueprint: `get_collection` Tauri Query

This blueprint documents the exact architectural pattern used by the `get_collection` Tauri command in this repository so new queries (for example, `get_wishlists`) can be implemented consistently.

Files referenced below use this codebase's layout. Example links (path + line ranges) point to canonical locations:

- Command adapter: [src-tauri/src/collecting/interface/command_handlers.rs](src-tauri/src/collecting/interface/command_handlers.rs#L1-L84)
- Application query: [src-tauri/src/collecting/application/collection_query.rs](src-tauri/src/collecting/application/collection_query.rs#L1-L51)
- Domain repository trait: [src-tauri/src/collecting/domain/repository.rs](src-tauri/src/collecting/domain/repository.rs#L1-L40)
- Infra repo + mappers: [src-tauri/src/collecting/infrastructure/repositories.rs](src-tauri/src/collecting/infrastructure/repositories.rs#L1-L520)
- DB helpers: [src-tauri/src/collecting/infrastructure/database.rs](src-tauri/src/collecting/infrastructure/database.rs#L1-L160)
- App state & UoW: [src-tauri/src/state.rs](src-tauri/src/state.rs#L1-L80), [src-tauri/src/core/infrastructure/unit_of_work.rs](src-tauri/src/core/infrastructure/unit_of_work.rs#L1-L40)
- Error conversion: [src-tauri/src/core/domain/domain_error.rs](src-tauri/src/core/domain/domain_error.rs#L1-L60), [src-tauri/src/core/infrastructure/error.rs](src-tauri/src/core/infrastructure/error.rs#L1-L140)

---

## Overview

Pattern summary (call flow):

Tauri Command (Adapter) -> Application Query/Use-case -> Domain Repository trait -> Infrastructure Repository (SQLx, mappers) -> DB

Key patterns:

- DI: `AppState` (contains `SqlitePool`) is passed into commands with `tauri::State<'_, AppState>`. Commands create a `SqliteUnitOfWork` from the pool, then obtain a repository from the UnitOfWork via feature-specific extension trait (e.g., `CollectingUowExt::collection_repository`).
- Mapping: infra `Row` structs are produced by `sqlx::query_as` and converted to domain `View`/value objects by dedicated mapper functions returning `Result<DomainType, DomainError>`.
- Errors: Domain and sqlx errors are modelled as `DomainError` (thiserror) and mapped to a serializable `CommandError` (thiserror + serde) for Tauri responses via `From` implementations.
- Async: Everything I/O-facing is `async` and `await`-driven; commands, UoW creation, repo methods, and DB helpers are all `async`.

---

## Command Signature (Adapter)

Pattern used:

- Attribute: `#[tauri::command]` and `#[specta::specta]` (when present for type metadata).
- Signature example:

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_collection(state: tauri::State<'_, AppState>) -> Result<CollectionView, CommandError> { ... }
```

Notes:

- Commands accept `tauri::State<'_, AppState>` (not raw pool) to keep DI centralized.
- Return type is `Result<DTO, CommandError>` where `CommandError` is serializable with `serde` and used by the frontend.

### Handling query parameters

How to accept and validate query parameters in a command adapter:

- Signature styles:
  - Simple typed parameters (preferred when parameters are primitives or small DTOs):

  ```rust
  #[tauri::command]
  pub async fn get_items(state: tauri::State<'_, AppState>, page: i64, per_page: i64) -> Result<ItemsView, CommandError> { ... }
  ```

  - Optional parameters using `Option<T>`:

  ```rust
  #[tauri::command]
  pub async fn get_items(state: tauri::State<'_, AppState>, filter: Option<String>) -> Result<ItemsView, CommandError> { ... }
  ```

  - Complex query payloads: pass a small DTO struct (works well with `specta` metadata):

  ```rust
  #[derive(Debug, serde::Deserialize, specta::Type)]
  pub struct ItemsQuery { pub page: Option<i64>, pub per_page: Option<i64>, pub q: Option<String> }

  #[tauri::command]
  pub async fn get_items(state: tauri::State<'_, AppState>, query: ItemsQuery) -> Result<ItemsView, CommandError> { ... }
  ```

- Validation & mapping:
  - Validate/normalize parameter values in the adapter before creating the UoW or calling the application query. Convert invalid input into `CommandError::ValidationError` or a specific `CommandError` variant.
  - Map primitive parameters into domain value objects where appropriate (e.g., parse id strings into `CollectionId` using `FromStr` or domain constructors); prefer `TryFrom`/`TryInto` or `FromStr` and convert errors into `DomainError`/`CommandError`.

- Passing parameters to the application layer:
  - Keep application `execute` signatures explicit about parameters: either accept the `uow` plus parameters (e.g., `execute(uow, params)`) or create a small query DTO in the application layer. Example:

  ```rust
  pub async fn execute(uow: &mut SqliteUnitOfWork<'_>, page: i64, per_page: i64) -> Result<ItemsView, DomainError> { /* ... */ }
  ```

  - The adapter should convert inputs and call the application function, then convert `DomainError` -> `CommandError` as usual.

- Error handling example (validate early):

```rust
let page = if page <= 0 { return Err(CommandError::ValidationError(HashMap::new())) } else { page };

let per_page = per_page.clamp(1, 100);

// Preferred: use the AppState helper which centralizes error mapping
let mut uow = state.unit_of_work().await?;
let result = GetItemsQuery::execute(&mut uow, page, per_page).await;
uow.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?;
result.map_err(Into::into)
```

Notes:

- Prefer validating at the adapter boundary so application handlers and repos receive already-normalized, typed inputs.
- Use `specta::Type` on DTOs to expose typed metadata to the frontend when needed.

---

## State Access & Dependency Injection

How DI works in this codebase:

- `AppState` holds a `sqlx::SqlitePool` and helper constructors (`db_pool()`, `unit_of_work()`). See `src-tauri/src/state.rs`.
- Preferred: command adapters should call `state.unit_of_work().await?` to obtain a transaction-bound `SqliteUnitOfWork`.
  This centralizes the CommandError conversion and keeps adapters concise. Use `SqliteUnitOfWork::new(&state.db_pool()).await` only when the adapter needs explicit, custom error mapping.
- The UnitOfWork exposes an extension trait (`CollectingUowExt`) which provides `fn collection_repository(&mut self) -> Box<dyn CollectionRepository + '_>`; this boxes a concrete `SqliteCollectionRepository` bound to the transaction/connection.

This pattern ensures:

- Concrete infra (SQLx connection/transaction) is created inside the adapter/UoW boundary.
- Application and domain layers depend on the `CollectionRepository` trait only (clean boundary).

---

## Application Query / Use-case

Typical structure (see `GetCollectionQuery`):

- A unit struct `GetCollectionQuery` with an associated `pub async fn execute(unit_of_work: &mut SqliteUnitOfWork<'_>) -> Result<CollectionView, DomainError>`.
- The `execute` method asks the unit_of_work for a repository, calls the domain method (e.g., `find_view()`), and returns domain DTOs or `DomainError`.

Example signature (docs):

```rust
pub struct GetCollectionQuery;

impl GetCollectionQuery {
    pub async fn execute(unit_of_work: &mut SqliteUnitOfWork<'_>) -> Result<CollectionView, DomainError> {
        let mut repo = unit_of_work.collection_repository();
        let view = repo.find_view().await?;
        Ok(view)
    }
}
```

Notes:

- The application layer only manipulates domain types and errors (`DomainError`).

---

## Domain Repository Contract

Contract example:

```rust
#[async_trait::async_trait]
pub trait CollectionRepository: Send + Sync {
    async fn find_view(&mut self) -> Result<CollectionView, DomainError>;
    async fn save(&mut self, collection: &mut Collection) -> Result<(), DomainError>;
}
```

Guidelines:

- Keep repository methods small and focused (e.g., `find_view`, `find_by_id`, `save`).
- Return `DomainError` to allow the application layer to convert to `CommandError` at the boundary.

---

## Infrastructure: Repo Implementation, DB helpers, Mappers

Repository implementation responsibilities:

- Execute SQLx queries using `sqlx::query_as::<_, RowType>(sql).bind(...).fetch_all(fetch_one).await`.
- Map infra rows to domain view/value types using dedicated `Mapper` methods (pure functions returning `Result<..., DomainError>`).
- Aggregate multiple DB calls (e.g., collection, items, purchase info) and stitch results into domain `View`.

Important details:

- DB helper functions (e.g., `get_collection`, `get_collection_items`) return `Result<Option<Row>, DomainError>` or `Result<Vec<Row>, DomainError>`, and map `sqlx::Error` into `DomainError::Infrastructure` via `.map_err(DomainError::Infrastructure)`.
- `SqliteCollectionRepository::find_view` takes `self.executor` (a `&mut sqlx::SqliteConnection` or a transaction) and calls those DB helpers; it uses `CollectionMapper::row_to_collection` to build the final `CollectionView`.

---

## Data Mapping Pattern

Mapping flow:

1. SQLx returns `Row` structs (defined in `infrastructure::entities`).
2. Repository collects rows (collection row, items rows, owned rolling stock, purchase info) and arranges them into maps keyed by `CollectionItemId`.
3. Repository calls `CollectionMapper::row_to_collection_item(...)` for each item and `CollectionMapper::row_to_collection(...)` for the whole aggregate.
4. Mapper functions use domain value constructors (e.g., `MonetaryAmount::from_db`) and return `Result<DomainView, DomainError>`.

Design notes:

- Keep mappers pure (no DB/IO), return domain errors for invalid data.
- Perform validation in mappers so the domain layer receives valid value objects.

---

## UnitOfWork & Transaction Semantics

Pattern:

- `SqliteUnitOfWork::new(pool).await` begins a transaction and yields a `SqliteUnitOfWork { tx: Transaction }`.
- Repositories created from the UoW use the transaction (`&mut self.tx`) as their executor so multiple DB operations are atomic.
- In read-only queries, commands still often `commit()` to close transaction cleanly. For pure reads, commit is optional but present in this codebase.

Examples:

```rust
// Preferred pattern: obtain UoW via `AppState::unit_of_work()` which returns a `Result<SqliteUnitOfWork, CommandError>`
let mut unit_of_work = state.unit_of_work().await?;
let result = GetCollectionQuery::execute(&mut unit_of_work).await;
unit_of_work.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?;
```

---

## Error Handling & Conversion

Pattern summary:

- Domain and infra errors are modeled in `DomainError` (thiserror). `DomainError::Infrastructure(#[from] sqlx::Error)` lets `?` convert sqlx errors automatically.
- The application/command layer returns `Result<T, CommandError>` where `CommandError` is `Serialize` and `thiserror::Error`.
- Implement `From<DomainError> for CommandError` and `From<sqlx::Error> for CommandError` so errors crossing from application/domain can be transformed into serializable `CommandError` values to return to frontend.

Typical conversion implementation (already present):

```rust
impl From<DomainError> for CommandError {
    fn from(e: DomainError) -> Self { /* map variants to CommandError */ }
}

impl From<sqlx::Error> for CommandError {
    fn from(e: sqlx::Error) -> Self { CommandError::DatabaseError(e.to_string()) }
}
```

Guidelines:

- Keep `CommandError` serializable and stable — the frontend relies on message and variant names.
- Add context at adapter level using `map_err(|e| CommandError::DatabaseError(e.to_string()))` when creating UoW or committing transactions.

---

## Pseudo-code Templates

Use the templates below when implementing new Tauri queries (example: `get_wishlists`). Replace `Collection`/`collection` with the new feature names.

- Command adapter template:

```rust
// src-tauri/src/<feature>/interface/command_handlers.rs
#[tauri::command]
#[specta::specta]
pub async fn get_<feature>(state: tauri::State<'_, AppState>) -> Result<<FeatureView>, CommandError> {
    // obtain unit-of-work via AppState helper (preferred)
    let mut uow = state.unit_of_work().await?;

    // execute application query
    let result = Get<FeatureCamelCase>Query::execute(&mut uow).await;

    // commit / close transaction cleanly
    uow.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // map/return
    result.map_err(Into::into)
}
```

- Application query template:

```rust
// src-tauri/src/<feature>/application/<feature>_query.rs
pub struct Get<FeatureCamelCase>Query;

impl Get<FeatureCamelCase>Query {
    pub async fn execute(uow: &mut SqliteUnitOfWork<'_>) -> Result<<FeatureView>, DomainError> {
        let mut repo = uow.<feature>_repository(); // via UoW extension trait
        let view = repo.find_view().await?;
        Ok(view)
    }
}
```

- Domain repo trait template:

```rust
#[async_trait::async_trait]
pub trait <FeatureCamelCase>Repository: Send + Sync {
    async fn find_view(&mut self) -> Result<<FeatureView>, DomainError>;
    // other methods like save/find_by_id/update
}
```

- Infra repo template (outline):

```rust
pub struct Sqlite<FeatureCamelCase>Repository<'conn> {
    executor: &'conn mut sqlx::SqliteConnection,
}

#[async_trait::async_trait]
impl<'conn> <FeatureCamelCase>Repository for Sqlite<FeatureCamelCase>Repository<'conn> {
    async fn find_view(&mut self) -> Result<<FeatureView>, DomainError> {
        let row = database::get_<feature>(&mut *self.executor, &id).await?;
        // build maps for child rows
        let child_rows = database::get_<feature>_children(&mut *self.executor, &row.id).await?;
        // map rows -> domain view
        let items = child_rows.into_iter().map(|r| Mapper::row_to_item(r)).collect::<Result<_, _>>()?;
        Mapper::row_to_<feature>(row, items)
    }
}
```

- TryFrom-based conversion template:

```rust
use std::convert::TryFrom;

// Convert a DB Row directly into a domain view using TryFrom.
// Implementations should perform parsing/validation and return DomainError on failure.
impl TryFrom<<Feature>Row> for <FeatureView> {
    type Error = DomainError;

    fn try_from(row: <Feature>Row) -> Result<Self, Self::Error> {
        // parse money, ids, dates -> Domain types; return DomainError on failure
        // Example pattern:
        // let id = CollectionId::from_str(&row.id).map_err(|e| DomainError::Validation(e.to_string()))?;
        // let total = MonetaryAmount::from_db(row.total_value_amount, Some(&row.total_value_currency))
        //     .map_err(|e| DomainError::Validation(e.to_string()))?;
        // build and return the domain view
        unimplemented!()
    }
}

impl TryFrom<ItemRow> for ItemView {
    type Error = DomainError;

    fn try_from(row: ItemRow) -> Result<Self, Self::Error> {
        // parse fields and construct ItemView, returning DomainError on validation/parse errors
        unimplemented!()
    }
}
```

---

## Wiring Checklist (copy/paste)

- Add command function to feature `interface/command_handlers.rs` following the adapter template.
- Add application query in `application/` with `execute(uow)` signature.
- Update UnitOfWork extension trait (feature uow ext) to return boxed repo implementation:
  - Implement `CollectingUowExt for SqliteUnitOfWork` equivalent for the new feature.
- Implement infra repository in `infrastructure/repositories.rs` using `sqlx` and mappers.
- Add DB helper queries in `infrastructure/database.rs` returning `Result<..., DomainError>`.
- Map `DomainError` -> `CommandError` via `From` conversions if new domain variants are required.
- Register new command in `src-tauri/src/lib.rs` `collect_commands![]` macro.

---

## Testing Checklist

- Unit tests for mapper functions (invalid money/currency, missing enums) returning `DomainError::Validation`.
- Unit tests for application query using a mock/stub repo implementing the repository trait.
- Integration test that uses `fixtures/*.sql` to populate an ephemeral SQLite file and call `Get<FeatureQuery>::execute` via an in-memory UoW.
- End-to-end test calling the Tauri command (if integration harness exists) and asserting serialized `CommandError` forms on failures.

---

## Example: `get_wishlists` quick mapping notes

Follow the same steps as `get_collection`:

1. Add `src-tauri/src/wishlist/interface/command_handlers.rs::get_wishlists(state)` adapter.
2. Add `src-tauri/src/wishlist/application/wishlist_query.rs::GetWishlistsQuery::execute(uow)`.
3. Define `WishlistRepository` trait in `domain/repository.rs` with `find_view()`.
4. Implement `SqliteWishlistRepository` in `infrastructure/repositories.rs` and `database.rs` helpers for `get_wishlists` and children rows.
5. Implement `WishlistMapper::row_to_wishlist`.

---

## Appendix: Quick reference patterns

- Create UoW:
  - `SqliteUnitOfWork::new(&state.db_pool()).await.map_err(|e| CommandError::DatabaseError(e.to_string()))?`
- Get repository from UoW:
  - `let mut repo = unit_of_work.<feature>_repository();` (via UoW ext trait)
- Propagate errors with `?` inside repos and mappers using `DomainError`.
- Convert `DomainError` to `CommandError` at command boundary with `result.map_err(Into::into)` or `?` when return type is `Result<T, CommandError>` and `From<DomainError> for CommandError` is implemented.

---
