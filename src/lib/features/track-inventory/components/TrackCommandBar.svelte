<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import * as Select from '$lib/components/ui/select';
  import { Settings, Plus, Trash2, Edit2 } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import type { TrackInventoryListItem, TrackInventoryView } from '$lib/features/track-inventory';

  interface Props {
    inventories: TrackInventoryListItem[];
    activeInventoryId: string | null;
    activeInventory: TrackInventoryView | null;
    onSelect: (id: string) => void;
    onRename?: () => void;
    onDelete?: () => void;
    onAddPurchase?: () => void;
  }

  const {
    inventories,
    activeInventoryId,
    activeInventory,
    onSelect,
    onRename,
    onDelete,
    onAddPurchase
  }: Props = $props();

  const activeInventoryName = $derived(
    inventories.find((inv) => inv.id === activeInventoryId)?.name ?? ''
  );

  const totalPieces = $derived(
    activeInventory
      ? activeInventory.items.reduce((acc, item) => acc + Number(item.quantity), 0)
      : 0
  );

  const totalValue = $derived.by(() => {
    if (!activeInventory || activeInventory.purchases.length === 0) {
      return regionalManager.formatCurrencyWith(0, regionalManager.currency);
    }
    const raw = activeInventory.purchases.reduce((acc, p) => acc + Number(p.price.amount), 0);
    const currency = activeInventory.purchases[0]?.price.currency ?? regionalManager.currency;
    return regionalManager.formatCurrencyWith(raw, currency);
  });

  const lastPurchaseDate = $derived.by(() => {
    if (!activeInventory || activeInventory.purchases.length === 0) return '—';
    const dates = activeInventory.purchases.map((p) => new Date(p.purchase_date).getTime());
    const latest = Math.max(...dates);
    return new Date(latest).toLocaleDateString(regionalManager.locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  });

  let showSettings = $state(false);
</script>

<div class="space-y-0">
  <!-- Command Bar -->
  <div class="flex items-stretch rounded-t-[8px] border border-layout-border bg-layout-surface">
    <!-- Zone A: Inventory Selector -->
    <div class="flex w-[300px] shrink-0 flex-col gap-1.5 border-r border-layout-border px-4 py-3">
      <span class="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
        {m.track_inventory_active_label()}
      </span>
      <Select.Root
        type="single"
        value={activeInventoryId ?? undefined}
        onValueChange={(v) => {
          if (v) onSelect(v);
        }}
      >
        <Select.Trigger class="h-8 w-full border-layout-border bg-layout-surface">
          {#if activeInventoryName}
            <span class="font-bold text-primary">{activeInventoryName}</span>
          {:else}
            <span class="text-muted-foreground">{m.track_inventory_select_placeholder()}</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each inventories as inv (inv.id)}
            <Select.Item value={inv.id} label={inv.name} />
          {/each}
        </Select.Content>
      </Select.Root>
    </div>

    <!-- Zone B: Flattened Metrics -->
    <div class="flex flex-1 items-center border-r border-layout-border px-4 py-3">
      <div class="grid w-full grid-cols-3 gap-4">
        <!-- Total Pieces -->
        <div class="flex flex-col gap-0.5">
          <span
            class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
          >
            {m.track_inventories_card_total_quantity()}
          </span>
          <span class="font-mono text-base leading-none font-bold text-foreground">
            {totalPieces}
          </span>
        </div>

        <!-- Inventory Value -->
        <div class="flex flex-col gap-0.5">
          <span
            class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
          >
            {m.track_inventory_value_label()}
          </span>
          <span class="font-mono text-base leading-none font-bold text-foreground">
            {totalValue}
          </span>
        </div>

        <!-- Last Purchase -->
        <div class="flex flex-col gap-0.5">
          <span
            class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
          >
            {m.track_inventory_last_purchase()}
          </span>
          <span class="font-mono text-base leading-none font-bold text-foreground">
            {lastPurchaseDate}
          </span>
        </div>
      </div>
    </div>

    <!-- Zone C: Actions -->
    <div class="flex shrink-0 items-center gap-3 px-4 py-3">
      <!-- Management dropdown -->
      <div class="relative">
        <Button
          variant="outline"
          class="h-9 border-layout-border bg-layout-surface text-muted-foreground hover:text-foreground"
          onclick={() => (showSettings = !showSettings)}
        >
          <Settings size={16} class={showSettings ? 'rotate-90 transition-transform' : ''} />
          <span>{m.track_inventory_management_button()}</span>
        </Button>

        {#if showSettings}
          <div
            class="absolute top-full right-0 z-20 mt-2 w-48 overflow-hidden rounded-xl border border-layout-border bg-layout-surface shadow-2xl"
          >
            {#if onRename}
              <button
                onclick={() => {
                  showSettings = false;
                  onRename?.();
                }}
                class="flex w-full items-center gap-3 px-4 py-3 text-sm text-muted-foreground transition-colors hover:bg-white/5 hover:text-foreground"
              >
                <Edit2 size={16} />
                <span>{m.track_inventory_rename_button()}</span>
              </button>
            {/if}
            {#if onDelete}
              <button
                onclick={() => {
                  showSettings = false;
                  onDelete?.();
                }}
                class="flex w-full items-center gap-3 border-t border-layout-border px-4 py-3 text-sm text-muted-foreground transition-colors hover:bg-red-950/30 hover:text-red-500"
              >
                <Trash2 size={16} />
                <span>{m.inventory_delete_action()}</span>
              </button>
            {/if}
          </div>
        {/if}
      </div>

      {#if onAddPurchase}
        <Button
          variant="rusty"
          class="h-9 px-5 shadow-lg shadow-amber-500/10"
          onclick={onAddPurchase}
        >
          <Plus size={16} />
          <span>{m.track_inventory_detail_add_purchase()}</span>
        </Button>
      {/if}
    </div>
  </div>

  <!-- Amber accent line -->
  <div
    class="h-[2px] rounded-b-[8px] bg-gradient-to-r from-amber-500/60 via-amber-500/30 to-transparent"
  ></div>
</div>
