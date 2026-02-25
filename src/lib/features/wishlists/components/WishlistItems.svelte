<script lang="ts">
  import { Heart } from 'lucide-svelte';
  import type { WishlistItem, WishlistPreview } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import WishlistItemCard from './WishlistItemCard.svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components';

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
    if (otherTargets.length === 1) {
      // Direct move if only one target
      onMove?.({ itemId, fromId: activeWishlistId!, toId: otherTargets[0].id });
    } else {
      movingItemId = itemId;
    }
  }

  function handleMoveConfirm(targetId: string) {
    if (movingItemId && activeWishlistId) {
      onMove?.({ itemId: movingItemId, fromId: activeWishlistId, toId: targetId });
      movingItemId = null;
    }
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

<!-- Move To List Dialog -->
{#if movingItemId}
  <Dialog.Root open={!!movingItemId} onOpenChange={() => (movingItemId = null)}>
    <Dialog.Content class="border-zinc-800 bg-[#0c0c0c] text-white">
      <Dialog.Header>
        <Dialog.Title>Move to List</Dialog.Title>
        <Dialog.Description class="text-zinc-500">Select a destination wishlist.</Dialog.Description
        >
      </Dialog.Header>
      <div class="grid gap-2 py-4">
        {#each otherTargets as target (target.id)}
          <button
            onclick={() => handleMoveConfirm(target.id)}
            class="flex items-center justify-between rounded-xl border border-zinc-900 bg-zinc-900/50 px-4 py-3 text-left transition-all hover:border-amber-500/30 hover:bg-zinc-800"
          >
            <span class="font-bold">{target.name}</span>
            <span class="text-xs text-zinc-500">{target.count} items</span>
          </button>
        {/each}
      </div>
      <Dialog.Footer>
        <Button variant="ghost" onclick={() => (movingItemId = null)}>Cancel</Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
