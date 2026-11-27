# Frontend Architecture Standards

## Preamble

This document defines **normative** architectural standards for the Rusty Shed frontend application. These standards are enforced through automated checks (TypeScript strictness, ESLint boundaries, CI verification) and code review.

**Core Design Principle:**

> Application logic must be testable **without Tauri running**. This requirement cascades into all architectural decisions: explicit dependency injection, repository interfaces, centralized error translation, and strict layer boundaries.

---

## 1. Architectural Principles

### 1.1 Clean Architecture

The frontend application follows Clean Architecture principles:

- **Dependency Rule:** Code dependencies point inward. Domain and application layers never depend on outer layers (presentation, infrastructure).
- **Independent Testability:** Each layer can be tested in isolation using appropriate test doubles.
- **Framework Agnostic:** Business logic knows nothing about Tauri, Svelte, or the DOM.
- **Database Agnostic:** Application logic depends on repository interfaces, not concrete implementations.

### 1.2 Hexagonal Architecture (Ports & Adapters)

Application-layer code defines the interfaces it needs (ports). Infrastructure provides implementations (adapters):

```
Application Layer (defines interfaces)
         ↑
    Repository
    Interface
         ↑
Infrastructure Layer (provides implementations)
    TauriCollectionRepository
         ↑
    Generated Bindings
         ↑
    Tauri IPC → Rust
```

### 1.3 Single Source of Truth

- **Rust is the domain authority:** Business rules, invariants, and persistent state live in Rust.
- **Frontend domain is frontend-specific:** Selection logic, presentation validation, UI workflows.
- **Generated bindings are the contract:** Specta bindings ensure the frontend contract stays synchronized with the Rust backend.

### 1.4 Feature-Based Organization

Features are vertical slices of the application, not horizontal layers. Each feature owns its domain, application, infrastructure, and presentation code.

This prevents:

- Implicit global dependencies
- Cross-feature coupling
- Unclear feature boundaries
- Difficult feature removal

### 1.5 Testability-First

Testability is not a goal—it is a **constraint**. If code cannot be tested without external systems (Tauri, database, network), the architecture has failed.

---

## 2. Technical Stack

| Concern                  | Choice                                            | Version |
| ------------------------ | ------------------------------------------------- | ------- |
| **Frontend Framework**   | Svelte 5 (Runes: `$state`, `$derived`, `$effect`) | 5.x     |
| **Backend Framework**    | Tauri 2.0                                         | 2.x     |
| **Type Bridge**          | tauri-specta                                      | Latest  |
| **Reactive State**       | Svelte 5 runes (`$state`, `$derived`)             | Native  |
| **Component Library**    | shadcn-svelte (bits-ui)                           | Latest  |
| **CSS Framework**        | Tailwind CSS 4                                    | 4.x     |
| **Type Safety**          | TypeScript strict mode                            | 5.x+    |
| **i18n**                 | Paraglide-JS                                      | Latest  |
| **Unit Testing**         | Vitest                                            | Latest  |
| **Component Testing**    | Svelte Testing Library + @testing-library         | Latest  |
| **E2E Testing**          | Playwright                                        | Latest  |
| **Linting & Boundaries** | ESLint + eslint-plugin-boundaries                 | Latest  |
| **Static Analysis**      | svelte-check                                      | Latest  |

---

## 3. Architectural Layers

Each feature is organized into four **conceptual** layers. Dependencies flow inward; outer layers depend on inner layers, never the reverse.

### 3.1 Domain Layer

**Responsibility:** Business rules, entities, value objects, and invariants that remain true regardless of how the application is delivered.

**What lives here:**

- Type definitions (entities, value objects)
- Business rules and validation
- Exceptions and invariants
- Domain-specific logic

**What does NOT live here:**

- Framework imports (Svelte, Tauri)
- Generated bindings
- UI state
- Async operations (domain is synchronous)

**Example: `collection/domain/collection.types.ts`**

```typescript
// Pure TypeScript, no framework dependencies

export interface CollectionItem {
  id: string;
  name: string;
  scale: 'H0' | 'N' | 'TT';
  year?: number;
}

export function validateItemName(name: string): void {
  if (!name || name.trim().length === 0) {
    throw new DomainError('name_required', 'Item name cannot be empty');
  }
  if (name.length > 255) {
    throw new DomainError('name_too_long', 'Item name must not exceed 255 characters');
  }
}

export interface CollectionRules {
  canAddItem(item: CollectionItem): void; // Throws if violated
  canDeleteItem(itemId: string): void;
}
```

### 3.2 Application Layer

**Responsibility:** Orchestrates domain logic and coordinates use cases. Defines the interfaces (ports) it needs; never implements IPC, persistence, or UI concerns.

**What lives here:**

- Use cases (application services)
- Repository interfaces (ports)
- State coordination logic
- Workflow orchestration
- Error aggregation

**What does NOT live here:**

- Tauri invocations
- Generated bindings
- Svelte component logic
- UI state management

**Example: `collection/application/ports/CollectionRepository.ts`**

```typescript
// Application defines what it needs; infrastructure implements it

import type { CollectionItem, CreateCollectionItemInput } from '../domain/collection.types';

export interface CollectionRepository {
  getAll(): Promise<CollectionItem[]>;
  getById(id: string): Promise<CollectionItem | null>;
  add(input: CreateCollectionItemInput): Promise<CollectionItem>;
  update(id: string, input: Partial<CreateCollectionItemInput>): Promise<CollectionItem>;
  delete(id: string): Promise<void>;
}
```

