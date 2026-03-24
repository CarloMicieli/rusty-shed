<script lang="ts">
  import { Heart } from 'lucide-svelte';
  import type { WishlistItem, WishlistPreview } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import WishlistItemCard from './WishlistItemCard.svelte';
  import MoveModelModal from './MoveModelModal.svelte';

  const { items, activeWishlistId, otherTargets, onRemove, onMove, onPurchase } = $props<{
    items: WishlistItem[];
    activeWishlistId: string | null;
    otherTargets: WishlistPreview[];
    onRemove?: (detail: { itemId: string; wishlistId: string }) => void;
    onMove?: (detail: { itemId: string; fromId: string; toId: string }) => void;
    onPurchase?: (itemId: string) => void;
  }>();

  const wishlistId = $derived(activeWishlistId ?? '');

  let movingItemId = $state<string | null>(null);

  function handleRemove(itemId: string) {
    if (activeWishlistId) {
      onRemove?.({ itemId, wishlistId: activeWishlistId });
    }
  }

  function handleMoveTrigger(itemId: string) {
    movingItemId = itemId;
  }
</script>

{#if activeWishlistId && items.length === 0}
  <div
    class="col-span-full flex h-64 flex-col items-center justify-center rounded-3xl border border-white/5 bg-white/5 backdrop-blur-md"
  >
    <div class="relative mb-4">
      <div class="absolute inset-0 animate-pulse rounded-full bg-amber-500/5 blur-xl"></div>
      <Heart size={48} class="relative text-zinc-800" />
    </div>
    <p class="text-sm font-medium tracking-widest text-zinc-500 uppercase">
      {m.wishlists_items_empty()}
    </p>
  </div>
{:else if items.length > 0}
  {#each items as item (item.id)}
    <WishlistItemCard
      {item}
      {wishlistId}
      onRemove={handleRemove}
      onMove={handleMoveTrigger}
      onPurchase={onPurchase ? (itemId) => onPurchase(itemId) : undefined}
    />
  {/each}
{/if}

<MoveModelModal
  open={!!movingItemId}
  itemId={movingItemId ?? ''}
  fromWishlistId={activeWishlistId ?? ''}
  {otherTargets}
  onClose={() => (movingItemId = null)}
  onMove={(detail) => {
    onMove?.(detail);
    movingItemId = null;
  }}
/>
