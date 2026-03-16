<script lang="ts">
  import type { Manufacturer, RailwayCompany, SellerView } from '$lib/bindings';
  import type { AddModelFormState } from '$lib/features/collection/types/AddModelFormTypes';
  import RollingStockEntry from './RollingStockEntry.svelte';
  import PurchaseSection from './PurchaseSection.svelte';
  import RailwayModelBaseForm from '$lib/shared/components/RailwayModelBaseForm.svelte';
  import scales from '$lib/data/constants/scales.json';
  import categories from '$lib/data/constants/categories.json';
  import powerMethods from '$lib/data/constants/powerMethods.json';

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
    onAddRollingStock,
    onRemoveRollingStock,
    onTogglePurchaseSection
  }: Props = $props();
</script>

<!-- Base Railway Model Form (shared component) -->
<RailwayModelBaseForm
  dark={true}
  {manufacturers}
  categoryOptions={categories}
  scaleOptions={scales}
  powerMethodOptions={powerMethods}
  {form}
  validationErrors={validationErrors as Record<string, string | undefined>}
  {onAddRollingStock}
>
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
</RailwayModelBaseForm>

<!-- Purchase Section -->
<PurchaseSection
  dark={true}
  bind:purchase={form.purchase}
  {sellers}
  bind:expanded={showPurchaseSection}
  onToggle={onTogglePurchaseSection}
/>
