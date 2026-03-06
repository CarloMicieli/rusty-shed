---
name: svelte-test-writer
description: Use this skill when writing Svelte 5 component tests to ensure they are reliable and don't hang due to unresolved promises or improper handling of effects.
---

# 🧪 svelte-test-writer Skill Definition

## Skill Name
**svelte-test-writer**

## Purpose
Write **reliable, non-hanging Svelte 5 component tests** that properly handle Runes (`$state`, `$derived`, `$effect`) and asynchronous operations.

## When to Use This Skill
✅ Writing new Svelte component tests
✅ Fixing failing or hanging Svelte tests
✅ Testing components with effects, onMount, or reactive state
✅ Testing async operations (IPC, data fetching, user interactions)
✅ Setting up mocks for Tauri commands or services

## Constraints & Hard Rules

### Rule 1: All Mocks Must Return Resolved Promises
```typescript
// ✅ CORRECT
vi.fn().mockResolvedValue({ ok: true, data: undefined });
vi.fn().mockImplementation(async (cmd) => ({ ok: true, data: 'value' }));

// ❌ WRONG (causes hangs)
vi.fn().mockImplementation(() => new Promise(() => {}));
const pendingPromise = () => new Promise<never>((resolve) => setTimeout(resolve, 60_000));
```

**Reason:** Svelte 5 effects wait for promises. An unresolved promise blocks the entire test suite.

---

### Rule 2: Never Use `createRawSnippet` for Component Children
```typescript
// ❌ WRONG (memory leaks, hangs)
function createChildrenSnippet() {
  return createRawSnippet(() => ({
    render: () => '<span>Content</span>',
    setup: () => {}
  }));
}
render(Layout, { children: createChildrenSnippet() });

// ✅ CORRECT (wrapper component or plain props)
render(Layout, {
  props: {
    // Pass data as props instead
  }
});
```

**Reason:** Raw snippets bypass Svelte's lifecycle management.

---

### Rule 3: Always Use `await waitFor()` for State Assertions
```typescript
// ❌ WRONG (DOM not updated yet)
mockFetch.mockResolvedValue({ ok: true, data: 'value' });
render(MyComponent);
expect(screen.getByText('value')).toBeInTheDocument(); // Fails!

// ✅ CORRECT (wait for effects to settle)
mockFetch.mockResolvedValue({ ok: true, data: 'value' });
render(MyComponent);
await waitFor(
  () => {
    expect(screen.getByText('value')).toBeInTheDocument();
  },
  { timeout: 2000 }
);
```

**Reason:** Svelte 5 batches updates. The DOM doesn't reflect state changes immediately.

---

### Rule 4: Never Use Raw Proxy Mocks
```typescript
// ❌ WRONG (conflicts with Svelte's reactivity)
vi.mock('$lib/state', () => ({
  myState: new Proxy({}, { get: () => 'value' })
}));

// ✅ CORRECT (plain objects)
vi.mock('$lib/state', () => ({
  myState: {
    value: 'test',
    fetch: vi.fn().mockResolvedValue(undefined)
  }
}));
```

**Reason:** Svelte uses Proxies internally; custom Proxies create conflicts.

---

### Rule 5: Always Call `cleanup()` in `beforeEach`
```typescript
// ✅ REQUIRED
describe('MyComponent', () => {
  beforeEach(() => {
    cleanup(); // CRITICAL
    vi.clearAllMocks();
  });
});
```

**Reason:** Prevents stale mocks, DOM leaks, and test interference.

---

## Process: Writing a Svelte 5 Test

### Step 1: Set Up Mocks with Resolved Promises
```typescript
const mockFetch = vi.fn().mockResolvedValue({
  ok: true,
  data: { id: 1, name: 'Test' }
});
```

### Step 2: Render the Component
```typescript
render(MyComponent, { props: { onFetch: mockFetch } });
```

### Step 3: Wait for State Updates
```typescript
await waitFor(
  () => {
    expect(screen.getByText('Test')).toBeInTheDocument();
  },
  { timeout: 2000 }
);
```

### Step 4: Verify Mock Calls
```typescript
expect(mockFetch).toHaveBeenCalled();
```

---

## Gold Standard Test Template

Always use this structure:

