import { describe, it, expect, vi, beforeEach } from 'vitest';
import { cleanup, render, screen, waitFor, fireEvent } from '@testing-library/svelte';
import { createRawSnippet } from 'svelte';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn())
}));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

vi.mock('$lib/tauri-logger', () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
    trace: vi.fn()
  }
}));

const mockSafeInvoke = vi.hoisted(() => vi.fn());
vi.mock('$lib/services', () => ({
  safeInvoke: mockSafeInvoke,
  getErrorMessage: vi.fn((e: unknown) => String(e))
}));

vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: {
    initialize: vi.fn().mockResolvedValue(undefined),
    markOnboardingCompleted: vi.fn().mockResolvedValue(undefined),
    settings: {
      currency: 'EUR',
      language: 'en',
      theme: 'steampunk-dark',
      measureUnit: 'Metric',
      favouriteScale: '',
      powerMethod: 'DC',
      has_completed_onboarding: false
    }
  }
}));

vi.mock('$lib/stores/themeStore.svelte', () => ({
  themeState: {
    initializeFromSettings: vi.fn().mockResolvedValue(undefined),
    setTheme: vi.fn().mockResolvedValue(undefined)
  }
}));

vi.mock('$lib/stores/app.svelte', () => ({
  appState: {
    version: '',
    setVersion: vi.fn()
  }
}));

vi.mock('$lib/state/collection.svelte', () => ({
  collectionStore: {
    fetch: vi.fn().mockResolvedValue(undefined),
    collection: null
  }
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

vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  createWishlistState: vi.fn(() => ({
    fetchWishlists: vi.fn().mockResolvedValue(undefined),
    wishlists: []
  })),
  setWishlistContext: vi.fn()
}));

vi.mock('$lib/features/dashboard/DashboardState.svelte', () => ({
  createDashboardState: vi.fn(() => ({})),
  setDashboardContext: vi.fn(),
  getDashboardContext: vi.fn(() => ({
    load: vi.fn().mockResolvedValue(undefined),
    loadBudget: vi.fn().mockResolvedValue(undefined)
  }))
}));

vi.mock('$lib/features/budget/services/BudgetService.svelte', () => ({
  createBudgetService: vi.fn(() => ({}))
}));

vi.mock('$lib/features/budget/BudgetState.svelte', () => ({
  createBudgetState: vi.fn(() => mockBudgetState)
}));

vi.mock('$lib/state/finance.svelte', () => ({
  financeState: mockFinanceState
}));

vi.mock('$lib/features/depot/DepotState.svelte', () => ({
  createDepotState: vi.fn(() => ({})),
  setDepotContext: vi.fn()
}));

vi.mock('$lib/features/track-inventory', () => ({
  TrackInventoryService: class TrackInventoryService {},
  setTrackInventoryContext: vi.fn()
}));

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
vi.mock('$lib/features/wishlists/AddWishlistItemDrawer.svelte', () => ({
  default: function AddWishlistItemDrawerStub() {}
}));
vi.mock('$lib/features/maintenance/components/LogMaintenanceDrawer.svelte', () => ({
  default: function LogMaintenanceDrawerStub() {}
}));

import Layout from '../../../routes/+layout.svelte';

function createChildrenSnippet() {
  return createRawSnippet(() => ({
    render: () => '<span data-testid="layout-slot"></span>',
    setup: () => {}
  }));
}

describe('onboarding gate integration', () => {
  beforeEach(async () => {
    cleanup();
    vi.clearAllMocks();

    const { settingsState } = await import('$lib/features/settings/SettingsState.svelte');
    settingsState.initialize = vi.fn().mockResolvedValue(undefined);
    settingsState.settings.has_completed_onboarding = false;

    mockSafeInvoke.mockImplementation(async (command: string) => {
      if (command === 'show_main_window') return { ok: true, data: undefined };
      if (command === 'get_app_version') return { ok: true, data: '1.0.0' };
      if (command === 'init_database') return { ok: true, data: undefined };
      return { ok: true, data: undefined };
    });
  });

  it('clicking Skip and Start Fresh marks onboarding complete and loads main shell', async () => {
    const { settingsState } = await import('$lib/features/settings/SettingsState.svelte');

    render(Layout, { children: createChildrenSnippet() });

    await waitFor(() => expect(screen.getByText('onboarding_title')).toBeInTheDocument(), {
      timeout: 2000
    });

    await fireEvent.click(screen.getByText('onboarding_continue'));
    await waitFor(() => expect(screen.getByText('onboarding_step_2_title')).toBeInTheDocument(), {
      timeout: 2000
    });

    await fireEvent.click(screen.getByText('onboarding_continue'));
    await waitFor(() => expect(screen.getByText('onboarding_step_3_title')).toBeInTheDocument(), {
      timeout: 2000
    });
    await fireEvent.click(screen.getByText('onboarding_skip_start_fresh'));

    await waitFor(() => expect(settingsState.markOnboardingCompleted).toHaveBeenCalled(), {
      timeout: 2000
    });

    await waitFor(() => expect(screen.getByTestId('layout-slot')).toBeInTheDocument(), {
      timeout: 2000
    });
  });
});
