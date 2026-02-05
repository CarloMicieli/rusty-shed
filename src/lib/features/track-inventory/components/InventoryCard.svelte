<script lang="ts">
  import type { TrackInventoryListItem } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import { resolve } from '$app/paths';
  import { Package } from 'lucide-svelte';

  interface Props {
    inventory: TrackInventoryListItem;
  }

  const { inventory }: Props = $props();
</script>

<a
  href={resolve(`/my-tracks/${inventory.id}`)}
  class="variant-ghost-surface hover:variant-soft-primary card flex flex-col gap-3 p-6 transition-all hover:scale-[1.02]"
>
  <div class="flex items-start justify-between">
    <div class="flex items-center gap-3">
      <div class="variant-filled-primary flex h-12 w-12 items-center justify-center rounded-lg">
        <Package size={24} />
      </div>
      <div>
        <h3 class="h4 font-bold">{inventory.name}</h3>
        {#if inventory.description}
          <p class="text-surface-300 text-sm">{inventory.description}</p>
        {/if}
      </div>
    </div>
  </div>

  <div class="border-surface-700/50 flex items-center gap-6 border-t pt-3">
    <div class="flex flex-col">
      <span class="text-surface-400 text-xs">{m.track_inventories_card_total_quantity()}</span>
      <span class="text-accent-500 h3 font-bold">{inventory.total_quantity}</span>
    </div>
    {#if inventory.total_items > 0}
      <div class="flex flex-col">
        <span class="text-surface-400 text-xs">Types</span>
        <span class="text-lg font-semibold">{inventory.total_items}</span>
      </div>
    {/if}
  </div>
</a>
