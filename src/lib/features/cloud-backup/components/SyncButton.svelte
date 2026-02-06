<script lang="ts">
  /**
   * SyncButton Component
   *
   * Provides a button to trigger backup sync to Google Drive.
   * Shows sync progress and status messages.
   * Disables when offline, not connected, or already syncing.
   *
   * @component
   * @example
   * ```svelte
   * <SyncButton />
   * ```
   */

  import { Button } from '$lib/components';
  import { AlertCircle, Cloud, Loader2 } from 'lucide-svelte';
  import { getCloudBackupController } from '../controllers/cloudBackup.svelte';
  import * as m from '$lib/paraglide/messages.js';

  const controller = getCloudBackupController();

  let isSyncing = $derived(controller.isSyncing);
  let syncProgress = $derived(controller.syncProgress);
  let isConnected = $derived(controller.isConnected);
  let isOnline = $derived(controller.isOnline);
  let error = $derived(controller.error);
  let syncStatusMessage = $derived(controller.syncStatusMessage);

  async function handleSync() {
    if (!isConnected) {
      // Show error or redirect to connect
      return;
    }

    try {
      await controller.syncNow();
    } catch (err) {
      console.error('Sync failed:', err);
    }
  }

  function handleDismissError() {
    controller.clearError();
  }
</script>

<!-- Sync Button with Status -->
<div class="space-y-2">
  <div class="flex items-center gap-2">
    <Button
      onclick={handleSync}
      disabled={isSyncing || !isConnected || !isOnline}
      variant="default"
      size="sm"
      class="gap-2"
    >
      {#if isSyncing}
        <Loader2 class="h-4 w-4 animate-spin" />
        <span>{m.cloud_backup_syncing()} {Math.round(syncProgress)}%</span>
      {:else if !isOnline}
        <AlertCircle class="h-4 w-4" />
        <span>{m.cloud_backup_offline_short()}</span>
      {:else}
        <Cloud class="h-4 w-4" />
        <span>{m.cloud_backup_sync_now()}</span>
      {/if}
    </Button>
  </div>

  {#if !isOnline && !isSyncing}
    <p class="text-sm text-amber-700">{m.cloud_backup_offline()}</p>
  {/if}

  <!-- Progress Bar -->
  {#if isSyncing}
    <div class="h-2 w-full rounded-full bg-gray-200">
      <div
        class="h-2 rounded-full bg-blue-500 transition-all duration-300"
        style="width: {syncProgress}%"
      ></div>
    </div>
    <p class="text-sm text-gray-600">{syncStatusMessage}</p>
  {:else if syncStatusMessage && syncProgress === 100}
    <p class="text-sm text-green-600">{syncStatusMessage}</p>
  {/if}

  <!-- Error Message -->
  {#if error}
    <div class="flex items-start gap-2 rounded-md border border-red-200 bg-red-50 p-3">
      <AlertCircle class="mt-0.5 h-4 w-4 flex-shrink-0 text-red-600" />
      <div class="flex-1">
        <p class="text-sm text-red-800">{error}</p>
      </div>
      <button
        onclick={handleDismissError}
        class="flex-shrink-0 text-red-400 hover:text-red-600"
        aria-label={m.common_dismiss()}
      >
        ×
      </button>
    </div>
  {/if}
</div>
