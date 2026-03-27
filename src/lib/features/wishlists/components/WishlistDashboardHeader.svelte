<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import type { WishlistPreview, WishlistItem } from '$lib/bindings';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    wishlists: WishlistPreview[];
    activeWishlistId: string | null;
    items: WishlistItem[];
    onSelect: (id: string) => void;
  }

  const { wishlists, activeWishlistId, items, onSelect }: Props = $props();

  const activeWishlistName = $derived(wishlists.find((w) => w.id === activeWishlistId)?.name ?? '');

  const pricedItems = $derived(items.filter((item) => item.desiredPrice != null));

  const totals = $derived.by(() => {
    let high = 0;
    let normal = 0;
    let low = 0;
    for (const item of pricedItems) {
      const amount = Number(item.desiredPrice!.amount);
      if (item.priority === 'HIGH') high += amount;
      else if (item.priority === 'NORMAL') normal += amount;
      else if (item.priority === 'LOW') low += amount;
    }
    return { high, normal, low, total: high + normal + low };
  });

  const percentages = $derived.by(() => {
    if (totals.total === 0) return { high: 0, normal: 0, low: 0 };
    return {
      high: (totals.high / totals.total) * 100,
      normal: (totals.normal / totals.total) * 100,
      low: (totals.low / totals.total) * 100
    };
  });

  const currency = $derived(pricedItems[0]?.desiredPrice?.currency ?? 'EUR');

  function formatAmount(cents: number, curr: string): string {
    return regionalManager.formatCurrencyWith(cents, curr);
  }
</script>

<div class="flex items-stretch rounded-[8px] border border-layout-border bg-layout-surface">
  <!-- Zone A: List Switcher -->
  <div class="flex w-[300px] shrink-0 flex-col gap-1.5 border-r border-layout-border px-4 py-3">
    <span class="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
      {m.wishlists_active_list_label()}
    </span>
    <Select.Root
      type="single"
      value={activeWishlistId ?? undefined}
      onValueChange={(v) => {
        if (v) onSelect(v);
      }}
    >
      <Select.Trigger class="h-8 w-full border-layout-border bg-layout-surface">
        {#if activeWishlistName}
          <span class="font-bold text-primary">{activeWishlistName}</span>
        {:else}
          <span class="text-muted-foreground">{m.wishlists_select_list_placeholder()}</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each wishlists as wl (wl.id)}
          <Select.Item value={wl.id} label={wl.name} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <!-- Zone B: Metrics -->
  <div class="flex w-[300px] shrink-0 flex-col border-r border-layout-border px-4 py-3">
    {#if pricedItems.length === 0}
      <div class="flex items-center justify-center py-2">
        <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
          {m.wishlist_value_bar_no_price_data()}
        </span>
      </div>
    {:else}
      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-0.5">
          <span
            class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
          >
            {m.wishlist_stat_total_cost()}
          </span>
          <span class="font-mono text-base leading-none font-bold text-primary">
            {formatAmount(totals.total, currency)}
          </span>
        </div>
        <div class="flex flex-col gap-0.5">
          <span
            class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
          >
            {m.wishlist_stat_item_count()}
          </span>
          <span class="font-mono text-base leading-none font-bold text-foreground">
            {items.length}
          </span>
        </div>
      </div>
    {/if}
  </div>

  <!-- Zone C: Procurement Status -->
  <div class="flex flex-1 flex-col gap-2 px-4 py-3">
    <span class="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
      {m.wishlist_procurement_summary()}
    </span>
    {#if pricedItems.length > 0}
      <!-- Gauge bar -->
      <div
        data-testid="gauge-bar"
        class="relative h-2.5 overflow-hidden rounded-full bg-layout-border"
      >
        <div class="absolute inset-y-0 left-0 flex h-full" style="width: 100%">
          {#if percentages.high > 0}
            <div
              class="h-full bg-red-500 transition-all duration-500"
              style="width: {percentages.high}%; box-shadow: 0 0 8px rgba(239,68,68,0.5), 0 0 16px rgba(239,68,68,0.2);"
            ></div>
          {/if}
          {#if percentages.normal > 0}
            <div
              class="h-full bg-orange-500 transition-all duration-500"
              style="width: {percentages.normal}%"
            ></div>
          {/if}
          {#if percentages.low > 0}
            <div
              class="h-full bg-primary/25 transition-all duration-500"
              style="width: {percentages.low}%"
            ></div>
          {/if}
        </div>
        <div class="pointer-events-none absolute inset-0 flex items-stretch justify-around">
          {#each [0, 1, 2] as _ (_)}
            <div class="h-full w-px bg-layout-surface/70"></div>
          {/each}
        </div>
      </div>

      <!-- Priority legend -->
      <div class="flex justify-between gap-2">
        <div class="flex flex-col gap-0.5">
          <div class="flex items-center gap-1.5">
            <div class="h-1.5 w-1.5 rounded-full bg-red-500"></div>
            <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
              {m.wishlist_priority_high()}
            </span>
          </div>
          <span class="font-mono text-xs text-foreground/70">
            {formatAmount(totals.high, currency)}
          </span>
        </div>
        <div class="flex flex-col items-center gap-0.5">
          <div class="flex items-center gap-1.5">
            <div class="h-1.5 w-1.5 rounded-full bg-orange-500"></div>
            <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
              {m.wishlist_priority_normal()}
            </span>
          </div>
          <span class="font-mono text-xs text-foreground/70">
            {formatAmount(totals.normal, currency)}
          </span>
        </div>
        <div class="flex flex-col items-end gap-0.5">
          <div class="flex items-center gap-1.5">
            <div class="h-1.5 w-1.5 rounded-full bg-primary/25"></div>
            <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
              {m.wishlist_priority_low()}
            </span>
          </div>
          <span class="font-mono text-xs text-foreground/70">
            {formatAmount(totals.low, currency)}
          </span>
        </div>
      </div>
    {/if}
  </div>
</div>
