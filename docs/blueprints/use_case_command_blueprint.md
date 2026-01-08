# Blueprint: `add_collection_item` Tauri Command

*Generated: 2026-01-08*

This blueprint documents the exact architectural pattern used by a write/command operation in this repository using the `add_collection_item` (add a collection item) flow as the canonical example. It mirrors the style and level of detail used in `get_collection_blueprint.md` but focusses on Commands (state-changing operations) where domain invariants are enforced and persistence must be staged and committed.

Files referenced below use this codebase's layout. Example links (path + suggested line ranges) point to canonical locations:

- Command adapter: `src-tauri/src/collecting/interface/command_handlers.rs` (add command should be placed here)
- Recommended adapter DTO: `src-tauri/src/collecting/interface/types.rs` (recommended new file)
- Application use-case (recommended): `src-tauri/src/collecting/application/add_collection_item.rs` (new use-case file)
- Domain command type: `src-tauri/src/collecting/domain/command.rs` (contains `AddCollectionItem`)
- Domain aggregate mutator: `src-tauri/src/collecting/domain/collection.rs` (contains `Collection::add_item`)
- Domain repository trait: `src-tauri/src/collecting/domain/repository.rs` (pattern; collection repo trait is in domain module)
- Infra repo + event persistence mapping: `src-tauri/src/collecting/infrastructure/repositories.rs` (contains `SqliteCollectionRepository::save` and helper insert/update methods)
- DB helpers: `src-tauri/src/collecting/infrastructure/database.rs` (read/write SQL helpers)
- Mappers: `src-tauri/src/collecting/infrastructure/mappers.rs` (row -> domain view conversions)
- App state & UoW: `src-tauri/src/state.rs`, `src-tauri/src/core/infrastructure/unit_of_work.rs` (pattern used across the project)
- Error conversion: `src-tauri/src/core/domain/domain_error.rs`, `src-tauri/src/core/infrastructure/error.rs`

---

## Overview

Pattern summary (call flow):

Tauri Command (Adapter) -> Input DTO mapping/validation -> Application Use Case -> Domain Aggregate (mutate via command) -> Repository `save()` (Infrastructure) -> DB helpers / SQL

Key patterns and principles:

- DI via `AppState`: Commands receive `tauri::State<'_, AppState>` and create a `SqliteUnitOfWork` from the internal pool. The UoW is the boundary for a transaction.
- Unit-of-Work & Transaction semantics: The adapter creates the UoW (which begins a transaction). The adapter calls the application use-case with `&mut SqliteUnitOfWork<'_>`. On success the adapter calls `uow.commit().await` to persist changes; if the command returns an error the UoW is dropped and the transaction is rolled back.
- Domain commands vs DTOs: The frontend / adapter should pass a small DTO (e.g., `AddCollectionItemInput`) which is `serde::Deserialize` + `specta::Type`. Convert this DTO to the domain `AddCollectionItem` (see `src-tauri/src/collecting/domain/command.rs`) using `TryFrom`/`TryInto` to perform validation and map structural issues into `DomainError::Validation` / `DomainError::ValidationError`.
- Aggregate & Event-sourcing-ish staging: `Collection::add_item` builds a `CollectionEvent::RailwayModelAdded` and calls `apply` to mutate in-memory state. The event is appended to `collection.pending_events`. Repository `save` iterates `pending_events` and executes concrete DB statements for each event.
- Error modelling: Business and validation errors are `DomainError` (thiserror). The interface/adapter converts these into `CommandError` (serializable) via `From<DomainError> for CommandError` or `.map_err(Into::into)` so errors cross the Tauri IPC boundary safely.

---

## Command Adapter (Signature & Responsibilities)

Pattern used in this repository:

- Attributes: `#[tauri::command]` and `#[specta::specta]` are applied to adapter functions.
- Adapter function accepts `state: tauri::State<'_, AppState>` and the input DTO (simple primitives or a small struct).
- Return type: `Result<DTO, CommandError>` where `CommandError` is the serializable error enum in `core::infrastructure::error`.

Recommended signature pattern for `add_collection_item`:

