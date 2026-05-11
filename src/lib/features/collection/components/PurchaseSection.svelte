<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ChevronDown } from 'lucide-svelte';
  import { Textarea, CurrencyInput, DatePickerField } from '$lib/components';
  import { FormSelect } from '$lib/components/drawer';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import type { SellerView } from '$lib/bindings';
  import type { PurchaseFormState } from '$lib/features/collection/types/AddModelFormTypes';

  interface Props {
    /** Purchase state bound two-way */
    purchase: PurchaseFormState;
    /** Available sellers for dropdown */
    sellers: SellerView[];
    /** Whether section is expanded */
    expanded: boolean;
    /** Toggle expanded state */
    onToggle: () => void;
    /** Enable mechanical dark mode styling */
    dark?: boolean;
  }

  let {
    purchase = $bindable(),
    sellers,
    expanded = $bindable(),
    onToggle,
    dark = false
  }: Props = $props();

  const darkTextarea =
    'w-full rounded-md border border-layout-border bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-2 focus:ring-primary/30 focus:outline-none resize-none';

  // Map sellers to FormSelect option shape
  const sellerOptions = $derived(sellers.map((s) => ({ value: s.id, label: s.name })));

  // Purchase conditions
  const purchaseConditionOptions = [
    { value: 'NEW', label: 'New' },
    { value: 'PRE_OWNED', label: 'Pre-owned' }
  ];

  // Model conditions
  const modelConditionOptions = [
    { value: 'MINT', label: 'Mint' },
    { value: 'NEAR_MINT', label: 'Near Mint' },
    { value: 'EXCELLENT', label: 'Excellent' },
    { value: 'VERY_GOOD', label: 'Very Good' },
    { value: 'GOOD', label: 'Good' },
    { value: 'FAIR', label: 'Fair' },
    { value: 'POOR', label: 'Poor' },
    { value: 'FOR_PARTS', label: 'For Parts' }
  ];

  // Box conditions
  const boxConditionOptions = [
    { value: 'ORIGINAL_MINT', label: 'Original - Mint' },
    { value: 'ORIGINAL_GOOD', label: 'Original - Good' },
    { value: 'ORIGINAL_WORN', label: 'Original - Worn' },
    { value: 'REPLACEMENT_BOX', label: 'Replacement Box' },
    { value: 'NO_BOX', label: 'No Box' }
  ];
</script>

<div
  class={`purchase-section overflow-hidden rounded-lg border ${
    dark ? 'border-layout-border bg-zinc-950/90' : 'border-border bg-card text-card-foreground'
  }`}
