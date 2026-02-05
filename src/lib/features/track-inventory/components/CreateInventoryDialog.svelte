<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X } from 'lucide-svelte';
  import { Input, Textarea, Button } from '$lib/components';

  interface Props {
    open?: boolean;
    onClose?: () => void;
    onCreate?: (name: string, description: string) => void;
  }

  const { open = $bindable(false), onClose, onCreate }: Props = $props();

  let name = $state('');
  let description = $state('');
  let submitting = $state(false);
  let error = $state<string | null>(null);

  function handleClose() {
    if (submitting) return;
    name = '';
    description = '';
    error = null;
    onClose?.();
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();

    if (!name.trim()) {
      error = m.track_inventory_create_validation_name();
      return;
    }

    try {
      submitting = true;
      error = null;
      await onCreate?.(name.trim(), description.trim());
      name = '';
      description = '';
      handleClose();
    } catch (err) {
      console.error('Failed to create inventory:', err);
      error = err instanceof Error ? err.message : 'Failed to create inventory';
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
    aria-labelledby="create-inventory-title"
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
        <h2 id="create-inventory-title" class="h3 font-bold">
          {m.track_inventory_create_dialog_title()}
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
          <span>{m.track_inventory_create_field_name()}</span>
          <Input type="text" bind:value={name} disabled={submitting} required />
        </label>

        <label class="label">
          <span>{m.track_inventory_create_field_description()}</span>
          <Textarea bind:value={description} rows={3} disabled={submitting} />
        </label>

        {#if error}
          <div class="variant-filled-error rounded-lg p-3 text-sm">
            {error}
          </div>
        {/if}

        <div class="flex justify-end gap-2">
          <Button type="button" onclick={handleClose} variant="ghost" disabled={submitting}>
            {m.track_inventory_create_cancel()}
          </Button>
          <Button type="submit" variant="default" disabled={submitting}>
            {#if submitting}
              <span class="animate-pulse">...</span>
            {:else}
              {m.track_inventory_create_submit()}
            {/if}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
