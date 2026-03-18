<script lang="ts">
  import type { Manufacturer, RailwayCompany, SellerView } from '$lib/bindings';
  import type { AddModelFormState } from '$lib/features/collection/types/AddModelFormTypes';
  import * as m from '$lib/paraglide/messages.js';
  import { Plus } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import ModelInfoSection from '$lib/components/drawer/sections/ModelInfoSection.svelte';
  import RollingStockEntry from './RollingStockEntry.svelte';
  import PurchaseSection from './PurchaseSection.svelte';

  interface ValidationErrors {
    manufacturerId?: string;
    productCode?: string;
    description?: string;
    category?: string;
    scale?: string;
    powerMethod?: string;
    epoch?: string;
    rollingStocks?: string;
    rollingStockErrors?: Array<{
      railwayCompanyId?: string;
      seriesCode?: string;
      category?: string;
    }>;
  }

  interface Props {
    /** Full form state, bound two-way */
    form: AddModelFormState;
    /** Reference data for dropdowns */
    manufacturers: Manufacturer[];
    railwayCompanies: RailwayCompany[];
    sellers: SellerView[];
    /** Whether the purchase section is expanded */
    showPurchaseSection: boolean;
    /** Validation errors to display */
    validationErrors: ValidationErrors;
    /** Whether reference data is still loading */
    isLoading?: boolean;
    /** Callbacks */
    onAddRollingStock: () => void;
    onRemoveRollingStock: (uid: string) => void;
    onTogglePurchaseSection: () => void;
  }

  let {
    form = $bindable(),
    manufacturers,
    railwayCompanies,
    sellers,
    showPurchaseSection = $bindable(),
    validationErrors,
    isLoading = false,
    onAddRollingStock,
    onRemoveRollingStock,
    onTogglePurchaseSection
  }: Props = $props();
</script>

<!-- Model Info Section (shared drawer component) -->
<ModelInfoSection
  bind:manufacturerId={form.manufacturerId}
  bind:productCode={form.productCode}
  bind:description={form.description}
  bind:category={form.category}
  bind:scale={form.scale}
  bind:powerMethod={form.powerMethod}
  bind:epoch={form.epoch}
  {manufacturers}
  {isLoading}
  errors={validationErrors}
/>

<!-- Rolling Stocks Section -->
<div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  <section>
    <div class="mb-4 flex items-center justify-between">
      <p class="text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
        {m.add_model_section_rolling_stock()}
      </p>
      <Button
        type="button"
        variant="ghost"
        size="sm"
        class="border border-[#1F1F1F] bg-transparent text-[#E0E0E0] hover:bg-[rgba(212,138,66,0.15)]"
        onclick={onAddRollingStock}
      >
        <Plus size={16} />
        <span>{m.add_model_add_rolling_stock()}</span>
      </Button>
    </div>

    {#if validationErrors.rollingStocks}
      <p class="mb-3 text-sm text-destructive">{validationErrors.rollingStocks}</p>
    {/if}

    <div class="space-y-4">
      {#each form.rollingStocks as entry, index (entry.uid)}
        <RollingStockEntry
          dark={true}
          bind:entry={form.rollingStocks[index]}
          {railwayCompanies}
          canRemove={form.rollingStocks.length > 1}
          onRemove={() => onRemoveRollingStock(entry.uid)}
          errors={validationErrors.rollingStockErrors?.[index]}
        />
      {/each}
    </div>
  </section>
</div>

<!-- Purchase Section -->
<PurchaseSection
  dark={true}
  bind:purchase={form.purchase}
  {sellers}
  bind:expanded={showPurchaseSection}
  onToggle={onTogglePurchaseSection}
/>
