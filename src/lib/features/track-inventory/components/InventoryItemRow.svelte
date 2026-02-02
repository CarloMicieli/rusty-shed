<script lang="ts">
  import type { TrackInventoryItemView } from '$lib/features/track-inventory';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import { AlertTriangle, Edit2 } from 'lucide-svelte';

  interface Props {
    item: TrackInventoryItemView;
    inventoryId: string;
  }

  const { item, inventoryId }: Props = $props();
  const service = getTrackInventoryContext();

  let isEditingRequired = $state(false);
  // avoid capturing `item` initial value in state to prevent svelte warning
  let requiredInput = $state('');
  let isSaving = $state(false);
  let error = $state<string | null>(null);

  const hasShortage = $derived(item.required > 0 && item.quantity < item.required);
  const shortageCount = $derived(item.required - item.quantity);

  async function saveRequired() {
    const newRequired = parseInt(requiredInput, 10);
    if (isNaN(newRequired) || newRequired < 0) {
      error = 'Invalid required quantity';
      return;
    }

    isSaving = true;
    error = null;
    try {
      await service.setItemRequired(inventoryId, item.track_product.track_id, newRequired);
      item.required = BigInt(newRequired);
      isEditingRequired = false;
    } catch (err) {
      console.error('Failed to update required quantity:', err);
      error = err instanceof Error ? err.message : 'Failed to update';
    } finally {
      isSaving = false;
    }
  }

  function startEditing() {
    requiredInput = item.required.toString();
    error = null;
    isEditingRequired = true;
  }

  function cancelEditing() {
    isEditingRequired = false;
    requiredInput = item.required.toString();
    error = null;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      saveRequired();
    } else if (event.key === 'Escape') {
      cancelEditing();
    }
  }
</script>

<div
  class="variant-ghost-surface flex items-center justify-between rounded-lg border p-4"
  class:border-surface-700={!hasShortage}
  class:border-warning-500={hasShortage}
  style:background-color={hasShortage ? 'rgba(var(--color-warning-500) / 0.05)' : undefined}
>
  <div class="flex flex-1 flex-col gap-1">
    <div class="flex items-center gap-2">
      <h4 class="font-semibold">
        {item.track_product.description || item.track_product.product_code}
      </h4>
      {#if hasShortage}
        <div class="flex items-center gap-1 text-warning-500">
          <AlertTriangle size={16} />
          <span class="text-xs font-medium">
            {m.track_inventory_item_shortage({ count: shortageCount })}
          </span>
        </div>
      {/if}
    </div>
    <div class="flex items-center gap-4 text-sm text-surface-400">
      <span>{item.track_product.manufacturer_name}</span>
      <span>•</span>
      <span>{item.track_product.product_code}</span>
      <span>•</span>
      <span>{item.track_product.track_type}</span>
    </div>
  </div>

  <div class="flex items-center gap-6">
    <div class="flex flex-col items-center">
      <span class="text-xs text-surface-400">{m.track_inventory_item_quantity()}</span>
      <span class="text-lg font-bold" class:text-warning-500={hasShortage}>
        {item.quantity}
      </span>
    </div>
    <div class="flex flex-col items-center">
      <span class="text-xs text-surface-400">{m.track_inventory_item_required()}</span>
      {#if isEditingRequired}
        <div class="flex flex-col items-center gap-1">
          <div class="flex items-center gap-2">
            <input
              type="number"
              min="0"
              bind:value={requiredInput}
              onkeydown={handleKeydown}
              disabled={isSaving}
              class="input w-20 rounded px-2 py-1 text-center text-sm"
            />
            <button
              onclick={saveRequired}
              disabled={isSaving}
              class="variant-filled-primary btn btn-sm"
            >
              Save
            </button>
            <button onclick={cancelEditing} disabled={isSaving} class="variant-ghost btn btn-sm">
              Cancel
            </button>
          </div>
          {#if error}
            <span class="text-xs text-error-500">{error}</span>
          {/if}
        </div>
      {:else}
        <button
          onclick={startEditing}
          class="group flex items-center gap-1 transition-colors hover:text-primary-500"
        >
          <span class="text-lg font-semibold">{item.required}</span>
          <Edit2 size={14} class="opacity-0 transition-opacity group-hover:opacity-100" />
        </button>
      {/if}
    </div>
  </div>
</div>
