import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: vi.fn(() => 'en'),
  setLocale: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
}));

const { mockFetchSettings, mockSaveSettings } = vi.hoisted(() => ({
  mockFetchSettings: vi.fn(),
  mockSaveSettings: vi.fn()
}));

vi.mock('$lib/services', () => ({
  fetchSettings: mockFetchSettings,
  saveSettings: mockSaveSettings,
  getErrorMessage: vi.fn((e: unknown) => String(e)),
  safeInvoke: vi.fn()
}));

vi.mock('$lib/services/errors', () => ({
  getToastMessage: vi.fn((e: unknown) => String(e))
}));

vi.mock('$lib/stores/locale', () => ({
  setActiveLocale: vi.fn()
}));

vi.mock('$lib/stores/themeStore.svelte', () => ({
  themeStore: {
    setTheme: vi.fn().mockResolvedValue(undefined),
    initializeFromSettings: vi.fn().mockResolvedValue(undefined)
  }
}));

const mockCloudBackupController = vi.hoisted(() => ({
  isConnected: false,
  lastSyncAt: null,
  backups: [] as unknown[],
  connect: vi.fn(),
  disconnect: vi.fn(),
  sync: vi.fn(),
  fetchBackups: vi.fn()
}));

vi.mock('$lib/features/cloud-backup', () => ({
  getCloudBackupController: vi.fn(() => mockCloudBackupController)
}));

// Stub complex child components
vi.mock('$lib/components/SettingsForm.svelte', () => ({
  default: function SettingsFormStub() {}
}));
vi.mock('$lib/features/cloud-backup/components/GoogleConnectButton.svelte', () => ({
  default: function GoogleConnectButtonStub() {}
}));
vi.mock('$lib/features/cloud-backup/components/ConnectivityIndicator.svelte', () => ({
  default: function ConnectivityIndicatorStub() {}
}));
vi.mock('$lib/features/cloud-backup/components/SyncButton.svelte', () => ({
  default: function SyncButtonStub() {}
}));
vi.mock('$lib/features/cloud-backup/components/BackupList.svelte', () => ({
  default: function BackupListStub() {}
}));
vi.mock('$lib/features/cloud-backup/components/RestoreConfirmModal.svelte', () => ({
  default: function RestoreConfirmModalStub() {}
}));
vi.mock('$lib/features/database-backup/components/DataManagementSection.svelte', () => ({
  default: function DataManagementSectionStub() {}
}));
vi.mock('$lib/components/PageHeader.svelte', () => ({
  default: function PageHeaderStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import SettingsPage from '../../routes/settings/+page.svelte';

const MOCK_SETTINGS = {
  currency: 'EUR',
  language: 'en',
  theme: 'steampunk-dark',
  measureUnit: 'Metric',
  favouriteScale: 'HO',
  powerSystem: 'DC'
};

describe('routes/settings/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders without throwing', () => {
    mockFetchSettings.mockImplementation(() => new Promise(() => {}));
    expect(() => render(SettingsPage)).not.toThrow();
  });

  it('shows a loading skeleton while settings are being fetched', () => {
    mockFetchSettings.mockImplementation(() => new Promise(() => {}));
    const { container } = render(SettingsPage);
    const loadingElement = container.querySelector('.animate-pulse');
    expect(loadingElement).not.toBeNull();
  });

  it('shows an error message when fetchSettings fails', async () => {
    mockFetchSettings.mockResolvedValue({ ok: false, error: 'Failed to load settings' });

    render(SettingsPage);

    await waitFor(() => {
      expect(screen.getByText('errors_retry_page')).toBeInTheDocument();
    });
  });

  it('renders the settings area after a successful load', async () => {
    mockFetchSettings.mockResolvedValue({ ok: true, data: MOCK_SETTINGS });

    render(SettingsPage);

    await waitFor(() => {
      // SettingsForm is rendered (stubbed). We verify the cloud backup section
      // heading is visible which is rendered directly in this route.
      expect(screen.getByText('cloud_backup_title')).toBeInTheDocument();
    });
  });

  it('calls fetchSettings on mount', async () => {
    mockFetchSettings.mockResolvedValue({ ok: true, data: MOCK_SETTINGS });

    render(SettingsPage);

    await waitFor(() => {
      expect(mockFetchSettings).toHaveBeenCalledOnce();
    });
  });
});
