import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockController = vi.hoisted(() => ({
  backups: [] as unknown[],
  isConnected: false,
  isOnline: false,
  restoreBackup: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/features/cloud-backup/controllers/cloudBackup.svelte', () => ({
  getCloudBackupController: () => mockController
}));
vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: { locale: 'en-US' }
}));
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const mocked = Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
  mocked.cloud_backup_restore_confirm_keyword = () => 'RESTORE';
  mocked.cloud_backup_restore_confirm_placeholder = () => 'Type RESTORE';
  return mocked;
});

import BackupList from '$lib/features/cloud-backup/components/BackupList.svelte';
import RestoreConfirmModal from '$lib/features/cloud-backup/components/RestoreConfirmModal.svelte';

describe('cloud backup components', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('prompts the user to connect when disconnected', () => {
    mockController.isConnected = false;
    mockController.isOnline = false;
    mockController.backups = [];
    render(BackupList);

    expect(screen.getByText('cloud_backup_backups_connect_prompt')).toBeInTheDocument();
  });

  it('shows the offline state before backup contents', () => {
    mockController.isConnected = true;
    mockController.isOnline = false;
    render(BackupList);

    expect(screen.getByText('cloud_backup_backups_offline_prompt')).toBeInTheDocument();
  });

  it('shows an empty backup list when connected and online', () => {
    mockController.isConnected = true;
    mockController.isOnline = true;
    mockController.backups = [];
    render(BackupList);

    expect(screen.getByText('cloud_backup_backups_empty')).toBeInTheDocument();
  });

  it('renders backup details and emits the selected backup id', async () => {
    mockController.isConnected = true;
    mockController.isOnline = true;
    mockController.backups = [
      {
        id: 'backup-1',
        createdAt: '2026-11-27T18:00:00Z',
        sizeFormatted: '24 MB',
        recordCount: 1,
        isInitial: true
      },
      {
        id: 'backup-2',
        createdAt: '2026-11-26T18:00:00Z',
        sizeFormatted: '20 MB',
        recordCount: 12,
        isInitial: false
      }
    ];
    const onRestore = vi.fn();
    render(BackupList, { props: { onRestore } });

    expect(screen.getByText('cloud_backup_backups_count_multiple')).toBeInTheDocument();
    expect(screen.getAllByRole('button', { name: 'cloud_backup_restore' })).toHaveLength(2);

    await fireEvent.click(screen.getAllByRole('button', { name: 'cloud_backup_restore' })[0]);
    expect(onRestore).toHaveBeenCalledWith('backup-1');
  });

  it('blocks incorrect restore confirmation text', async () => {
    mockController.restoreBackup.mockResolvedValue(undefined);
    render(RestoreConfirmModal, {
      props: {
        open: true,
        backupId: 'backup-1',
        backupLabel: 'Initial backup'
      }
    });

    const input = screen.getByPlaceholderText('Type RESTORE');
    await fireEvent.input(input, { target: { value: 'wrong' } });
    await fireEvent.click(screen.getByRole('button', { name: 'cloud_backup_restore' }));

    expect(mockController.restoreBackup).not.toHaveBeenCalled();
  });

  it('restores with the exact confirmation keyword and closes', async () => {
    mockController.restoreBackup.mockResolvedValue(undefined);
    const onConfirm = vi.fn();
    render(RestoreConfirmModal, {
      props: {
        open: true,
        backupId: 'backup-1',
        backupLabel: 'Initial backup',
        onConfirm
      }
    });

    const input = screen.getByPlaceholderText('Type RESTORE');
    await fireEvent.input(input, { target: { value: 'RESTORE' } });
    await fireEvent.click(screen.getByRole('button', { name: 'cloud_backup_restore' }));

    expect(mockController.restoreBackup).toHaveBeenCalledWith('backup-1', 'RESTORE');
    expect(onConfirm).toHaveBeenCalledOnce();
  });

  it('surfaces restore failures and keeps the modal open', async () => {
    mockController.restoreBackup.mockRejectedValue(new Error('Restore failed'));
    render(RestoreConfirmModal, {
      props: {
        open: true,
        backupId: 'backup-1',
        backupLabel: 'Initial backup'
      }
    });

    await fireEvent.input(screen.getByPlaceholderText('Type RESTORE'), {
      target: { value: 'RESTORE' }
    });
    await fireEvent.click(screen.getByRole('button', { name: 'cloud_backup_restore' }));

    expect(await screen.findByText('Restore failed')).toBeInTheDocument();
  });
});
