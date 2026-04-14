<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import * as Select from '$lib/components/ui/select';
  import { Settings, Plus, Trash2, Edit2, X } from 'lucide-svelte';
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
    const defaultCurrency = regionalManager.currency || 'EUR';
    if (!activeInventory || activeInventory.purchases.length === 0) {
      return regionalManager.formatCurrencyWith(0, defaultCurrency);
    }
    const raw = activeInventory.purchases.reduce((acc, p) => acc + Number(p.price.amount), 0);
    const currency = activeInventory.purchases[0]?.price.currency ?? defaultCurrency;
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

<div
  class="flex flex-col gap-4 rounded-[8px] border border-layout-border bg-layout-surface px-4 py-3"
>
  <div class="flex flex-col gap-3 lg:flex-row lg:items-end lg:justify-between">
    <div class="w-full max-w-[360px]">
      <div class="flex flex-col gap-1.5">
        <span class="text-xs tracking-wider text-muted-foreground uppercase">
          {m.track_inventory_active_label()}
        </span>
        <div class="flex items-center gap-1">
          <div class="min-w-0 flex-1">
            <Select.Root
              type="single"
              value={activeInventoryId ?? undefined}
              onValueChange={(v) => {
                if (v) onSelect(v);
              }}
            >
              <Select.Trigger class="h-9 w-full border-layout-border bg-layout-surface">
                {#if activeInventoryName}
                  <span class="font-bold text-primary">{activeInventoryName}</span>
                {:else}
                  <span class="text-muted-foreground">{m.track_inventory_select_placeholder()}</span
                  >
                {/if}
              </Select.Trigger>
              <Select.Content>
                {#each inventories as inv (inv.id)}
                  <Select.Item value={inv.id} label={inv.name} />
                {/each}
              </Select.Content>
            </Select.Root>
          </div>

          <div class="relative">
            <Button
              variant="outline"
              size="icon"
              class="h-9 w-9 cursor-pointer border-border text-muted-foreground transition-all duration-150 ease-out hover:bg-muted hover:text-foreground"
              onclick={() => (showSettings = !showSettings)}
              aria-expanded={showSettings}
              aria-haspopup="menu"
              aria-label={m.track_inventory_management_button()}
            >
              {#if showSettings}
                <X size={18} />
              {:else}
                <Settings size={18} />
              {/if}
            </Button>

            {#if showSettings}
              <div
                class="absolute top-11 left-0 z-50 w-56 animate-in rounded-[8px] border border-border bg-card p-1 shadow-2xl duration-200 fade-in zoom-in"
                onmouseleave={() => (showSettings = false)}
                role="menu"
                tabindex="-1"
              >
                {#if onRename}
                  <button
                    onclick={() => {
                      showSettings = false;
                      onRename?.();
                    }}
                    class="flex w-full items-center rounded-[8px] px-3 py-2 text-sm text-foreground transition-colors hover:bg-muted"
                  >
                    <Edit2 size={14} class="mr-2" />
                    {m.track_inventory_rename_button()}
                  </button>
                {/if}
                {#if onRename && onDelete}
                  <div class="my-1 h-px bg-border"></div>
                {/if}
                {#if onDelete}
                  <button
                    onclick={() => {
                      showSettings = false;
                      onDelete?.();
                    }}
                    class="flex w-full items-center rounded-[8px] px-3 py-2 text-sm text-red-400 transition-colors hover:bg-red-900/20"
                  >
                    <Trash2 size={14} class="mr-2" />
                    {m.inventory_delete_action()}
                  </button>
                {/if}
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>

    <div class="flex items-center gap-2 self-end">
      {#if onAddPurchase}
        <Button
          variant="default"
          class="h-9 rounded-sm bg-primary px-5 text-primary-foreground transition-all duration-150 ease-out hover:brightness-110 active:scale-[0.99]"
          onclick={onAddPurchase}
        >
          <Plus size={16} />
          <span>{m.track_inventory_detail_add_purchase()}</span>
        </Button>
      {/if}
    </div>
  </div>

  <div class="grid gap-3 border-t border-layout-border pt-4 sm:grid-cols-3">
    <div class="rounded-sm border border-border bg-card p-3">
      <p class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.track_inventories_card_total_quantity()}
      </p>
      <p class="font-mono text-lg leading-none font-bold text-foreground">{totalPieces}</p>
    </div>

    <div class="rounded-sm border border-border bg-card p-3">
      <p class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.track_inventory_value_label()}
      </p>
      <p class="font-mono text-lg leading-none font-bold text-foreground">{totalValue}</p>
    </div>

    <div class="rounded-sm border border-border bg-card p-3">
      <p class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.track_inventory_last_purchase()}
      </p>
      <p class="font-mono text-lg leading-none font-bold text-foreground">{lastPurchaseDate}</p>
    </div>
  </div>
</div>
