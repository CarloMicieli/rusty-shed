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
    <h2 class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
      {m.wishlists_sidebar_title()}
    </h2>
  </div>

  <div class="space-y-0.5">
    {#if wishlists.length === 0}
      <div class="flex flex-col items-center justify-center py-8 text-center">
        <Heart size={28} class="mb-2 text-muted-foreground opacity-30" />
        <p class="text-[10px] font-medium tracking-widest text-muted-foreground uppercase">
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
              ? 'border-l-2 border-primary bg-primary/15 text-foreground'
              : 'border-l-2 border-transparent text-muted-foreground hover:bg-primary/5 hover:text-foreground'
          )}
          onclick={() => onSelect?.(wl.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect?.(wl.id)}
        >
          <div class="flex min-w-0 items-center gap-3">
            <Heart
              size={16}
              class={wl.id === activeId
                ? 'shrink-0 text-primary'
                : 'shrink-0 text-muted-foreground transition-colors group-hover:text-primary'}
            />
            <div class="flex min-w-0 flex-col">
              <span class="truncate text-sm font-bold tracking-tight">{wl.name}</span>
              <span class="font-mono text-[10px] tracking-wider text-muted-foreground">
                {wl.count}
                {m.stats_rolling_stocks()}
              </span>
            </div>
          </div>

          <div class="flex shrink-0 items-center gap-2">
            {#if wl.isDefault}
              <Badge
                class="border border-primary/20 bg-primary/10 text-[9px] font-bold text-primary uppercase"
              >
                {m.wishlists_sidebar_default_badge()}
              </Badge>
            {/if}

            <button
              class="rounded-[8px] border border-layout-border p-1 text-muted-foreground opacity-0 transition-all group-hover:opacity-100 hover:border-red-800/40 hover:bg-red-900/20 hover:text-red-400"
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
