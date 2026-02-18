<script lang="ts">
  import type { TrackInventoryView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import InventoryItemRow from './InventoryItemRow.svelte';
  import PurchaseHistory from './PurchaseHistory.svelte';
  import {
    ChevronLeft,
    Settings,
    Plus,
    Box,
    Euro,
    Calendar,
    TrainTrack,
    History,
    Trash2,
    Edit2
  } from 'lucide-svelte';
  import { resolve } from '$app/paths';
  import { Button } from '$lib/components';

  interface Props {
    inventory: TrackInventoryView;
    onRename?: () => void;
    onDelete?: () => void;
    onAddPurchase?: () => void;
  }

  const { inventory, onRename, onDelete, onAddPurchase }: Props = $props();

  // Derived statistics
  const totalPieces = $derived(
    inventory.items.reduce((acc, item) => acc + Number(item.quantity), 0)
  );

  const totalValueRaw = $derived(
    inventory.purchases.reduce((acc, p) => acc + Number(p.price.amount), 0)
  );

  const totalValue = $derived(
    new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: inventory.purchases[0]?.price.currency || 'EUR'
    }).format(totalValueRaw / 100)
  );

  const lastPurchaseDate = $derived.by(() => {
    if (inventory.purchases.length === 0) return '—';
    const dates = inventory.purchases.map((p) => new Date(p.purchase_date).getTime());
    const latest = Math.max(...dates);
    return new Date(latest).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  });

  let showSettings = $state(false);
</script>

