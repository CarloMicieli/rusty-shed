<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { AlertTriangle, X, Loader2 } from 'lucide-svelte';
  import { Button } from '$lib/components';

  interface Props {
    open?: boolean;
    inventoryName?: string;
    onClose?: () => void;
    onConfirm?: () => void;
  }

  let { open = $bindable(false), inventoryName = '', onClose, onConfirm }: Props = $props();

  let deleting = $state(false);

  $effect(() => {
    if (open) {
      // Prevent background scrolling
      const originalStyle = window.getComputedStyle(document.body).overflow;
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = originalStyle;
      };
    }
  });

  function handleClose() {
    if (deleting) return;
    open = false;
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

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) handleClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-[100] bg-black/80 backdrop-blur-md transition-all duration-300"
    onclick={handleClose}
    aria-hidden="true"
  ></div>

  <!-- Dialog -->
  <div
    class="fixed top-1/2 left-1/2 z-[101] w-full max-w-md -translate-x-1/2 -translate-y-1/2 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="delete-modal-title"
  >
    <div
      class="flex flex-col overflow-hidden rounded-2xl border border-red-500/20 bg-layout-surface shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-xl bg-red-500/10 text-red-500"
          >
            <AlertTriangle size={20} />
          </div>
          <h2
            id="delete-modal-title"
            class="text-lg font-bold tracking-tight text-zinc-100 uppercase"
          >
            {m.track_inventory_delete_confirm_title()}
          </h2>
        </div>
        <button
          onclick={handleClose}
          class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
          disabled={deleting}
        >
          <X size={20} />
        </button>
      </div>

      <div class="p-6">
        <div class="space-y-4">
          <p class="text-sm leading-relaxed text-zinc-400">
            {m.track_inventory_delete_confirm_message()}
          </p>
          {#if inventoryName}
            <div class="rounded-xl border border-white/5 bg-zinc-950 p-4">
              <span
                class="mb-1 block text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
                >Deleting Inventory</span
              >
              <p class="text-lg font-bold text-red-400">"{inventoryName}"</p>
            </div>
          {/if}
        </div>

        <div class="mt-8 flex justify-end gap-3">
          <Button
            type="button"
            variant="ghost"
            onclick={handleClose}
            class="px-6 text-zinc-500 hover:bg-transparent hover:text-white"
            disabled={deleting}
          >
            {m.track_inventory_delete_cancel()}
          </Button>
          <Button
            type="button"
            variant="destructive"
            class="h-11 min-w-[120px] bg-red-600 px-8 font-bold text-white hover:bg-red-500"
            onclick={handleConfirm}
            disabled={deleting}
          >
            {#if deleting}
              <Loader2 size={18} class="mr-2 animate-spin" />
              <span>Deleting...</span>
            {:else}
              {m.track_inventory_delete_confirm()}
            {/if}
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}
