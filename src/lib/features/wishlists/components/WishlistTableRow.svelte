<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Trash2, ShoppingCart, MoveRight, TrainFront, Box, Users, Layers } from 'lucide-svelte';
  import type { WishlistItem, RailwayModelView, MonetaryAmount } from '$lib/bindings';
  import { onMount } from 'svelte';
  import { commands } from '$lib/bindings';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { goto } from '$app/navigation';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    item: WishlistItem;
    wishlistId: string;
    onRemove?: (itemId: string) => void;
    onMove?: (itemId: string) => void;
    onPurchase?: (itemId: string) => void;
  }

  let { item, wishlistId, onRemove, onMove, onPurchase }: Props = $props();

  let photoUrl = $state<string | null>(null);
  let modelDetails = $state<RailwayModelView | null>(null);

  onMount(async () => {
    try {
      const [modelResult, imageResult] = await Promise.all([
        commands.getRailwayModelById(item.railwayModelId, getLocale()),
        commands.getRailwayModelImage(item.railwayModelId)
      ]);

      if (modelResult.status === 'ok' && modelResult.data) {
        modelDetails = modelResult.data;
      }

      if (imageResult.status === 'ok' && imageResult.data.hasImage && imageResult.data.imagePath) {
        const filePath = imageResult.data.imagePath;
        const ext = filePath.split('.').pop()?.toLowerCase() ?? 'jpg';
        const mimes: Record<string, string> = {
          jpg: 'image/jpeg',
          jpeg: 'image/jpeg',
          png: 'image/png',
          webp: 'image/webp'
        };
        const bytes = await readFile(filePath);
        photoUrl = URL.createObjectURL(new Blob([bytes], { type: mimes[ext] ?? 'image/jpeg' }));
      }
    } catch (e) {
      console.warn('Failed to load model details for wishlist table row', e);
    }
  });

  const desiredPriceStr = $derived.by(() => {
    if (!item.desiredPrice) return null;
    const price = item.desiredPrice as MonetaryAmount;
    return regionalManager.formatCurrencyWith(
      Number(price.amount || 0),
      (price.currency as string) || 'EUR'
    );
  });

  const PlaceholderIcon = $derived.by(() => {
    const cat = modelDetails?.category?.toLowerCase() ?? '';
    if (cat.includes('locomotive')) return TrainFront;
    if (cat.includes('freight')) return Box;
    if (cat.includes('passenger')) return Users;
    if (cat.includes('railcar') || cat.includes('train_set')) return Layers;
    return TrainFront;
  });

  const priorityDotClass = $derived.by(() => {
    if (item.priority === 'HIGH') return 'bg-[#D48A42] shadow-[0_0_4px_rgba(212,138,66,0.6)]';
    if (item.priority === 'NORMAL') return 'bg-[#D48A42]/50';
    return 'bg-zinc-600';
  });

  const statusConfig = $derived.by((): { label: string; classes: string } => {
    switch (item.status) {
      case 'WANTED':
        return {
          label: m.wishlist_item_status_wanted(),
          classes: 'bg-[rgba(212,138,66,0.15)] border-[#D48A42]/30 text-[#D48A42]'
        };
      case 'ON_ORDER':
        return {
          label: m.wishlist_item_status_on_order(),
          classes: 'bg-blue-900/20 border-blue-700/30 text-blue-400'
        };
      case 'PURCHASED':
        return {
          label: m.wishlist_item_status_purchased(),
          classes: 'bg-emerald-900/20 border-emerald-700/30 text-emerald-400'
        };
      case 'IGNORED':
        return {
          label: m.wishlist_item_status_ignored(),
          classes: 'bg-zinc-800/50 border-zinc-700/30 text-zinc-500'
        };
      default:
        return { label: item.status, classes: 'bg-zinc-800/50 border-zinc-700/30 text-zinc-500' };
    }
  });

  function handleRowClick() {
    void goto(`/wishlists/${wishlistId}/items/${item.id}`);
  }

  function handleRowKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      void goto(`/wishlists/${wishlistId}/items/${item.id}`);
    }
  }
