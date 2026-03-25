<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { TrackPurchaseView } from '$lib/features/track-inventory';
  import { Store, ArrowRight } from 'lucide-svelte';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    purchase: TrackPurchaseView;
  }

  const { purchase }: Props = $props();

  const formattedPrice = $derived(
    regionalManager.formatCurrencyWith(purchase.price.amount, purchase.price.currency)
  );
</script>

<div
  class="group flex items-center justify-between gap-4 rounded-xl border border-transparent p-3 transition-all hover:border-white/5 hover:bg-white/5"
>
  <div class="flex items-center gap-4">
    <!-- Icon or Date -->
    <div
      class="flex min-w-[50px] flex-col items-center justify-center rounded-lg border border-white/5 bg-zinc-900 px-2 py-1.5"
    >
      <span class="text-[10px] font-bold tracking-tighter text-zinc-500 uppercase">
        {new Date(purchase.purchase_date).toLocaleDateString(regionalManager.locale, {
          month: 'short'
        })}
      </span>
      <span class="text-sm font-bold text-zinc-100">
        {new Date(purchase.purchase_date).getDate()}
      </span>
    </div>

    <div>
      <h4 class="text-sm font-bold text-zinc-100 transition-colors group-hover:text-amber-500">
        {purchase.track_product.description || purchase.track_product.product_code}
      </h4>
      <div
        class="flex items-center gap-2 text-[10px] font-medium tracking-wider text-zinc-500 uppercase"
      >
        <span class="text-zinc-600">{purchase.track_product.manufacturer_name}</span>
        {#if purchase.seller_name}
          <span class="h-1 w-1 rounded-full bg-zinc-800"></span>
          <div class="flex items-center gap-1">
            <Store size={10} class="text-zinc-700" />
            <span>{purchase.seller_name}</span>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <div class="flex items-center gap-6">
    <!-- Quantity -->
    <div class="flex flex-col items-end">
      <span class="mb-1 text-[9px] leading-none font-bold tracking-widest text-zinc-600 uppercase"
        >{m.track_purchase_qty()}</span
      >
      <span class="font-mono text-base font-bold text-zinc-400">
        {purchase.quantity.toString().padStart(2, '0')}
      </span>
    </div>

    <!-- Price -->
    <div class="flex min-w-[80px] flex-col items-end">
      <span class="mb-1 text-[9px] leading-none font-bold tracking-widest text-zinc-600 uppercase"
        >{m.track_purchase_total()}</span
      >
      <span class="font-mono text-base font-bold tracking-tighter text-amber-500/80">
        {formattedPrice}
      </span>
    </div>

    <div
      class="-translate-x-2 opacity-0 transition-all duration-300 group-hover:translate-x-0 group-hover:opacity-100"
    >
      <ArrowRight size={14} class="text-zinc-700" />
    </div>
  </div>
</div>
