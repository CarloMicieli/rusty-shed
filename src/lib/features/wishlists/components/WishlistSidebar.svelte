<script lang="ts">
  import { Sparkles, Heart } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';

  const { wishlists, activeId, onSelect, onCreate, onDelete } = $props<{
    wishlists: WishlistPreview[];
    activeId: string | null;
    onSelect?: (id: string) => void;
    onCreate?: () => void;
    onDelete?: (id: string) => void;
  }>();
</script>

<aside class="space-y-4 rounded-2xl border border-surface-700/50 bg-surface-900 p-4">
  <div class="flex items-center justify-between">
    <h2 class="h5 font-semibold tracking-tight">Wishlists</h2>
    <button class="variant-soft-primary btn btn-sm" onclick={() => onCreate?.()}>
      <Sparkles size={16} />
      <span>Create New List</span>
    </button>
  </div>

  <div class="space-y-2">
    {#if wishlists.length === 0}
      <p class="text-sm text-surface-400">Wishlist is empty</p>
    {:else}
      {#each wishlists as wl (wl.id)}
        <div
          role="button"
          tabindex="0"
          class="btn w-full justify-between gap-3 text-left"
          class:variant-filled-primary={wl.id === activeId}
          class:variant-ghost-surface={wl.id !== activeId}
          onclick={() => onSelect?.(wl.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect?.(wl.id)}
        >
          <div class="flex items-center gap-2">
            <Heart size={16} />
            <div class="flex flex-col">
              <span class="font-semibold">{wl.name}</span>
              <span class="text-xs text-surface-400">{wl.count} items</span>
            </div>
          </div>
          <div class="flex items-center gap-2">
            {#if wl.is_default}
              <span class="variant-soft-primary badge text-[10px] uppercase">Default</span>
            {/if}
            <button
              class="btn-icon btn btn-icon-sm"
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
