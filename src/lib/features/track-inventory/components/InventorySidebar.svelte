<script lang="ts">
  import { TrainTrack } from 'lucide-svelte';
  import type { TrackInventoryListItem } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import { cn } from '$lib/utils';

  const { inventories, activeId, onSelect } = $props<{
    inventories: TrackInventoryListItem[];
    activeId: string | null;
    onSelect?: (id: string) => void;
  }>();
</script>

<aside class="space-y-4">
  <div class="px-2">
    <h2 class="text-xs font-bold tracking-[0.2em] text-zinc-500 uppercase">
      {m.track_inventory_sidebar_title()}
    </h2>
  </div>

  <div class="space-y-1">
    {#if inventories.length === 0}
      <div class="flex flex-col items-center justify-center py-8 text-center text-zinc-600">
        <TrainTrack size={32} class="mb-2 opacity-20" />
        <p class="text-xs font-medium tracking-widest uppercase">
          {m.track_inventories_empty_title()}
        </p>
      </div>
    {:else}
      {#each inventories as inv (inv.id)}
        <div
          role="button"
          tabindex="0"
          class={cn(
            'group flex w-full cursor-pointer items-center gap-3 rounded-lg px-3 py-3 transition-all duration-200',
            inv.id === activeId
              ? 'border-l-2 border-amber-500 bg-amber-500/10 text-white shadow-[0_0_15px_rgba(245,158,11,0.05)]'
              : 'text-zinc-400 hover:bg-white/5'
          )}
          onclick={() => onSelect?.(inv.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect?.(inv.id)}
        >
          <TrainTrack
            size={18}
            class={inv.id === activeId
              ? 'text-amber-500'
              : 'text-zinc-600 transition-colors group-hover:text-zinc-400'}
          />
          <div class="flex min-w-0 flex-col">
            <span class="truncate font-bold tracking-tight">{inv.name}</span>
            <span class="font-mono text-[10px] tracking-wider text-zinc-500">
              {inv.total_quantity} pcs
            </span>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</aside>
