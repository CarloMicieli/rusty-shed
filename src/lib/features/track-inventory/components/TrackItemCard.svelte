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
  const shortageCount = $derived(Math.max(0, Number(item.required) - Number(item.quantity)));

  async function saveRequired() {
    const newRequired = parseInt(requiredInput, 10);
    if (isNaN(newRequired) || newRequired < 0) {
      error = m.track_inventory_invalid_required();
      return;
    }

    isSaving = true;
    error = null;
    try {
      await service.setItemRequired(inventoryId, item.track_product.track_id, newRequired);
      item.required = Number(newRequired);
      isEditingRequired = false;
    } catch {
      error = m.track_inventory_update_failed();
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
      void saveRequired();
    } else if (event.key === 'Escape') {
      cancelEditing();
    }
  }
</script>

<article
  class="flex h-full flex-col gap-4 rounded-sm border border-border bg-card p-4 transition-all duration-150 ease-out hover:border-primary/40"
>
  <header class="space-y-2">
    <p class="text-xs tracking-wider text-muted-foreground uppercase">
      {item.track_product.manufacturer_name}
    </p>
    <h4 class="line-clamp-2 font-bebas text-xl tracking-widest text-foreground uppercase">
      {item.track_product.description || item.track_product.product_code}
    </h4>
    <div class="flex flex-wrap items-center gap-2">
      <span
        class="rounded-sm border border-border bg-background/50 px-2 py-1 font-mono text-xs text-muted-foreground uppercase"
      >
        {item.track_product.product_code}
      </span>
      <span
        class="rounded-sm border border-border bg-background/50 px-2 py-1 font-mono text-xs text-muted-foreground uppercase"
      >
        {item.track_product.track_type}
      </span>
    </div>
  </header>

  {#if hasShortage}
    <div
      class="flex items-center gap-2 rounded-sm border border-destructive/40 bg-destructive/10 px-3 py-2 text-destructive"
    >
      <AlertTriangle size={14} />
      <span class="text-xs font-medium tracking-wider uppercase">
        {m.track_inventory_item_shortage({ count: shortageCount })}
      </span>
    </div>
  {/if}

  <div class="grid grid-cols-2 gap-3">
    <div class="rounded-sm border border-border bg-background/50 p-3">
      <p class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.track_inventory_item_quantity()}
      </p>
      <p class="font-mono text-2xl leading-none font-bold text-foreground">
        {item.quantity.toString().padStart(2, '0')}
      </p>
    </div>

    <div class="rounded-sm border border-border bg-background/50 p-3">
      <p class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.track_inventory_item_required()}
      </p>

      {#if isEditingRequired}
        <div class="mt-2 flex items-center gap-2">
          <Input
            type="number"
            min="0"
            bind:value={requiredInput}
            onkeydown={handleKeydown}
            disabled={isSaving}
            class="h-8 w-16 border-border bg-card px-2 text-center font-mono text-sm"
            autofocus
          />
          <button
            type="button"
            onclick={() => void saveRequired()}
            disabled={isSaving}
            aria-label={m.save_button()}
            class="inline-flex h-8 w-8 cursor-pointer items-center justify-center rounded-sm bg-primary text-primary-foreground transition-all duration-150 ease-out hover:brightness-110 active:scale-95 disabled:opacity-50"
          >
            <Check size={14} />
          </button>
          <button
            type="button"
            onclick={cancelEditing}
            disabled={isSaving}
            aria-label={m.app_cancel()}
            class="inline-flex h-8 w-8 cursor-pointer items-center justify-center rounded-sm border border-border bg-card text-muted-foreground transition-all duration-150 ease-out hover:bg-muted hover:text-foreground disabled:opacity-50"
          >
            <X size={14} />
          </button>
        </div>
      {:else}
        <button
          type="button"
          onclick={startEditing}
          class="mt-1 inline-flex cursor-pointer items-center gap-2 font-mono text-2xl leading-none font-bold text-foreground transition-all duration-150 ease-out hover:text-primary"
        >
          {item.required.toString().padStart(2, '0')}
          <Edit2 size={12} />
        </button>
      {/if}
    </div>
  </div>

  {#if error}
    <p class="text-xs tracking-wider text-destructive uppercase">{error}</p>
  {/if}
</article>
