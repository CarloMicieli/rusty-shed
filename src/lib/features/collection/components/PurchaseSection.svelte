<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ChevronDown, ChevronRight } from 'lucide-svelte';
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
  class="purchase-section rounded-lg border"
  class:border-layout-border={dark}
  class:bg-layout-surface={dark}
  class:border-border={!dark}
  class:bg-card={!dark}
  class:text-card-foreground={!dark}
>
  <!-- Section Header -->
  <button
    type="button"
    class="flex w-full items-center justify-between p-4 text-left text-foreground"
    class:hover:bg-[rgba(255,255,255,0.03)]={dark}
    class:hover:bg-muted={!dark}
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
    {#if expanded}
      <ChevronDown size={20} />
    {:else}
      <ChevronRight size={20} />
    {/if}
  </button>

  <!-- Section Content -->
  {#if expanded}
    <div
      class="space-y-4 border-t p-4"
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

        <!-- Purchase Date -->
        <div>
          <label for="purchase-date" class="mb-1 block">
            {#if dark}
              <span class="text-[10px] text-muted-foreground uppercase"
                >{m.add_model_purchase_date()}</span
              >
              <span class="ml-1 text-muted-foreground/50">(optional)</span>
            {:else}
              <span class="text-sm text-muted-foreground">{m.add_model_purchase_date()}</span>
              <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
            {/if}
          </label>
          <DatePickerField id="purchase-date" bind:value={purchase.purchaseDate} />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4">
        <!-- Price Amount -->
        <div>
          <label for="price-amount" class="mb-1 block">
            {#if dark}
              <span class="text-[10px] text-muted-foreground uppercase">{m.add_model_price()}</span>
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
            placeholder="0.00"
            class="w-full"
            label={m.add_model_price()}
            inputClass={dark
              ? 'bg-transparent border-layout-border text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-primary/30'
              : ''}
          />
        </div>
      </div>

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
