<script lang="ts">
  import { CircleDollarSign } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { WishlistItem } from '$lib/bindings';

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

  function formatAmount(cents: number, curr: string): string {
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: curr
    }).format(cents / 100);
  }
</script>

<div class="rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  {#if pricedItems.length === 0}
    <div class="flex items-center justify-center gap-2 py-2">
      <CircleDollarSign class="size-8 text-zinc-700" />
      <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
        {m.wishlist_value_bar_no_price_data()}
      </span>
    </div>
  {:else}
    <!-- Top row -->
    <div class="mb-3 flex items-center justify-between">
      <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
        {m.wishlist_value_bar_total_label()}
      </span>
      <span class="font-mono text-lg font-bold text-white">
        {formatAmount(totals.total, currency)}
      </span>
    </div>

    <!-- Segmented bar -->
    <div class="mb-3 flex h-3 overflow-hidden rounded-full bg-[#1A1A1A]">
      {#if percentages.high > 0}
        <div
          class="h-full bg-[#D48A42] transition-all duration-500"
          style="width: {percentages.high}%"
        ></div>
      {/if}
      {#if percentages.normal > 0}
        <div
          class="h-full bg-[#D48A42]/60 transition-all duration-500"
          style="width: {percentages.normal}%"
        ></div>
      {/if}
      {#if percentages.low > 0}
        <div
          class="h-full bg-[#D48A42]/30 transition-all duration-500"
          style="width: {percentages.low}%"
        ></div>
      {/if}
    </div>

    <!-- Footer labels -->
    <div class="flex justify-between gap-2">
      <div class="flex flex-col gap-0.5">
        <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
          {m.wishlist_priority_high()}
        </span>
        <span class="font-mono text-xs text-white/70">
          {formatAmount(totals.high, currency)}
        </span>
      </div>
      <div class="flex flex-col items-center gap-0.5">
        <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
          {m.wishlist_priority_normal()}
        </span>
        <span class="font-mono text-xs text-white/70">
          {formatAmount(totals.normal, currency)}
        </span>
      </div>
      <div class="flex flex-col items-end gap-0.5">
        <span class="font-mono text-[10px] tracking-widest text-[#808080] uppercase">
          {m.wishlist_priority_low()}
        </span>
        <span class="font-mono text-xs text-white/70">
          {formatAmount(totals.low, currency)}
        </span>
      </div>
    </div>
  {/if}
</div>
