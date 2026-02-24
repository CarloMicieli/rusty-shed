<script lang="ts">
  import type { TrackInventoryListItem } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import { resolve } from '$app/paths';
  import { TrainTrack, ChevronRight } from 'lucide-svelte';

  interface Props {
    inventory: TrackInventoryListItem;
  }

  const { inventory }: Props = $props();
</script>

<a
  href={resolve(`/railway-tracks/${inventory.id}`)}
  class="group relative flex flex-col gap-6 overflow-hidden rounded-2xl border border-white/10 bg-[#0c0c0c] p-6 transition-all duration-300 hover:-translate-y-1 hover:border-amber-500/50 hover:shadow-[0_0_30px_rgba(245,158,11,0.1)]"
>
  <!-- Subtle gradient glow -->
  <div
    class="absolute -top-12 -right-12 h-32 w-32 rounded-full bg-amber-500/5 blur-[60px] transition-all group-hover:bg-amber-500/10"
  ></div>

  <div class="relative z-10 flex items-start justify-between">
    <div class="flex gap-4">
      <div
        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl border border-white/5 bg-zinc-900 text-zinc-400 transition-colors group-hover:border-amber-500/20 group-hover:text-amber-500"
      >
        <TrainTrack size={24} />
      </div>
      <div>
        <h3 class="text-xl font-bold text-zinc-100 transition-colors group-hover:text-white">
          {inventory.name}
        </h3>
        {#if inventory.description}
          <p class="mt-1 line-clamp-2 text-sm leading-relaxed text-zinc-500">
            {inventory.description}
          </p>
        {/if}
      </div>
    </div>
  </div>

  <div class="relative z-10 mt-auto flex items-center gap-3">
    <div
      class="rounded-full border border-white/5 bg-zinc-900/60 px-3 py-1.5 text-[10px] font-bold tracking-wider text-zinc-300 uppercase"
    >
      {inventory.total_quantity}
      {m.track_inventories_card_total_quantity()}
    </div>
    {#if inventory.total_items > 0}
      <div
        class="rounded-full border border-white/5 bg-zinc-900/60 px-3 py-1.5 text-[10px] font-bold tracking-wider text-zinc-400 uppercase"
      >
        {inventory.total_items} Types
      </div>
    {/if}

    <div
      class="ml-auto -translate-x-2 opacity-0 transition-all duration-300 group-hover:translate-x-0 group-hover:opacity-100"
    >
      <ChevronRight size={18} class="text-amber-500" />
    </div>
  </div>
</a>
