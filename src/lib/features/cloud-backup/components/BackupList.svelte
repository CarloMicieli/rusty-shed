<script lang="ts">
  /**
   * BackupList Component
   *
   * Displays a list of available backups from Google Drive.
   * Shows backup details like creation date, size, and record count.
   * Allows restoring a backup by clicking a restore button.
   *
   * @component
   * @example
   * ```svelte
   * <BackupList onRestore={(id) => console.log('Restore', id)} />
   * ```
   */

  import { Button } from '$lib/components';
  import { Calendar, Database, Download } from 'lucide-svelte';
  import { getCloudBackupController } from '../controllers/cloudBackup.svelte';
  import type { BackupListItem } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';

  /**
   * Props for BackupList component
   */
  interface Props {
    /** Callback fired when a restore action is requested */
    onRestore?: (backupId: string) => void;
  }

  let { onRestore }: Props = $props();

  const controller = getCloudBackupController();

  let backups = $derived(controller.backups);
  let isConnected = $derived(controller.isConnected);
  let isOnline = $derived(controller.isOnline);

  function handleRestore(backup: BackupListItem) {
    onRestore?.(backup.id);
  }

  function formatDate(isoString: string): string {
    try {
      const date = new Date(isoString);
      return new Intl.DateTimeFormat(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      }).format(date);
    } catch {
      return isoString;
    }
  }

  function getBackupLabel(backup: BackupListItem): string {
    if (backup.isInitial) {
      return m.cloud_backup_initial_badge();
    }

    return formatDate(backup.createdAt);
  }

  function formatRecordCount(count: number): string {
    if (count === 1) {
      return m.cloud_backup_record_count_single({ count: count.toLocaleString() });
    }

    return m.cloud_backup_record_count_multiple({ count: count.toLocaleString() });
  }
</script>

<!-- Backup List -->
<div class="space-y-3">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-semibold">{m.cloud_backup_backups_title()}</h3>
    {#if backups.length > 0}
      {#if backups.length === 1}
        <span class="text-sm text-muted-foreground">
          {m.cloud_backup_backups_count_single({ count: backups.length })}
        </span>
      {:else}
        <span class="text-sm text-muted-foreground">
          {m.cloud_backup_backups_count_multiple({ count: backups.length })}
        </span>
      {/if}
    {/if}
  </div>

  {#if !isConnected}
    <div class="rounded-md border border-amber-200 bg-amber-50 p-4">
      <p class="text-sm text-amber-800">{m.cloud_backup_backups_connect_prompt()}</p>
    </div>
  {:else if !isOnline}
    <div class="rounded-md border border-amber-200 bg-amber-50 p-4">
      <p class="text-sm text-amber-800">{m.cloud_backup_backups_offline_prompt()}</p>
    </div>
  {:else if backups.length === 0}
    <div class="rounded-md border border-gray-200 bg-gray-50 p-4">
      <p class="text-sm text-gray-600">{m.cloud_backup_backups_empty()}</p>
    </div>
  {:else}
    <div class="space-y-2">
      {#each backups as backup (backup.id)}
        <div
          class="flex items-center justify-between rounded-lg border border-gray-200 bg-white p-4 transition-colors hover:bg-gray-50"
        >
          <div class="flex flex-1 items-start gap-3">
            <Database class="mt-0.5 h-5 w-5 flex-shrink-0 text-blue-600" />
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-2">
                <span class="font-medium text-gray-900">{getBackupLabel(backup)}</span>
                {#if backup.isInitial}
                  <span
                    class="inline-flex items-center rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-800"
                  >
                    {m.cloud_backup_initial_badge()}
                  </span>
                {/if}
              </div>
              <div class="mt-1 flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-gray-500">
                <div class="flex items-center gap-1">
                  <Calendar class="h-3.5 w-3.5" />
                  <span>{formatDate(backup.createdAt)}</span>
                </div>
                <span>•</span>
                <span>{backup.sizeFormatted}</span>
                {#if Number(backup.recordCount) > 0}
                  <span>•</span>
                  <span>{formatRecordCount(Number(backup.recordCount))}</span>
                {/if}
              </div>
            </div>
          </div>
          <Button
            onclick={() => handleRestore(backup)}
            variant="outline"
            size="sm"
            class="flex-shrink-0 gap-2"
          >
            <Download class="h-4 w-4" />
            <span>{m.cloud_backup_restore()}</span>
          </Button>
        </div>
      {/each}
    </div>
  {/if}
</div>
