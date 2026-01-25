# ADR 8: Standardize Tauri Command and Use Case Data Structures

Status: Accepted

Date: 2026-01-23

Deciders: Project Lead

## 1. Context and Problem Statement

Tauri commands are the transport boundary between the Svelte frontend and the Rust backend. Without explicit, enforced conventions for request/response shapes and where validation occurs, business rules leak into transport code, TypeScript/Rust types drift, and security/validation gaps appear.

This ADR defines naming, derivation, and responsibility rules to make the transport ↔ domain boundary explicit, typed, and testable, supporting serde, specta, and validator.

## 2. Decision Drivers

- Strong type safety and cross-language type generation (Rust ↔ TypeScript via specta).
- Clear separation of concerns (transport validation vs business logic).
- Reusability and consistency across handlers and use cases.
- Minimize runtime errors and security surface by validating at the transport edge.
- Compatibility with Clean Architecture/DDD layering (domain/use-cases inside, transport/infrastructure outside).

## 3. Considered Options

1. Reuse domain types for transport (no suffix): fewer types, but couples domain to transport, exposes internals, and prevents clean validation at the edge.
2. Use explicit transport DTOs + domain Inputs with suffixes (`Args`/`Input`/`Query`/`Criteria`) — provides clear boundaries and aligns with specta/serde/validator tooling. (Chosen)
3. Use verbose prefixing (`Transport`/`CreateUserPayload`) — explicit but more verbose and less idiomatic than succinct suffixes.

## 4. Decision Outcome

Chosen Option: **Option 2: a lightweight, consistent suffix scheme and derivation rules with clear handler responsibilities.**

### Conventions (mandatory)

#### Write Paths (Tauri commands that mutate state)

- Payload struct names MUST use the `Args` suffix, e.g. `CreateCollectionArgs`.
- `Args` **MUST** derive: `Debug`, `Clone`, `validator::Validate`, `specta::Type`, `serde::Deserialize`.
- Location recommendation: `interface` modules (e.g., `src/{feature}/interface`).
- Command handler responsibility:
  - Run `args.validate()` and convert to Use Case Input.
  - Map validation errors to a serializable `CommandError` (use `thiserror` with serde tagging).
  - Construct Use Case Input (guaranteed-valid representation) and invoke the use case.

#### Use Cases (the Write Path internals)

- Input struct names MUST use the `Input` suffix, e.g. `CreateCollectionInput`.
- Inputs MUST derive: `Debug`, `Clone`.
- Use Cases receive only validated/normalized Input values and contain business logic only.

#### Read Paths (Queries)

- Query parameter structs MUST use the `Query` suffix, e.g. `GetCollectionQuery`.
- Complex filtering/search structs MUST use the `Criteria` suffix, e.g. `TransactionCriteria`.
- Query/Criteria MUST derive: `Debug`, `Clone`, `specta::Type`, `serde::Deserialize`.
- Read command handlers validate/normalize Query/Criteria as needed, then call read-only application

### Justification

### Consequences

### Technical Details

Minimal examples

Args (transport)

```rust
     use validator::Validate;
     use specta::Type;
     use serde::Deserialize;

     #[derive(Debug, Clone, Validate, Type, Deserialize)]
     pub struct CreateUserArgs {
         #[validate(length(min = 1))]
         pub username: String,
         #[validate(email)]
         pub email: String,
         #[validate(length(min = 8))]
         pub password: String,
     }
```

Use Case Input (domain)

```rust
     #[derive(Debug, Clone)]
     pub struct CreateUserInput {
         pub username: String,
         pub email: String,
         pub password_hash: String,
     }
```

Command handler (thin wrapper)

```rust
     #[tauri::command]
     pub async fn create_user_command(args: CreateUserArgs, state: State<'_, AppState>) -> Result<UserDto, AppError> {
         args.validate().map_err(AppError::validation)?;
         let input = CreateUserInput {
             username: args.username.trim().to_owned(),
             email: args.email.to_lowercase(),
             password_hash: hash_password(&args.password),
         };
         user_usecases::create_user(&state.repo, input).await
     }
```

Query / Criteria example

```rust
     #[derive(Debug, Clone, Type, Deserialize)]
     pub struct GetOrdersQuery {
         pub customer_id: Option<Uuid>,
         pub status: Option<OrderStatus>,
     }

     #[derive(Debug, Clone, Type, Deserialize)]
     pub struct TransactionCriteria {
         pub min_amount: Option<i64>,
         pub max_amount: Option<i64>,
         pub from: Option<chrono::NaiveDateTime>,
         pub to: Option<chrono::NaiveDateTime>,
     }
```

## 5. More Information
