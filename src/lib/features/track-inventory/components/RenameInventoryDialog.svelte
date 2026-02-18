<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Loader2 } from 'lucide-svelte';
  import { Input, Button } from '$lib/components';

  interface Props {
    open?: boolean;
    currentName?: string;
    onClose?: () => void;
    onRename?: (newName: string) => void;
  }

  let { open = $bindable(false), currentName = '', onClose, onRename }: Props = $props();

  let name = $state('');
  let submitting = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    if (open) {
      name = currentName;
      // Prevent background scrolling
      const originalStyle = window.getComputedStyle(document.body).overflow;
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = originalStyle;
      };
    }
  });

  function handleClose() {
    if (submitting) return;
    open = false;
    error = null;
    onClose?.();
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();

    if (!name.trim()) {
      error = m.track_inventory_rename_validation_name();
      return;
    }

    if (name.trim() === currentName) {
      handleClose();
      return;
    }

    try {
      submitting = true;
      error = null;
      await onRename?.(name.trim());
      handleClose();
    } catch (err) {
      console.error('Failed to rename inventory:', err);
      error = err instanceof Error ? err.message : 'Failed to rename inventory';
    } finally {
      submitting = false;
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
    class="fixed top-1/2 left-1/2 z-[101] w-full max-w-lg -translate-x-1/2 -translate-y-1/2 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="rename-modal-title"
  >
    <div
      class="flex flex-col overflow-hidden rounded-2xl border border-amber-500/20 bg-[#0c0c0c] shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <h2
          id="rename-modal-title"
          class="text-xl font-bold tracking-tight text-zinc-100 uppercase"
        >
          {m.track_inventory_rename_dialog_title()}
        </h2>
        <button
          onclick={handleClose}
          class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
          disabled={submitting}
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="space-y-6 p-6">
        <div class="space-y-2">
          <label
            for="rename-inventory-input"
            class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >
            {m.track_inventory_rename_field_name()}
          </label>
          <Input
            id="rename-inventory-input"
            bind:value={name}
            class="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-white/20 focus:ring-amber-500 focus-visible:ring-0 focus-visible:ring-offset-0"
            disabled={submitting}
            required
            autofocus
          />
        </div>

        {#if error}
          <div class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-sm text-red-400">
            {error}
          </div>
        {/if}

        <div class="flex justify-end gap-3 pt-4">
          <Button
            type="button"
            variant="ghost"
            onclick={handleClose}
            class="px-6 text-zinc-500 hover:bg-transparent hover:text-white"
            disabled={submitting}
          >
            {m.track_inventory_rename_cancel()}
          </Button>
          <Button
            type="submit"
            variant="rusty"
            class="h-12 min-w-[120px] px-8"
            disabled={submitting || name.trim() === currentName}
          >
            {#if submitting}
              <Loader2 size={18} class="mr-2 animate-spin" />
              <span>Saving...</span>
            {:else}
              {m.track_inventory_rename_submit()}
            {/if}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
