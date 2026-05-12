<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import ModelSearchSection from '$lib/features/collection/components/ModelSearchSection.svelte';
  import type { Manufacturer, RailwayCompany, SellerView } from '$lib/bindings';
  import type { AddModelFormState } from '$lib/features/collection/types/AddModelFormTypes';

  let {
    form = $bindable(),
    manufacturers = [],
    railwayCompanies = [],
    sellers = [],
    showPurchaseSection = $bindable(false),
    isRollingStockExpanded = $bindable(false),
    validationErrors = {},
    isLoading = false,
    onAddRollingStock = () => {},
    onRemoveRollingStock = () => {},
    onTogglePurchaseSection = () => {}
  }: {
    form: AddModelFormState;
    manufacturers?: Manufacturer[];
    railwayCompanies?: RailwayCompany[];
    sellers?: SellerView[];
    showPurchaseSection?: boolean;
    isRollingStockExpanded?: boolean;
    validationErrors?: Record<string, unknown>;
    isLoading?: boolean;
    onAddRollingStock?: () => void;
    onRemoveRollingStock?: (uid: string) => void;
    onTogglePurchaseSection?: () => void;
  } = $props();
</script>

<Tooltip.Provider>
  <ModelSearchSection
    bind:form
    {manufacturers}
    {railwayCompanies}
    {sellers}
    bind:showPurchaseSection
    bind:isRollingStockExpanded
    {validationErrors}
    {isLoading}
    {onAddRollingStock}
    {onRemoveRollingStock}
    {onTogglePurchaseSection}
  />
</Tooltip.Provider>
