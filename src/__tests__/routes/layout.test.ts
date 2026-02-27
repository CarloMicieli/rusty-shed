import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import { createRawSnippet } from 'svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', () => new Proxy({}, { get: (_t, k) => () => String(k) }));

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
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
  themeStore: {
    initializeFromSettings: vi.fn().mockResolvedValue(undefined),
    setTheme: vi.fn().mockResolvedValue(undefined)
  }
}));

// ── App version / version store ──

vi.mock('$lib/stores/app', () => ({
  setAppVersion: vi.fn(),
  appVersion: { subscribe: vi.fn(() => vi.fn()) }
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

// ── Heavy UI children ──

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

describe('routes/+layout.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Remove any leftover #app-loading nodes
    document.getElementById('app-loading')?.remove();
  });

  it('renders without throwing', () => {
    mockSafeInvoke.mockImplementation(() => new Promise(() => {}));
    expect(() => render(Layout)).not.toThrow();
  });

  it('shows a loading spinner during application initialisation', () => {
    mockSafeInvoke.mockImplementation(() => new Promise(() => {})); // never resolves
    const { container } = render(Layout);
    const spinner = container.querySelector('.animate-spin');
    expect(spinner).not.toBeNull();
  });

  it('shows "Rusty Shed" brand during loading', () => {
    mockSafeInvoke.mockImplementation(() => new Promise(() => {}));
    render(Layout);
    expect(screen.getByText('Rusty Shed')).toBeInTheDocument();
  });

  it('shows a "Startup Failed" error when init_database fails', async () => {
    mockSafeInvoke.mockImplementation(async (command: string) => {
      if (command === 'show_main_window') return { ok: true, data: undefined };
      if (command === 'get_app_version') return { ok: true, data: '1.0.0' };
      if (command === 'init_database') return { ok: false, error: { message: 'DB init error' } };
      return { ok: true, data: undefined };
    });

    render(Layout);

    await waitFor(() => expect(screen.getByText('Startup Failed')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('renders the main application shell after successful startup', async () => {
    mockSuccessfulStartup();
    // Provide an empty children snippet so {@render children()} in the main
    // shell doesn't throw invalid_snippet once loading resolves.
    const children = createRawSnippet(() => ({
      render: () => '<span data-testid="layout-slot"></span>',
      setup: () => {}
    }));
    const { container } = render(Layout, { children });
    await waitFor(
      () => {
        expect(container.querySelector('.animate-spin')).toBeNull();
        expect(screen.getByLabelText('Notifications')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it('calls init_database during startup', async () => {
    mockSuccessfulStartup();
    render(Layout);
    await waitFor(() => expect(mockSafeInvoke).toHaveBeenCalledWith('init_database'), {
      timeout: 2000
    });
  });

  it('removes the #app-loading spinner element on mount', async () => {
    const loader = document.createElement('div');
    loader.id = 'app-loading';
    document.body.appendChild(loader);
    mockSuccessfulStartup();
    render(Layout);
    await waitFor(() => expect(document.getElementById('app-loading')).toBeNull(), {
      timeout: 2000
    });
  });

  it('calls settingsState.initialize on mount', async () => {
    const { settingsState } = await import('$lib/features/settings/SettingsState.svelte');
    mockSuccessfulStartup();
    render(Layout);
    await waitFor(() => expect(settingsState.initialize).toHaveBeenCalled(), { timeout: 2000 });
  });
});