>
  <!-- Section Header -->
  <button
    type="button"
    class={`flex w-full items-center justify-between px-4 py-3 text-left text-foreground transition-all duration-300 ${
      dark ? 'hover:bg-zinc-900' : 'hover:bg-muted'
    }`}
    onclick={onToggle}
    aria-expanded={expanded}
  >
    {#if dark}
      <p class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
        {m.add_model_section_purchase()}
      </p>
    {:else}
      <h3 class="text-lg font-semibold">{m.add_model_section_purchase()}</h3>
    {/if}
    <ChevronDown
      size={16}
      class={`text-muted-foreground transition-transform duration-300 ${expanded ? 'rotate-180' : ''}`}
    />
  </button>

  <!-- Section Content -->
  {#if expanded}
    <div
      class="space-y-4 border-t bg-zinc-900/60 px-4 py-4 transition-all duration-300"
      class:border-layout-border={dark}
      class:border-border={!dark}
    >
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        <!-- Seller -->
        <FormSelect
          id="seller"
          label="{m.add_model_seller()} (optional)"
          options={sellerOptions}
          bind:value={purchase.sellerId}
          placeholder="-- {m.add_model_seller()} --"
        />

        <!-- Purchase Type toggle -->
        <div>
          <div class="mb-1 block">
            {#if dark}
              <span class="text-[10px] text-muted-foreground uppercase"
                >{m.add_model_purchase_type()}</span
              >
            {:else}
              <span class="text-sm text-muted-foreground">{m.add_model_purchase_type()}</span>
            {/if}
          </div>
          <div class="flex gap-1 rounded-lg border border-border/60 p-0.5">
            <button
              type="button"
              class="flex-1 rounded px-2 py-1 text-xs transition-colors {purchase.purchaseType ===
              'STANDARD'
                ? 'bg-primary/15 font-medium text-primary'
                : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => {
                purchase.purchaseType = 'STANDARD';
              }}
            >
              {m.add_model_purchase_type_standard()}
            </button>
            <button
              type="button"
              class="flex-1 rounded px-2 py-1 text-xs transition-colors {purchase.purchaseType ===
              'PREORDER'
                ? 'bg-primary/15 font-medium text-primary'
                : 'text-muted-foreground hover:text-foreground'}"
              onclick={() => {
                purchase.purchaseType = 'PREORDER';
              }}
            >
              {m.add_model_purchase_type_preorder()}
            </button>
          </div>
        </div>

        <!-- Purchase Date (label changes for preorders) -->
        <div>
          <label for="purchase-date" class="mb-1 block">
            {#if dark}
              <span class="text-[10px] text-muted-foreground uppercase">
                {purchase.purchaseType === 'PREORDER'
                  ? m.add_model_preorder_date()
                  : m.add_model_purchase_date()}
              </span>
              <span class="ml-1 text-muted-foreground/50">(optional)</span>
            {:else}
              <span class="text-sm text-muted-foreground">
                {purchase.purchaseType === 'PREORDER'
                  ? m.add_model_preorder_date()
                  : m.add_model_purchase_date()}
              </span>
              <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
            {/if}
          </label>
          <DatePickerField id="purchase-date" bind:value={purchase.purchaseDate} />
        </div>
      </div>

      {#if purchase.purchaseType === 'STANDARD'}
        <div class="grid grid-cols-1 gap-4">
          <!-- Price Amount -->
          <div>
            <label for="price-amount" class="mb-1 block">
              {#if dark}
                <span class="text-[10px] text-muted-foreground uppercase"
                  >{m.add_model_price()}</span
                >
                <span class="ml-1 text-muted-foreground/50">(optional)</span>
              {:else}
                <span class="text-sm text-muted-foreground">{m.add_model_price()}</span>
                <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
              {/if}
            </label>
            <CurrencyInput
              id="price-amount"
              bind:value={purchase.priceAmount}
              symbol={regionalManager.getCurrencySymbol(purchase.priceCurrency)}
              placeholder={m.placeholder_amount()}
              class="w-full"
              label={m.add_model_price()}
              inputClass={dark
                ? 'bg-transparent border-layout-border text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-primary/30'
                : ''}
            />
          </div>
        </div>
      {:else}
        <!-- Preorder fields -->
        <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
          <!-- Deposit Amount -->
          <div>
            <label for="deposit-amount" class="mb-1 block">
              {#if dark}
                <span class="text-[10px] text-muted-foreground uppercase"
                  >{m.add_model_deposit_amount()}</span
                >
              {:else}
                <span class="text-sm text-muted-foreground">{m.add_model_deposit_amount()}</span>
              {/if}
            </label>
            <CurrencyInput
              id="deposit-amount"
              bind:value={purchase.depositAmount}
              symbol={regionalManager.getCurrencySymbol(
                purchase.depositCurrency ?? purchase.priceCurrency
              )}
              placeholder={m.placeholder_amount()}
              class="w-full"
              label={m.add_model_deposit_amount()}
              inputClass={dark
                ? 'bg-transparent border-layout-border text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-primary/30'
                : ''}
            />
          </div>

          <!-- Preorder Total Amount -->
          <div>
            <label for="preorder-total-amount" class="mb-1 block">
              {#if dark}
                <span class="text-[10px] text-muted-foreground uppercase"
                  >{m.add_model_preorder_total()}</span
                >
              {:else}
                <span class="text-sm text-muted-foreground">{m.add_model_preorder_total()}</span>
              {/if}
            </label>
            <CurrencyInput
              id="preorder-total-amount"
              bind:value={purchase.preorderTotalAmount}
              symbol={regionalManager.getCurrencySymbol(
                purchase.preorderTotalCurrency ?? purchase.priceCurrency
              )}
              placeholder={m.placeholder_amount()}
              class="w-full"
              label={m.add_model_preorder_total()}
              inputClass={dark
                ? 'bg-transparent border-layout-border text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-primary/30'
                : ''}
            />
          </div>
        </div>

        <!-- Remaining balance derived display -->
        {#if (purchase.preorderTotalAmount ?? 0) > 0 || (purchase.depositAmount ?? 0) > 0}
          {@const remaining = (purchase.preorderTotalAmount ?? 0) - (purchase.depositAmount ?? 0)}
          <p class="text-xs text-muted-foreground">
            {m.add_model_remaining_balance()}:
            <span class:text-destructive={remaining < 0} class:text-primary={remaining >= 0}>
              {regionalManager.formatCurrencyWith(
                remaining,
                purchase.preorderTotalCurrency ?? purchase.priceCurrency
              )}
            </span>
          </p>
        {/if}

        <!-- Expected Delivery Date -->
        <div>
          <label for="expected-date" class="mb-1 block">
            {#if dark}
              <span class="text-[10px] text-muted-foreground uppercase"
                >{m.add_model_expected_date()}</span
              >
              <span class="ml-1 text-muted-foreground/50">(optional)</span>
            {:else}
              <span class="text-sm text-muted-foreground">{m.add_model_expected_date()}</span>
              <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
            {/if}
          </label>
          <DatePickerField id="expected-date" bind:value={purchase.expectedDate} />
        </div>
      {/if}

      <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
        <!-- Purchase Condition -->
        <FormSelect
          id="purchase-condition"
          label="{m.add_model_purchase_condition()} (optional)"
          options={purchaseConditionOptions}
          bind:value={purchase.purchaseCondition}
          placeholder={m.form_new_model_select_placeholder()}
        />

        <!-- Model Condition -->
        <FormSelect
          id="model-condition"
          label="{m.add_model_model_condition()} (optional)"
          options={modelConditionOptions}
          bind:value={purchase.modelCondition}
          placeholder={m.form_new_model_select_placeholder()}
        />

        <!-- Box Condition -->
        <FormSelect
          id="box-condition"
          label="{m.add_model_box_condition()} (optional)"
          options={boxConditionOptions}
          bind:value={purchase.boxCondition}
          placeholder={m.form_new_model_select_placeholder()}
        />
      </div>

      <!-- Notes -->
      <div>
        <label for="notes" class="mb-1 block">
          {#if dark}
            <span class="text-[10px] text-muted-foreground uppercase">{m.add_model_notes()}</span>
            <span class="ml-1 text-muted-foreground/50">(optional)</span>
          {:else}
            <span class="text-sm text-muted-foreground">{m.add_model_notes()}</span>
            <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
          {/if}
        </label>
        {#if dark}
          <textarea
            id="notes"
            bind:value={purchase.notes}
            rows={3}
            placeholder={m.add_model_notes_placeholder()}
            class={darkTextarea}
          ></textarea>
        {:else}
          <Textarea
            id="notes"
            bind:value={purchase.notes}
            rows={3}
            placeholder={m.add_model_notes_placeholder()}
            class="w-full"
          />
        {/if}
      </div>
    </div>
  {/if}
</div>
