<script lang="ts">
  import { CircleDollarSign } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { WishlistItem } from '$lib/bindings';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    items: WishlistItem[];
  }

  const { items }: Props = $props();

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

  const avgPrice = $derived(
    pricedItems.length > 0 ? Math.round(totals.total / pricedItems.length) : 0
  );

  function formatAmount(cents: number, curr: string): string {
    return regionalManager.formatCurrencyWith(cents, curr);
  }
</script>

<div class="rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  <!-- Card Header -->
  <div class="mb-4 flex items-center justify-between border-b border-[#1F1F1F] pb-3">
    <span class="font-mono text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.wishlist_procurement_summary()}
    </span>
    <CircleDollarSign size={14} class="text-[#808080]" />
  </div>

  {#if pricedItems.length === 0}
    <!-- Empty state -->
    <div class="flex flex-col items-center justify-center gap-2 py-6">
      <CircleDollarSign size={32} class="text-[#808080] opacity-30" />
      <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
        {m.wishlist_value_bar_no_price_data()}
      </span>
    </div>
  {:else}
    <!-- 3-Column Stats Footer -->
    <div class="mb-4 grid grid-cols-3 gap-2">
      <div class="flex flex-col gap-1">
        <span class="font-mono text-[10px] leading-none tracking-widest text-[#808080] uppercase">
          {m.wishlist_stat_total_cost()}
        </span>
        <span class="font-mono text-base leading-none font-bold text-[#D48A42]">
          {formatAmount(totals.total, currency)}
        </span>
      </div>
      <div class="flex flex-col items-center gap-1">
        <span class="font-mono text-[10px] leading-none tracking-widest text-[#808080] uppercase">
          {m.wishlist_stat_item_count()}
        </span>
        <span class="font-mono text-base leading-none font-bold text-[#E0E0E0]">
          {items.length}
        </span>
      </div>
      <div class="flex flex-col items-end gap-1">
        <span
          class="text-right font-mono text-[10px] leading-none tracking-widest text-[#808080] uppercase"
        >
          {m.wishlist_stat_avg_price()}
        </span>
        <span class="font-mono text-base leading-none font-bold text-[#D48A42]">
          {formatAmount(avgPrice, currency)}
        </span>
      </div>
    </div>

    <!-- Mechanical Gauge Bar -->
    <div class="mb-3">
      <!-- Track with tick marks -->
      <div class="relative mb-1.5 h-2.5 overflow-hidden rounded-full bg-[#1F1F1F]">
        <!-- Filled segments -->
        <div class="absolute inset-y-0 left-0 flex h-full" style="width: 100%">
          {#if percentages.high > 0}
            <div
              class="h-full bg-[#D48A42] transition-all duration-500"
              style="width: {percentages.high}%; box-shadow: 0 0 8px rgba(212,138,66,0.5), 0 0 16px rgba(212,138,66,0.2);"
            ></div>
          {/if}
          {#if percentages.normal > 0}
            <div
              class="h-full bg-[#D48A42]/55 transition-all duration-500"
              style="width: {percentages.normal}%"
            ></div>
          {/if}
          {#if percentages.low > 0}
            <div
              class="h-full bg-[#D48A42]/25 transition-all duration-500"
              style="width: {percentages.low}%"
            ></div>
          {/if}
        </div>
        <!-- Tick marks overlay (4 evenly spaced) -->
        <div class="pointer-events-none absolute inset-0 flex items-stretch justify-around">
          {#each [0, 1, 2] as _ (_)}
            <div class="h-full w-px bg-[#050505]/70"></div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Priority Legend Footer -->
    <div class="flex justify-between gap-2">
      <div class="flex flex-col gap-0.5">
        <div class="flex items-center gap-1.5">
          <div class="h-1.5 w-1.5 rounded-full bg-[#D48A42]"></div>
          <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
            {m.wishlist_priority_high()}
          </span>
        </div>
        <span class="font-mono text-xs text-[#E0E0E0]/70">
          {formatAmount(totals.high, currency)}
        </span>
      </div>
      <div class="flex flex-col items-center gap-0.5">
        <div class="flex items-center gap-1.5">
          <div class="h-1.5 w-1.5 rounded-full bg-[#D48A42]/55"></div>
          <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
            {m.wishlist_priority_normal()}
          </span>
        </div>
        <span class="font-mono text-xs text-[#E0E0E0]/70">
          {formatAmount(totals.normal, currency)}
        </span>
      </div>
      <div class="flex flex-col items-end gap-0.5">
        <div class="flex items-center gap-1.5">
          <div class="h-1.5 w-1.5 rounded-full bg-[#D48A42]/25"></div>
          <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
            {m.wishlist_priority_low()}
          </span>
        </div>
        <span class="font-mono text-xs text-[#E0E0E0]/70">
          {formatAmount(totals.low, currency)}
        </span>
      </div>
    </div>
  {/if}
</div>
