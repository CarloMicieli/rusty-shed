<script lang="ts">
  import { Heart, Trash2 } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';
  import { Badge } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';
  import { cn } from '$lib/utils';

  const { wishlists, activeId, onSelect, onDelete } = $props<{
    wishlists: WishlistPreview[];
    activeId: string | null;
    onSelect?: (id: string) => void;
    onDelete?: (id: string) => void;
  }>();
</script>

<aside class="space-y-4">
  <div class="flex items-center justify-between px-2">
    <h2 class="text-xs font-bold tracking-[0.2em] text-zinc-500 uppercase">
      {m.wishlists_sidebar_title()}
    </h2>
  </div>

  <div class="space-y-1">
    {#if wishlists.length === 0}
      <div class="flex flex-col items-center justify-center py-8 text-center text-zinc-600">
        <Heart size={32} class="mb-2 opacity-20" />
        <p class="text-xs font-medium tracking-widest uppercase">{m.wishlists_items_empty()}</p>
      </div>
    {:else}
      {#each wishlists as wl (wl.id)}
        <div
          role="button"
          tabindex="0"
          class={cn(
            'group relative flex w-full cursor-pointer items-center justify-between gap-3 rounded-lg px-3 py-3 text-left transition-all duration-200',
            wl.id === activeId
              ? 'border-l-2 border-amber-500 bg-amber-500/10 text-white shadow-[0_0_15px_rgba(245,158,11,0.05)]'
              : 'text-zinc-400 hover:bg-white/5'
          )}
          onclick={() => onSelect?.(wl.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect?.(wl.id)}
        >
          <div class="flex items-center gap-3">
            <Heart
              size={18}
              class={wl.id === activeId
                ? 'text-amber-500'
                : 'text-zinc-600 transition-colors group-hover:text-zinc-400'}
            />
            <div class="flex min-w-0 flex-col">
              <span class="truncate font-bold tracking-tight">{wl.name}</span>
              <span class="font-mono text-[10px] tracking-wider text-zinc-500">
                {wl.count}
                {m.stats_rolling_stocks()}
              </span>
            </div>
          </div>

          <div class="flex items-center gap-2">
            {#if wl.isDefault}
              <Badge
                class="border-amber-500/20 bg-amber-500/10 text-[9px] font-bold text-amber-500 uppercase ring-1 ring-amber-500/20"
              >
                {m.wishlists_sidebar_default_badge()}
              </Badge>
            {/if}

            <button
              class="rounded p-1 text-zinc-600 opacity-0 transition-all group-hover:opacity-100 hover:bg-zinc-800 hover:text-red-400"
              type="button"
              title={m.wishlists_delete_list_title()}
              onclick={(event) => {
                event.stopPropagation();
                onDelete?.(wl.id);
              }}
            >
              <Trash2 size={14} />
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</aside>
