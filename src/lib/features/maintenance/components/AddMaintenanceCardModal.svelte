<script lang="ts">
  import { X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getMaintenanceState } from '../MaintenanceState.svelte';
  import RollingStockSelector from './RollingStockSelector.svelte';
  import { toaster } from '$lib/toaster';

  let { open, onClose } = $props<{ open: boolean; onClose: () => void }>();

  const maintenanceState = getMaintenanceState();

  let selectedRollingStockId = $state<string | null>(null);
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);

  const isFormValid = $derived(selectedRollingStockId !== null);

  async function handleSubmit() {
    if (!isFormValid || !selectedRollingStockId) return;

    isSubmitting = true;
    error = null;

    try {
      await maintenanceState.createMaintenanceCard(selectedRollingStockId);

      // Show success toast
      toaster.success({
        id: crypto.randomUUID(),
        title: m.maintenance_create_card_success(),
        duration: 3000
      });

      // Reset and close
      selectedRollingStockId = null;
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : m.maintenance_create_card_error();
      console.error('[AddMaintenanceCardModal] Submit error:', err);
    } finally {
      isSubmitting = false;
    }
  }

  function handleClose() {
    selectedRollingStockId = null;
    error = null;
    onClose();
  }
</script>

{#if open}
  <div class="modal-backdrop fixed inset-0 z-50 flex items-center justify-center bg-surface-900/80">
    <div class="m-4 w-full max-w-md space-y-4 card p-6">
      <!-- Header -->
      <div class="flex items-center justify-between">
        <h3 class="h3">{m.maintenance_create_card_title()}</h3>
        <button type="button" class="btn-icon btn-icon-sm" onclick={handleClose}>
          <X size={20} />
        </button>
      </div>

      <!-- Form -->
      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="space-y-4"
      >
        <RollingStockSelector bind:selectedId={selectedRollingStockId} />

        {#if error}
          <div class="alert variant-filled-error">
            <p class="text-sm">{error}</p>
          </div>
        {/if}

        {#if !isFormValid && selectedRollingStockId === null}
          <p class="text-sm text-surface-500">{m.maintenance_create_card_validation()}</p>
        {/if}

        <!-- Actions -->
        <div class="flex justify-end gap-3">
          <button type="button" class="variant-ghost-surface btn" onclick={handleClose}>
            {m.maintenance_create_card_cancel()}
          </button>
          <button
            type="submit"
            class="variant-filled-primary btn"
            disabled={!isFormValid || isSubmitting}
          >
            {isSubmitting ? m.app_loading() : m.maintenance_create_card_submit()}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
