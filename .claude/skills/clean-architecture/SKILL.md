---
name: clean-architecture
description: Use this skill when designing system architecture, organizing code by layers, implementing domain-driven design, separating concerns, defining bounded contexts, or when the user asks about architecture decisions, code organization, DDD, or separation of concerns.
version: 1.0.0
---

# Clean Architecture & Domain-Driven Design

This skill provides comprehensive guidelines for designing and implementing systems using Clean Architecture principles and Domain-Driven Design (DDD) in a Tauri 2.0 + Rust + Svelte context.

## When This Skill Applies

Use this skill when:

- Designing new features or modules
- Organizing code into layers and bounded contexts
- Making architectural decisions
- Implementing domain models, entities, value objects
- Defining use cases or application services
- Creating adapters for external systems (DB, API, UI)
- User mentions: architecture, DDD, clean architecture, layers, domain model, bounded context

## Core Principles

### The Dependency Rule

**Dependencies point inward**. Inner layers never depend on outer layers.

```
┌─────────────────────────────────────────┐
│         Infrastructure (Adapters)       │ ← Tauri commands, DB, File system
├─────────────────────────────────────────┤
│         Application (Use Cases)         │ ← Orchestration, workflows
├─────────────────────────────────────────┤
│         Domain (Business Logic)         │ ← Entities, Value Objects, Rules
└─────────────────────────────────────────┘
```

### Layer Responsibilities

**Domain Layer (Core)**

- Pure business logic and rules
- No dependencies on frameworks or external systems
- Entities, Value Objects, Aggregates, Domain Events
- Domain Services for operations that don't belong to entities
- Repository interfaces (not implementations)

**Application Layer**

- Use cases and application services
- Orchestrates domain objects to fulfill use cases
- Depends only on domain layer
- Defines interfaces for infrastructure (ports)
- Transaction boundaries

**Infrastructure Layer (Adapters)**

- Implements repository interfaces
- Tauri commands (UI adapter)
- Database access (persistence adapter)
- File system, HTTP clients, external APIs
- Framework-specific code

## Tauri 2.0 Clean Architecture

### Directory Structure

```
src-tauri/src/
├── main.rs                      # App entry point
├── lib.rs                       # Library exports
├── core/                        # Shared domain concepts
│   ├── domain/                  # Core domain types
│   │   ├── value_objects.rs     # Shared value objects
│   │   └── errors.rs            # Domain errors
│   └── infrastructure/          # Shared infrastructure
│       ├── database.rs          # DB connection setup
│       └── error.rs             # Infrastructure errors
├── <bounded_context>/           # e.g., catalog, inventory, dashboard
│   ├── domain/                  # Domain layer
│   │   ├── mod.rs               # Domain exports
│   │   ├── model.rs             # Entities & Aggregates
│   │   ├── value_objects.rs     # Value Objects
│   │   ├── repository.rs        # Repository traits
│   │   └── service.rs           # Domain Services
│   ├── application/             # Application layer
│   │   ├── mod.rs               # Application exports
│   │   ├── use_cases.rs         # Use case implementations
│   │   └── dto.rs               # Data Transfer Objects
│   └── infrastructure/          # Infrastructure layer
│       ├── mod.rs               # Infrastructure exports
│       ├── commands.rs          # Tauri commands
│       ├── repositories.rs      # Repository implementations
│       └── entities.rs          # Database entities (sqlx)
```

### Example: Catalog Bounded Context

```
src-tauri/src/catalog/
├── domain/
│   ├── mod.rs
│   ├── model.rs               # CatalogItem, Brand (entities)
│   ├── value_objects.rs       # ItemNumber, Scale, Price
│   ├── repository.rs          # CatalogRepository trait
│   └── service.rs             # CatalogService
├── application/
│   ├── mod.rs
│   ├── get_catalog_items.rs   # GetCatalogItems use case
│   ├── add_catalog_item.rs    # AddCatalogItem use case
│   └── dto.rs                 # CatalogItemDto
└── infrastructure/
    ├── mod.rs
    ├── commands.rs            # Tauri commands
    ├── repositories.rs        # SqliteCatalogRepository
    └── entities.rs            # Database row structs
```

## Domain Layer Patterns

### Entities

Entities have identity and lifecycle. Their identity persists beyond their attributes.