**Example: `collection/application/use-cases/CreateCollectionItem.ts`**

```typescript
import type { CollectionRepository } from '../ports/CollectionRepository';
import type { CreateCollectionItemInput } from '../../domain/collection.types';
import { validateItemName } from '../../domain/collection.rules';

export class CreateCollectionItem {
  constructor(private repository: CollectionRepository) {}

  async execute(input: CreateCollectionItemInput) {
    // Validate domain rules
    validateItemName(input.name);

    // Delegate to repository (abstraction, not Tauri)
    return this.repository.add(input);
  }
}
```

**Key Detail:** The use case depends on `CollectionRepository` interface, not any concrete Tauri implementation.

### 3.3 Infrastructure Layer

**Responsibility:** Provides concrete implementations of application-layer interfaces. Handles Tauri IPC, error translation, serialization, and all technical details.

**What lives here:**

- Repository implementations (adapters)
- Tauri command wrappers
- Generated Specta bindings
- Error mappers (Tauri → Application errors)
- Event adapters
- Platform-specific implementations

**What does NOT live here:**

- Business logic
- UI state
- Domain rules

**Example: `collection/infrastructure/TauriCollectionRepository.ts`**

```typescript
import { commands } from '$lib/infrastructure/tauri/bindings';
import type { CollectionRepository } from '../application/ports/CollectionRepository';
import type { CollectionItem, CreateCollectionItemInput } from '../domain/collection.types';

export class TauriCollectionRepository implements CollectionRepository {
  async getAll(): Promise<CollectionItem[]> {
    const result = await commands.getCollection();
    return result; // Specta has already typed this
  }

  async add(input: CreateCollectionItemInput): Promise<CollectionItem> {
    try {
      return await commands.createCollectionItem(input);
    } catch (error) {
      throw mapTauriError(error); // Centralized error translation
    }
  }

  async getById(id: string): Promise<CollectionItem | null> {
    try {
      return await commands.getCollectionItem(id);
    } catch (error) {
      if (isTauriNotFoundError(error)) {
        return null;
      }
      throw mapTauriError(error);
    }
  }

  async update(id: string, input: Partial<CreateCollectionItemInput>): Promise<CollectionItem> {
    try {
      return await commands.updateCollectionItem(id, input);
    } catch (error) {
      throw mapTauriError(error);
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await commands.deleteCollectionItem(id);
    } catch (error) {
      if (!isTauriNotFoundError(error)) {
        throw mapTauriError(error);
      }
    }
  }
}
```

**Key Detail:** The Tauri repository is **the only place** that imports `commands`. All error handling is centralized here.

### 3.4 Presentation Layer

**Responsibility:** Svelte components, layouts, and UI state management. Receives data from application state; never calls Tauri or repositories directly.

**What lives here:**

- Svelte components and pages
- UI state (form data, local filters, animations)
- Event handlers
- Responsive layouts

**What does NOT live here:**

- Business logic
- Tauri invocations
- Generated bindings
- Repository logic

**Example: `collection/presentation/CollectionView.svelte`**

```svelte
<script lang="ts">
  import { useCollectionContext } from '../collection.context';

  const collection = useCollectionContext();
  const { items, loading, error } = $derived.by(() => ({
    items: collection.items,
    loading: collection.loading,
    error: collection.error
  }));
</script>

<div class="collection">
  {#if loading}
    <p>Loading...</p>
  {:else if error}
    <p class="error">{error.message}</p>
  {:else}
    {#each items as item (item.id)}
      <ItemCard {item} onDelete={() => collection.deleteItem(item.id)} />
    {/each}
  {/if}
</div>
```

---

## 4. State Management (Svelte 5 Runes)

### 4.1 Reactive State Classes (`.svelte.ts` files)

State classes coordinate operations, hold reactive state, and expose a clean API to components. They are **not** generic state frameworks; they are feature-specific.

**Responsibilities:**

- Hold reactive state (`$state`)
- Expose read-only state via getters
- Coordinate operations via injected use cases/services
- Manage loading and error status
- Handle race conditions and cancellation

**Non-Responsibilities:**

- Calling Tauri directly
- Implementing business logic
- Translating Tauri errors

**Example: `collection/application/CollectionState.svelte.ts`**

```typescript
import type { CreateCollectionItem } from './use-cases/CreateCollectionItem';
import type { GetCollection } from './use-cases/GetCollection';
import type { CollectionItem } from '../domain/collection.types';
import { toAppError } from '$lib/core/errors/AppError';

export class CollectionState {
  #items = $state<CollectionItem[]>([]);
  #loading = $state(false);
  #error = $state<AppError | null>(null);
  #abortController: AbortController | null = null;

  constructor(
    private readonly getCollection: GetCollection,
    private readonly createItem: CreateCollectionItem
  ) {}

  get items() {
    return this.#items;
  }

  get loading() {
    return this.#loading;
  }

  get error() {
    return this.#error;
  }

  async load() {
    this.#loading = true;
    this.#error = null;

    try {
      this.#items = await this.getCollection.execute();
    } catch (err) {
      this.#error = toAppError(err);
    } finally {
      this.#loading = false;
    }
  }

  async addItem(input: CreateCollectionItemInput) {
    this.#error = null;

    try {
      const item = await this.createItem.execute(input);
      this.#items = [...this.#items, item];
      return item;
    } catch (err) {
      this.#error = toAppError(err);
      return null;
    }
  }

  async deleteItem(id: string) {
    this.#error = null;

    try {
      await this.deleteItemUseCase.execute(id);
      this.#items = this.#items.filter((item) => item.id !== id);
    } catch (err) {
      this.#error = toAppError(err);
    }
  }
}
```

