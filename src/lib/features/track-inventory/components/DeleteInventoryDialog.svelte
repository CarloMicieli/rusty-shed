<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { AlertTriangle, X } from 'lucide-svelte';

  interface Props {
    open?: boolean;
    inventoryName?: string;
    onClose?: () => void;
    onConfirm?: () => void;
  }

  const { open = $bindable(false), inventoryName = '', onClose, onConfirm }: Props = $props();

  let deleting = $state(false);

  function handleClose() {
    if (deleting) return;
    onClose?.();
  }

  async function handleConfirm() {
    try {
      deleting = true;
      await onConfirm?.();
      handleClose();
    } catch (err) {
      console.error('Failed to delete inventory:', err);
    } finally {
      deleting = false;
    }
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="delete-inventory-title"
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) handleClose();
    }}
    onkeydown={(e) => {
      if (e.key === 'Escape') handleClose();
    }}
  >
    <div class="variant-filled-surface card w-full max-w-md space-y-4 p-6">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="variant-filled-error flex h-10 w-10 items-center justify-center rounded-full">
            <AlertTriangle size={20} />
          </div>
          <h2 id="delete-inventory-title" class="h3 font-bold">
            {m.track_inventory_delete_confirm_title()}
          </h2>
        </div>
        <button
          onclick={handleClose}
          class="variant-ghost-surface btn-icon btn-icon-sm"
          disabled={deleting}
        >
          <X size={20} />
        </button>
      </div>

      <div class="space-y-2">
        <p class="text-surface-300">
          {m.track_inventory_delete_confirm_message()}
        </p>
        {#if inventoryName}
          <p class="font-semibold">"{inventoryName}"</p>
        {/if}
      </div>

      <div class="flex justify-end gap-2">
        <button
          type="button"
          onclick={handleClose}
          class="variant-ghost-surface btn"
          disabled={deleting}
        >
          {m.track_inventory_delete_cancel()}
        </button>
        <button
          type="button"
          onclick={handleConfirm}
          class="variant-filled-error btn"
          disabled={deleting}
        >
          {#if deleting}
            <span class="animate-pulse">...</span>
          {:else}
            {m.track_inventory_delete_confirm()}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
