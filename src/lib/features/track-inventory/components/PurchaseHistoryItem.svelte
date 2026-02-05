<script lang="ts">
  import type { TrackPurchaseView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import { Package } from 'lucide-svelte';

  interface Props {
    purchase: TrackPurchaseView;
  }

  const { purchase }: Props = $props();

  const formattedDate = $derived(
    new Date(purchase.purchase_date).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    })
  );

  const formattedPrice = $derived((purchase.price.amount / 100n).toString());
</script>

<div
  class="variant-ghost-surface group border-surface-700 hover:border-surface-600 rounded-lg border p-4 transition-all"
>
  <div class="flex items-start justify-between gap-4">
    <div class="flex flex-1 items-start gap-3">
      <div
        class="variant-filled-surface flex h-10 w-10 shrink-0 items-center justify-center rounded-lg"
      >
        <Package size={20} />
      </div>

      <div class="flex flex-1 flex-col gap-1">
        <div class="flex items-start justify-between">
          <div>
            <h4 class="font-medium">
              {purchase.track_product.description || purchase.track_product.product_code}
            </h4>
            <p class="text-surface-400 text-sm">
              {purchase.track_product.manufacturer_name}
            </p>
          </div>
        </div>

        <div class="text-surface-300 flex flex-wrap items-center gap-3 text-sm">
          <div class="flex items-center gap-1">
            <span class="text-surface-100 font-semibold">{purchase.quantity}×</span>
            <span>{m.track_purchase_history_item_quantity()}</span>
          </div>

          <span class="text-surface-600">•</span>

          <div class="flex items-center gap-1">
            <span class="text-surface-100 font-semibold">
              {formattedPrice}
              {purchase.price.currency}
            </span>
            <span class="text-surface-400">({m.track_purchase_history_item_price()})</span>
          </div>

          {#if purchase.seller_name}
            <span class="text-surface-600">•</span>
            <div class="flex items-center gap-1">
              <span class="text-surface-400">from</span>
              <span class="text-surface-100 font-medium">{purchase.seller_name}</span>
            </div>
          {/if}
        </div>
      </div>
    </div>

    <div class="flex shrink-0 flex-col items-end gap-1">
      <time class="text-surface-200 text-sm font-medium" datetime={purchase.purchase_date}>
        {formattedDate}
      </time>
    </div>
  </div>
</div>
