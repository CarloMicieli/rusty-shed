import { describe, it, expect, vi, beforeEach } from 'vitest';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import { createRawSnippet } from 'svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn())
}));

// Use the same async importOriginal pattern as other test files.
// A plain Proxy returning a function for *every* key (including ES module
// symbols like Symbol.toStringTag) confuses Vitest's module introspection
// and can cause the worker to hang before any test runs.
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

// Mock tauri-logger to prevent it from attempting `await import('@tauri-apps/plugin-log')`.
// In the test environment __TAURI_INTERNALS__ is defined (set by setup.ts), so
// isTauri=true and every log call would trigger the dynamic plugin import, which
// hangs indefinitely in happy-dom.
vi.mock('$lib/tauri-logger', () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
    trace: vi.fn()
  }
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    success: vi.fn(),
    error: vi.fn(),
    loading: vi.fn(),
    signal: vi.fn(),
    warning: vi.fn()
  }
}));

// ── Settings state ──

vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: {
    initialize: vi.fn().mockResolvedValue(undefined),
    settings: null
  }
}));

// ── Theme store ──

vi.mock('$lib/stores/themeStore.svelte', () => ({
  themeState: {
    initializeFromSettings: vi.fn().mockResolvedValue(undefined),
    setTheme: vi.fn().mockResolvedValue(undefined)
  }
}));

// ── App state store ──

vi.mock('$lib/stores/app.svelte', () => ({
  appState: {
    version: '',
    setVersion: vi.fn()
  }
}));

// ── safeInvoke from $lib/services ──

const mockSafeInvoke = vi.hoisted(() => vi.fn());
vi.mock('$lib/services', () => ({
  safeInvoke: mockSafeInvoke,
  getErrorMessage: vi.fn((e: unknown) => String(e))
}));

// ── Collection store (used in layout onMount) ──

vi.mock('$lib/state/collection.svelte', () => ({
  collectionStore: {
    fetch: vi.fn().mockResolvedValue(undefined),
    collection: null
  }
}));

// ── Feature contexts ──

vi.mock('$lib/features/collection/CollectionState.svelte', () => ({
  collectionState: {},
  createCollectionState: vi.fn(() => ({})),
  setCollectionContext: vi.fn(),
  getCollectionContext: vi.fn(() => ({})),
  availableScales: []
}));

vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  createWishlistState: vi.fn(() => ({
    fetchWishlists: vi.fn().mockResolvedValue(undefined),
    wishlists: []
  })),
  setWishlistContext: vi.fn(),
  getWishlistContext: vi.fn(() => ({ wishlists: [], fetchWishlists: vi.fn() }))
}));

vi.mock('$lib/features/dashboard/DashboardState.svelte', () => ({
  createDashboardState: vi.fn(() => ({})),
  setDashboardContext: vi.fn(),
  getDashboardContext: vi.fn(() => ({}))
}));

const { mockBudgetState, mockFinanceState } = vi.hoisted(() => ({
  mockBudgetState: {
    load: vi.fn().mockResolvedValue(undefined),
    loadMonthlyRecords: vi.fn().mockResolvedValue(undefined)
  },
  mockFinanceState: {
    ensureLoaded: vi.fn().mockResolvedValue(undefined),
    startListening: vi.fn().mockResolvedValue(undefined),
    stopListening: vi.fn()
  }
}));

vi.mock('$lib/features/budget/services/BudgetService.svelte', () => ({
  createBudgetService: vi.fn(() => ({})),
  getBudgetService: vi.fn(() => ({}))
}));

vi.mock('$lib/features/budget/BudgetState.svelte', () => ({
  createBudgetState: vi.fn(() => mockBudgetState),
  getBudgetState: vi.fn(() => mockBudgetState)
}));

vi.mock('$lib/state/finance.svelte', () => ({
  financeState: mockFinanceState
}));

vi.mock('$lib/features/depot/DepotState.svelte', () => ({
  createDepotState: vi.fn(() => ({})),
  setDepotContext: vi.fn(),
  getDepotContext: vi.fn(() => ({}))
}));

vi.mock('$lib/features/track-inventory', () => ({
  TrackInventoryService: class TrackInventoryService {},
  setTrackInventoryContext: vi.fn(),
  getTrackInventoryContext: vi.fn(() => ({}))
}));

vi.mock('$lib/features/export/export.controller.svelte', () => ({
  exportController: {},
  createExportController: vi.fn(() => ({})),
  setExportContext: vi.fn(),
  getExportContext: vi.fn(() => ({}))
}));

// ── Heavy UI children ──