```rust
// domain/model.rs
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct CatalogItem {
    id: CatalogItemId,
    brand: Brand,
    item_number: ItemNumber,
    description: String,
    scale: Scale,
    price: Option<Price>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl CatalogItem {
    /// Creates a new catalog item.
    pub fn new(
        brand: Brand,
        item_number: ItemNumber,
        description: String,
        scale: Scale,
    ) -> Self {
        Self {
            id: CatalogItemId::new(),
            brand,
            item_number,
            description,
            scale,
            price: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn id(&self) -> &CatalogItemId {
        &self.id
    }

    pub fn set_price(&mut self, price: Price) -> Result<(), DomainError> {
        // Business rule: price must be positive
        if price.amount() <= 0.0 {
            return Err(DomainError::InvalidPrice);
        }
        self.price = Some(price);
        Ok(())
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
```

### Value Objects

Value objects have no identity. They are defined by their attributes and are immutable.

```rust
// domain/value_objects.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemNumber(String);

impl ItemNumber {
    /// Creates a new item number.
    ///
    /// # Errors
    /// Returns error if the item number format is invalid.
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::InvalidItemNumber("Cannot be empty".into()));
        }

        if value.len() > 50 {
            return Err(DomainError::InvalidItemNumber("Too long".into()));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ItemNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Price {
    amount: f64,
    currency: Currency,
}

impl Price {
    pub fn new(amount: f64, currency: Currency) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::InvalidPrice);
        }
        Ok(Self { amount, currency })
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    USD,
    EUR,
    GBP,
}
```

### Repository Traits (Domain Layer)

Define interfaces in domain, implement in infrastructure.

```rust
// domain/repository.rs
use async_trait::async_trait;

#[async_trait]
pub trait CatalogRepository {
    /// Finds a catalog item by ID.
    async fn find_by_id(&self, id: &CatalogItemId) -> Result<Option<CatalogItem>, RepositoryError>;

    /// Saves a catalog item.
    async fn save(&self, item: &CatalogItem) -> Result<(), RepositoryError>;

    /// Finds all catalog items matching criteria.
    async fn find_all(&self, criteria: CatalogCriteria) -> Result<Vec<CatalogItem>, RepositoryError>;

    /// Deletes a catalog item.
    async fn delete(&self, id: &CatalogItemId) -> Result<(), RepositoryError>;
}

pub struct CatalogCriteria {
    pub brand: Option<Brand>,
    pub scale: Option<Scale>,
    pub search: Option<String>,
}
```

### Domain Services

Operations that don't naturally belong to a single entity.

```rust
// domain/service.rs
pub struct CatalogService;

impl CatalogService {
    /// Validates that an item number is unique within a brand.
    pub async fn is_item_number_unique(
        &self,
        brand: &Brand,
        item_number: &ItemNumber,
        repo: &dyn CatalogRepository,
    ) -> Result<bool, RepositoryError> {
        let criteria = CatalogCriteria {
            brand: Some(brand.clone()),
            scale: None,
            search: Some(item_number.value().to_string()),
        };

        let items = repo.find_all(criteria).await?;
        Ok(items.is_empty())
    }
}
```

## Application Layer Patterns

### Use Cases

Each use case represents a single application operation.

```rust
// application/add_catalog_item.rs
use crate::catalog::domain::*;

pub struct AddCatalogItemUseCase<R: CatalogRepository> {
    repository: R,
    service: CatalogService,
}

impl<R: CatalogRepository> AddCatalogItemUseCase<R> {
    pub fn new(repository: R, service: CatalogService) -> Self {
        Self { repository, service }
    }

    /// Adds a new catalog item.
    ///
    /// # Errors
    /// Returns error if validation fails or persistence fails.
    pub async fn execute(
        &self,
        request: AddCatalogItemRequest,
    ) -> Result<CatalogItemId, ApplicationError> {
        // Validate input
        let brand = Brand::from_str(&request.brand)?;
        let item_number = ItemNumber::new(request.item_number)?;
        let scale = Scale::from_str(&request.scale)?;

        // Business rule: item number must be unique per brand
        if !self.service.is_item_number_unique(&brand, &item_number, &self.repository).await? {
            return Err(ApplicationError::DuplicateItemNumber);
        }

        // Create entity
        let mut item = CatalogItem::new(brand, item_number, request.description, scale);

        // Set optional price
        if let Some(price_value) = request.price {
            let price = Price::new(price_value, Currency::USD)?;
            item.set_price(price)?;
        }

        // Persist
        self.repository.save(&item).await?;

        Ok(item.id().clone())
    }
}

#[derive(Debug)]
pub struct AddCatalogItemRequest {
    pub brand: String,
    pub item_number: String,
    pub description: String,
    pub scale: String,
    pub price: Option<f64>,
}
```