</script>

<tr
  class="group cursor-pointer border-b border-[#1F1F1F] transition-colors duration-150 last:border-b-0 hover:bg-[rgba(212,138,66,0.04)]"
  onclick={handleRowClick}
  onkeydown={handleRowKeydown}
  tabindex="0"
>
  <!-- Priority -->
  <td class="px-4 py-3" onclick={(e) => e.stopPropagation()}>
    <div
      class="h-2 w-2 rounded-full {priorityDotClass}"
      title={item.priority === 'HIGH'
        ? m.wishlist_priority_high()
        : item.priority === 'NORMAL'
          ? m.wishlist_priority_normal()
          : m.wishlist_priority_low()}
    ></div>
  </td>

  <!-- Model: thumbnail + brand/name -->
  <td class="px-4 py-3">
    <div class="flex items-center gap-3">
      <!-- Thumbnail -->
      <div
        class="relative h-8 w-8 shrink-0 overflow-hidden rounded-[4px] border border-[#1F1F1F] bg-[#1F1F1F]"
      >
        {#if photoUrl}
          <img src={photoUrl} alt="" aria-hidden="true" class="h-full w-full object-cover" />
        {:else}
          <div class="flex h-full w-full items-center justify-center opacity-30">
            <PlaceholderIcon size={16} class="text-[#808080]" />
          </div>
        {/if}
      </div>
      <!-- Text -->
      <div class="flex min-w-0 flex-col">
        <span class="truncate text-[10px] font-bold tracking-widest text-[#808080] uppercase">
          {modelDetails?.manufacturer.display ?? '—'}
        </span>
        <span class="truncate text-sm font-semibold text-[#E0E0E0]">
          {modelDetails?.description ?? item.railwayModelId}
        </span>
      </div>
    </div>
  </td>

  <!-- Product Code -->
  <td class="px-4 py-3">
    <span class="font-mono text-xs text-[#808080]">
      {modelDetails?.productCode ?? '—'}
    </span>
  </td>

  <!-- Price Target -->
  <td class="px-4 py-3">
    {#if desiredPriceStr}
      <span class="font-mono text-sm font-bold text-[#D48A42]">{desiredPriceStr}</span>
    {:else}
      <span class="font-mono text-xs text-[#808080]">—</span>
    {/if}
  </td>

  <!-- Status pill -->
  <td class="px-4 py-3">
    <span
      class="inline-flex items-center rounded-[8px] border px-2 py-0.5 font-mono text-[10px] font-bold tracking-widest uppercase {statusConfig.classes}"
    >
      {statusConfig.label}
    </span>
  </td>

  <!-- Actions -->
  <td class="px-4 py-3" onclick={(e) => e.stopPropagation()}>
    <div class="flex items-center gap-1.5">
      {#if onPurchase && item.status !== 'PURCHASED'}
        <button
          type="button"
          title={m.wishlist_item_status_purchased()}
          class="rounded-[8px] border border-[#1F1F1F] p-1.5 text-[#808080] transition-colors hover:border-emerald-700/40 hover:bg-emerald-900/20 hover:text-emerald-400"
          onclick={() => onPurchase!(item.id)}
        >
          <ShoppingCart size={13} />
        </button>
      {/if}
      {#if onMove}
        <button
          type="button"
          title="Move to another list"
          class="rounded-[8px] border border-[#1F1F1F] p-1.5 text-[#808080] transition-colors hover:border-[#D48A42]/40 hover:bg-[rgba(212,138,66,0.1)] hover:text-[#D48A42]"
          onclick={() => onMove!(item.id)}
        >
          <MoveRight size={13} />
        </button>
      {/if}
      {#if onRemove}
        <button
          type="button"
          title="Remove from list"
          class="rounded-[8px] border border-[#1F1F1F] p-1.5 text-[#808080] transition-colors hover:border-red-800/40 hover:bg-red-900/20 hover:text-red-400"
          onclick={() => onRemove!(item.id)}
        >
          <Trash2 size={13} />
        </button>
      {/if}
    </div>
  </td>
</tr>
