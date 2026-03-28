# ADR 11: Automated Testing Strategy for the Rust Backend

Status: Proposed

Date: 2026-03-28

Deciders: Project Lead

## Context and Problem Statement

The model railway collection management app is built with Tauri 2, Svelte 5 Runes, and a Rust backend following Clean Architecture (Hexagonal / Ports and Adapters). As the codebase grows, we need a clear, layered testing strategy that:

1. Ensures high confidence in **Domain Logic** (e.g., scale conversions, inventory valuations).
2. Verifies **Persistence** (SQLx / SQLite) without bloating test runtimes.
3. Decouples tests from the **Tauri Command layer** to allow for easier logic migration or CLI support in the future.

Without a defined strategy, test coverage will be inconsistent, slow, and tightly coupled to the database or the Tauri runtime.

## Decision Drivers

- **Independence:** Domain and Application logic must be testable without the Tauri runtime or a live SQLite file.
- **Validation:** Domain invariants (e.g., "A locomotive cannot have a negative scale") must be guaranteed at the type or logic level.
- **Integration:** The SQLx implementation must be verified to correctly satisfy the Repository traits defined in the Application layer.
- **Speed:** The majority of tests (Domain + Application) should run without a database to preserve a fast feedback loop.
- **Stability:** Changing the database schema should only break Infrastructure tests, not Application (Use Case) tests.

## Considered Options

1. **Ad-hoc Tests Only:** Write tests wherever convenient with no layering strategy.
2. **End-to-End Tests Only:** Use WebDriver / Playwright to exercise the full Tauri app.
3. **Layered Testing Pyramid (Hexagonal / Clean Architecture):** Align test types with architectural layers.

## Decision Outcome

Chosen option: **Option 3: Layered Testing Pyramid**, because it mirrors the Clean Architecture dependency rule (dependencies point inward) and gives the best balance of speed, coverage, and maintainability.

### Consequences

- **Good:** Fast feedback loop — 90% of tests (Domain + Application) run without a database.
- **Good:** Database tests are isolated and do not leak state between runs (via `#[sqlx::test]`).
- **Good:** Changing a database schema only breaks Infrastructure tests, leaving Use Case tests stable.
- **Bad:** Requires maintaining Repository traits (Ports) for all external dependencies, which adds some boilerplate.
- **Bad:** Managing `sqlx::test` requires the `DATABASE_URL` to be available during `cargo test` (or using a temporary file).
- **Neutral:** Testing the Interface layer can be tricky because it often requires mocking `tauri::AppHandle` if commands emit events.

---

## Pros and Cons of the Options

### Option 1: Ad-hoc Tests Only

- **Pros:** No upfront design cost; developers can write tests however they prefer.
- **Cons:** Results in a slow, fragile test suite that is tightly coupled to the database or the Tauri runtime; no guarantee of coverage for business rules.

### Option 2: End-to-End Tests Only

- **Pros:** Tests the full user journey, catching integration problems.
- **Cons:** Very slow; tests are brittle and expensive to maintain; provides no isolation for debugging failures in business logic.

### Option 3: Layered Testing Pyramid (Hexagonal / Clean Architecture)

- **Pros:** Fast, focused tests for each layer; failures are easy to localise; Domain logic is fully exercised without infrastructure concerns.
- **Cons:** More initial setup (traits, mocks, test utilities); requires discipline to keep each layer's tests within their scope.

---

## Implementation Plan

### Layer 1: Domain (Entities & Value Objects)

- **Test type:** Unit Tests (`#[test]`).
- **Tooling:** Standard `cargo test`.
- **Strategy:** Because the Domain layer has zero external dependencies by definition, tests here are fast and exhaustive. Every validation rule and business calculation must be covered.
- **Focus:** Validation logic, scale-to-gauge calculations, and business rules.

```rust
// domain/entities/locomotive.rs
impl Locomotive {
    pub fn new(name: String, scale: Scale) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self { name, scale })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locomotive_creation_fails_on_empty_name() {
        let result = Locomotive::new("".to_string(), Scale::HO);
        assert!(matches!(result, Err(DomainError::EmptyName)));
    }
}
```