### Data Transfer Objects (DTOs)

```rust
// application/dto.rs
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CatalogItemDto {
    pub id: String,
    pub brand: String,
    pub item_number: String,
    pub description: String,
    pub scale: String,
    pub price: Option<f64>,
    pub currency: Option<String>,
}

impl From<CatalogItem> for CatalogItemDto {
    fn from(item: CatalogItem) -> Self {
        Self {
            id: item.id().to_string(),
            brand: item.brand().to_string(),
            item_number: item.item_number().value().to_string(),
            description: item.description().to_string(),
            scale: item.scale().to_string(),
            price: item.price().map(|p| p.amount()),
            currency: item.price().map(|p| p.currency().to_string()),
        }
    }
}
```

## Infrastructure Layer Patterns

### Repository Implementation

```rust
// infrastructure/repositories.rs
use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::catalog::domain::*;
use crate::catalog::infrastructure::entities::*;

pub struct SqliteCatalogRepository {
    pool: SqlitePool,
}

impl SqliteCatalogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogRepository for SqliteCatalogRepository {
    async fn find_by_id(&self, id: &CatalogItemId) -> Result<Option<CatalogItem>, RepositoryError> {
        let entity = sqlx::query_as::<_, CatalogItemEntity>(
            "SELECT * FROM catalog_items WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        match entity {
            Some(e) => Ok(Some(e.try_into()?)),
            None => Ok(None),
        }
    }

    async fn save(&self, item: &CatalogItem) -> Result<(), RepositoryError> {
        let entity = CatalogItemEntity::from(item.clone());

        sqlx::query(
            "INSERT INTO catalog_items (id, brand, item_number, description, scale, price, currency, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                brand = excluded.brand,
                item_number = excluded.item_number,
                description = excluded.description,
                scale = excluded.scale,
                price = excluded.price,
                currency = excluded.currency"
        )
        .bind(&entity.id)
        .bind(&entity.brand)
        .bind(&entity.item_number)
        .bind(&entity.description)
        .bind(&entity.scale)
        .bind(&entity.price)
        .bind(&entity.currency)
        .bind(&entity.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_all(&self, criteria: CatalogCriteria) -> Result<Vec<CatalogItem>, RepositoryError> {
        // Implement query with filters
        todo!()
    }

    async fn delete(&self, id: &CatalogItemId) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM catalog_items WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
```

### Database Entities (sqlx)

```rust
// infrastructure/entities.rs
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CatalogItemEntity {
    pub id: String,
    pub brand: String,
    pub item_number: String,
    pub description: String,
    pub scale: String,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub created_at: String,
}

impl From<CatalogItem> for CatalogItemEntity {
    fn from(item: CatalogItem) -> Self {
        Self {
            id: item.id().to_string(),
            brand: item.brand().to_string(),
            item_number: item.item_number().value().to_string(),
            description: item.description().to_string(),
            scale: item.scale().to_string(),
            price: item.price().map(|p| p.amount()),
            currency: item.price().map(|p| p.currency().to_string()),
            created_at: item.created_at().to_rfc3339(),
        }
    }
}

impl TryFrom<CatalogItemEntity> for CatalogItem {
    type Error = RepositoryError;

    fn try_from(entity: CatalogItemEntity) -> Result<Self, Self::Error> {
        // Convert entity to domain model
        // Includes validation
        todo!()
    }
}
```

### Tauri Commands (Adapters)

```rust
// infrastructure/commands.rs
use tauri::State;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct AddCatalogItemCommand {
    pub brand: String,
    pub item_number: String,
    pub description: String,
    pub scale: String,
    pub price: Option<f64>,
}

/// Adds a new catalog item.
#[tauri::command]
#[specta::specta]
pub async fn add_catalog_item(
    command: AddCatalogItemCommand,
    pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    // Create infrastructure dependencies
    let repository = SqliteCatalogRepository::new(pool.inner().clone());
    let service = CatalogService;
    let use_case = AddCatalogItemUseCase::new(repository, service);

    // Convert command to request
    let request = AddCatalogItemRequest {
        brand: command.brand,
        item_number: command.item_number,
        description: command.description,
        scale: command.scale,
        price: command.price,
    };

    // Execute use case
    let item_id = use_case
        .execute(request)
        .await
        .map_err(|e| e.to_string())?;

    Ok(item_id.to_string())
}

/// Gets all catalog items.
#[tauri::command]
#[specta::specta]
pub async fn get_catalog_items(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<CatalogItemDto>, String> {
    let repository = SqliteCatalogRepository::new(pool.inner().clone());
    let use_case = GetCatalogItemsUseCase::new(repository);

    let items = use_case
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let dtos = items.into_iter().map(CatalogItemDto::from).collect();

    Ok(dtos)
}
```