vi.mock('$lib/services/error-id', () => ({
  generateErrorId: vi.fn(() => 'test-error-id')
}));
vi.mock('$lib/components/SidebarNavigation.svelte', () => ({
  default: function SidebarNavigationStub() {}
}));
vi.mock('$lib/components/BottomNavigation.svelte', () => ({
  default: function BottomNavigationStub() {}
}));
vi.mock('$lib/components/SearchBar.svelte', () => ({
  default: function SearchBarStub() {}
}));
vi.mock('$lib/components/ui/sonner', () => ({
  Toaster: function ToasterStub() {}
}));
vi.mock('$lib/features/acquisition/AcquisitionDrawer.svelte', () => ({
  default: function AcquisitionDrawerStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import Layout from '../../routes/+layout.svelte';

// Helper to set up default successful safeInvoke responses
function mockSuccessfulStartup() {
  mockSafeInvoke.mockImplementation(async (command: string) => {
    if (command === 'show_main_window') return { ok: true, data: undefined };
    if (command === 'get_app_version') return { ok: true, data: '1.0.0' };
    if (command === 'init_database') return { ok: true, data: undefined };
    return { ok: true, data: undefined };
  });
}

// Helper to create an empty children snippet
function createChildrenSnippet() {
  return createRawSnippet(() => ({
    render: () => '<span data-testid="layout-slot"></span>',
    setup: () => {}
  }));
}

describe('routes/+layout.svelte', () => {
  beforeEach(() => {
    cleanup(); // Force unmount from previous test before clearing mocks
    vi.clearAllMocks();
    // Remove any leftover #app-loading nodes
    document.getElementById('app-loading')?.remove();

    mockBudgetState.load = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.loadMonthlyRecords = vi.fn().mockResolvedValue(undefined);
    mockFinanceState.ensureLoaded = vi.fn().mockResolvedValue(undefined);
    mockFinanceState.startListening = vi.fn().mockResolvedValue(undefined);
    mockFinanceState.stopListening = vi.fn();
  });

  // A deferred promise that never resolves within the test lifetime, used to
  // keep the component in the "loading" state without flooding the microtask queue.
  const pendingPromise = () => new Promise<never>((resolve) => setTimeout(resolve, 60_000));

  it('renders without throwing', () => {
    mockSafeInvoke.mockImplementation(pendingPromise);
    expect(() => render(Layout, { children: createChildrenSnippet() })).not.toThrow();
  });

  it('shows a loading spinner during application initialisation', () => {
    mockSafeInvoke.mockImplementation(pendingPromise);
    const { container } = render(Layout, { children: createChildrenSnippet() });
    const spinner = container.querySelector('.animate-spin');
    expect(spinner).not.toBeNull();
  });

  it('shows app brand during loading', () => {
    mockSafeInvoke.mockImplementation(pendingPromise);
    render(Layout, { children: createChildrenSnippet() });
    expect(screen.getByText('app_name')).toBeInTheDocument();
  });

  it('shows a "Startup Failed" error when init_database fails', async () => {
    mockSafeInvoke.mockImplementation(async (command: string) => {
      if (command === 'show_main_window') return { ok: true, data: undefined };
      if (command === 'get_app_version') return { ok: true, data: '1.0.0' };
      if (command === 'init_database') return { ok: false, error: { message: 'DB init error' } };
      return { ok: true, data: undefined };
    });

    render(Layout, { children: createChildrenSnippet() });

    // Advance fake timers so the onMount async chain and the out:fade microtask
    // sequence both complete before waitFor starts polling the DOM.
    await vi.advanceTimersByTimeAsync(100);

    await waitFor(() => expect(screen.getByText('signal_failure_headline')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('renders the main application shell after successful startup', async () => {
    mockSuccessfulStartup();
    const { container } = render(Layout, { children: createChildrenSnippet() });
    await waitFor(
      () => {
        expect(container.querySelector('.animate-spin')).toBeNull();
        expect(screen.getByLabelText('notifications_label')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it('calls init_database during startup', async () => {
    mockSuccessfulStartup();
    render(Layout, { children: createChildrenSnippet() });
    await waitFor(() => expect(mockSafeInvoke).toHaveBeenCalledWith('init_database'), {
      timeout: 2000
    });
  });

  it('removes the #app-loading spinner element on mount', async () => {
    const loader = document.createElement('div');
    loader.id = 'app-loading';
    document.body.appendChild(loader);
    mockSuccessfulStartup();
    render(Layout, { children: createChildrenSnippet() });
    await waitFor(() => expect(document.getElementById('app-loading')).toBeNull(), {
      timeout: 2000
    });
  });

  it('calls settingsState.initialize on mount', async () => {
    const { settingsState } = await import('$lib/features/settings/SettingsState.svelte');
    mockSuccessfulStartup();
    render(Layout, { children: createChildrenSnippet() });
    await waitFor(() => expect(settingsState.initialize).toHaveBeenCalled(), { timeout: 2000 });
  });
});
