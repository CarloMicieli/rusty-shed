<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Plus, Tag, Store } from 'lucide-svelte';
  import type { TrackProductView, SellerView, Currency } from '$lib/bindings';
  import type { Component } from 'svelte';
  import { Input, Button, DatePickerField } from '$lib/components';
  import { FormPrice } from '$lib/components/drawer';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';

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
  const currencySymbol = $derived(regionalManager.getCurrencySymbol(priceCurrency));

  const productOptions = $derived(
    products.map((p) => ({
      value: p.track_id,
      label: `${p.manufacturer_name} • ${p.description || p.product_code}`
    }))
  );

  const sellerOptions = $derived(sellers.map((s) => ({ value: s.id, label: s.name })));

  // Auto-focus the quantity input when a product is selected
  $effect(() => {
    if (selectedProductId) {
      // Defer so the DOM has settled after product selection
      setTimeout(() => {
        (document.getElementById('purchase-quantity') as HTMLInputElement | null)?.focus();
      }, 50);
    }
  });
</script>

<div class="space-y-5">
  <!-- ── Track Product (full width) ──────────────────────────────── -->
  <div class="space-y-2">
    <label
      for="track-product-select"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.track_purchase_field_product()}
    </label>
    <div class="flex gap-2">
      <div class="flex-1">
        <SearchableSelect
          id="track-product-select"
          options={productOptions}
          bind:value={selectedProductId}
          placeholder={m.track_purchase_field_product_placeholder()}
          disabled={submitting}
        />
      </div>
      <Button
        type="button"
        variant="outline"
        onclick={onCreateProduct}
        class="h-12 w-12 shrink-0 border-white/10 bg-zinc-900/50 p-0 text-zinc-400 hover:bg-zinc-800 hover:text-white"
        disabled={submitting}
        title={m.track_purchase_create_product()}
      >
        <Plus size={20} />
      </Button>
    </div>
  </div>

  <!-- ── Row 1: Quantity + Unit Price ────────────────────────────── -->
  <div class="grid grid-cols-2 gap-4">
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
          class="h-12 rounded-xl border-white/10 bg-zinc-950 pl-10 text-zinc-100 focus:border-amber-500/50 focus:ring-0"
          disabled={submitting}
          required
        />
        <Tag size={16} class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600" />
      </div>
    </div>

    <!-- Unit Price -->
    <div class="space-y-2">
      <FormPrice
        label={m.track_purchase_field_market_price()}
        id="purchase-price"
        bind:value={priceAmount}
        symbol={currencySymbol}
        placeholder="0.00"
        disabled={submitting}
        required
        inputClass="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-amber-500/50 focus:ring-0"
      />
      {#if quantity > 1 && priceAmount !== null}
        <p class="ml-1 text-xs text-zinc-500">
          {m.track_purchase_total()}:
          <span class="font-mono text-zinc-300">{totalDisplay} {priceCurrency}</span>
        </p>
      {/if}
    </div>
  </div>

  <!-- ── Row 2: Seller (full width) ────────────────────────────────── -->
  <div class="space-y-2">
    <label
      for="purchase-seller"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.track_purchase_field_seller()}
    </label>
    <SearchableSelect
      id="purchase-seller"
      options={sellerOptions}
      bind:value={selectedSellerId}
      emptyOption={{ value: '', label: m.track_purchase_field_no_seller() }}
      icon={Store as unknown as Component<{
        size?: number | undefined;
        class?: string | undefined;
      }>}
      disabled={submitting}
    />
  </div>

  <!-- ── Row 3: Purchase Date (full width) ─────────────────────── -->
  <div class="space-y-2">
    <label
      for="purchase-date"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.track_purchase_field_transaction_date()}
    </label>
    <DatePickerField id="purchase-date" bind:value={purchaseDate} disabled={submitting} />
  </div>

  {#if error}
    <div
      class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-xs font-bold tracking-wider text-red-500 uppercase"
    >
      {m.track_purchase_field_system_error()}: {error}
    </div>
  {/if}
</div>
