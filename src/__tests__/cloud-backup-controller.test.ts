import { beforeEach, describe, expect, it, vi } from 'vitest';

const listenCallbacks = vi.hoisted(() => new Map<string, (event: { payload: unknown }) => void>());
const listenMock = vi.hoisted(() => vi.fn());
const commandMocks = vi.hoisted(() => ({
  cloudBackupConnectGoogle: vi.fn(),
  cloudBackupDisconnectGoogle: vi.fn(),
  cloudBackupSyncNow: vi.fn(),
  cloudBackupListBackups: vi.fn(),
  cloudBackupRestore: vi.fn(),
  cloudBackupGetConnectionStatus: vi.fn(),
  cloudBackupCheckConnectivity: vi.fn()
}));
const connectivityMocks = vi.hoisted(() => ({
  setConnectivityOffline: vi.fn(),
  setConnectivityStatus: vi.fn()
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: listenMock.mockImplementation(
    async (eventName: string, callback: (event: { payload: unknown }) => void) => {
      listenCallbacks.set(eventName, callback);
      return vi.fn();
    }
  )
}));
vi.mock('$lib/bindings', () => ({ commands: commandMocks }));
vi.mock('$lib/features/cloud-backup/stores/connectivity', () => connectivityMocks);
vi.mock('$lib/paraglide/messages.js', () => ({
  cloud_backup_syncing: () => 'Syncing',
  cloud_backup_sync_success: () => 'Sync complete',
  cloud_backup_sync_failed: ({ error }: { error: string }) => `Sync failed: ${error}`,
  cloud_backup_error_not_connected: () => 'Not connected',
  cloud_backup_offline: () => 'Offline',
  cloud_backup_restore_confirm_error: () => 'Enter RESTORE',
  cloud_backup_restore_confirm_keyword: () => 'RESTORE'
}));

import { CloudBackupController } from '$lib/features/cloud-backup/controllers/cloudBackup.svelte';