```rust
#[tauri::command]
#[specta::specta]
pub async fn add_collection_item(
    state: tauri::State<'_, AppState>,
    input: AddCollectionItemInput,
) -> Result<CollectionView, CommandError> {
    // adapter body
}
```

Responsibilities of the adapter:

- Validate/normalize incoming primitives where cheap (e.g., clamp page numbers). For complex validation convert via `TryFrom` and return `CommandError::ValidationError` on failure.
- Create the `SqliteUnitOfWork` from `state.db_pool()` (use `state.unit_of_work().await` convenience if present).
- Call the Application Use Case (see below) passing `&mut uow` and the domain command object.
- If the use-case returns `Ok`, call `uow.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?` and return the domain DTO to the frontend. If the use-case returns `Err`, convert it to `CommandError` (via `.map_err(Into::into)` or `?`) and return early.

Notes on DTO design and `specta`:

- Use `specta::Type` on the input DTO to generate matching TypeScript types for the frontend.
- Keep the adapter DTO minimal and primitives-friendly. Example: `AddCollectionItemInput` should expose id strings and basic primitives; conversions to domain-specific newtypes (like `RailwayModelId`) happen in `TryFrom`.
- Avoid business rule checks in the adapter — map/validate structural issues here, but keep business invariants inside domain/use-case.

---

## Application Use Case (Orchestration)

Recommended location: `src-tauri/src/collecting/application/add_collection_item.rs`

Pattern (unit struct + `execute`):

```rust
pub struct AddCollectionItemCommand;

impl AddCollectionItemCommand {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        add_collection_item: AddCollectionItem,
    ) -> Result<CollectionView, DomainError> {
        // orchestrate domain & persistence
    }
}
```

Responsibilities:

- Obtain the repository via the unit-of-work extension trait: `let mut repo = unit_of_work.collection_repository();`.
- Two common implementation patterns are possible:
  - Pattern A (preferred in this repo): Load the aggregate (or create default), call `collection.add_item(add_collection_item)`, then call `repo.save(&mut collection)` to persist the aggregate's pending events. This keeps domain logic inside the aggregate and lets the repository handle persistence of events.
  - Pattern B: Call a repository method `repo.add_item(collection_id, add_collection_item).await?` which implements the insertion directly on the infra side. This is simpler but moves domain mutation logic into infra and makes unit testing the domain harder.
- In this codebase we use Pattern A: domain aggregates (see `Collection::add_item`) produce events and `SqliteCollectionRepository::save` iterates events to perform SQL operations.
- Return the final `CollectionView` (or created `CollectionItem` view) or a `DomainError` on failure.

Example use-case skeleton (following pattern A):

```rust
pub async fn execute(
    unit_of_work: &mut SqliteUnitOfWork<'_>,
    input: AddCollectionItem,
) -> Result<CollectionView, DomainError> {
    let mut repo = unit_of_work.collection_repository();

    // Load aggregate (example uses default collection for single-user assumption)
    let mut collection_view = repo.find_view().await?; // returns CollectionView
    let mut collection = Collection::from_view(collection_view)?; // or load domain aggregate via repo

    collection.add_item(input);
    repo.save(&mut collection).await?;

    // Optionally re-read a view or construct a view to return
    let view = repo.find_view().await?;
    Ok(view)
}
```

Notes:

- Prefer returning domain-level `CollectionView` (DTO) that the frontend expects.
- Keep the use-case small and focused: orchestrate, handle exceptions, don't perform UI concerns.

---

## Domain: Command Object & Aggregate Mutation

Files to inspect:

- Domain command type: `src-tauri/src/collecting/domain/command.rs` — contains `AddCollectionItem` struct.
- Aggregate mutator: `src-tauri/src/collecting/domain/collection.rs` — contains `Collection::add_item` and `apply`.

`AddCollectionItem` is a domain-safe struct with strongly-typed fields (value objects/newtypes):

- `railway_model_id: RailwayModelId`
- `rolling_stock_ids: Vec<RollingStockId>`
- `category: Category`
- `price: MonetaryAmount`
- `seller_id: Option<SellerId>`
- `added_date: NaiveDate`
- `purchase_date: NaiveDate`
- optional conditions and notes

