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
  import GaugeStatCard from '$lib/components/GaugeStatCard.svelte';
  import { resolve } from '$app/paths';
  import { Button } from '$lib/components';

  interface Props {
    inventory: TrackInventoryView;
    showBackButton?: boolean;
    onRename?: () => void;
    onDelete?: () => void;
    onAddPurchase?: () => void;
  }

  const { inventory, showBackButton = false, onRename, onDelete, onAddPurchase }: Props = $props();

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
      {#if showBackButton}
        <a
          href={resolve('/railway-tracks')}
          class="mt-1 flex h-10 w-10 items-center justify-center rounded-xl border border-white/5 bg-zinc-900/50 text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
          aria-label={m.track_inventory_back_label()}
        >
          <ChevronLeft size={22} />
        </a>
      {/if}
      <div>
        <div class="flex items-center gap-3">
          <p class="text-[10px] font-bold tracking-[0.3em] text-zinc-500 uppercase">
            {m.track_inventory_section_label()}
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
          <span>{m.track_inventory_detail_add_purchase()}</span>
        </Button>
      {/if}
    </div>
  </div>

  <!-- 2. Statistical Dashboard -->
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
    <GaugeStatCard label="Total Pieces" value={totalPieces} icon={Box} unit="pcs" />
    <GaugeStatCard label="Inventory Value" value={totalValue} icon={Euro} />
    <GaugeStatCard label="Last Purchase" value={lastPurchaseDate} icon={Calendar} />
  </div>

  <div class="space-y-12">
    <!-- 3. Items List Section -->
    <div class="space-y-6">
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
            variant="rusty"
            class="mt-6 shadow-lg shadow-amber-500/10"
            onclick={onAddPurchase}
          >
            {m.track_inventory_detail_add_first_piece()}
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
    <div class="space-y-6">
      <div class="flex items-center gap-3">
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
