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

<aside class="flex flex-col gap-4">
  <div class="flex items-center justify-between px-2">
    <h2 class="text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.wishlists_sidebar_title()}
    </h2>
  </div>

  <div class="space-y-0.5">
    {#if wishlists.length === 0}
      <div class="flex flex-col items-center justify-center py-8 text-center">
        <Heart size={28} class="mb-2 text-[#808080] opacity-30" />
        <p class="text-[10px] font-medium tracking-widest text-[#808080] uppercase">
          {m.wishlists_items_empty()}
        </p>
      </div>
    {:else}
      {#each wishlists as wl (wl.id)}
        <div
          role="button"
          tabindex="0"
          class={cn(
            'group relative flex w-full cursor-pointer items-center justify-between gap-3 py-2.5 pr-3 pl-3 text-left transition-all duration-200',
            wl.id === activeId
              ? 'border-l-2 border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#E0E0E0]'
              : 'border-l-2 border-transparent text-[#808080] hover:bg-[rgba(212,138,66,0.05)] hover:text-[#E0E0E0]'
          )}
          onclick={() => onSelect?.(wl.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect?.(wl.id)}
        >
          <div class="flex min-w-0 items-center gap-3">
            <Heart
              size={16}
              class={wl.id === activeId
                ? 'shrink-0 text-[#D48A42]'
                : 'shrink-0 text-[#808080] transition-colors group-hover:text-[#D48A42]'}
            />
            <div class="flex min-w-0 flex-col">
              <span class="truncate text-sm font-bold tracking-tight">{wl.name}</span>
              <span class="font-mono text-[10px] tracking-wider text-[#808080]">
                {wl.count}
                {m.stats_rolling_stocks()}
              </span>
            </div>
          </div>

          <div class="flex shrink-0 items-center gap-2">
            {#if wl.isDefault}
              <Badge
                class="border border-[#D48A42]/20 bg-[rgba(212,138,66,0.1)] text-[9px] font-bold text-[#D48A42] uppercase"
              >
                {m.wishlists_sidebar_default_badge()}
              </Badge>
            {/if}

            <button
              class="rounded-[8px] border border-[#1F1F1F] p-1 text-[#808080] opacity-0 transition-all group-hover:opacity-100 hover:border-red-800/40 hover:bg-red-900/20 hover:text-red-400"
              type="button"
              title={m.wishlists_delete_list_title()}
              onclick={(event) => {
                event.stopPropagation();
                onDelete?.(wl.id);
              }}
            >
              <Trash2 size={13} />
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</aside>