`Collection::add_item` responsibilities:

- Create new identifiers (e.g., `CollectionItemId::default()` and `PurchaseInfoId::default()`), build `OwnedRollingStockIds` for each provided `rolling_stock_id` and then create a `CollectionEvent::RailwayModelAdded` event containing all details.
- Call `self.apply(&event)` which mutates the in-memory collection (updates summary counts, total value, push new `CollectionItem`).
- Append the event to `self.pending_events` so the repository can persist it.

Rationale for events & `pending_events`:

- Events both document the change and provide a compact, testable staging area for persistence. The repository does not need to re-run domain logic, it can map each event into SQL statements.
- This approach isolates domain rules inside aggregates and keeps infra code responsible only for translating events to DB changes.

---

## Infrastructure Repository: Persisting Events

File: `src-tauri/src/collecting/infrastructure/repositories.rs`

Key responsibilities and methods present in the repository implementation:

- `SqliteCollectionRepository::save(&mut self, collection: &mut Collection) -> Result<(), DomainError>`
  - Iterates `collection.pending_events` and matches on event variants.
  - For `CollectionCreated` events: insert a `collections` row (`insert_collection`).
  - For `RailwayModelAdded` events: update the collection summary (`update_collection_summary`), insert `collection_items` row (`insert_collection_item`), insert `owned_rolling_stocks` rows (`insert_owned_rolling_stocks`), and insert `purchase_infos` (`insert_purchase_info`).
  - After all events are applied in DB, it calls `update_collection_metadata` (bump version, updated_at) and clears `collection.pending_events = Vec::new()`.

- Helper methods are small and focused, each executing a single SQL statement and mapping SQL errors into `DomainError` using the `.with_domain_context(...)` helper which wraps `sqlx::Error` into `DomainError::Infrastructure`.

Why the repo handles events:

- The event -> SQL mapping is a single responsibility of the infra layer. The domain decides what happened (via the event), the infra maps it into concrete persistence actions.
- This keeps domain logic testable and infrastructure code straightforward.

---

## DB Helpers & Mappers

Files:

- `src-tauri/src/collecting/infrastructure/database.rs` — contains lower-level SQLx query helpers used by the repository (e.g., `get_collection`, `get_collection_items`, `get_purchase_infos`, `get_owned_rolling_stocks`).
- `src-tauri/src/collecting/infrastructure/mappers.rs` — contains `CollectionMapper::row_to_collection_item` and `CollectionMapper::row_to_collection` which perform validation and turn DB rows into domain view objects.

Patterns:

- DB helpers return `Result<Option<Row>, DomainError>` or `Result<Vec<Row>, DomainError>`, mapping `sqlx::Error` to `DomainError::Infrastructure`.
- Mappers perform parsing of DB fields into domain value objects (parse ids, MonetaryAmount, dates). Mappers should return `DomainError::Validation` or `DomainError::ValidationError` if DB data is malformed.
- Keep mappers pure and side-effect free — they should not call DB or logger.

---

## Error Handling & Conversion

Files:

- `src-tauri/src/core/domain/domain_error.rs` — `DomainError` (thiserror) variants used across domain and infra.
- `src-tauri/src/core/infrastructure/error.rs` — `CommandError` (serializable) returned by adapters.

Key patterns:

- Use `DomainError` inside the application and domain layers. Return `Result<T, DomainError>` from use-cases and repo methods.
- Convert `DomainError` into `CommandError` at the adapter boundary using `impl From<DomainError> for CommandError` and then `.map_err(Into::into)` or `?` when the adapter returns `Result<_, CommandError>`.

Example adapter error handling (pattern used across repo):

```text
let mut uow = SqliteUnitOfWork::new(&state.db_pool())
    .await
    .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
let result = AddCollectionItemCommand::execute(&mut uow, domain_command).await;
// commit only on success
match result {
    Ok(view) => {
        uow.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?;
        Ok(view)
    }
    Err(e) => Err(e.into()),
}
```

Notes about `thiserror` tagging for TypeScript mapping:

- The repository uses `thiserror` + `serde` (for command-level errors) to ensure the frontend can inspect error variant names if needed. Keep `CommandError` serializable and stable.

