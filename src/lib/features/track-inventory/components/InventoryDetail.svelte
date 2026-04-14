<script lang="ts">
  import type { TrackInventoryView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import TrackItemCard from './TrackItemCard.svelte';
  import PurchaseHistory from './PurchaseHistory.svelte';
  import { TrainTrack, History } from 'lucide-svelte';
  import { EmptyState } from '$lib/components';

  interface Props {
    inventory: TrackInventoryView;
    onAddPurchase?: () => void;
  }

  const { inventory, onAddPurchase }: Props = $props();
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
      <div class="rounded-sm border border-border bg-card p-3">
        <EmptyState
          icon={TrainTrack}
          title={m.track_inventory_detail_items_tab()}
          description={m.track_inventory_detail_empty_items()}
          ctaLabel={m.track_inventory_detail_add_first_piece()}
          onCta={onAddPurchase}
        />
      </div>
    {:else}
      <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
        {#each inventory.items as item (item.track_id)}
          <TrackItemCard {item} inventoryId={inventory.id} />
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
