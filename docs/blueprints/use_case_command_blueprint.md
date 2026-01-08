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

## Preferred Use-Case Pattern

We standardize on a stateless, canonical shape for application-level use-cases to keep call sites simple and consistent with Query patterns.

Contract (canonical):

- Use a unit struct as the type marker: `pub struct MyUseCase;`.
- Expose a single associated (static) async entry point: `pub async fn execute(...) -> Result<T, DomainError>`.
- Do not require callers to construct or hold an instance of the use-case. Example callers should use `MyUseCase::execute(&mut uow, input).await`.

Rationale:

- Stateless use-cases are clearer as pure operations; the unit struct acts only as a namespace.
- Matches existing Query pattern (unit struct + associated `execute`) and reduces API surface area.
- Simplifies testing and avoids unnecessary allocation or lifetime issues.

Example use-case implementation:

```rust
pub struct CreateRailwayModelUseCase;

impl CreateRailwayModelUseCase {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        input: CreateRailwayModelInput,
    ) -> Result<RailwayModelId, DomainError> {
        // business logic
    }
}
```

Example adapter usage (command handler):

```rust
let mut uow = state.unit_of_work().await?;
let result = CreateRailwayModelUseCase::execute(&mut uow, input).await;
match result {
    Ok(id) => { uow.commit().await?; Ok(id) }
    Err(e) => Err(e.into()),
}
```

Exceptions:

- If a use-case requires injected, long-lived dependencies (e.g., a client with connection pooling that must be stored on the use-case), document it explicitly and keep it as an instance-based use-case. Prefer refactoring to pass dependencies as function arguments where feasible.

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
let mut uow = state.unit_of_work().await?;
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

- Always create a `SqliteUnitOfWork` in the adapter for write commands. Prefer obtaining it via `state.unit_of_work().await?`, which centralizes error mapping and keeps adapters concise. It creates a transaction-bound `SqliteUnitOfWork { tx: Transaction }`.
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

    // obtain uow via AppState helper (preferred)
    let mut uow = state.unit_of_work().await?;

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
