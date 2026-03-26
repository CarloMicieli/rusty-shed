<script lang="ts">
  import { Heart } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { WishlistItem, WishlistPreview } from '$lib/bindings';
  import WishlistTableRow from './WishlistTableRow.svelte';
  import MoveModelModal from './MoveModelModal.svelte';

  interface Props {
    items: WishlistItem[];
    activeWishlistId: string | null;
    otherTargets: WishlistPreview[];
    onRemove?: (detail: { itemId: string; wishlistId: string }) => void;
    onMove?: (detail: { itemId: string; fromId: string; toId: string }) => void;
    onPurchase?: (itemId: string) => void;
  }

  const { items, activeWishlistId, otherTargets, onRemove, onMove, onPurchase }: Props = $props();

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
    class="flex h-48 flex-col items-center justify-center gap-3 rounded-[8px] border border-layout-border bg-layout-surface"
  >
    <Heart size={36} class="text-muted-foreground opacity-20" />
    <p class="font-mono text-[10px] font-medium tracking-widest text-muted-foreground uppercase">
      {m.wishlists_items_empty()}
    </p>
  </div>
{:else if items.length > 0}
  <div class="overflow-hidden rounded-[8px] border border-layout-border bg-layout-surface">
    <table class="w-full">
      <thead>
        <tr class="border-b border-layout-border">
          <!-- Priority -->
          <th class="w-10 px-4 py-3 text-left">
            <span
              class="font-mono text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_table_col_priority()}
            </span>
          </th>
          <!-- Model -->
          <th class="px-4 py-3 text-left">
            <span
              class="font-mono text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_table_col_model()}
            </span>
          </th>
          <!-- Product Code -->
          <th class="px-4 py-3 text-left">
            <span
              class="font-mono text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_table_col_product_code()}
            </span>
          </th>
          <!-- Price Target -->
          <th class="px-4 py-3 text-left">
            <span
              class="font-mono text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_table_col_price_target()}
            </span>
          </th>
          <!-- Status -->
          <th class="px-4 py-3 text-left">
            <span
              class="font-mono text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_table_col_status()}
            </span>
          </th>
          <!-- Actions -->
          <th class="px-4 py-3 text-right">
            <span
              class="font-mono text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_table_col_actions()}
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        {#each items as item (item.id)}
          <WishlistTableRow
            {item}
            wishlistId={activeWishlistId ?? ''}
            onRemove={handleRemove}
            onMove={handleMoveTrigger}
            {onPurchase}
          />
        {/each}
      </tbody>
    </table>
  </div>
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