```typescript
import { render, screen, waitFor, cleanup } from '@testing-library/svelte';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MyComponent from './MyComponent.svelte';

describe('MyComponent', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // Mock with resolved promise
  const mockData = vi.fn().mockResolvedValue({
    ok: true,
    data: { name: 'Test' }
  });

  it('renders without throwing', () => {
    expect(() => render(MyComponent)).not.toThrow();
  });

  it('loads and displays data', async () => {
    render(MyComponent, { props: { onFetch: mockData } });

    await waitFor(
      () => {
        expect(screen.getByText('Test')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it('handles errors', async () => {
    mockData.mockRejectedValueOnce(new Error('Network error'));

    render(MyComponent);

    await waitFor(
      () => {
        expect(screen.getByText(/Network error/)).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });
});
```

---

## Pre-Commit Verification Checklist

Before submitting a test, verify:

- [ ] **No unresolved promises** — All mocks use `.mockResolvedValue()` or `async`
- [ ] **No `createRawSnippet`** — For component children, use wrapper components instead
- [ ] **All state assertions use `await waitFor()`** — No synchronous assertions after state changes
- [ ] **No raw Proxy mocks** — Mocks return plain objects
- [ ] **`cleanup()` in `beforeEach`** — Tests don't interfere with each other
- [ ] **Reasonable timeout** — Max 2-5 seconds per test
- [ ] **No test hangs** — If a test hangs, check rules 1, 2, 3
- [ ] **Mock all async calls** — Don't let real Tauri/API calls happen

---

## Debugging Hanging Tests

If a test hangs indefinitely:

1. **Check for unresolved promises:**
   ```bash
   grep -r "new Promise(() => {})" src/__tests__
   grep -r "pendingPromise" src/__tests__
   ```

2. **Check for `createRawSnippet`:**
   ```bash
   grep -r "createRawSnippet" src/__tests__
   ```

3. **Verify all mocks have `.mockResolvedValue()`:**
   ```bash
   grep -r "mockImplementation" src/__tests__ | grep -v "mockResolvedValue\|async"
   ```

4. **Run with verbose output:**
   ```bash
   pnpm test -- --reporter=verbose
   ```

5. **Check `beforeEach` for cleanup:**
   ```bash
   grep -A 3 "beforeEach" src/__tests__ | grep -v cleanup
   ```

---

## Common Patterns

### Pattern: Component with onMount Fetch
```typescript
it('fetches data on mount', async () => {
  mockFetch.mockResolvedValue({ ok: true, data: 'loaded' });
  render(MyComponent);

  await waitFor(
    () => expect(screen.getByText('loaded')).toBeInTheDocument(),
    { timeout: 2000 }
  );
});
```

### Pattern: User Interaction with Async Effect
```typescript
it('saves data on button click', async () => {
  mockSave.mockResolvedValue({ ok: true, data: undefined });

  const { getByRole } = render(MyComponent);
  await userEvent.click(getByRole('button', { name: 'Save' }));

  await waitFor(
    () => expect(mockSave).toHaveBeenCalled(),
    { timeout: 2000 }
  );
});
```

### Pattern: Error Handling
```typescript
it('shows error on fetch failure', async () => {
  mockFetch.mockRejectedValue(new Error('Failed'));
  render(MyComponent);

  await waitFor(
    () => expect(screen.getByText(/Failed/)).toBeInTheDocument(),
    { timeout: 2000 }
  );
});
```

---

## Documentation & Resources

- **Full Guide:** [docs/testing/SVELTE_5_TESTING.md](SVELTE_5_TESTING.md)
- **Test Template:** [docs/testing/test-template.svelte.ts](test-template.svelte.ts)
- **Memory Note:** [.claude/projects/.../memory/svelte-testing.md](../../.claude/projects/-home-carlo-Projects-rusty-shed/memory/svelte-testing.md)
- **Project Tests:** [src/__tests__/](../../src/__tests__/)
- **Svelte 5 Runes:** https://svelte.dev/docs/svelte/runes
- **Testing Library:** https://testing-library.com/docs/svelte-testing-library/intro

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-06 | Initial skill definition for Svelte 5 testing |

---

## Skill Keywords
`svelte`, `testing`, `vitest`, `svelte-5`, `runes`, `effects`, `async-testing`, `mocking`, `test-debugging`

## Skill Status
**Active** — Use for all new Svelte component tests