### Layer 2: Application (Use Cases & Queries)

- **Test type:** Integration Tests (Logic).
- **Tooling:** `mockall` for trait mocking; `#[tokio::test]` for async use cases.
- **Strategy:** Test Use Cases in isolation by mocking the Repository traits (Ports). This allows verifying complex scenarios (e.g., "Database is down" or "Item not found") without a real database.
- **Focus:** Ensuring the orchestration flow is correct. Example: when a user adds a locomotive, the Use Case must first check whether the ID exists, then call the Repository `save` method.

```rust
// The Port (Trait)
#[mockall::automock]
pub trait LocomotiveRepository {
    async fn save(&self, loco: Locomotive) -> Result<(), RepoError>;
}

// The Use Case test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_loco_use_case_orchestration() {
        let mut mock_repo = MockLocomotiveRepository::new();
        mock_repo.expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = AddLocomotiveUseCase::new(mock_repo);
        let result = use_case.execute(loco_data).await;

        assert!(result.is_ok());
    }
}
```

### Layer 3: Infrastructure (Adapters: SQLx / SQLite)

- **Test type:** Integration Tests (Persistence).
- **Tooling:** `#[sqlx::test]` macro.
- **Strategy:** Run tests against a real SQLite database. The `#[sqlx::test]` macro automatically handles migrations and creates a fresh connection pool for every test, preventing state leakage between runs.
- **Focus:** SQL syntax correctness, database constraints (e.g., Foreign Keys), and correct mapping of database rows back into Domain Entities.

```rust
#[sqlx::test]
async fn test_sqlite_persistence(pool: SqlitePool) {
    let repo = SqliteLocoRepository::new(pool);
    let loco = Locomotive::new("Big Boy".into(), Scale::HO).unwrap();

    repo.save(loco).await.expect("Should save to SQLite");
    let saved = repo.find_by_name("Big Boy").await.unwrap();
    assert_eq!(saved.name, "Big Boy");
}
```

### Layer 4: Interface (Tauri Command Handlers)

- **Test type:** Smoke Tests.
- **Tooling:** `#[tokio::test]`; `tauri::test::mock_builder` for `tauri::State` injection.
- **Strategy:** Tauri commands must be kept thin — their only responsibilities are to extract state, call the correct Application Use Case, and map the result to a serializable format for Svelte. We treat the command as a regular Rust async function and inject `tauri::State` manually in tests. We do not test business logic here; that is the responsibility of the Domain and Application layers.
- **Focus:** Proper wiring — that inputs are correctly passed to the Application layer and results are correctly serialized.
- **Constraint:** Interface tests must not contain assertions about business logic.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tauri::test::{mock_builder, mock_context};

    #[tokio::test]
    async fn smoke_test_add_locomotive_command() {
        let mut mock_repo = MockLocoRepo::new();
        mock_repo.expect_save().returning(|_| Ok(())).once();

        let state = AppState { loco_repo: mock_repo };
        let app = mock_builder().build(mock_context()).unwrap();
        app.manage(state);

        let state_handle = app.state::<AppState>();
        let result = add_locomotive(state_handle, "Orient Express".into()).await;

        assert!(result.is_ok(), "Command should successfully bridge to the Use Case");
    }
}
```

---

## Recommended Testing Tools

| Tool | Purpose |
| --- | --- |
| `cargo test` | Standard unit and integration test runner |
| `mockall` | The standard crate for trait mocking in Rust |
| `sqlx::test` | Macro for isolated database integration tests with automatic migrations |
| `tokio::test` | Async test runtime for use cases and command smoke tests |
| `fake-rs` | Generating realistic dummy data (train names, manufacturers) |
| `pretty_assertions` | Improved diff output for large Domain struct comparisons |

---

## More Information

- [mockall crate documentation](https://docs.rs/mockall)
- [sqlx testing documentation](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
- [fake-rs crate documentation](https://docs.rs/fake)
- [pretty_assertions crate documentation](https://docs.rs/pretty_assertions)