**Key Details:**

- Private fields with `#` enforce encapsulation
- Getters expose state as read-only
- Operations are coordinated, not implemented
- Error translation happens via `toAppError` (not raw Tauri errors)
- No direct Tauri calls

### 4.2 Handling Concurrent Operations

Define explicit policies for concurrent requests to prevent race conditions.

**Approaches:**

1. **Latest-Wins (default for queries):** The most recent request overwrites earlier responses.

```typescript
#requestId = 0;

async load() {
  const thisRequestId = ++this.#requestId;
  const items = await this.getCollection.execute();

  if (thisRequestId === this.#requestId) {
    this.#items = items; // Ignore stale responses
  }
}
```

2. **Cancellation (for long-running operations):**

```typescript
async load() {
  if (this.#abortController) {
    this.#abortController.abort();
  }

  this.#abortController = new AbortController();
  this.#loading = true;

  try {
    this.#items = await this.getCollection.execute(this.#abortController.signal);
  } catch (err) {
    if (err instanceof DOMException && err.name === 'AbortError') {
      return; // Cancelled, do nothing
    }
    this.#error = toAppError(err);
  } finally {
    this.#loading = false;
  }
}
```

3. **Optimistic Updates (for mutations):**

```typescript
async addItem(input: CreateCollectionItemInput) {
  const snapshot = [...this.#items];
  const tempItem = { id: 'temp-' + Date.now(), ...input };

  this.#items = [...this.#items, tempItem];

  try {
    const item = await this.createItem.execute(input);
    this.#items = this.#items.map(i => i.id === tempItem.id ? item : i);
  } catch (err) {
    this.#items = snapshot;
    this.#error = toAppError(err);
  }
}
```

### 4.3 Derived State

Use `$derived` for reactive computations, **not** `$effect`.

```typescript
export class CollectionState {
  #items = $state<CollectionItem[]>([]);
  #filter = $state('');

  get items() {
    return this.#items;
  }

  get filteredItems() {
    return $derived.by(() => {
      return this.#items.filter((item) =>
        item.name.toLowerCase().includes(this.#filter.toLowerCase())
      );
    });
  }

  setFilter(filter: string) {
    this.#filter = filter;
  }
}
```

**Never use `$effect` for derived state.** Svelte's `$derived` is designed for this and is more efficient.

---

## 5. Backend Integration & Tauri IPC

### 5.1 Generated Bindings (Specta)

Generated bindings are **infrastructure-only**. They must not appear in domain, application, or presentation layers.

**Location:** `src/lib/infrastructure/tauri/bindings.ts`

**Usage:** Only in infrastructure repositories and adapters:

```typescript
// ✅ Allowed: Infrastructure
import { commands } from '$lib/infrastructure/tauri/bindings';

export class TauriCollectionRepository implements CollectionRepository {
  async getAll() {
    return commands.getCollection();
  }
}

// ❌ Not allowed: Application
// Use the repository interface instead
```

**CI Verification:** Generated bindings must be regenerated and verified before commit:

```bash
pnpm specta:generate
git diff --exit-code src/lib/infrastructure/tauri/bindings.ts
```

Add this check to CI to ensure frontend types stay synchronized with Rust signatures.

### 5.2 Error Handling Architecture

Errors flow through a translation boundary at the infrastructure layer. Application and presentation layers never handle Tauri-specific errors.

**Define Application Errors:**

`src/lib/core/errors/AppError.ts`

```typescript
export type AppError =
  | { type: 'not-found'; message: string }
  | { type: 'validation'; message: string }
  | { type: 'conflict'; message: string }
  | { type: 'permission'; message: string }
  | { type: 'network'; message: string }
  | { type: 'unexpected'; message: string };

export function toAppError(error: unknown): AppError {
  if (error instanceof DOMException && error.name === 'AbortError') {
    return { type: 'unexpected', message: 'Operation cancelled' };
  }

  if (error instanceof Error) {
    return { type: 'unexpected', message: error.message };
  }

  return { type: 'unexpected', message: 'Unknown error' };
}
```

**Map Tauri Errors:**

`src/lib/infrastructure/tauri/TauriErrorMapper.ts`

```typescript
export function mapTauriError(error: unknown): AppError {
  if (typeof error !== 'object' || error === null) {
    return { type: 'unexpected', message: 'Unknown error' };
  }

  // Match your CommandError enum from Rust
  if ('ValidationError' in error) {
    return {
      type: 'validation',
      message: (error as any).ValidationError
    };
  }

  if ('NotFound' in error) {
    return {
      type: 'not-found',
      message: (error as any).NotFound
    };
  }

  if ('ConflictError' in error) {
    return {
      type: 'conflict',
      message: (error as any).ConflictError
    };
  }

  return { type: 'unexpected', message: JSON.stringify(error) };
}
```

**Use in Repositories:**

```typescript
export class TauriCollectionRepository implements CollectionRepository {
  async add(input: CreateCollectionItemInput): Promise<CollectionItem> {
    try {
      return await commands.createCollectionItem(input);
    } catch (error) {
      throw mapTauriError(error);
    }
  }
}
```

**Result:** State classes and components never see Tauri errors; they only see typed `AppError`.