<div class="space-y-10">
  <!-- 1. Header & Primary Actions -->
  <div class="flex flex-col gap-6 md:flex-row md:items-start md:justify-between">
    <div class="flex items-start gap-4">
      <a
        href={resolve('/my-tracks')}
        class="mt-1 flex h-10 w-10 items-center justify-center rounded-xl border border-white/5 bg-zinc-900/50 text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
        aria-label="Back to inventories"
      >
        <ChevronLeft size={22} />
      </a>
      <div>
        <div class="flex items-center gap-3">
          <p class="text-[10px] font-bold tracking-[0.3em] text-zinc-500 uppercase">
            Track Inventory
          </p>
          <div class="h-px w-8 bg-zinc-800"></div>
        </div>
        <h1 class="mt-1 text-4xl font-bold tracking-tight text-zinc-100">{inventory.name}</h1>
        {#if inventory.description}
          <p class="mt-2 max-w-xl text-sm leading-relaxed text-zinc-400">
            {inventory.description}
          </p>
        {/if}
      </div>
    </div>

    <div class="flex flex-wrap items-center gap-3">
      <!-- Rename & Delete in Management Suite -->
      <div class="relative">
        <Button
          variant="outline"
          class="h-11 border-white/10 bg-zinc-900/50 text-zinc-300 hover:bg-zinc-800"
          onclick={() => (showSettings = !showSettings)}
        >
          <Settings size={18} class={showSettings ? 'rotate-90 transition-transform' : ''} />
          <span>Management</span>
        </Button>

        {#if showSettings}
          <div
            class="absolute top-full right-0 z-20 mt-2 w-48 overflow-hidden rounded-xl border border-white/10 bg-[#0c0c0c] shadow-2xl"
          >
            {#if onRename}
              <button
                onclick={() => {
                  showSettings = false;
                  onRename();
                }}
                class="flex w-full items-center gap-3 px-4 py-3 text-sm text-zinc-300 transition-colors hover:bg-zinc-800 hover:text-white"
              >
                <Edit2 size={16} />
                <span>Rename</span>
              </button>
            {/if}
            {#if onDelete}
              <button
                onclick={() => {
                  showSettings = false;
                  onDelete();
                }}
                class="flex w-full items-center gap-3 border-t border-white/5 px-4 py-3 text-sm text-zinc-500 transition-colors hover:bg-red-950/30 hover:text-red-500"
              >
                <Trash2 size={16} />
                <span>Delete Inventory</span>
              </button>
            {/if}
          </div>
        {/if}
      </div>

      {#if onAddPurchase}
        <Button
          variant="rusty"
          class="h-11 px-6 shadow-lg shadow-amber-500/10"
          onclick={onAddPurchase}
        >
          <Plus size={18} />
          <span>Add Track Purchase</span>
        </Button>
      {/if}
    </div>
  </div>

  <!-- 2. Statistical Dashboard -->
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
    <!-- Card 1: Total Pieces -->
    <div
      class="flex flex-col gap-3 rounded-2xl border border-white/5 bg-zinc-900/30 p-6 backdrop-blur-sm"
    >
      <div class="flex items-center justify-between">
        <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
          Total Pieces
        </span>
        <Box size={16} class="text-zinc-600" />
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-3xl font-bold text-zinc-100">{totalPieces}</span>
        <span class="text-sm font-medium text-zinc-500">pcs</span>
      </div>
    </div>

    <!-- Card 2: Total Value -->
    <div
      class="flex flex-col gap-3 rounded-2xl border border-white/5 bg-zinc-900/30 p-6 backdrop-blur-sm"
    >
      <div class="flex items-center justify-between">
        <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
          Inventory Value
        </span>
        <Euro size={16} class="text-zinc-600" />
      </div>
      <span class="text-3xl font-bold text-amber-500/90">{totalValue}</span>
    </div>

    <!-- Card 3: Last Purchase Date -->
    <div
      class="flex flex-col gap-3 rounded-2xl border border-white/5 bg-zinc-900/30 p-6 backdrop-blur-sm"
    >
      <div class="flex items-center justify-between">
        <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
          Last Purchase
        </span>
        <Calendar size={16} class="text-zinc-600" />
      </div>
      <span class="text-2xl font-bold text-zinc-300">{lastPurchaseDate}</span>
    </div>
  </div>

  <div class="grid grid-cols-1 gap-12 lg:grid-cols-5">
    <!-- 3. Items List Section -->
    <div class="space-y-6 lg:col-span-3">
      <div class="flex items-center gap-3">
        <TrainTrack size={20} class="text-amber-500/60" />
        <h3 class="text-lg font-bold tracking-tight text-zinc-200">
          {m.track_inventory_detail_items_tab()}
        </h3>
        <div
          class="ml-auto flex items-center gap-2 rounded-full bg-zinc-900/50 px-3 py-1 text-[10px] font-bold text-zinc-500"
        >
          <span class="text-zinc-300">{inventory.items.length}</span>
          TYPES
        </div>
      </div>

      {#if inventory.items.length === 0}
        <div
          class="flex flex-col items-center justify-center rounded-3xl border border-dashed border-white/5 bg-zinc-900/10 py-20 text-center"
        >
          <TrainTrack size={48} class="mb-4 text-zinc-700 opacity-20" />
          <p class="max-w-[200px] text-sm text-zinc-500">
            {m.track_inventory_detail_empty_items()}
          </p>
          <Button
            variant="outline"
            class="mt-6 border-white/5 bg-zinc-900/50"
            onclick={onAddPurchase}
          >
            Add First Piece
          </Button>
        </div>
      {:else}
        <div class="space-y-3">
          {#each inventory.items as item (item.track_id)}
            <InventoryItemRow {item} inventoryId={inventory.id} />
          {/each}
        </div>
      {/if}
    </div>

    <!-- 4. Purchase History Section -->
    <div class="space-y-6 lg:col-span-2">
      <div class="flex items-center gap-3 border-b border-white/5 pb-4 lg:border-0 lg:pb-0">
        <History size={20} class="text-zinc-500" />
        <h3 class="text-lg font-bold tracking-tight text-zinc-200">
          {m.track_inventory_detail_history_tab()}
        </h3>
      </div>

      <div class="rounded-2xl border border-white/5 bg-zinc-950/20 p-1">
        <PurchaseHistory purchases={inventory.purchases} />
      </div>
    </div>
  </div>
</div>
