<script lang="ts">
  import type { TrackInventoryView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import InventoryItemRow from './InventoryItemRow.svelte';
  import PurchaseHistory from './PurchaseHistory.svelte';
  import { TrainTrack, History } from 'lucide-svelte';
  import { Button } from '$lib/components';

  interface Props {
    inventory: TrackInventoryView;
    onAddPurchase?: () => void;
  }

  const { inventory, onAddPurchase }: Props = $props();
</script>

<div class="space-y-12">
  <!-- Items List Section -->
  <div class="space-y-6">
    <div class="flex items-center gap-3">
      <TrainTrack size={20} class="text-amber-500/60" />
      <h3 class="text-lg font-bold tracking-tight text-zinc-200">
        {m.track_inventory_detail_items_tab()}
      </h3>
      <div
        class="ml-auto flex items-center gap-2 rounded-full bg-zinc-900/50 px-3 py-1 text-[10px] font-bold text-zinc-500"
      >
        <span class="text-zinc-300">{inventory.items.length}</span>
        TYPES
      </div>
    </div>

    {#if inventory.items.length === 0}
      <div
        class="flex flex-col items-center justify-center rounded-3xl border border-dashed border-white/5 bg-zinc-900/10 py-20 text-center"
      >
        <TrainTrack size={48} class="mb-4 text-zinc-700 opacity-20" />
        <p class="max-w-[200px] text-sm text-zinc-500">
          {m.track_inventory_detail_empty_items()}
        </p>
        <Button variant="rusty" class="mt-6 shadow-lg shadow-amber-500/10" onclick={onAddPurchase}>
          {m.track_inventory_detail_add_first_piece()}
        </Button>
      </div>
    {:else}
      <div class="space-y-3">
        {#each inventory.items as item (item.track_id)}
          <InventoryItemRow {item} inventoryId={inventory.id} />
        {/each}
      </div>
    {/if}
  </div>

  <!-- Purchase History Section -->
  <div class="space-y-6">
    <div class="flex items-center gap-3">
      <History size={20} class="text-zinc-500" />
      <h3 class="text-lg font-bold tracking-tight text-zinc-200">
        {m.track_inventory_detail_history_tab()}
      </h3>
    </div>

    <div class="rounded-2xl border border-white/5 bg-zinc-950/20 p-1">
      <PurchaseHistory purchases={inventory.purchases} />
    </div>
  </div>
</div>