### 5.3 Tauri's Error Handling Modes

By default, Specta generates `Promise<T>` for Rust `Result<T, E>`, with errors rejected as promises.

**Example (default behavior):**

```rust
#[tauri::command]
pub fn create_item(input: Input) -> Result<Item, CommandError> {
  // ...
}
```

Becomes:

```typescript
// Default: errors reject the promise
commands.createItem(input).catch((error) => {
  /* Tauri error */
});
```

**Alternative (Explicit Result Type):**

Tauri Specta also supports `ErrorHandlingMode::Result`, which generates a discriminated result:

```rust
#[specta::specta]
#[tauri::command(async)]
pub fn create_item(input: Input) -> Result<Item, CommandError> {
  // ...
}
```

Becomes:

```typescript
// With ErrorHandlingMode::Result
type Result<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E };
const result = await commands.createItem(input);
if (result.status === 'ok') {
  /* success */
} else {
  /* error */
}
```

**Recommendation:** Use the default promise-based approach. The infrastructure layer translates promise rejections into `AppError` using `mapTauriError`. This keeps the boundary clean.

---

## 6. Feature Organization & Boundaries

Each feature is a cohesive vertical slice containing domain, application, infrastructure, and presentation code.

### 6.1 Directory Structure

```
src/lib/
├── core/
│   ├── errors/
│   │   ├── AppError.ts
│   │   └── ErrorMapper.ts
│   ├── result/
│   │   └── Result.ts
│   └── types/
│       └── common.types.ts
│
├── infrastructure/
│   └── tauri/
│       ├── bindings.ts (autogenerated, do not edit)
│       ├── TauriErrorMapper.ts
│       └── TauriEventAdapter.ts
│
├── features/
│   ├── collection/
│   │   ├── domain/
│   │   │   ├── collection.types.ts
│   │   │   └── collection.rules.ts
│   │   │
│   │   ├── application/
│   │   │   ├── ports/
│   │   │   │   └── CollectionRepository.ts
│   │   │   ├── use-cases/
│   │   │   │   ├── GetCollection.ts
│   │   │   │   ├── CreateCollectionItem.ts
│   │   │   │   ├── UpdateCollectionItem.ts
│   │   │   │   └── DeleteCollectionItem.ts
│   │   │   ├── CollectionState.svelte.ts
│   │   │   └── index.ts
│   │   │
│   │   ├── infrastructure/
│   │   │   └── TauriCollectionRepository.ts
│   │   │
│   │   ├── presentation/
│   │   │   ├── CollectionView.svelte
│   │   │   ├── components/
│   │   │   │   └── ItemCard.svelte
│   │   │   └── dialogs/
│   │   │       └── CreateItemDialog.svelte
│   │   │
│   │   ├── collection.context.ts
│   │   └── index.ts
│   │
│   ├── wishlist/
│   │   └── (same structure)
│   │
│   ├── depot/
│   │   └── (same structure)
│   │
│   └── dashboard/
│       ├── presentation/
│       ├── domain/
│       └── application/
│
├── shared/
│   ├── components/
│   │   └── Button.svelte (non-feature UI components)
│   ├── hooks/
│   │   └── useMediaQuery.ts
│   └── utils/
│       └── format.ts
│
└── app/
    └── composition-root.ts
```

### 6.2 File Responsibilities

| File                           | Responsibility                               |
| ------------------------------ | -------------------------------------------- |
| `domain/*.types.ts`            | Entities, value objects, type definitions    |
| `domain/*.rules.ts`            | Validation functions, business invariants    |
| `application/ports/*.ts`       | Interface contracts (what the app needs)     |
| `application/use-cases/*.ts`   | Single-responsibility use case classes       |
| `application/*State.svelte.ts` | Reactive state coordination                  |
| `application/index.ts`         | Public exports (use cases, state interfaces) |
| `infrastructure/*.ts`          | Repository implementations, error mappers    |
| `presentation/*.svelte`        | Svelte components (UI, layout, interaction)  |
| `*.context.ts`                 | Svelte Context API providers and consumers   |
| `index.ts` (feature root)      | Public exports (types, context, state)       |

### 6.3 Feature Boundaries

**Features must not:**

- Access another feature's internal implementation (`application/`, `infrastructure/`, `domain/`)
- Share state classes (state classes are feature-scoped)
- Import application or domain code from other features

**Features may:**

- Use shared components from `src/lib/shared/`
- Import types from other features' `index.ts` (public exports only)
- Appear as a consumer in another feature's application layer (e.g., dashboard consuming collection types)

**Example: Dashboard consuming Collection:**

```typescript
// ✅ Allowed: Importing from public exports
import type { CollectionItem } from '$lib/features/collection/index';

// ❌ Not allowed: Reaching into internal layers
// import { CollectionState } from '$lib/features/collection/application/CollectionState.svelte';
```

---

## 7. Testing Strategy

The testing pyramid ensures confidence at multiple levels without excessive E2E overhead.

### 7.1 Testing Pyramid

```
        ╭─────────────╮
        │   E2E (few) │  Critical user journeys with actual Tauri
        ├─────────────┤
        │  Component  │  UI behavior with injected state
        │ Integration │  Feature workflows
        ├─────────────┤
        │    Unit     │  Domain rules, use cases, state coordination
        │  (many)     │  Pure logic, no Tauri, no DOM
        ╰─────────────╯
```

### 7.2 Unit Tests (Domain & Application)

**What to test:**

- Domain validation rules
- Use case execution with fake repositories
- State coordination logic
- Error handling paths

