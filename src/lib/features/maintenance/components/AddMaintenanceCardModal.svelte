<script lang="ts">
  import { X, Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import { getMaintenanceState } from '../MaintenanceState.svelte';
  import RollingStockSelector from './RollingStockSelector.svelte';
  import { toaster } from '$lib/toaster';

  let { open, onClose } = $props<{ open: boolean; onClose: () => void }>();

  const maintenanceState = getMaintenanceState();

  let selectedRollingStockId = $state<string | null>(null);
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);

  const isFormValid = $derived(selectedRollingStockId !== null);

  // Prevent background scrolling when modal is open
  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });

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
      const msg = err instanceof Error ? err.message : String(err);
      if (msg.toLowerCase().includes('already exists') || msg.toLowerCase().includes('conflict')) {
        error = m.maintenance_card_already_exists();
      } else {
        error = msg || m.maintenance_create_card_error();
      }
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
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 p-4 backdrop-blur-md"
    aria-modal="true"
    role="dialog"
  >
    <div
      class="relative w-full max-w-md overflow-hidden rounded-2xl border border-white/10 bg-[#0c0c0c] shadow-2xl"
    >
      <!-- Background Context Icon -->
      <div class="pointer-events-none absolute -top-6 -right-6 text-white/5">
        <Wrench size={120} strokeWidth={1} />
      </div>

      <!-- Header -->
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <div class="flex items-center gap-3">
          <div class="rounded-lg bg-amber-500/10 p-2 text-amber-500">
            <Wrench size={20} />
          </div>
          <h3 class="text-lg font-bold text-zinc-100">{m.maintenance_create_card_title()}</h3>
        </div>
        <button
          type="button"
          class="text-zinc-500 transition-colors hover:text-white"
          onclick={handleClose}
          aria-label={m.dialog_close_button()}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form -->
      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="space-y-6 p-6"
      >
        <div class="space-y-4">
          <RollingStockSelector bind:selectedId={selectedRollingStockId} />

          {#if error}
            <div class="rounded-lg border border-red-500/20 bg-red-500/10 p-3 text-xs text-red-400">
              {error}
            </div>
          {/if}

          {#if !isFormValid && selectedRollingStockId === null}
            <p class="text-xs font-medium text-amber-500/70">
              {m.maintenance_create_card_validation()}
            </p>
          {/if}
        </div>

        <!-- Actions -->
        <div class="flex items-center justify-end gap-4 border-t border-white/5 pt-6">
          <Button variant="ghost" onclick={handleClose} class="text-zinc-500 hover:text-zinc-100">
            {m.maintenance_create_card_cancel()}
          </Button>
          <Button
            type="submit"
            variant="rusty"
            disabled={!isFormValid || isSubmitting}
            class="min-w-[120px]"
          >
            {#if isSubmitting}
              <span
                class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-black border-t-transparent"
              ></span>
              {m.app_loading()}
            {:else}
              {m.maintenance_create_card_submit()}
            {/if}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
