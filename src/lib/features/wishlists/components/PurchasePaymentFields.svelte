<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { SellerView } from '$lib/bindings';

  interface Props {
    priceAmount: string;
    priceCurrency: string;
    purchaseDate: string;
    selectedSellerId: string;
    selectedCondition: string;
    sellers: SellerView[];
    isSubmitting: boolean;
    today: string;
  }

  let {
    priceAmount = $bindable(),
    priceCurrency = $bindable(),
    purchaseDate = $bindable(),
    selectedSellerId = $bindable(),
    selectedCondition = $bindable(),
    sellers,
    isSubmitting,
    today
  }: Props = $props();

  const CONDITION_OPTIONS = [
    { value: 'New', label: m.purchase_dialog_condition_new() },
    { value: 'PreOwnedLikeNew', label: m.purchase_dialog_condition_pre_owned_like_new() },
    { value: 'PreOwnedVeryGood', label: m.purchase_dialog_condition_pre_owned_very_good() },
    { value: 'PreOwnedGood', label: m.purchase_dialog_condition_pre_owned_good() },
    { value: 'PreOwnedAcceptable', label: m.purchase_dialog_condition_pre_owned_acceptable() }
  ];
</script>

<!-- Price -->
<div class="space-y-2">
  <label
    for="purchase-price"
    class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.purchase_dialog_price_label()}
  </label>
  <div class="flex gap-2">
    <input
      id="purchase-price"
      type="number"
      min="0"
      step="0.01"
      placeholder={m.purchase_dialog_price_placeholder()}
      bind:value={priceAmount}
      disabled={isSubmitting}
      required
      class="h-12 flex-1 appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 placeholder-zinc-600 focus:border-white/20 focus:outline-none"
    />
    <select
      bind:value={priceCurrency}
      disabled={isSubmitting}
      class="h-12 appearance-none rounded-xl border border-white/10 bg-zinc-950 px-3 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
    >
      <option value="EUR">EUR</option>
      <option value="USD">USD</option>
      <option value="GBP">GBP</option>
      <option value="JPY">JPY</option>
    </select>
  </div>
</div>

<!-- Purchase Date -->
<div class="space-y-2">
  <label
    for="purchase-date"
    class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.purchase_dialog_date_label()}
  </label>
  <input
    id="purchase-date"
    type="date"
    max={today}
    bind:value={purchaseDate}
    disabled={isSubmitting}
    required
    class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
  />
</div>

<!-- Seller (optional) -->
<div class="space-y-2">
  <label
    for="purchase-seller"
    class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.purchase_dialog_seller_label()}
  </label>
  <select
    id="purchase-seller"
    bind:value={selectedSellerId}
    disabled={isSubmitting}
    class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
  >
    <option value="">{m.purchase_dialog_seller_placeholder()}</option>
    {#each sellers as seller (seller.id)}
      <option value={seller.id}>{seller.name}</option>
    {/each}
  </select>
</div>

<!-- Condition (optional) -->
<div class="space-y-2">
  <label
    for="purchase-condition"
    class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.purchase_dialog_condition_label()}
  </label>
  <select
    id="purchase-condition"
    bind:value={selectedCondition}
    disabled={isSubmitting}
    class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
  >
    <option value="">{m.purchase_dialog_condition_placeholder()}</option>
    {#each CONDITION_OPTIONS as opt (opt.value)}
      <option value={opt.value}>{opt.label}</option>
    {/each}
  </select>
</div>
