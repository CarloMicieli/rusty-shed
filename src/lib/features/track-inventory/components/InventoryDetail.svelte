<script lang="ts">
  import type { TrackInventoryView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import InventoryItemRow from './InventoryItemRow.svelte';
  import PurchaseHistory from './PurchaseHistory.svelte';
  import { ChevronLeft, Edit, Trash2 } from 'lucide-svelte';
  import { resolve } from '$app/paths';

  interface Props {
    inventory: TrackInventoryView;
    onRename?: () => void;
    onDelete?: () => void;
    onAddPurchase?: () => void;
  }

  const { inventory, onRename, onDelete, onAddPurchase }: Props = $props();
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex items-start justify-between">
    <div class="flex items-center gap-4">
      <a href={resolve('/my-tracks')} class="variant-ghost-surface btn-icon btn-icon-sm">
        <ChevronLeft size={20} />
      </a>
      <div>
        <h1 class="h2 font-bold">{inventory.name}</h1>
        {#if inventory.description}
          <p class="text-surface-300">{inventory.description}</p>
        {/if}
      </div>
    </div>

    <div class="flex items-center gap-2">
      {#if onRename}
        <button
          onclick={onRename}
          class="variant-ghost-surface btn-icon btn-icon-sm"
          title={m.track_inventory_rename_button()}
        >
          <Edit size={18} />
        </button>
      {/if}
      {#if onDelete}
        <button
          onclick={onDelete}
          class="variant-ghost-surface btn-icon btn-icon-sm hover:text-error-500"
          title={m.track_inventory_delete_button()}
        >
          <Trash2 size={18} />
        </button>
      {/if}
    </div>
  </div>

  <!-- Action buttons -->
  <div class="flex gap-3">
    {#if onAddPurchase}
      <button onclick={onAddPurchase} class="variant-filled-primary btn gap-2">
        {m.track_inventory_detail_add_purchase()}
      </button>
    {/if}
  </div>

  <!-- Items list -->
  <div class="space-y-4">
    <h3 class="h4 font-semibold">{m.track_inventory_detail_items_tab()}</h3>
    {#if inventory.items.length === 0}
      <div class="variant-ghost-surface rounded-lg p-8 text-center">
        <p class="text-surface-400">{m.track_inventory_detail_empty_items()}</p>
      </div>
    {:else}
      <div class="space-y-3">
        {#each inventory.items as item (item.track_id)}
          <InventoryItemRow {item} inventoryId={inventory.id} />
        {/each}
      </div>
    {/if}
  </div>

  <!-- Purchase history -->
  <div class="space-y-4">
    <h3 class="h4 font-semibold">{m.track_inventory_detail_history_tab()}</h3>
    <PurchaseHistory purchases={inventory.purchases} />
  </div>
</div>