---

## Validation & `TryFrom` pattern

Recommended approach for mapping the adapter DTO to domain `AddCollectionItem`:

- Define an input struct for the adapter that mirrors the frontend shape and is `serde::Deserialize + specta::Type` (e.g., `AddCollectionItemInput`).
- Implement `TryFrom<AddCollectionItemInput> for AddCollectionItem` where this conversion performs structural validation (parse id strings into `RailwayModelId`, parse rolling stock ids, convert monetary fields into `MonetaryAmount`, check required date invariants, etc.).
- On validation failures return `DomainError::Validation` or `DomainError::ValidationError` with field-level errors.

Snippet showing the pattern:

```text
impl TryFrom<AddCollectionItemInput> for AddCollectionItem {
    type Error = DomainError;

    fn try_from(input: AddCollectionItemInput) -> Result<Self, Self::Error> {
        // parse railway model id
        let railway_model_id = RailwayModelId::from_str(&input.railway_model_id)
            .map_err(|_| DomainError::Validation("railway_model_id invalid".into()))?;

        // parse rolling stock ids -> Vec<RollingStockId>
        let rolling_stock_ids = input
            .rolling_stock_ids
            .into_iter()
            .map(|s| RollingStockId::from_str(&s).map_err(|_| DomainError::Validation("rolling_stock_id invalid".into())))
            .collect::<Result<Vec<_>, _>>()?;

        // convert monetary amount
        let price = MonetaryAmount::new(input.price_amount, Currency::from_str(&input.price_currency)?)?;

        Ok(AddCollectionItem { /* ... */ })
    }
}
```

Adapter should map `TryFrom` errors to `CommandError` via the use-case and final `.map_err(Into::into)`.

---

## Transaction & Commit Notes

- Always create a `SqliteUnitOfWork` in the adapter for write commands. It creates a transaction-bound `SqliteUnitOfWork { tx: Transaction }`.
- The repository implementations re-borrow the transaction via an extension trait (e.g., `CollectingUowExt::collection_repository`) and operate on `&mut self.tx` so all repo calls are within the same transaction.
- On success: adapter calls `uow.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?`.
- On error: dropping the `SqliteUnitOfWork` will roll back the transaction automatically (or you can explicitly `rollback()` depending on UoW implementation).
- For read-only commands: committing is optional but recommended to close the transaction cleanly.

---

## Pseudo-code Templates

Adapter template (command handler):

```rust
// src-tauri/src/collecting/interface/command_handlers.rs
#[tauri::command]
#[specta::specta]
pub async fn add_collection_item(
    state: tauri::State<'_, AppState>,
    input: AddCollectionItemInput,
) -> Result<CollectionView, CommandError> {
    // map input -> domain
    let domain_cmd = AddCollectionItem::try_from(input).map_err(Into::into)?;

    // create uow
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // execute
    let result = AddCollectionItemCommand::execute(&mut uow, domain_cmd).await;

    match result {
        Ok(view) => {
            uow.commit().await.map_err(|e| CommandError::DatabaseError(e.to_string()))?;
            Ok(view)
        }
        Err(e) => Err(e.into()),
    }
}
```

Application use-case template:

```rust
// src-tauri/src/collecting/application/add_collection_item.rs
pub struct AddCollectionItemCommand;

impl AddCollectionItemCommand {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        add_collection_item: AddCollectionItem,
    ) -> Result<CollectionView, DomainError> {
        let mut repo = unit_of_work.collection_repository();

        // load or create aggregate
        let mut collection = match repo.find().await? {
            Some(domain) => domain,
            None => Collection::default(),
        };

        collection.add_item(add_collection_item);

        // save will iterate pending_events and persist them
        repo.save(&mut collection).await?;

        // optionally return a fresh view
        let view = repo.find_view().await?;
        Ok(view)
    }
}
```

Repository save pattern example (already implemented):

- Iterate `collection.pending_events` and match each event variant.
- For each `RailwayModelAdded` event, call helper `update_collection_summary`, `insert_collection_item`, `insert_owned_rolling_stocks`, `insert_purchase_info`.
- Finally call `update_collection_metadata` and clear `pending_events`.

