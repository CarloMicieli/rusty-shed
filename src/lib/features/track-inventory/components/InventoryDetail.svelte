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
      <TrainTrack size={20} class="text-primary" />
      <h3 class="font-bebas text-2xl tracking-widest text-foreground uppercase">
        {m.track_inventory_detail_items_tab()}
      </h3>
      <div
        class="ml-auto flex items-center gap-2 rounded-sm border border-border bg-card px-3 py-1 font-mono text-[10px] font-bold text-muted-foreground uppercase"
      >
        <span class="text-foreground">{inventory.items.length}</span>
        TYPES
      </div>
    </div>

    {#if inventory.items.length === 0}
      <div
        class="flex flex-col items-center justify-center rounded-sm border border-dashed border-border bg-card/30 py-20 text-center"
      >
        <TrainTrack size={48} class="mb-4 text-zinc-700 opacity-20" />
        <p class="max-w-[200px] text-sm text-zinc-500">
          {m.track_inventory_detail_empty_items()}
        </p>
        <Button
          variant="default"
          class="variant-steampunk-lever mt-6 rounded-sm"
          onclick={onAddPurchase}
        >
          {m.track_inventory_detail_add_first_piece()}
        </Button>
      </div>
    {:else}
      <div class="space-y-3 rounded-sm border border-border bg-card p-4">
        {#each inventory.items as item (item.track_id)}
          <InventoryItemRow {item} inventoryId={inventory.id} />
        {/each}
      </div>
    {/if}
  </div>

  <!-- Purchase History Section -->
  <div class="space-y-6">
    <div class="flex items-center gap-3">
      <History size={20} class="text-primary" />
      <h3 class="font-bebas text-2xl tracking-widest text-foreground uppercase">
        {m.track_inventory_detail_history_tab()}
      </h3>
    </div>

    <div class="rounded-sm border border-border bg-card/50 p-1">
      <PurchaseHistory purchases={inventory.purchases} />
    </div>
  </div>
</div>