## Frontend Architecture

### Feature-First Organization

```
src/lib/features/catalog/
├── components/
│   ├── CatalogItemCard.svelte
│   ├── CatalogItemForm.svelte
│   └── CatalogList.svelte
├── stores/
│   └── catalogStore.svelte.ts
├── services/
│   └── catalogService.ts
└── index.ts
```

### Service Layer (Frontend Adapter)

```typescript
// src/lib/features/catalog/services/catalogService.ts
import { addCatalogItem, getCatalogItems } from '$lib/bindings';
import type { CatalogItemDto } from '$lib/bindings';

export class CatalogService {
  async getItems(): Promise<CatalogItemDto[]> {
    return getCatalogItems();
  }

  async addItem(data: AddItemData): Promise<string> {
    return addCatalogItem(data);
  }
}

export const catalogService = new CatalogService();
```

### Store (Frontend State)

```typescript
// src/lib/features/catalog/stores/catalogStore.svelte.ts
import { catalogService } from '../services/catalogService';
import type { CatalogItemDto } from '$lib/bindings';

export class CatalogStore {
  items = $state<CatalogItemDto[]>([]);
  loading = $state(false);
  error = $state<string | null>(null);

  async loadItems() {
    this.loading = true;
    this.error = null;
    try {
      this.items = await catalogService.getItems();
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Unknown error';
    } finally {
      this.loading = false;
    }
  }
}

export const catalogStore = new CatalogStore();
```

## Bounded Contexts

### Identifying Bounded Contexts

- Group related domain concepts
- Each context has its own ubiquitous language
- Contexts communicate through well-defined interfaces
- Example contexts: Catalog, Inventory, Collection, Dashboard

### Context Mapping

```rust
// Core domain types (shared)
src-tauri/src/core/domain/

// Bounded contexts
src-tauri/src/catalog/        // Catalog management
src-tauri/src/inventory/      // Inventory tracking
src-tauri/src/collection/     // User collections
src-tauri/src/dashboard/      // Summary views
```

### Anti-Corruption Layer

When integrating with external systems, use an anti-corruption layer.

```rust
// infrastructure/external/scalemates_adapter.rs
pub struct ScalematesAdapter {
    client: reqwest::Client,
}

impl ScalematesAdapter {
    /// Fetches item from external API and converts to domain model.
    pub async fn fetch_item(&self, item_number: &str) -> Result<CatalogItem, AdapterError> {
        let response = self.client
            .get(format!("https://api.scalemates.com/items/{}", item_number))
            .send()
            .await?;

        let external_item: ScalematesItemDto = response.json().await?;

        // Convert external model to domain model (anti-corruption)
        self.to_domain_model(external_item)
    }

    fn to_domain_model(&self, dto: ScalematesItemDto) -> Result<CatalogItem, AdapterError> {
        // Map external structure to domain model
        // Validate and enforce domain rules
        todo!()
    }
}
```

## Best Practices

### Do's

- **Dependency Rule**: Dependencies point inward
- **Domain Purity**: Keep domain layer free of framework code
- **Rich Domain Models**: Put behavior in domain entities
- **Validation**: Validate at domain layer (value objects, entity methods)
- **Interfaces**: Define repository/service interfaces in domain
- **Use Cases**: One use case per application operation
- **Immutability**: Value objects are immutable
- **Ubiquitous Language**: Use domain terminology in code

### Don'ts

- **Don't** put business logic in Tauri commands
- **Don't** let domain depend on infrastructure
- **Don't** use database entities in domain layer
- **Don't** expose domain entities directly to UI
- **Don't** mix concerns across layers
- **Don't** create anemic domain models (just getters/setters)
- **Don't** skip validation in domain layer
- **Don't** use technical terms when domain terms exist

## Testing Strategy

### Domain Layer

- Unit test entities, value objects, domain services
- No mocks needed (pure business logic)
- Test invariants and business rules

### Application Layer

- Test use cases with mocked repositories
- Verify orchestration logic
- Test error handling

### Infrastructure Layer

- Integration tests with real database (test database)
- Test repository implementations
- Test Tauri command mapping

## Resources

- [Clean Architecture (Robert C. Martin)](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Domain-Driven Design (Eric Evans)](https://www.domainlanguage.com/ddd/)
- [Rust DDD Example](https://github.com/valignatev/ddd-rust)