---

## Wiring Checklist (copy/paste)

- Add an adapter function to `src-tauri/src/collecting/interface/command_handlers.rs` following the adapter template above.
- Add a `AddCollectionItemInput` DTO in the `interface` module with `serde::Deserialize + specta::Type`.
- Implement `TryFrom<AddCollectionItemInput> for AddCollectionItem` in `src-tauri/src/collecting/domain/command.rs` or a companion mapper file.
- Add application use-case `src-tauri/src/collecting/application/add_collection_item.rs` implementing `AddCollectionItemCommand::execute(&mut uow, add_collection_item)`.
- Ensure `CollectingUowExt` (in `infrastructure/repositories.rs`) provides `collection_repository()` and repository implements `save` behavior for events.
- Add any required DB helper functions to `src-tauri/src/collecting/infrastructure/database.rs` and mappers in `mappers.rs`.
- Register new command in `src-tauri/src/lib.rs` (or wherever commands are collected) via `collect_commands![]` macro.

---

## Testing Checklist

- Unit tests for `TryFrom<AddCollectionItemInput>` to cover happy path and field validation failures.
- Unit tests for `Collection::add_item` to validate `pending_events` contents and `apply` behavior (summary update, total_value handling).
- Unit/integration tests for `SqliteCollectionRepository::save` using `sqlx::test` with fixtures (similar to existing tests) to verify inserted rows for `collection_items`, `purchase_infos`, and `owned_rolling_stocks`.
- End-to-end test invoking the Tauri command (if integration harness exists) asserting serializable `CommandError` shapes on failures.

---

## Quality Gates

Before marking work done follow these checks (project policy):

- Backend: `cargo fmt` and `cargo clippy -- -D warnings` pass.
- Frontend (if adding TypeScript/specta types): `pnpm check` and `pnpm lint` pass.
- Localization: No hardcoded user-facing strings in Svelte components; use Paraglide messages (e.g., `import * as m from '$paraglide/messages'`).
- Tests: Unit and integration tests added/updated and passing locally via `cargo test` and `pnpm test` where applicable.

---

## Example: End-to-end flow (compact)

1. Frontend constructs an input object matching `AddCollectionItemInput` and calls the Tauri command `add_collection_item`.
2. Adapter deserializes to `AddCollectionItemInput` (specta ensures TS type), calls `AddCollectionItem::try_from(input)` to validate/parse.
3. Adapter creates `SqliteUnitOfWork` and calls `AddCollectionItemCommand::execute(&mut uow, domain_cmd).await`.
4. Use-case loads domain aggregate `Collection`, calls `collection.add_item(domain_cmd)`, and calls `repo.save(&mut collection).await`.
5. `SqliteCollectionRepository::save` iterates events and runs SQL insert/update helper methods for each event.
6. Adapter calls `uow.commit().await` and returns serialized `CollectionView` to the frontend. On error, adapter converts `DomainError` into `CommandError` and returns the error over the Tauri IPC.

---

## References (canonical files inspected)

- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/interface/command_handlers.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/application/collection_query.rs (pattern)
- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/domain/command.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/domain/collection.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/infrastructure/repositories.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/infrastructure/database.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/collecting/infrastructure/mappers.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/state.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/core/domain/domain_error.rs
- /home/carlo/Projects/rusty-shed/src-tauri/src/core/infrastructure/error.rs


---

Requirements coverage

- Command Adapter signature & pattern: Done (documented + template)
- Application Use Case pattern: Done (template + recommendation)
- Domain command & aggregate mutator: Done (referenced and explained)
- Infra repository save & helpers: Done (detailed description)
- DB helpers & mappers responsibilities: Done
- Error mapping DomainError -> CommandError: Done
- Validation & TryFrom: Done (recommended and example)
- Transaction semantics and commit: Done
- Wiring & Tests checklist: Done


If you'd like, I can now commit this file into the repository (create it at `docs/blueprints/TAURI_USE_CASE_COMMAND_BLUEPRINT.md`) and then run a quick repository check (lint/tests). Say "create the file" to proceed or tell me any edits you'd like to the blueprint before creation.
