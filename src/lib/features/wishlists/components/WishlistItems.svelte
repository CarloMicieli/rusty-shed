<script lang="ts">
  import { Heart } from 'lucide-svelte';
  import type { WishlistItem, WishlistPreview } from '$lib/bindings';
  import { resolveTagIcon } from '$lib/config/icons';
  import * as m from '$lib/paraglide/messages.js';

  const { items, activeWishlistId, otherTargets, onRemove, onMove } = $props<{
    items: WishlistItem[];
    activeWishlistId: string | null;
    otherTargets: WishlistPreview[];
    onRemove?: (detail: { itemId: string; wishlistId: string }) => void;
    onMove?: (detail: { itemId: string; fromId: string; toId: string }) => void;
  }>();

  const moveTargets = $state<Record<string, string>>({});
  const fallbackIcon = resolveTagIcon('heart');
  const FallbackIcon = fallbackIcon;

  function handleMoveSelect(itemId: string, destId: string) {
    moveTargets[itemId] = destId;
  }

  function handleMove(itemId: string) {
    const fromId = activeWishlistId;
    if (!fromId) return;
    const destination = moveTargets[itemId] ?? otherTargets[0]?.id;
    if (!destination) return;
    onMove?.({ itemId, fromId, toId: destination });
  }
</script>

{#if activeWishlistId && items.length === 0}
  <div
    class="border-surface-700/60 text-surface-400 col-span-full rounded-xl border border-dashed p-6 text-center"
  >
    {m.wishlists_items_empty()}
  </div>
{:else if items.length > 0}
  {#each items as item (item.id)}
    <div class="border-surface-700/50 bg-surface-800 rounded-xl border p-4 shadow-sm">
      <div class="mb-3 flex items-center gap-2">
        {#if FallbackIcon}
          <FallbackIcon size={16} class="text-accent-400" />
        {:else}
          <Heart size={16} class="text-accent-400" />
        {/if}
        <span class="text-sm font-semibold">{item.railway_model_id as unknown as string}</span>
      </div>
      <div class="text-surface-400 flex items-center justify-between text-xs">
        <span>{item.status}</span>
        <span>{item.priority}</span>
      </div>
      <div class="mt-3 flex items-center gap-2">
        <button
          class="variant-ghost-surface btn btn-sm"
          onclick={() =>
            activeWishlistId && onRemove?.({ itemId: item.id, wishlistId: activeWishlistId })}
        >
          Delete
        </button>
        <div class="flex flex-1 items-center gap-2">
          <select
            class="select-sm variant-ghost-surface select w-full"
            disabled={otherTargets.length === 0}
            value={moveTargets[item.id] ?? otherTargets[0]?.id ?? ''}
            onchange={(event) =>
              handleMoveSelect(item.id, (event.currentTarget as HTMLSelectElement).value)}
          >
            {#if otherTargets.length === 0}
              <option value="" disabled selected>Create another list to move</option>
            {:else}
              {#each otherTargets as target (target.id)}
                <option value={target.id}>{target.name}</option>
              {/each}
            {/if}
          </select>
          <button
            class="variant-soft-primary btn btn-sm"
            disabled={otherTargets.length === 0}
            onclick={() => handleMove(item.id)}
          >
            Move
          </button>
        </div>
      </div>
    </div>
  {/each}
{/if}
