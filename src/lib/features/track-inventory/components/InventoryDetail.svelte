<script lang="ts">
  import type { TrackInventoryView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import TrackItemCard from './TrackItemCard.svelte';
  import PurchaseHistory from './PurchaseHistory.svelte';
  import { TrainTrack, History } from 'lucide-svelte';
  import { Button } from '$lib/components';

  interface Props {
    inventory: TrackInventoryView;
    onAddPurchase?: () => void;
    onInventoryChanged?: () => void | Promise<void>;
  }

  const { inventory, onAddPurchase, onInventoryChanged }: Props = $props();
</script>

<div class="space-y-12">
  <!-- Items List Section -->
  <section class="space-y-6 rounded-[8px] border border-layout-border bg-layout-surface p-4">
    <div class="flex items-center gap-3">
      <TrainTrack size={20} class="text-primary" />
      <h3 class="font-bebas text-2xl tracking-widest text-foreground uppercase">
        {m.track_inventory_detail_items_tab()}
      </h3>
      <div
        class="ml-auto flex items-center gap-2 rounded-sm border border-border bg-card px-3 py-1 font-mono text-xs text-muted-foreground uppercase"
      >
        <span class="text-foreground">{inventory.items.length}</span>
        {m.track_inventory_detail_types_label()}
      </div>
    </div>

    {#if inventory.items.length === 0}
      <div
        class="flex flex-col items-center justify-center gap-4 rounded-sm border border-dashed border-border bg-background/50 px-4 py-10 text-center"
      >
        <div
          class="flex h-12 w-12 items-center justify-center rounded-full border border-border bg-card text-muted-foreground"
        >
          <TrainTrack size={22} />
        </div>
        <div class="max-w-sm space-y-1.5">
          <h4 class="font-bebas text-xl tracking-widest text-foreground uppercase">
            {m.track_inventory_detail_items_tab()}
          </h4>
          <p class="text-sm text-muted-foreground">
            {m.track_inventory_detail_empty_items()}
          </p>
        </div>
        <Button
          variant="outline"
          size="sm"
          class="cursor-pointer rounded-sm border-primary text-primary transition-all duration-150 ease-out hover:bg-primary/10"
          onclick={onAddPurchase}
        >
          {m.track_inventory_detail_add_first_piece()}
        </Button>
      </div>
    {:else}
      <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
        {#each inventory.items as item (item.track_id)}
          <TrackItemCard {item} inventoryId={inventory.id} onDeleted={onInventoryChanged} />
        {/each}
      </div>
    {/if}
  </section>

  <!-- Purchase History Section -->
  <section class="space-y-6 rounded-[8px] border border-layout-border bg-layout-surface p-4">
    <div class="flex items-center gap-3">
      <History size={20} class="text-primary" />
      <h3 class="font-bebas text-2xl tracking-widest text-foreground uppercase">
        {m.track_inventory_detail_history_tab()}
      </h3>
    </div>

    <div class="rounded-sm border border-border bg-card p-4">
      <PurchaseHistory purchases={inventory.purchases} />
    </div>
  </section>
</div>
