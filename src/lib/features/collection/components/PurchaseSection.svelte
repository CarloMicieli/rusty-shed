<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ChevronDown, ChevronRight } from 'lucide-svelte';
  import { Textarea, CurrencyInput, DatePickerField } from '$lib/components';
  import { getCurrencySymbol } from '$lib/utils/currency';
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
  }

  let { purchase = $bindable(), sellers, expanded = $bindable(), onToggle }: Props = $props();

  // Purchase conditions (from bindings)
  const purchaseConditions = [
    { id: 'NEW', label: 'New' },
    { id: 'PRE_OWNED', label: 'Pre-owned' }
  ];

  // Model conditions
  const modelConditions = [
    { id: 'MINT', label: 'Mint' },
    { id: 'NEAR_MINT', label: 'Near Mint' },
    { id: 'EXCELLENT', label: 'Excellent' },
    { id: 'VERY_GOOD', label: 'Very Good' },
    { id: 'GOOD', label: 'Good' },
    { id: 'FAIR', label: 'Fair' },
    { id: 'POOR', label: 'Poor' },
    { id: 'FOR_PARTS', label: 'For Parts' }
  ];

  // Box conditions
  const boxConditions = [
    { id: 'ORIGINAL_MINT', label: 'Original - Mint' },
    { id: 'ORIGINAL_GOOD', label: 'Original - Good' },
    { id: 'ORIGINAL_WORN', label: 'Original - Worn' },
    { id: 'REPLACEMENT_BOX', label: 'Replacement Box' },
    { id: 'NO_BOX', label: 'No Box' }
  ];
</script>

<div class="purchase-section rounded-lg border border-border bg-card text-card-foreground">
  <!-- Section Header -->
  <button
    type="button"
    class="flex w-full items-center justify-between p-4 text-left text-foreground hover:bg-muted"
    onclick={onToggle}
    aria-expanded={expanded}
  >
    <h3 class="text-lg font-semibold">{m.add_model_section_purchase()}</h3>
    {#if expanded}
      <ChevronDown size={20} />
    {:else}
      <ChevronRight size={20} />
    {/if}
  </button>

  <!-- Section Content -->
  {#if expanded}
    <div class="space-y-4 border-t border-border p-4">
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        <!-- Seller -->
        <div>
          <label for="seller" class="mb-1 block">
            <span class="text-sm text-muted-foreground">{m.add_model_seller()}</span>
            <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
          </label>
          <select
            id="seller"
            bind:value={purchase.sellerId}
            class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value={null}>-- {m.add_model_seller()} --</option>
            {#each sellers as seller (seller.id)}
              <option value={seller.id}>{seller.name}</option>
            {/each}
          </select>
        </div>

        <!-- Purchase Date -->
        <div>
          <label for="purchase-date" class="mb-1 block">
            <span class="text-sm text-muted-foreground">{m.add_model_purchase_date()}</span>
            <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
          </label>
          <DatePickerField id="purchase-date" bind:value={purchase.purchaseDate} />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4">
        <!-- Price Amount -->
        <div>
          <label for="price-amount" class="mb-1 block">
            <span class="text-sm text-muted-foreground">{m.add_model_price()}</span>
            <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
          </label>
          <CurrencyInput
            id="price-amount"
            bind:value={purchase.priceAmount}
            symbol={getCurrencySymbol(purchase.priceCurrency)}
            placeholder="0.00"
            class="w-full"
            label={m.add_model_price()}
          />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
        <!-- Purchase Condition -->
        <div>
          <label for="purchase-condition" class="mb-1 flex items-baseline">
            <span class="text-sm text-muted-foreground">{m.add_model_purchase_condition()}</span>
            <span class="ml-1 text-xs whitespace-nowrap text-muted-foreground/60">(optional)</span>
          </label>
          <select
            id="purchase-condition"
            bind:value={purchase.purchaseCondition}
            class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value={null}>-- Select --</option>
            {#each purchaseConditions as condition (condition.id)}
              <option value={condition.id}>{condition.label}</option>
            {/each}
          </select>
        </div>

        <!-- Model Condition -->
        <div>
          <label for="model-condition" class="mb-1 flex items-baseline">
            <span class="text-sm text-muted-foreground">{m.add_model_model_condition()}</span>
            <span class="ml-1 text-xs whitespace-nowrap text-muted-foreground/60">(optional)</span>
          </label>
          <select
            id="model-condition"
            bind:value={purchase.modelCondition}
            class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value={null}>-- Select --</option>
            {#each modelConditions as condition (condition.id)}
              <option value={condition.id}>{condition.label}</option>
            {/each}
          </select>
        </div>

        <!-- Box Condition -->
        <div>
          <label for="box-condition" class="mb-1 flex items-baseline">
            <span class="text-sm text-muted-foreground">{m.add_model_box_condition()}</span>
            <span class="ml-1 text-xs whitespace-nowrap text-muted-foreground/60">(optional)</span>
          </label>
          <select
            id="box-condition"
            bind:value={purchase.boxCondition}
            class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value={null}>-- Select --</option>
            {#each boxConditions as condition (condition.id)}
              <option value={condition.id}>{condition.label}</option>
            {/each}
          </select>
        </div>
      </div>

      <!-- Notes -->
      <div>
        <label for="notes" class="mb-1 block">
          <span class="text-sm text-muted-foreground">{m.add_model_notes()}</span>
          <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
        </label>
        <Textarea
          id="notes"
          bind:value={purchase.notes}
          rows={3}
          placeholder="Additional notes about this purchase..."
          class="w-full"
        />
      </div>
    </div>
  {/if}
</div>
