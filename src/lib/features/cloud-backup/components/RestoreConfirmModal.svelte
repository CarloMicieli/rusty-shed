<script lang="ts">
  /**
   * RestoreConfirmModal Component
   *
   * Modal dialog for confirming a backup restore operation.
   * Requires the user to type a confirmation keyword to prevent accidental data loss.
   *
   * @component
   * @example
   * ```svelte
   * <RestoreConfirmModal
   *   backupId="abc123"
   *   backupLabel="Initial"
   *   bind:open={isModalOpen}
   *   onConfirm={() => location.reload()}
   * />
   * ```
   */

  import { Button } from '$lib/components';
  import { AlertTriangle, Loader2, X } from 'lucide-svelte';
  import { getCloudBackupController } from '../controllers/cloudBackup.svelte';
  import * as m from '$lib/paraglide/messages.js';

  /**
   * Props for the restore confirmation modal
   */
  interface Props {
    /** ID of the backup to restore */
    backupId: string;
    /** Label/description of the backup */
    backupLabel: string;
    /** Callback when user confirms the restore */
    onConfirm?: () => void;
    /** Callback when user cancels the restore */
    onCancel?: () => void;
    /** Whether the modal is open (bindable) */
    open?: boolean;
  }

  let { backupId, backupLabel, onConfirm, onCancel, open = $bindable(false) }: Props = $props();

  const controller = getCloudBackupController();

  const confirmationKeyword = m.cloud_backup_restore_confirm_keyword();

  let confirmationText = $state('');
  let isRestoring = $state(false);
  let errorMessage = $state<string | null>(null);

  let isValid = $derived(confirmationText === confirmationKeyword);

  async function handleConfirm() {
    if (!isValid) {
      errorMessage = m.cloud_backup_restore_confirm_error({ keyword: confirmationKeyword });
      return;
    }

    isRestoring = true;
    errorMessage = null;

    try {
      await controller.restoreBackup(backupId, confirmationText);

      onConfirm?.();

      // Reset state
      open = false;
      confirmationText = '';
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : String(err);
    } finally {
      isRestoring = false;
    }
  }

  function handleCancel() {
    onCancel?.();
    open = false;
    confirmationText = '';
    errorMessage = null;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      handleCancel();
    } else if (event.key === 'Enter' && isValid && !isRestoring) {
      handleConfirm();
    }
  }
</script>

<!-- Modal Backdrop -->
{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
    onclick={handleCancel}
    role="presentation"
  >
    <!-- Modal Content -->
    <div
      class="relative w-full max-w-md rounded-lg bg-white p-6 shadow-xl"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(event) => {
        if (event.key === 'Escape') {
          handleCancel();
        }
      }}
      role="dialog"
      aria-labelledby="restore-modal-title"
      aria-describedby="restore-modal-description"
      tabindex="-1"
    >
      <!-- Close Button -->
      <button
        onclick={handleCancel}
        class="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
        aria-label={m.common_cancel()}
        disabled={isRestoring}
      >
        <X class="h-5 w-5" />
      </button>

      <!-- Warning Icon -->
      <div class="mb-4 flex items-center justify-center">
        <div class="rounded-full bg-amber-100 p-3">
          <AlertTriangle class="h-6 w-6 text-amber-600" />
        </div>
      </div>

      <!-- Title -->
      <h2 id="restore-modal-title" class="mb-2 text-center text-xl font-semibold text-gray-900">
        {m.cloud_backup_restore_confirm_title()}
      </h2>

      <!-- Description -->
      <div id="restore-modal-description" class="space-y-3 text-sm text-gray-600">
        <p>{m.cloud_backup_restore_confirm_intro()}</p>
        <div class="rounded-md border border-gray-200 bg-gray-50 p-3">
          <p class="font-medium text-gray-900">{backupLabel}</p>
        </div>
        <div class="rounded-md border border-red-200 bg-red-50 p-3">
          <p class="mb-1 font-semibold text-red-900">{m.cloud_backup_restore_warning_title()}</p>
          <ul class="list-inside list-disc space-y-1 text-red-800">
            <li>{m.cloud_backup_restore_warning_replace()}</li>
            <li>{m.cloud_backup_restore_warning_backup()}</li>
            <li>{m.cloud_backup_restore_warning_reload()}</li>
          </ul>
        </div>
        <p>
          {m.cloud_backup_restore_confirm_prompt({ keyword: confirmationKeyword })}
        </p>
      </div>

      <!-- Confirmation Input -->
      <input
        type="text"
        bind:value={confirmationText}
        onkeydown={handleKeydown}
        placeholder={m.cloud_backup_restore_confirm_placeholder({ keyword: confirmationKeyword })}
        class="mt-3 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 focus:outline-none"
        disabled={isRestoring}
        autocomplete="off"
        spellcheck="false"
      />

      <!-- Error Message -->
      {#if errorMessage}
        <div class="mt-3 rounded-md border border-red-200 bg-red-50 p-3">
          <p class="text-sm text-red-800">{errorMessage}</p>
        </div>
      {/if}

      <!-- Actions -->
      <div class="mt-6 flex justify-end gap-3">
        <Button onclick={handleCancel} variant="outline" size="sm" disabled={isRestoring}>
          {m.common_cancel()}
        </Button>
        <Button
          onclick={handleConfirm}
          variant="destructive"
          size="sm"
          disabled={!isValid || isRestoring}
          class="gap-2"
        >
          {#if isRestoring}
            <Loader2 class="h-4 w-4 animate-spin" />
            <span>{m.cloud_backup_restoring()}</span>
          {:else}
            <span>{m.cloud_backup_restore()}</span>
          {/if}
        </Button>
      </div>
    </div>
  </div>
{/if}