describe('CloudBackupController', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    listenCallbacks.clear();
    listenMock.mockImplementation(
      async (eventName: string, callback: (event: { payload: unknown }) => void) => {
        listenCallbacks.set(eventName, callback);
        return vi.fn();
      }
    );
    commandMocks.cloudBackupGetConnectionStatus.mockResolvedValue({
      isConnected: false,
      email: null,
      connectedAt: null,
      lastSyncAt: null
    });
    commandMocks.cloudBackupCheckConnectivity.mockResolvedValue({
      isOnline: true,
      checkedAt: '2026-11-27T18:00:00Z'
    });
  });

  it('connects Google and stores the connection status', async () => {
    commandMocks.cloudBackupConnectGoogle.mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: '2026-11-27T18:00:00Z',
      lastSyncAt: null
    });
    const controller = new CloudBackupController();

    await controller.connectGoogle();

    expect(controller.isConnected).toBe(true);
    expect(controller.userEmail).toBe('person@example.com');
    expect(controller.isConnecting).toBe(false);
  });

  it('stores connection failures and rethrows them', async () => {
    commandMocks.cloudBackupConnectGoogle.mockRejectedValue(new Error('OAuth failed'));
    const controller = new CloudBackupController();

    await expect(controller.connectGoogle()).rejects.toThrow('OAuth failed');
    expect(controller.error).toBe('OAuth failed');
    expect(controller.isConnecting).toBe(false);
  });

  it('disconnects, clears the account, and clears backups', async () => {
    commandMocks.cloudBackupGetConnectionStatus.mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: '2026-11-27T18:00:00Z',
      lastSyncAt: null
    });
    commandMocks.cloudBackupListBackups.mockResolvedValue({
      backups: [{ id: 'backup-1' }]
    });
    commandMocks.cloudBackupDisconnectGoogle.mockResolvedValue(undefined);
    const controller = new CloudBackupController();
    await controller.refreshConnectionStatus();

    await controller.disconnectGoogle();

    expect(controller.isConnected).toBe(false);
    expect(controller.userEmail).toBeNull();
    expect(controller.backups).toEqual([]);
  });

  it('does not disconnect when already disconnected', async () => {
    const controller = new CloudBackupController();

    await controller.disconnectGoogle();

    expect(commandMocks.cloudBackupDisconnectGoogle).not.toHaveBeenCalled();
  });

  it('syncs, updates timestamps/progress, and reloads backups', async () => {
    commandMocks.cloudBackupGetConnectionStatus.mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: null,
      lastSyncAt: null
    });
    commandMocks.cloudBackupSyncNow.mockResolvedValue({
      id: 'sync-1',
      createdAt: '2026-11-27T18:05:00Z'
    });
    commandMocks.cloudBackupListBackups.mockResolvedValue({ backups: [{ id: 'backup-1' }] });
    const controller = new CloudBackupController();
    await controller.refreshConnectionStatus();

    await controller.syncNow();

    expect(controller.lastSyncAt).toBe('2026-11-27T18:05:00Z');
    expect(controller.syncProgress).toBe(100);
    expect(controller.syncStatusMessage).toBe('Sync complete');
    expect(controller.backups).toEqual([{ id: 'backup-1' }]);
    expect(controller.isSyncing).toBe(false);
  });

  it('does not sync while offline or disconnected', async () => {
    const controller = new CloudBackupController();

    await controller.syncNow();

    expect(commandMocks.cloudBackupSyncNow).not.toHaveBeenCalled();
  });

  it('records sync failures and leaves progress at zero', async () => {
    commandMocks.cloudBackupGetConnectionStatus.mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: null,
      lastSyncAt: null
    });
    commandMocks.cloudBackupSyncNow.mockRejectedValue(new Error('Drive unavailable'));
    const controller = new CloudBackupController();
    await controller.refreshConnectionStatus();

    await controller.syncNow();

    expect(controller.error).toBe('Drive unavailable');
    expect(controller.syncStatusMessage).toBe('Sync failed: Drive unavailable');
    expect(controller.syncProgress).toBe(0);
  });

  it('loads backups only while connected and online', async () => {
    commandMocks.cloudBackupGetConnectionStatus.mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: null,
      lastSyncAt: null
    });
    commandMocks.cloudBackupListBackups.mockResolvedValue({
      backups: [{ id: 'backup-1' }, { id: 'backup-2' }]
    });
    const controller = new CloudBackupController();
    await controller.refreshConnectionStatus();

    await controller.loadBackups();

    expect(controller.backups).toHaveLength(2);
  });

  it('validates restore connection, connectivity, and confirmation', async () => {
    const disconnected = new CloudBackupController();
    await expect(disconnected.restoreBackup('backup-1', 'RESTORE')).rejects.toThrow(
      'Not connected'
    );

    const offline = new CloudBackupController();
    vi.mocked(commandMocks.cloudBackupGetConnectionStatus).mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: null,
      lastSyncAt: null
    });
    vi.mocked(commandMocks.cloudBackupCheckConnectivity).mockResolvedValue({
      isOnline: false,
      checkedAt: '2026-11-27T18:00:00Z'
    });
    await offline.checkConnectivity();
    await offline.refreshConnectionStatus();
    await expect(offline.restoreBackup('backup-1', 'RESTORE')).rejects.toThrow('Offline');

    vi.mocked(commandMocks.cloudBackupCheckConnectivity).mockResolvedValue({
      isOnline: true,
      checkedAt: '2026-11-27T18:00:00Z'
    });
    await offline.checkConnectivity();
    await expect(offline.restoreBackup('backup-1', 'wrong')).rejects.toThrow('Enter RESTORE');
  });

  it('restores a backup and stores restore command failures', async () => {
    commandMocks.cloudBackupGetConnectionStatus.mockResolvedValue({
      isConnected: true,
      email: 'person@example.com',
      connectedAt: null,
      lastSyncAt: null
    });
    commandMocks.cloudBackupRestore
      .mockResolvedValueOnce(undefined)
      .mockRejectedValueOnce(new Error('Restore failed'));
    const controller = new CloudBackupController();
    await controller.refreshConnectionStatus();

    await expect(controller.restoreBackup('backup-1', 'RESTORE')).resolves.toBeUndefined();
    await expect(controller.restoreBackup('backup-2', 'RESTORE')).rejects.toThrow('Restore failed');
    expect(controller.error).toBe('Restore failed');
  });

  it('handles connectivity and sync-progress events', async () => {
    const controller = new CloudBackupController();
    await vi.waitFor(() => expect(listenCallbacks.size).toBeGreaterThanOrEqual(3));

    listenCallbacks.get('cloud-backup://connectivity-changed')?.({
      payload: { isOnline: false, checkedAt: '2026-11-27T18:10:00Z' }
    });
    listenCallbacks.get('cloud-backup://sync-progress')?.({
      payload: { operationId: 'sync-1', progressPercent: 42, stage: 'Uploading' }
    });

    expect(controller.isOnline).toBe(false);
    expect(controller.syncProgress).toBe(42);
    expect(controller.syncStatusMessage).toBe('Uploading');
  });

  it('clears errors', () => {
    const controller = new CloudBackupController();
    controller.clearError();
    expect(controller.error).toBeNull();
  });
});
