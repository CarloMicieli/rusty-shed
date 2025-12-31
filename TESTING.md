# Frontend Testing Guide — rusty-shed (Tauri + SvelteKit)

## Overview

This project uses **Vitest** for unit and integration testing of the SvelteKit frontend. Tests run in a Node environment with **happy-dom** (not a real browser), focusing on business logic, Svelte stores, and component behavior without E2E overhead.

## Quick Start

```bash
# Run tests in watch mode (development)
pnpm test:unit

# Run tests once (CI mode)
pnpm test

# Run tests with coverage report
pnpm test:coverage
```

## Test Architecture

### Directory Structure

```
src/
  __tests__/
    setup.ts                    # Global test setup, Tauri & SvelteKit mocks
    mocks/
      tauri.ts                  # Mock Tauri IPC (invoke) with delay/error helpers
      sveltekit.ts              # Mock $app/* modules (goto, page, etc.)
      toaster.ts                # Mock toast notifications
    lib/
      services/
        tauri.test.ts           # Tests for safeInvoke wrapper
        errors.test.ts          # Tests for error utilities
      stores/
        dashboardStore.test.ts  # Store logic tests
        collectionStore.test.ts # Store with optimistic updates
      components/
        AddWishlistItemModal.test.ts  # Component integration tests
```

## Testing Patterns for Tauri 2 Apps

### 1. **Mock Boundaries at IPC Layer**

✅ **Good:** Mock `invoke` from `@tauri-apps/api/core` to isolate frontend from Rust backend.

```typescript
// In test file
import { tauriMock } from '../../mocks/tauri';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: tauriMock.getInvokeMock()
}));

// Setup test
tauriMock.mockCommand('list_collection_items', mockData);
const result = await collectionStore.fetchCollection();
```

❌ **Avoid:** Trying to test actual Tauri IPC calls (requires running Tauri app).

### 2. **Use `safeInvoke` Wrapper for Error Handling**

All stores and components use `safeInvoke<T>(command, args)` which returns `SafeResult<T>`:

```typescript
type SafeResult<T> = 
  | { ok: true; data: T } 
  | { ok: false; error: NormalizedError };
```

**Benefits:**
- Consistent error shape across all Tauri commands
- Easy to test success/failure paths
- Type-safe responses

**Example Test:**

```typescript
it('should handle errors gracefully', async () => {
  const error = { DatabaseError: 'Connection failed' };
  tauriMock.mockCommandError('save_item', error);

  const result = await safeInvoke('save_item', { id: '123' });

  expect(result.ok).toBe(false);
  if (!result.ok) {
    expect(result.error.kind).toBe('database');
    expect(result.error.message).toBe('Connection failed');
  }
});
```

### 3. **Pure `$derived` Computations (Svelte 5)**

Svelte 5 uses runes (`$state`, `$derived`, `$effect`). For logic tests (.svelte.ts files), `$derived` values update **synchronously** when underlying `$state` changes.

```typescript
// Store with reactive logic
class CollectionStore {
  rawItems = $state<Item[]>([]);
  filters = $state({ query: '', scale: null });

  filteredItems = $derived.by(() => {
    return this.rawItems.filter(item => {
      // filtering logic
    });
  });
}

// Test derived values synchronously
it('should filter items by query', () => {
  collectionStore.rawItems = mockItems;
  collectionStore.setQuery('ICE');

  // Assert immediately - $derived updates synchronously
  expect(collectionStore.filteredItems).toHaveLength(1);
  expect(collectionStore.filteredItems[0].title).toBe('ICE 3');
});
```

**Key Point:** No need for `await` or `waitFor` when testing pure `$derived` logic in stores.

### 4. **Snapshot Isolation for Optimistic Updates**

Stores implement optimistic updates with snapshot/revert pattern to handle race conditions:

```typescript
// Store pattern
createItem = async (input: Input) => {
  const snapshot = [...this.rawItems];
  const tempItem = { id: 'temp-123', ...input };

  // Optimistic update
  this.rawItems = [...this.rawItems, tempItem];

  const result = await safeInvoke('create_item', { input });

  if (!result.ok) {
    // Revert on error
    this.rawItems = snapshot;
    return null;
  }

  // Replace temp with real item
  this.rawItems = this.rawItems.map(i => 
    i.id === tempItem.id ? result.data : i
  );
  return result.data;
};
```

**Test Pattern (4-Step):**

```typescript
it('should optimistically add item and revert on error', async () => {
  const error = { ValidationError: { name: 'Required' } };
  tauriMock.mockCommandErrorWithDelay('create_item', 50, error);

  // 1. Trigger action
  const createPromise = collectionStore.createItem(input);

  // 2. Assert optimistic state immediately
  expect(collectionStore.rawItems.length).toBe(1);
  expect(collectionStore.rawItems[0].id).toMatch(/^temp-/);

  // 3. Resolve/reject mock
  const result = await createPromise;

  // 4. Assert final state (revert)
  expect(result).toBeNull();
  expect(collectionStore.rawItems).toEqual([]);
});
```

**Delay Helpers:**

```typescript
// Mock command that succeeds after delay
tauriMock.mockCommandWithDelay('save_item', 50, successData);

// Mock command that fails after delay
tauriMock.mockCommandErrorWithDelay('save_item', 50, errorData);
```

### 5. **Prop-Driven Component Design**

Components should receive data via props and emit events, minimizing direct store access in component logic.

✅ **Testable Component:**

```svelte
<script lang="ts">
  let { items = [], onDelete } = $props();
</script>

{#each items as item}
  <button onclick={() => onDelete(item.id)}>Delete</button>
{/each}
```

**Test:**

