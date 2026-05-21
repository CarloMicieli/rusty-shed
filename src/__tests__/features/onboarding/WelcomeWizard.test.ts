import { describe, it, expect, vi, beforeEach } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import WelcomeWizard from '$lib/features/onboarding/WelcomeWizard.svelte';

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

const mockSaveOnboardingSettings = vi.hoisted(() => vi.fn());
vi.mock('$lib/services/settings', () => ({
  saveOnboardingSettings: mockSaveOnboardingSettings
}));

const mockRunLocalArchiveImport = vi.hoisted(() => vi.fn());
vi.mock('$lib/services/import/localImport', () => ({
  runLocalArchiveImport: mockRunLocalArchiveImport
}));

const mockCloudBackupController = vi.hoisted(() => ({
  backups: [] as Array<{
    id: string;
    label: string;
    createdAt: string;
    sizeBytes: number;
    sizeFormatted: string;
    recordCount: number;
    isInitial: boolean;
  }>,
  isConnected: false,
  isOnline: true,
  refreshConnectionStatus: vi.fn().mockResolvedValue(undefined),
  connectGoogle: vi.fn().mockResolvedValue(undefined),
  loadBackups: vi.fn().mockResolvedValue(undefined),
  restoreBackup: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/features/cloud-backup', () => ({
  getCloudBackupController: () => mockCloudBackupController
}));

vi.mock('$lib/features/cloud-backup/components/RestoreConfirmModal.svelte', () => ({
  default: function RestoreConfirmModalStub() {}
}));

describe('WelcomeWizard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockSaveOnboardingSettings.mockResolvedValue({ ok: true, data: {} });
    mockRunLocalArchiveImport.mockResolvedValue({ ok: true });
    mockCloudBackupController.backups = [];
    mockCloudBackupController.isConnected = false;
    mockCloudBackupController.isOnline = true;
    mockCloudBackupController.refreshConnectionStatus.mockResolvedValue(undefined);
    mockCloudBackupController.connectGoogle.mockResolvedValue(undefined);
    mockCloudBackupController.loadBackups.mockResolvedValue(undefined);
  });

  it('supports Step 1 language and theme interactions', async () => {
    render(WelcomeWizard, { onComplete: vi.fn() });

    const languageTrigger = screen.getByLabelText('settings_language_label');
    expect(languageTrigger).toHaveTextContent('settings_language_option_english');

    await fireEvent.click(screen.getByText('settings_theme_light'));
    await fireEvent.click(screen.getByText('onboarding_continue'));

    await waitFor(() => expect(mockSaveOnboardingSettings).toHaveBeenCalled(), { timeout: 2000 });
  });

  it('supports Step 2 keyboard selection flow', async () => {
    render(WelcomeWizard, { onComplete: vi.fn() });

    await fireEvent.click(screen.getByText('onboarding_continue'));
    await waitFor(() => expect(screen.getByText('onboarding_step_2_title')).toBeInTheDocument(), {
      timeout: 2000
    });

    const [scaleGroup] = screen.getAllByRole('radiogroup');
    await fireEvent.keyDown(scaleGroup, { key: 'ArrowRight' });
    await fireEvent.keyDown(scaleGroup, { key: 'ArrowRight' });

    await fireEvent.click(screen.getByText('onboarding_continue'));
    await waitFor(() => expect(screen.getByText('onboarding_step_3_title')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('locks import actions while local import is running', async () => {
    const deferred = new Promise<{ ok: boolean }>((resolve) => {
      setTimeout(() => resolve({ ok: true }), 30);
    });
    mockRunLocalArchiveImport.mockReturnValue(deferred);

    render(WelcomeWizard, { onComplete: vi.fn() });
    await fireEvent.click(screen.getByText('onboarding_continue'));
    await fireEvent.click(screen.getByText('onboarding_continue'));

    const localButton = screen.getByText('onboarding_import_local');
    await fireEvent.click(localButton);

    expect(localButton.closest('button')).toBeDisabled();
    await waitFor(() => expect(mockRunLocalArchiveImport).toHaveBeenCalled(), { timeout: 2000 });
  });

  it('opens the cloud backup picker when restore from Google Drive is selected', async () => {
    mockCloudBackupController.backups = [
      {
        id: 'backup-1',
        label: 'Latest Backup',
        createdAt: '2026-05-21T10:00:00Z',
        sizeBytes: 1024,
        sizeFormatted: '1 KB',
        recordCount: 10,
        isInitial: false
      }
    ];
    mockCloudBackupController.isConnected = true;

    render(WelcomeWizard, { onComplete: vi.fn() });
    await fireEvent.click(screen.getByText('onboarding_continue'));
    await fireEvent.click(screen.getByText('onboarding_continue'));

    await fireEvent.click(screen.getByText('onboarding_import_drive'));

    await waitFor(
      () => expect(screen.getByText('cloud_backup_backups_title')).toBeInTheDocument(),
      {
        timeout: 2000
      }
    );
  });

  it('applies transition classes to the step panel container', () => {
    const { container } = render(WelcomeWizard, { onComplete: vi.fn() });
    const panel = container.querySelector('section');

    expect(panel).not.toBeNull();
    expect(panel).toHaveClass('transition-transform');
    expect(panel).toHaveClass('duration-150');
    expect(panel).toHaveClass('ease-out');
  });
});
