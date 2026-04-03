<script lang="ts">
  import type { TrackInventoryItemView } from '$lib/features/track-inventory';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import { AlertTriangle, Edit2, Check, X } from 'lucide-svelte';
  import { Input } from '$lib/components';

  interface Props {
    item: TrackInventoryItemView;
    inventoryId: string;
  }

  const { item, inventoryId }: Props = $props();
  const service = getTrackInventoryContext();

  let isEditingRequired = $state(false);
  let requiredInput = $state('');
  let isSaving = $state(false);
  let error = $state<string | null>(null);

  const hasShortage = $derived(
    Number(item.required) > 0 && Number(item.quantity) < Number(item.required)
  );

  const rowClass = $derived(
    hasShortage ? 'bg-destructive/5 border-destructive/30' : 'bg-card border-border'
  );
  const shortageCount = $derived(Number(item.required) - Number(item.quantity));

  async function saveRequired() {
    const newRequired = parseInt(requiredInput, 10);
    if (isNaN(newRequired) || newRequired < 0) {
      error = 'Invalid value';
      return;
    }

    isSaving = true;
    error = null;
    try {
      await service.setItemRequired(inventoryId, item.track_product.track_id, newRequired);
      item.required = Number(newRequired);
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
  class="group relative flex flex-col gap-4 overflow-hidden rounded-sm border p-5 shadow-[2px_2px_0px_0px_var(--border)] transition-all duration-300 {rowClass}"
>
  <div class="flex items-start justify-between">
    <div class="flex flex-col gap-1">
      <div class="flex items-center gap-3">
        <h4 class="text-base font-bold text-foreground">
          {item.track_product.description || item.track_product.product_code}
        </h4>
        {#if hasShortage}
          <div
            class="flex items-center gap-1.5 rounded-none border-l-4 border-destructive bg-destructive/10 px-2 py-1 font-mono text-destructive"
          >
            <AlertTriangle size={12} />
            <span class="text-[10px] font-bold tracking-wider uppercase">
              {m.track_inventory_item_shortage({ count: shortageCount })}
            </span>
          </div>
        {/if}
      </div>
      <div
        class="flex flex-wrap items-center gap-x-4 gap-y-1 text-[11px] font-medium tracking-wide text-muted-foreground uppercase"
      >
        <span>{item.track_product.manufacturer_name}</span>
        <span class="h-1 w-1 rounded-full bg-border"></span>
        <span class="text-foreground/60">{item.track_product.product_code}</span>
        <span class="h-1 w-1 rounded-full bg-border"></span>
        <span>{item.track_product.track_type}</span>
      </div>
    </div>

    <div class="flex items-center gap-8">
      <!-- Quantity - Ledger Style -->
      <div class="flex flex-col items-end">
        <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
          {m.track_inventory_item_quantity()}
        </span>
        <span
          class="font-mono text-2xl font-bold tracking-tighter"
          class:text-destructive={hasShortage}
          class:text-foreground={!hasShortage}
        >
          {item.quantity.toString().padStart(2, '0')}
        </span>
      </div>

      <!-- Required - Ledger Style -->
      <div class="flex flex-col items-end">
        <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
          {m.track_inventory_item_required()}
        </span>

        {#if isEditingRequired}
          <div class="mt-1 flex items-center gap-2">
            <Input
              type="number"
              min="0"
              bind:value={requiredInput}
              onkeydown={handleKeydown}
              disabled={isSaving}
              class="h-9 w-16 border-border bg-background px-2 text-center font-mono text-sm focus:ring-primary"
              autofocus
            />
            <button
              onclick={saveRequired}
              disabled={isSaving}
              class="variant-steampunk-lever flex h-9 w-9 items-center justify-center rounded-sm bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              <Check size={16} />
            </button>
            <button
              onclick={cancelEditing}
              disabled={isSaving}
              class="flex h-9 w-9 items-center justify-center rounded-sm bg-card text-muted-foreground hover:bg-card/80 disabled:opacity-50"
            >
              <X size={16} />
            </button>
          </div>
        {:else}
          <button
            onclick={startEditing}
            class="group/edit relative mt-1 flex items-baseline gap-2 transition-colors hover:text-primary"
          >
            <span
              class="font-mono text-2xl font-bold tracking-tighter text-muted-foreground group-hover/edit:text-primary"
            >
              {item.required.toString().padStart(2, '0')}
            </span>
            <Edit2 size={12} class="opacity-0 transition-opacity group-hover/edit:opacity-100" />
          </button>
        {/if}
      </div>
    </div>
  </div>

  {#if error}
    <div class="mt-2 text-[10px] font-bold tracking-widest text-red-500 uppercase">
      ERROR: {error}
    </div>
  {/if}
</div>