**What NOT to test:**

- Svelte compiler output
- Component rendering (that's component testing)
- Actual Tauri IPC

**Example: Domain validation**

```typescript
// collection/domain/__tests__/collection.rules.test.ts
import { validateItemName } from '../collection.rules';
import { DomainError } from '$lib/core/errors/AppError';

describe('validateItemName', () => {
  it('accepts valid names', () => {
    expect(() => validateItemName('ICE 3')).not.toThrow();
  });

  it('rejects empty names', () => {
    expect(() => validateItemName('')).toThrow(DomainError);
  });

  it('rejects names exceeding 255 characters', () => {
    const longName = 'x'.repeat(256);
    expect(() => validateItemName(longName)).toThrow(DomainError);
  });
});
```

**Example: Use case with fake repository**

```typescript
// collection/application/__tests__/CreateCollectionItem.test.ts
import { CreateCollectionItem } from '../use-cases/CreateCollectionItem';
import type { CollectionRepository } from '../ports/CollectionRepository';

class FakeCollectionRepository implements CollectionRepository {
  items: CollectionItem[] = [];

  async add(input: CreateCollectionItemInput) {
    const item = { ...input, id: crypto.randomUUID() };
    this.items.push(item);
    return item;
  }

  async getAll() {
    return this.items;
  }

  async getById(id: string) {
    return this.items.find((item) => item.id === id) ?? null;
  }

  async update(id: string, input: Partial<CreateCollectionItemInput>) {
    const item = this.items.find((i) => i.id === id);
    if (!item) throw new Error('Not found');
    Object.assign(item, input);
    return item;
  }

  async delete(id: string) {
    this.items = this.items.filter((i) => i.id !== id);
  }
}

describe('CreateCollectionItem', () => {
  it('creates an item and adds it to the repository', async () => {
    const repo = new FakeCollectionRepository();
    const useCase = new CreateCollectionItem(repo);

    const item = await useCase.execute({ name: 'ICE 3', scale: 'H0' });

    expect(item.name).toBe('ICE 3');
    expect(repo.items).toHaveLength(1);
  });

  it('validates the item name', async () => {
    const repo = new FakeCollectionRepository();
    const useCase = new CreateCollectionItem(repo);

    await expect(useCase.execute({ name: '', scale: 'H0' })).rejects.toThrow();
  });
});
```

**Example: State coordination**

```typescript
// collection/application/__tests__/CollectionState.test.ts
import { CollectionState } from '../CollectionState.svelte';
import type { GetCollection } from '../use-cases/GetCollection';

class FakeGetCollection implements GetCollection {
  constructor(private items: CollectionItem[]) {}

  async execute() {
    return this.items;
  }
}

describe('CollectionState', () => {
  it('loads items and updates state', async () => {
    const getCollection = new FakeGetCollection([{ id: '1', name: 'ICE 3', scale: 'H0' }]);
    const state = new CollectionState(getCollection);

    expect(state.loading).toBe(false);
    expect(state.items).toHaveLength(0);

    await state.load();

    expect(state.loading).toBe(false);
    expect(state.items).toHaveLength(1);
  });

  it('sets error state on failure', async () => {
    const getCollection = new FakeGetCollection([]);
    const failingUseCase = {
      execute: async () => {
        throw new Error('Network error');
      }
    };

    const state = new CollectionState(failingUseCase);

    await state.load();

    expect(state.error).not.toBeNull();
    expect(state.error?.type).toBe('unexpected');
  });
});
```

### 7.3 Component Tests (Presentation)

**What to test:**

- User interactions (clicks, form input)
- DOM updates after state changes
- Event handler invocations

**Example: Component with injected state**

```typescript
// collection/presentation/__tests__/CollectionView.test.ts
import { render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import CollectionView from '../CollectionView.svelte';

class FakeCollectionState {
  items = [{ id: '1', name: 'ICE 3', scale: 'H0' }];
  loading = false;
  error = null;

  async deleteItem(id: string) {
    this.items = this.items.filter((i) => i.id !== id);
  }
}

describe('CollectionView', () => {
  it('renders items from state', () => {
    const state = new FakeCollectionState();
    render(CollectionView, { props: { state } });

    expect(screen.getByText('ICE 3')).toBeInTheDocument();
  });

  it('calls deleteItem when delete button is clicked', async () => {
    const user = userEvent.setup();
    const state = new FakeCollectionState();
    const deleteItemSpy = vi.spyOn(state, 'deleteItem');

    render(CollectionView, { props: { state } });

    const deleteButton = screen.getByRole('button', { name: /delete/i });
    await user.click(deleteButton);

    expect(deleteItemSpy).toHaveBeenCalledWith('1');
  });

  it('shows loading indicator while loading', () => {
    const state = new FakeCollectionState();
    state.loading = true;

    render(CollectionView, { props: { state } });

    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });
});
```

### 7.4 Integration Tests (Feature Workflows)

Test multiple layers together without Tauri.

```typescript
// collection/application/__tests__/CollectionFeature.integration.test.ts
describe('Collection Feature Workflow', () => {
  it('creates and retrieves an item', async () => {
    // Compose the feature with fake Tauri
    const repo = new FakeCollectionRepository();
    const getCollection = new GetCollection(repo);
    const createItem = new CreateCollectionItem(repo);
    const state = new CollectionState(getCollection, createItem);

    // Load empty collection
    await state.load();
    expect(state.items).toHaveLength(0);

    // Add an item
    const item = await state.addItem({ name: 'ICE 3', scale: 'H0' });
    expect(item).not.toBeNull();
    expect(state.items).toHaveLength(1);

    // Reload and verify
    await state.load();
    expect(state.items[0]?.name).toBe('ICE 3');
  });
});
```

### 7.5 E2E Tests (Critical Journeys)

Use Playwright to test against the actual packaged application with real Tauri IPC.

```typescript
// test/e2e/collection.e2e.ts
import { test, expect } from '@playwright/test';

test('user can create and delete a collection item', async ({ page }) => {
  await page.goto('/collection');

  // Click create button
  await page.click('button:has-text("Add Item")');

  // Fill form
  await page.fill('input[name="name"]', 'ICE 3');
  await page.selectOption('select[name="scale"]', 'H0');

  // Submit
  await page.click('button:has-text("Create")');

  // Verify item appears
  await expect(page.locator('text=ICE 3')).toBeVisible();

  // Delete item
  await page.click('button:has-text("Delete")');

  // Verify removal
  await expect(page.locator('text=ICE 3')).not.toBeVisible();
});
```

### 7.6 Generated Bindings Verification (CI)

Add a CI step to ensure Specta bindings are always up to date.

```yaml
# .github/workflows/ci.yml
- name: Check Specta bindings are current
  run: |
    pnpm specta:generate
    git diff --exit-code src/lib/infrastructure/tauri/bindings.ts
```

---

## 8. Architectural Guardrails & Enforcement

### 8.1 TypeScript Strictness

**Requirement:** `tsconfig.json` must have:

```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitThis": true,
    "noPropertyAccessFromIndexSignature": true,
    "useUnknownInCatchVariables": true
  }
}
```

**Enforcement:** `pnpm svelte-check` exits with error on type violations.

### 8.2 ESLint Boundaries (eslint-plugin-boundaries)

**Prevent cross-layer imports:**

```javascript
// eslint.config.js
export default [
  {
    plugins: { boundaries: boundariesPlugin },
    rules: {
      'boundaries/no-unknown': ['error', { default: 'deny' }],
      'boundaries/no-crossing': [
        'error',
        {
          zone: 'presentation',
          from: ['presentation'],
          disallow: ['infrastructure', 'application', 'domain']
        },
        {
          zone: 'application',
          from: ['application'],
          disallow: ['infrastructure']
        },
        {
          zone: 'infrastructure',
          from: ['infrastructure'],
          disallow: ['domain']
        }
      ]
    }
  }
];
```

This ensures:

- Presentation cannot import infrastructure or generated bindings
- Application cannot import Tauri APIs directly
- Domain is completely isolated

### 8.3 No Generated Bindings Outside Infrastructure

**Enforcement:** Use ESLint to prevent imports of `bindings.ts` outside infrastructure:

```javascript
{
  rules: {
    'no-restricted-imports': [
      'error',
      {
        patterns: [
          {
            group: ['$lib/infrastructure/tauri/bindings'],
            message: 'Generated bindings are infrastructure-only. Use repository interfaces instead.'
          }
        ]
      }
    ]
  }
}
```

### 8.4 Circular Dependency Detection

Use Dependency Cruiser to prevent circular imports:

```javascript
// .dependency-cruiser.cjs
module.exports = {
  forbidden: [
    {
      name: 'no-circular',
      severity: 'error',
      type: 'circular'
    }
  ]
};
```

Run in CI: `npx depcruise src`

### 8.5 Use Case Test Coverage

Code review requirement: Every use case must have corresponding unit tests that verify:

- Success path
- Error paths
- Edge cases

---

## 9. Dependency Injection & Composition Root

### 9.1 Composition Root Pattern

The composition root assembles all dependencies and injects them into the application.

**Location:** `src/lib/app/composition-root.ts`

```typescript
import { TauriCollectionRepository } from '$lib/features/collection/infrastructure/TauriCollectionRepository';
import { GetCollection } from '$lib/features/collection/application/use-cases/GetCollection';
import { CreateCollectionItem } from '$lib/features/collection/application/use-cases/CreateCollectionItem';
import { CollectionState } from '$lib/features/collection/application/CollectionState.svelte';

// Compose the collection feature
function createCollectionFeature() {
  const repository = new TauriCollectionRepository();
  const getCollection = new GetCollection(repository);
  const createItem = new CreateCollectionItem(repository);

  return {
    state: new CollectionState(getCollection, createItem),
    repository
  };
}

export function createApplication() {
  return {
    collection: createCollectionFeature()
    // wishlist, depot, dashboard...
  };
}
```

### 9.2 Feature Context Providers

Each feature provides its state via Context:

```typescript
// collection/collection.context.ts
import { getContext, setContext } from 'svelte';
import type { CollectionState } from './application/CollectionState.svelte';

const COLLECTION_KEY = Symbol('collection');

export function provideCollectionContext(state: CollectionState) {
  setContext(COLLECTION_KEY, state);
  return state;
}

export function useCollectionContext() {
  const state = getContext<CollectionState | undefined>(COLLECTION_KEY);
  if (!state) {
    throw new Error('CollectionContext not provided');
  }
  return state;
}
```

### 9.3 Root Layout Integration

```svelte
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import { createApplication } from '$lib/app/composition-root';
  import { provideCollectionContext } from '$lib/features/collection/collection.context';
  import { provideWishlistContext } from '$lib/features/wishlist/wishlist.context';

  const app = createApplication();

  provideCollectionContext(app.collection.state);
  provideWishlistContext(app.wishlist.state);
</script>

<slot />
```

---

## 10. Cross-Platform Considerations (Desktop, Tablet, Mobile)

### 10.1 Architecture Principle

**One application layer, multiple responsive presentations.**

Do not create separate application logic for each platform. Responsive design handles most differences.

### 10.2 Responsive Layouts

Use Tailwind breakpoints and conditional rendering:

```svelte
<script lang="ts">
  import { useCollectionContext } from '../collection.context';

  const collection = useCollectionContext();
  let showSidebar = $state(true);
</script>

<!-- Desktop: sidebar + main content -->
<div class="hidden gap-4 lg:grid lg:grid-cols-4">
  <aside class="lg:col-span-1">
    <CollectionFilters />
  </aside>
  <main class="lg:col-span-3">
    <CollectionGrid />
  </main>
</div>

<!-- Mobile: stacked layout with toggle -->
<div class="lg:hidden">
  <button onclick={() => (showSidebar = !showSidebar)}>Filters</button>
  {#if showSidebar}
    <CollectionFilters />
  {/if}
  <CollectionGrid />
</div>
```

### 10.3 Platform-Specific Infrastructure

If a platform requires different implementations, use adapter pattern:

```typescript
// infrastructure/platform/FileSystemPort.ts
export interface FileSystemPort {
  pickFile(): Promise<File>;
  saveFile(name: string, content: Uint8Array): Promise<void>;
}

// infrastructure/platform/DesktopFileSystem.ts
export class DesktopFileSystem implements FileSystemPort {
  async pickFile() {
    return commands.desktopPickFile();
  }

  async saveFile(name: string, content: Uint8Array) {
    return commands.desktopSaveFile({ name, content });
  }
}

// infrastructure/platform/MobileFileSystem.ts
export class MobileFileSystem implements FileSystemPort {
  async pickFile() {
    return commands.mobilePickFile();
  }

  async saveFile(name: string, content: Uint8Array) {
    return commands.mobileSaveFile({ name, content });
  }
}
```

---

## 11. Implementation Example: Complete Feature

### From Domain to Presentation

**Step 1: Define domain types and rules**

```typescript
// features/collection/domain/collection.types.ts
export interface CollectionItem {
  id: string;
  name: string;
  scale: 'H0' | 'N' | 'TT';
  year?: number;
}

export interface CreateCollectionItemInput {
  name: string;
  scale: 'H0' | 'N' | 'TT';
  year?: number;
}

// features/collection/domain/collection.rules.ts
export function validateItemName(name: string) {
  if (!name.trim()) throw new DomainError('name_required', 'Name is required');
  if (name.length > 255) throw new DomainError('name_too_long', 'Name is too long');
}
```

**Step 2: Define repository interface**

```typescript
// features/collection/application/ports/CollectionRepository.ts
export interface CollectionRepository {
  getAll(): Promise<CollectionItem[]>;
  add(input: CreateCollectionItemInput): Promise<CollectionItem>;
  delete(id: string): Promise<void>;
}
```

**Step 3: Implement use cases**

```typescript
// features/collection/application/use-cases/GetCollection.ts
export class GetCollection {
  constructor(private repository: CollectionRepository) {}

  async execute() {
    return this.repository.getAll();
  }
}

// features/collection/application/use-cases/CreateCollectionItem.ts
export class CreateCollectionItem {
  constructor(private repository: CollectionRepository) {}

  async execute(input: CreateCollectionItemInput) {
    validateItemName(input.name);
    return this.repository.add(input);
  }
}
```

**Step 4: Create state class**

```typescript
// features/collection/application/CollectionState.svelte.ts
export class CollectionState {
  #items = $state<CollectionItem[]>([]);
  #loading = $state(false);
  #error = $state<AppError | null>(null);

  constructor(
    private getCollection: GetCollection,
    private createItem: CreateCollectionItem
  ) {}

  get items() {
    return this.#items;
  }

  get loading() {
    return this.#loading;
  }

  get error() {
    return this.#error;
  }

  async load() {
    this.#loading = true;
    this.#error = null;
    try {
      this.#items = await this.getCollection.execute();
    } catch (err) {
      this.#error = toAppError(err);
    } finally {
      this.#loading = false;
    }
  }

  async addItem(input: CreateCollectionItemInput) {
    this.#error = null;
    try {
      const item = await this.createItem.execute(input);
      this.#items = [...this.#items, item];
      return item;
    } catch (err) {
      this.#error = toAppError(err);
      return null;
    }
  }
}
```

**Step 5: Implement Tauri repository**

```typescript
// features/collection/infrastructure/TauriCollectionRepository.ts
import { commands } from '$lib/infrastructure/tauri/bindings';
import { mapTauriError } from '$lib/infrastructure/tauri/TauriErrorMapper';

export class TauriCollectionRepository implements CollectionRepository {
  async getAll() {
    return commands.getCollection();
  }

  async add(input: CreateCollectionItemInput) {
    try {
      return await commands.createCollectionItem(input);
    } catch (error) {
      throw mapTauriError(error);
    }
  }

  async delete(id: string) {
    try {
      await commands.deleteCollectionItem(id);
    } catch (error) {
      throw mapTauriError(error);
    }
  }
}
```

**Step 6: Create Context provider**

```typescript
// features/collection/collection.context.ts
import { getContext, setContext } from 'svelte';

const KEY = Symbol('collection');

export function provideCollectionContext(state: CollectionState) {
  setContext(KEY, state);
  return state;
}

export function useCollectionContext() {
  return getContext<CollectionState>(KEY);
}
```

**Step 7: Build UI components**

```svelte
<!-- features/collection/presentation/CollectionView.svelte -->
<script lang="ts">
  import { useCollectionContext } from '../collection.context';

  const collection = useCollectionContext();

  collection.load();
</script>

<div class="collection">
  {#if collection.loading}
    <p>Loading...</p>
  {:else if collection.error}
    <p class="text-red-500">{collection.error.message}</p>
  {:else}
    <div class="grid gap-4">
      {#each collection.items as item (item.id)}
        <ItemCard {item} />
      {/each}
    </div>
  {/if}
</div>
```

---

## 12. Implementation Priorities

Implement these changes in phases:

### Phase 1: Establish Boundaries (Week 1-2)

- [ ] Create directory structure (domain, application/ports, application/use-cases, infrastructure, presentation for one feature)
- [ ] Extract repository interfaces from one feature
- [ ] Implement Tauri adapter for that feature
- [ ] Set up ESLint boundaries rules
- [ ] Configure TypeScript strict mode

**Verification:**

- `pnpm svelte-check` passes
- `pnpm lint` passes
- No application-layer imports of Tauri

### Phase 2: Dependency Injection (Week 2-3)

- [ ] Create composition root
- [ ] Inject use cases into state classes
- [ ] Remove direct Tauri calls from state classes
- [ ] Update Context providers
- [ ] Update root layout

**Verification:**

- State classes accept injected dependencies
- No direct Tauri calls outside infrastructure
- Application compiles and runs

### Phase 3: Testing Infrastructure (Week 3-4)

- [ ] Create fake repositories
- [ ] Add unit tests for domain rules
- [ ] Add unit tests for use cases
- [ ] Add unit tests for state classes
- [ ] Configure Vitest + Testing Library

**Verification:**

- `pnpm test` passes
- Domain and application layers testable without Tauri
- Coverage reports show adequate coverage

### Phase 4: Automated Enforcement (Week 4-5)

- [ ] Add ESLint boundary rules to CI
- [ ] Add Specta binding verification to CI
- [ ] Add circular dependency detection
- [ ] Add TypeScript strict mode enforcement
- [ ] Add test coverage gates

**Verification:**

- CI enforces all architectural rules
- No manual enforcement needed in code review

### Phase 5: Extend to All Features (Week 5-6)

- [ ] Migrate remaining features to new architecture
- [ ] Ensure cross-feature communication respects boundaries
- [ ] Add integration tests for multi-feature workflows

**Verification:**

- All features follow the architecture
- No backward compatibility issues

### Phase 6: Documentation & Review (Week 6-7)

- [ ] Update ADRs
- [ ] Document examples in this file
- [ ] Run architecture audit
- [ ] Team code review and training

---

## 13. Key Distinctions: Common Mistakes

### Generated Bindings vs. Repository Interfaces

**Wrong:**

```typescript
// ❌ Application layer importing generated bindings
import { commands } from '$lib/infrastructure/tauri/bindings';

export class GetCollection {
  async execute() {
    return commands.getCollection(); // Direct Tauri call
  }
}
```

**Right:**

```typescript
// ✅ Application defines port; infrastructure implements
export interface CollectionRepository {
  getAll(): Promise<CollectionItem[]>;
}

export class GetCollection {
  constructor(private repository: CollectionRepository) {}

  async execute() {
    return this.repository.getAll();
  }
}
```

**Why:** The application layer must not know Tauri exists. This enables testing without Tauri.

### Domain Logic vs. Frontend-Specific Logic

**Don't duplicate Rust business rules in TypeScript.**

Your Rust domain owns:

- Item validation
- Inventory constraints
- Scale-based rules

Your TypeScript domain owns:

- Selection state (which items are selected?)
- Filter state (how is the list filtered?)
- UI-specific validation (are required fields filled?)

---

## 14. Changes from Previous Version

| Previous Requirement                    | New Requirement                                      |
| --------------------------------------- | ---------------------------------------------------- |
| Feature controller calls Tauri directly | Controller depends on injected use case/port         |
| State classes wrapping Tauri invokes    | State classes coordinate; infrastructure handles IPC |
| Try/catch around every invoke           | Centralized error translation at boundary            |
| Generated bindings used throughout      | Generated bindings isolated to infrastructure        |
| No explicit testing strategy            | Testing pyramid with concrete examples               |
| No dependency injection                 | Explicit composition root and Context injection      |
| ESLint rules optional                   | ESLint boundaries enforced in CI                     |

---

## References

- [Clean Architecture - Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture - Alistair Cockburn](https://alistair.cockburn.us/hexagonal-architecture/)
- [Tauri Specta Documentation](https://specta.dev/docs/tauri-specta)
- [Svelte 5 Runes Documentation](https://svelte.dev/docs/svelte/what-are-runes)
- [Svelte Testing Documentation](https://github.com/sveltejs/svelte/tree/main/packages/svelte/tests)
- [Testing Library Best Practices](https://testing-library.com/docs/queries/about)
- [TypeScript Strict Mode](https://www.typescriptlang.org/tsconfig#strict)
- [ESLint Plugin Boundaries](https://github.com/javierbrey/eslint-plugin-boundaries)

---

**Last Updated:** November 26, 2026
**Version:** 2.0.0 (Normative Architecture Specification)
