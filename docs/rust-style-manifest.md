# Rust Style Manifest (src-tauri)

Purpose: a compact, actionable guide describing how to Rust code is written in `src-tauri/` so other agents can generate code indistinguishable from the existing codebase.

Checklist

- Deliver a consistent set of "how-to" rules for: Naming, Error Handling, Async, Formatting & Layout, Idiomatic choices.
- Include 3 minimal code examples that match repository conventions.
- Record which files were scanned as evidence.

Assumption

- The codebase prefers feature-local `type Result<T> = std::result::Result<T, AppError>` aliases and typed `thiserror` enums for API boundaries. `anyhow` is only used in tooling/tests.

Style Rules (How)

Naming Conventions

- Traits: CamelCase, capability-oriented names: `Repository`, `Notifier`, `Importer`. Avoid `Trait` suffix unless it disambiguates.
- Structs/Enums: CamelCase nouns: `Collection`, `DbCollectionRow`, `CollectionDto`.
- Persistence types: use `Db`, `Row`, or `Entity` suffix/prefix for infra types: `DbCollectionRow`, `CollectionEntity`.
- Conversion methods:
  - DB → domain: prefer free functions or mappers named `row_to_<entity>` in an `infrastructure::mappers` module (e.g., `row_to_collection`). Return a crate-local `Result`.
  - Domain → DTO: implement consuming conversions named `into_dto(self) -> Dto`.
  - DTO/Inbound → Domain: use fallible conversions: `try_from`, `TryFrom`, or `to_domain(&self) -> Result<_, ValidationError>`.
  - Follow `From`/`Into`/`TryFrom` semantics when appropriate; prefer explicit names where clarity matters.
- Modules: group by feature into `domain/`, `application/`, `infrastructure/`, `interface/` with small `mod.rs` or explicit `mod` files.

Error Handling

- Prefer `thiserror` for typed domain and command errors. Derive `serde::Serialize` for errors returned to the frontend.
- Use a crate/feature-local `type Result<T> = std::result::Result<T, DomainError>` alias at domain/application boundaries.
- Propagate with the `?` operator wherever possible. Use `map_err` or explicit `match` when converting errors between layers.
- Map `DomainError` → `CommandError` at the command adapter boundary; make `CommandError` stable and serializable for the frontend.
- Use `From<sqlx::Error>` (or `#[from]`) to convert infra errors into `DomainError::Infrastructure` to enable `?` in repos.

Async Preferences

- Runtime: `tokio` is the implicit runtime for async operations.
- DB: `sqlx` with `query!`/`query_as!` where applicable. Use `.fetch_one(...).await?` and `.fetch_all(...).await?`.
- Traits with async methods: use `#[async_trait::async_trait]` when trait methods must be async. Keep trait definitions minimal.
- Use-cases: one `async fn execute(...) -> Result<...>` per query/use-case. Commands are thin adapters that call these use-cases.
- For blocking or CPU-bound work, use `tokio::task::spawn_blocking`.
- Parallel DB calls: prefer `tokio::try_join!` or `futures::join!` for concurrency when results are independent.

Formatting & Layout

- Follow `rustfmt` defaults. Use CI gates `cargo fmt` and `cargo clippy -- -D warnings`.
- Keep trailing commas in multi-line expressions to reduce diffs.
- Match arms: one arm per line; short expression arms may be kept inline, multi-statement arms use block bodies. Let rustfmt decide alignment.
- Keep `mod.rs` minimal; prefer explicit module files and `pub use` for exports.
- Visibility: default to private; use `pub(crate)` for internal crate visibility and `pub` for cross-crate API.

Idiomatic Choices

- Lints: follow Clippy; prefer fixing warnings over suppressing them. Add targeted `#[allow(clippy::...)]` with explanatory comments when necessary.
- Lifetimes: prefer elision; add explicit lifetimes only when required (e.g., repo impls holding a connection/reference).
- Derives: prefer `derive` for `Clone`, `Debug`, `Serialize`, `Deserialize`, etc.
- Trait objects: use `Box<dyn Trait + Send + Sync>` for UoW-returned repos; keep short-lived borrows as `&mut impl Trait` in impls.
- Keep `async_trait` usage scoped to repository/trait definitions that require it.
- Keep mappers pure and fallible; perform validation in mappers so domain gets valid value objects.

Micro-rules (copy/paste)

- Create UoW:
  - `let mut uow = SqliteUnitOfWork::new(&state.db_pool()).await.map_err(|e| CommandError::DatabaseError(e.to_string()))?;`
- Repo method signature:
  - `async fn find_view(&mut self) -> Result<CollectionView, DomainError>;` (repository traits are Send + Sync)
- Mapper naming:
  - `fn row_to_collection(row: DbCollectionRow) -> Result<Collection, DomainError>`
- Conversion naming:
  - `impl Collection { pub fn into_dto(self) -> CollectionDto { ... } }`

Three short examples

1. DB → Domain mapper (`row_to_*`) pattern

```rust
// ...existing code...
use crate::domain::Collection;
use crate::infrastructure::entities::DbCollectionRow;
use crate::core::domain::domain_error::DomainError;

pub fn row_to_collection(row: DbCollectionRow) -> Result<Collection, DomainError> {
    Ok(Collection {
        id: row.id,
        name: row.name,
        price: crate::core::monetary::MonetaryAmount::from_db(row.price_amount, row.price_currency)?,
        // ...other fields...
    })
}
// ...existing code...
```

2. Error enum with serde tag (thiserror + serde)

```rust
#[derive(serde::Serialize, thiserror::Error, Debug)]
#[serde(tag = "type", content = "details")]
pub enum AppError {
    #[error("Unauthorized")]
    AuthError,
    #[error("Database failure: {0}")]
    DatabaseError(String),
    #[error("Validation failed: {0}")]
    ValidationError(String),
}
```

3. Async application use-case using `?` and repo pattern

```rust
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::collecting::application::collection_view::CollectionView;
use crate::core::domain::domain_error::DomainError;

pub struct GetCollectionQuery;

impl GetCollectionQuery {
    pub async fn execute(uow: &mut SqliteUnitOfWork<'_>) -> Result<CollectionView, DomainError> {
        let mut repo = uow.collection_repository();
        let view = repo.find_view().await?; // sqlx and mapper errors are converted into DomainError in repo
        Ok(view)
    }
}
```

Where I looked (evidence)

- `src-tauri/Cargo.toml`
- `src-tauri/build.rs`
- `src-tauri/src/collecting/infrastructure/mappers.rs`
- `src-tauri/src/collecting/infrastructure/entities.rs`
- `src-tauri/src/collecting/infrastructure/repositories.rs`
- `src-tauri/src/collecting/application/collection_query.rs`
- `src-tauri/src/core/domain/monetary_amount.rs`
- `src-tauri/src/core/domain/validation.rs`
- `docs/blueprints/get_collection_blueprint.md`

Using this manifest

- Supply this file to codegen agents as the single-source-of-truth for code style in `src-tauri/`.
- Enforce `cargo fmt` + `cargo clippy -- -D warnings` in CI and run the repo's `pnpm check`/`lint` gates for frontend.

Next steps (optional)

- Add CI check to validate manifest or link it from CONTRIBUTING.md.
- If you want, I can add a condensed `clippy.toml` or `rustfmt.toml` with the project's choices.
