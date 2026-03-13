<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Plus, Tag, Store } from 'lucide-svelte';
  import type { TrackProductView, SellerView, Currency } from '$lib/bindings';
  import { Input, Button, DatePickerField, CurrencyInput } from '$lib/components';
  import { getCurrencySymbol } from '$lib/utils/currency';

  interface Props {
    products?: TrackProductView[];
    sellers?: SellerView[];
    selectedProductId?: string;
    quantity?: number;
    priceAmount?: number | null;
    priceCurrency?: Currency;
    selectedSellerId?: string;
    purchaseDate?: string;
    submitting?: boolean;
    error?: string | null;
    onCreateProduct?: () => void;
  }

  let {
    products = [],
    sellers = [],
    selectedProductId = $bindable(''),
    quantity = $bindable(1),
    priceAmount = $bindable(null),
    priceCurrency = $bindable('EUR' as Currency),
    selectedSellerId = $bindable(''),
    purchaseDate = $bindable(new Date().toISOString().split('T')[0]),
    submitting = false,
    error = null,
    onCreateProduct
  }: Props = $props();

  const totalCents = $derived((priceAmount ?? 0) * quantity);
  const totalDisplay = $derived((totalCents / 100).toFixed(2));
  const currencySymbol = $derived(getCurrencySymbol(priceCurrency));
</script>

<div class="space-y-6">
  <!-- Product selector with create button -->
  <div class="space-y-2">
    <label
      for="track-product-select"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.track_purchase_field_product()}
    </label>
    <div class="flex gap-2">
      <div class="relative flex-1">
        <select
          id="track-product-select"
          class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
          bind:value={selectedProductId}
          disabled={submitting}
          required
        >
          <option value="" disabled selected>{m.track_purchase_field_product_placeholder()}</option>
          {#each products as product (product.track_id)}
            <option value={product.track_id}>
              {product.manufacturer_name} • {product.description || product.product_code}
            </option>
          {/each}
        </select>
        <div class="pointer-events-none absolute inset-y-0 right-4 flex items-center text-zinc-600">
          <Plus size={16} class="rotate-45" />
        </div>
      </div>
      <Button
        type="button"
        variant="outline"
        onclick={onCreateProduct}
        class="h-12 w-12 border-white/10 bg-zinc-900/50 p-0 text-zinc-400 hover:bg-zinc-800 hover:text-white"
        disabled={submitting}
      >
        <Plus size={20} />
      </Button>
    </div>
  </div>

  <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
    <!-- Quantity -->
    <div class="space-y-2">
      <label
        for="purchase-quantity"
        class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
      >
        {m.track_purchase_field_quantity()}
      </label>
      <div class="relative">
        <Input
          id="purchase-quantity"
          type="number"
          value={String(quantity)}
          oninput={(e) => (quantity = parseInt(e.currentTarget.value) || 1)}
          min="1"
          class="h-12 rounded-xl border-white/10 bg-zinc-950 pl-10 text-zinc-100 focus:border-white/20 focus:ring-0"
          disabled={submitting}
          required
        />
        <Tag size={16} class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600" />
      </div>
    </div>

    <!-- Unit price -->
    <div class="space-y-2">
      <label
        for="purchase-price"
        class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
      >
        {m.track_purchase_field_market_price()}
      </label>
      <CurrencyInput
        id="purchase-price"
        bind:value={priceAmount}
        symbol={currencySymbol}
        placeholder="0.00"
        disabled={submitting}
        required
        inputClass="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-white/20 focus:ring-0"
      />
      {#if quantity > 1 && priceAmount !== null}
        <p class="ml-1 text-xs text-zinc-500">
          Total: <span class="font-mono text-zinc-300">{totalDisplay} {priceCurrency}</span>
        </p>
      {/if}
    </div>

    <!-- Seller -->
    <div class="space-y-2">
      <label
        for="purchase-seller"
        class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
      >
        {m.track_purchase_field_seller()}
      </label>
      <div class="relative">
        <select
          id="purchase-seller"
          class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 pr-4 pl-10 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
          value={selectedSellerId}
          onchange={(e) => (selectedSellerId = e.currentTarget.value)}
          disabled={submitting}
        >
          <option value="">{m.track_purchase_field_no_seller()}</option>
          {#each sellers as seller (seller.id)}
            <option value={seller.id}>{seller.name}</option>
          {/each}
        </select>
        <Store size={16} class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600" />
      </div>
    </div>

    <!-- Purchase date -->
    <div class="space-y-2">
      <label
        for="purchase-date"
        class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
      >
        {m.track_purchase_field_transaction_date()}
      </label>
      <DatePickerField id="purchase-date" bind:value={purchaseDate} disabled={submitting} />
    </div>
  </div>

  {#if error}
    <div
      class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-xs font-bold tracking-wider text-red-500 uppercase"
    >
      {m.track_purchase_field_system_error()}: {error}
    </div>
  {/if}
</div>
