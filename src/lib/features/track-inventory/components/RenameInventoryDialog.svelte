<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X } from 'lucide-svelte';

  interface Props {
    open?: boolean;
    currentName?: string;
    onClose?: () => void;
    onRename?: (newName: string) => void;
  }

  const { open = $bindable(false), currentName = '', onClose, onRename }: Props = $props();

  let name = $state('');
  let submitting = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    if (open) {
      name = currentName;
    }
  });

  function handleClose() {
    if (submitting) return;
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
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="rename-inventory-title"
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
        <h2 id="rename-inventory-title" class="h3 font-bold">
          {m.track_inventory_rename_dialog_title()}
        </h2>
        <button
          onclick={handleClose}
          class="variant-ghost-surface btn-icon btn-icon-sm"
          disabled={submitting}
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="space-y-4">
        <label class="label">
          <span>{m.track_inventory_rename_field_name()}</span>
          <input type="text" class="input" bind:value={name} disabled={submitting} required />
        </label>

        {#if error}
          <div class="variant-filled-error rounded-lg p-3 text-sm">
            {error}
          </div>
        {/if}

        <div class="flex justify-end gap-2">
          <button
            type="button"
            onclick={handleClose}
            class="variant-ghost-surface btn"
            disabled={submitting}
          >
            {m.track_inventory_rename_cancel()}
          </button>
          <button type="submit" class="variant-filled-primary btn" disabled={submitting}>
            {#if submitting}
              <span class="animate-pulse">...</span>
            {:else}
              {m.track_inventory_rename_submit()}
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
