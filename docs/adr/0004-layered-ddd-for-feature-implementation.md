# ADR 4: Clean Architecture for Feature Implementation

Status: Accepted

Date: 2026-01-08

Deciders: Project Lead

## 1. Context and Problem Statement

As the Tauri application grows, we face increasing complexity in managing state, database transactions, and business logic. Without a formal structure, the Rust backend risks becoming a collection of ad-hoc modules that leak persistence and transport concerns into business rules. We need an architectural style that enforces clear boundaries, improves testability, and constrains where framework-specific code lives.

## 2. Decision Drivers

- Data Integrity: All database operations within a single user action must be atomic.
- Testability: Business logic must be testable without requiring a running SQLite instance or a Tauri environment.
- Type Safety: We must maintain strict type safety across the Rust↔TypeScript bridge.
- Decoupling: The core business rules should not depend on Tauri, SQLite, or other frameworks.
- Maintainability: Layers should have clear responsibilities so developers can reason about change impact.

## 3. Considered Options

### Option A: Clean Architecture with Unit of Work (The Proposed Design)

Adopt Clean Architecture (Uncle Bob) which organizes code into concentric rings: Entities (domain models and business rules), Use Cases / Interactors (application-specific business rules), Interface Adapters (controllers, presenters, gateways), and Frameworks & Drivers (Tauri, SQLite, web server, etc.). Dependencies point inward: outer layers depend on abstractions defined by inner layers.

- Good, because: Strong separation of concerns; core business rules are framework-agnostic and easily unit-testable. Adapters implement interfaces (traits) defined by inner layers, enabling mocking and in-memory implementations for tests. The Unit of Work pattern at the Interface/Infrastructure boundary provides transaction atomicity across repository operations.
- Bad, because: Requires discipline and some initial boilerplate (traits for ports, adapters for infrastructure, and small glue code).

Key mechanics for this repo:
- Domain layer (entities, domain errors) contains no framework imports and expresses business invariants.
- Use Case layer exposes interactor functions/structs that accept repository and output ports as trait objects/generics.
- Interface Adapters map transport (Tauri commands) and external models to Use Case inputs and map outputs back to view models or DTOs.
- Infrastructure layer implements repository traits, Unit of Work, and concrete DB transactions against SQLite.
- The Interface (Tauri commands) composes the Unit of Work and invokes Use Cases, ensuring transaction lifecycle is controlled at the outer edge.

### Option B: Transaction-Passing Functional Services

Pass a mutable SQLite transaction through functions (e.g., &mut SqliteTransaction) so that callers explicitly manage transactional context.

- Good, because: Minimal boilerplate and straightforward implementation.
- Bad, because: Business logic becomes aware of SQLite internals, making unit testing harder and coupling domain/use-cases to persistence technology.

### Option C: Vertical/Feature-Sliced Modules

Organize code by feature (vertical slice) where each feature contains its domain, use cases, and infrastructure code grouped together.

- Good, because: High cohesion and discoverability for feature developers.
- Bad, because: Shared concerns (transactions, cross-cutting domain concepts) can be duplicated or inconsistently implemented without strong conventions.

## 4. Decision Outcome

Chosen Option: Clean Architecture with Unit of Work (Option A)

### Justification

- Clean Architecture enforces dependency inversion: repositories and ports are defined as traits in inner layers and implemented by infrastructure, preventing leakage of persistence details into core business logic.
- The Unit of Work pattern complements Clean Architecture by providing a single transaction boundary that the Interface layer controls, satisfying the Data Integrity driver.
- This approach maximizes testability: Use Cases can be exercised with in-memory or mocked repositories, and Tauri-specific mapping is confined to adapters.

### Consequences

Positive:
- Core business rules remain framework-agnostic and highly testable.
- Infrastructure and framework changes (e.g., swapping SQLite for another datastore, or replacing Tauri with a web backend) have minimal impact on Use Cases and Domain logic.
- Transactional atomicity is explicit and controlled at the Interface/Infrastructure boundary.

Negative:
- Initial boilerplate: traits for ports, wrappers for the Unit of Work, and adapter implementations add files and initial cognitive overhead.
- Requires developer discipline to place code in the correct layer and to prefer trait-based ports for cross-layer interactions.

Neutral:
- Continued use of specta is recommended to keep Interface types in sync with the frontend.

## 5. Implementation Notes

- Define repository and output port traits in the Use Case layer (or a small `ports` module) so tests can supply fake implementations.
- Implement a concrete `SqliteUnitOfWork` in Infrastructure that provides transaction lifecycle methods and gives access to concrete Repository implementations.
- Tauri command handlers should:
  - Construct the Unit of Work (or obtain it from an application-level factory),
  - Begin a transaction, map input to Use Case input DTOs, invoke the Use Case, then commit/rollback the Unit of Work based on the result.
- Avoid passing SQLite transaction objects into Use Cases. Instead, provide repositories that capture the transactional context via the Unit of Work.

## 6. Examples (Sketch)

- Use Case signature example:

```
pub trait CollectionRepository: Send + Sync {
    fn find_by_id(&self, id: CollectionId) -> Result<Option<Collection>, RepoError>;
    fn save(&self, collection: &Collection) -> Result<(), RepoError>;
}

pub struct CreateCollectionUseCase<R: CollectionRepository> {
    repo: R,
}

impl<R: CollectionRepository> CreateCollectionUseCase<R> {
    pub fn execute(&self, input: CreateCollectionInput) -> Result<CreateCollectionOutput, UseCaseError> {
        // business logic here
    }
}
```

- Unit of Work sketch:

```
pub trait UnitOfWork {
    type Repos;
    fn begin(&mut self) -> Result<(), UowError>;
    fn commit(&mut self) -> Result<(), UowError>;
    fn rollback(&mut self) -> Result<(), UowError>;
}
```
