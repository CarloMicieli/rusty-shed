/**
 * Cloud Backup Feature Controller
 *
 * Manages Google Drive cloud backup state and operations.
 * Follows the controller pattern established in the codebase.
 */

import { listen } from '@tauri-apps/api/event';
import { commands } from '$lib/bindings';
import type { ConnectionStatusResponse, BackupListItem } from '$lib/bindings';
import { setConnectivityOffline, setConnectivityStatus } from '../stores/connectivity';
import * as m from '$lib/paraglide/messages.js';

interface CloudBackupState {
  connectionStatus: ConnectionStatusResponse | null;
  isConnecting: boolean;
  isDisconnecting: boolean;
  isOnline: boolean | null;
  error: string | null;
  isSyncing: boolean;
  syncProgress: number;
  lastSyncAt: string | null;
  backups: BackupListItem[];
  syncStatusMessage: string;
}

export class CloudBackupController {
  #state: CloudBackupState;

  constructor() {
    this.#state = $state<CloudBackupState>({
      connectionStatus: null,
      isConnecting: false,
      isDisconnecting: false,
      isOnline: null,
      error: null,
      isSyncing: false,
      syncProgress: 0,
      lastSyncAt: null,
      backups: [],
      syncStatusMessage: ''
    });

    // Initialize connection status on creation
    this.#refreshConnectionStatus();
    this.#checkConnectivity();
    this.#listenToConnectivityChanges();
    this.#listenToSyncProgress();
    this.#listenToRestoreComplete();
  }

  // ============================================================
  // PUBLIC GETTERS
  // ============================================================

  get connectionStatus(): ConnectionStatusResponse | null {
    return this.#state.connectionStatus;
  }

  get isConnected(): boolean {
    return this.#state.connectionStatus?.isConnected ?? false;
  }

  get userEmail(): string | null {
    return this.#state.connectionStatus?.email ?? null;
  }

  get isConnecting(): boolean {
    return this.#state.isConnecting;
  }

  get isDisconnecting(): boolean {
    return this.#state.isDisconnecting;
  }

  get isOnline(): boolean {
    return this.#state.isOnline ?? false;
  }

  get error(): string | null {
    return this.#state.error;
  }

  get isLoading(): boolean {
    return this.#state.isConnecting || this.#state.isDisconnecting;
  }

  get isSyncing(): boolean {
    return this.#state.isSyncing;
  }

  get syncProgress(): number {
    return this.#state.syncProgress;
  }

  get lastSyncAt(): string | null {
    return this.#state.lastSyncAt ?? this.#state.connectionStatus?.lastSyncAt ?? null;
  }

  get backups(): BackupListItem[] {
    return this.#state.backups;
  }

  get syncStatusMessage(): string {
    return this.#state.syncStatusMessage;
  }

  // ============================================================
  // PUBLIC ACTIONS
  // ============================================================

  /**
   * Initiate Google OAuth flow
   */
  async connectGoogle(): Promise<void> {
    if (this.#state.isConnecting) return;

    this.#state.isConnecting = true;
    this.#state.error = null;

    try {
      const result = await commands.cloudBackupConnectGoogle();
      this.#state.connectionStatus = result;
    } catch (err) {
      this.#state.error = err instanceof Error ? err.message : String(err);
      throw err;
    } finally {
      this.#state.isConnecting = false;
    }
  }

  /**
   * Disconnect Google account
   */
  async disconnectGoogle(): Promise<void> {
    if (this.#state.isDisconnecting || !this.userEmail) return;

    this.#state.isDisconnecting = true;
    this.#state.error = null;

    try {
      await commands.cloudBackupDisconnectGoogle();
      this.#state.connectionStatus = {
        isConnected: false,
        email: null,
        connectedAt: null,
        lastSyncAt: null
      };
      this.#state.backups = [];
    } catch (err) {
      this.#state.error = err instanceof Error ? err.message : String(err);
      throw err;
    } finally {
      this.#state.isDisconnecting = false;
    }
  }

  /**
   * Sync (backup) to Google Drive
   */
  async syncNow(): Promise<void> {
    if (this.#state.isSyncing || !this.isConnected || !this.isOnline) {
      return;
    }

    this.#state.isSyncing = true;
    this.#state.syncProgress = 0;
    this.#state.error = null;
    this.#state.syncStatusMessage = m.cloud_backup_syncing();

    try {
      const result = await commands.cloudBackupSyncNow();
      this.#state.lastSyncAt = result.createdAt;
      this.#state.syncStatusMessage = m.cloud_backup_sync_success();
      this.#state.syncProgress = 100;
      await this.loadBackups();
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      this.#state.error = errorMsg;
      this.#state.syncStatusMessage = m.cloud_backup_sync_failed({ error: errorMsg });
    } finally {
      this.#state.isSyncing = false;
    }
  }

  /**
   * Load list of available backups
   */
  async loadBackups(): Promise<void> {
    if (!this.isConnected || !this.isOnline) {
      return;
    }

    try {
      const result = await commands.cloudBackupListBackups();
      this.#state.backups = result.backups;
    } catch (err) {
      console.error('Failed to load backups:', err);
    }
  }

  /**
   * Restore database from a backup
   */
  async restoreBackup(backupId: string, confirmation: string): Promise<void> {
    if (!this.isConnected) {
      throw new Error(m.cloud_backup_error_not_connected());
    }

    if (!this.isOnline) {
      throw new Error(m.cloud_backup_offline());
    }

    if (confirmation !== 'RESTORE') {
      throw new Error(
        m.cloud_backup_restore_confirm_error({
          keyword: m.cloud_backup_restore_confirm_keyword()
        })
      );
    }

    this.#state.error = null;

    try {
      await commands.cloudBackupRestore({
        backup_id: backupId,
        confirmation
      });
      return;
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      this.#state.error = errorMsg;
      throw err;
    }
  }

  /**
   * Refresh connection status
   */
  async refreshConnectionStatus(): Promise<void> {
    await this.#refreshConnectionStatus();
  }

  /**
   * Check internet connectivity
   */
  async checkConnectivity(): Promise<void> {
    await this.#checkConnectivity();
  }

  /**
   * Clear error state
   */
  clearError(): void {
    this.#state.error = null;
  }

  // ============================================================
  // PRIVATE METHODS
  // ============================================================

  async #refreshConnectionStatus(): Promise<void> {
    try {
      const result = await commands.cloudBackupGetConnectionStatus();
      this.#state.connectionStatus = result;
      this.#state.error = null;

      if (result.isConnected) {
        await this.loadBackups();
      }
    } catch (err) {
      this.#state.error = err instanceof Error ? err.message : String(err);
    }
  }

  async #checkConnectivity(): Promise<void> {
    try {
      const result = await commands.cloudBackupCheckConnectivity();
      this.#state.isOnline = result.isOnline;
      setConnectivityStatus(result);
    } catch (err) {
      console.error('Failed to check connectivity:', err);
      this.#state.isOnline = false;
      setConnectivityOffline();
    }
  }

  async #listenToConnectivityChanges(): Promise<void> {
    try {
      await listen<{ isOnline: boolean; checkedAt: string }>(
        'cloud-backup://connectivity-changed',
        (event) => {
          this.#state.isOnline = event.payload.isOnline;
          setConnectivityStatus(event.payload);
        }
      );
    } catch (err) {
      console.error('Failed to listen for connectivity events:', err);
    }
  }

  async #listenToSyncProgress(): Promise<void> {
    try {
      await listen<{ operationId: string; progressPercent: number; stage: string }>(
        'cloud-backup://sync-progress',
        (event) => {
          this.#state.syncProgress = event.payload.progressPercent;
          this.#state.syncStatusMessage = event.payload.stage;
        }
      );
    } catch (err) {
      console.error('Failed to listen for sync progress events:', err);
    }
  }

  async #listenToRestoreComplete(): Promise<void> {
    try {
      await listen<{ backupId: string; restoredAt: string }>(
        'cloud-backup://restore-complete',
        () => {
          // Reload the app to pick up the restored database
          window.location.reload();
        }
      );
    } catch (err) {
      console.error('Failed to listen for restore-complete events:', err);
    }
  }
}

/**
 * Singleton instance
 */
let controllerInstance: CloudBackupController | null = null;

export function getCloudBackupController(): CloudBackupController {
  if (!controllerInstance) {
    controllerInstance = new CloudBackupController();
  }
  return controllerInstance;
}
