<script lang="ts">
  import { Heart } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';
  import { Badge } from '$lib/components';

  const { wishlists, activeId, onSelect, onDelete } = $props<{
    wishlists: WishlistPreview[];
    activeId: string | null;
    onSelect?: (id: string) => void;
    onDelete?: (id: string) => void;
  }>();
</script>

<aside class="space-y-4 rounded-2xl border border-border bg-sidebar p-4">
  <div class="flex items-center justify-between">
    <h2 class="h5 font-semibold tracking-tight">Wishlists</h2>
  </div>

  <div class="space-y-2">
    {#if wishlists.length === 0}
      <p class="text-surface-400 text-sm">Wishlist is empty</p>
    {:else}
      {#each wishlists as wl (wl.id)}
        <div
          role="button"
          tabindex="0"
          class="flex w-full items-center justify-between gap-3 rounded-md px-3 py-2 text-left transition-colors"
          class:bg-primary={wl.id === activeId}
          class:bg-sidebar-accent={wl.id === activeId}
          class:text-primary-foreground={wl.id === activeId}
          class:hover:bg-sidebar-accent={wl.id !== activeId}
          class:text-sidebar-foreground={wl.id !== activeId}
          onclick={() => onSelect?.(wl.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect?.(wl.id)}
        >
          <div class="flex items-center gap-2">
            <Heart size={16} />
            <div class="flex flex-col">
              <span class="font-semibold">{wl.name}</span>
              <span class="text-xs opacity-75">{wl.count} items</span>
            </div>
          </div>
          <div class="flex items-center gap-2">
            {#if wl.is_default}
              <Badge variant="secondary" class="text-[10px] uppercase">Default</Badge>
            {/if}
            <button
              class="h-6 w-6 rounded hover:bg-accent"
              type="button"
              onclick={(event) => {
                event.stopPropagation();
                onDelete?.(wl.id);
              }}
            >
              ✕
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</aside>