```typescript
it('should call onDelete with correct ID', async () => {
  const onDelete = vi.fn();
  const user = userEvent.setup();

  render(ItemList, { props: { items: mockItems, onDelete } });

  const button = screen.getByRole('button', { name: /delete/i });
  await user.click(button);

  expect(onDelete).toHaveBeenCalledWith('item-1');
});
```

❌ **Hard to Test:** Components that directly import and mutate stores make it difficult to isolate behavior.

## Testing Svelte 5 Components

### DOM Updates: Use `waitFor` and `userEvent`

For component tests, DOM updates are **not synchronous**. Use `@testing-library/svelte` utilities:

```typescript
import { render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

it('should update UI after async operation', async () => {
  const user = userEvent.setup();
  tauriMock.mockCommand('save_data', { success: true });

  render(MyComponent);

  const button = screen.getByRole('button', { name: /save/i });
  await user.click(button);

  // Wait for DOM to update after async operation
  await waitFor(() => {
    expect(screen.getByText('Saved successfully')).toBeInTheDocument();
  });
});
```

**Prefer:**
- `await user.click(...)` over `fireEvent.click(...)`
- `waitFor(() => expect(...))` for async DOM updates
- Direct assertions for synchronous state changes in stores

**Avoid:**
- Manual `flushSync()` calls (use `waitFor` instead)
- `fireEvent` (use `userEvent` for better browser simulation)

## Mocking Strategy

### Tauri Commands

```typescript
// Success case
tauriMock.mockCommand('get_item', { id: '123', name: 'Test' });

// Error case
tauriMock.mockCommandError('get_item', { NotFound: 'Item not found' });

// With delay (for race conditions)
tauriMock.mockCommandWithDelay('slow_operation', 100, resultData);

// Parameterized response
tauriMock.mockCommand('get_item_by_id', (args) => {
  return { id: args.id, name: `Item ${args.id}` };
});
```

### SvelteKit Modules

```typescript
// Mock navigation
import { goto } from '$app/navigation';

await user.click(linkButton);
expect(goto).toHaveBeenCalledWith('/my-collection');

// Mock page store
import { page } from '$app/stores';

page.set({
  url: new URL('http://localhost/collection?scale=H0'),
  params: { id: '123' },
  // ... other fields
});
```

### Toaster Notifications

```typescript
import { toaster } from '$lib/toaster';

await collectionStore.createItem(input);

expect(toaster.success).toHaveBeenCalledWith(
  expect.objectContaining({ title: 'Success' })
);
```

## Best Practices

### ✅ Do

- **Reset mocks between tests:** Use `beforeEach(() => { tauriMock.reset(); vi.clearAllMocks(); })`
- **Test happy path first:** Success cases establish baseline behavior
- **Test error paths:** Verify UI handles errors gracefully
- **Assert synchronously for stores:** `$state` and `$derived` update immediately in .svelte.ts files
- **Use `waitFor` for components:** DOM updates are async
- **Test optimistic updates:** Verify UI updates before AND after backend response

### ❌ Don't

- **Mock internal implementation details:** Mock at boundaries (IPC, navigation, toaster)
- **Test Svelte compiler output:** Focus on behavior, not how Svelte renders
- **Use real Tauri backend in tests:** Always mock `invoke` calls
- **Write browser/E2E tests here:** This suite is for unit/integration only
- **Assert on loading spinners:** Test loading state via store properties, not DOM elements

## Running Tests

### Watch Mode (Development)

```bash
pnpm test:unit
```

- Runs tests on file changes
- Shows only failed tests after initial run
- Fastest feedback loop

### CI Mode

```bash
pnpm test:ci
```

- Runs all tests once
- Exits with non-zero code on failure
- Use in GitHub Actions / CI pipelines

### Coverage

```bash
pnpm test:coverage
```

- Generates coverage report in `coverage/` directory
- Opens HTML report in browser
- Excludes: `node_modules/`, `src/__tests__/`, `**/*.d.ts`, generated files

## Troubleshooting

### Tests fail with "Unmocked Tauri command"

**Cause:** Test is calling `invoke` without a corresponding mock.

**Fix:** Add mock before the test runs:

```typescript
tauriMock.mockCommand('missing_command', mockResponse);
```

### `$derived` values not updating in tests

**Cause:** Likely testing a component (DOM), not store logic.

**Fix:** For components, use `waitFor`:

```typescript
await waitFor(() => {
  expect(screen.getByText('Updated Value')).toBeInTheDocument();
});
```

For stores, ensure you're calling methods that trigger state changes:

```typescript
collectionStore.setQuery('test'); // Triggers $derived update
expect(collectionStore.filteredItems).toHaveLength(1); // Assert immediately
```

### Mock not being used (test calls real implementation)

**Cause:** Mock is defined after the import.

**Fix:** Define mocks at the top of the file before imports, or use `vi.mock()` hoisting:

```typescript
vi.mock('@tauri-apps/api/core', () => ({
  invoke: tauriMock.getInvokeMock()
}));

// Then import modules that use invoke
import { myStore } from '$lib/stores/myStore';
```

### Component tests fail with "Cannot find module '$app/...'"

**Cause:** SvelteKit modules not mocked or alias not configured.

**Fix:** Check `vitest.config.ts` has correct alias:

```typescript
resolve: {
  alias: {
    $lib: resolve(__dirname, './src/lib'),
    $app: resolve(__dirname, './src/__tests__/mocks/sveltekit')
  }
}
```

## Related Documentation

- [Vitest Documentation](https://vitest.dev/)
- [Testing Library Svelte](https://testing-library.com/docs/svelte-testing-library/intro)
- [Tauri Testing Guide](https://v2.tauri.app/develop/tests/)
- [Svelte 5 Runes](https://svelte.dev/docs/svelte/what-are-runes)

---

**Last Updated:** December 31, 2025
