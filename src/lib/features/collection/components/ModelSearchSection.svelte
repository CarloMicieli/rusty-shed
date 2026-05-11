<script lang="ts">
  import type { Manufacturer, RailwayCompany, SellerView } from '$lib/bindings';
  import type { AddModelFormState } from '$lib/features/collection/types/AddModelFormTypes';
  import * as m from '$lib/paraglide/messages.js';
  import { ChevronDown, Plus } from 'lucide-svelte';
  import { Collapsible as CollapsiblePrimitive } from 'bits-ui';
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
    /** Whether the rolling stock section is expanded */
    isRollingStockExpanded: boolean;
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
    isRollingStockExpanded = $bindable(),
    validationErrors,
    isLoading = false,
    onAddRollingStock,
    onRemoveRollingStock,
    onTogglePurchaseSection
  }: Props = $props();

  const rollingStockCountLabel = $derived(m.add_model_rolling_stock_items_label());
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
<CollapsiblePrimitive.Root bind:open={isRollingStockExpanded}>
  <div class="overflow-hidden rounded-lg border border-layout-border bg-zinc-950/90">
    <CollapsiblePrimitive.Trigger
      type="button"
      class="flex w-full items-center justify-between px-4 py-3 text-left transition-all duration-300 hover:bg-zinc-900"
    >
      <div class="flex items-center gap-3">
        <p class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
          {m.add_model_section_rolling_stock()}
        </p>
        <span class="font-mono text-[10px] text-muted-foreground uppercase">
          ({form.rollingStocks.length}) {rollingStockCountLabel}
        </span>
      </div>
      <ChevronDown
        size={16}
        class={`text-muted-foreground transition-transform duration-300 ${isRollingStockExpanded ? 'rotate-180' : ''}`}
      />
    </CollapsiblePrimitive.Trigger>

    <CollapsiblePrimitive.Content
      class="border-t border-layout-border bg-zinc-900/60 px-4 py-4 transition-all duration-300"
    >
      <div class="mb-4 flex items-center justify-end">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          class="border border-layout-border bg-transparent text-foreground hover:bg-primary/15"
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
            canRemove={true}
            onRemove={() => onRemoveRollingStock(entry.uid)}
            errors={validationErrors.rollingStockErrors?.[index]}
          />
        {/each}
      </div>
    </CollapsiblePrimitive.Content>
  </div>
</CollapsiblePrimitive.Root>

<!-- Purchase Section -->
<PurchaseSection
  dark={true}
  bind:purchase={form.purchase}
  {sellers}
  bind:expanded={showPurchaseSection}
  onToggle={onTogglePurchaseSection}
/>
