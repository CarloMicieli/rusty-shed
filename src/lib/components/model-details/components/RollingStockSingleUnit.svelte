<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RailwayModelId } from '$lib/bindings';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import * as m from '$lib/paraglide/messages';
  import RollingStockListItemHeader from './RollingStockListItemHeader.svelte';
  import RollingStockUnitSpecsGrid from './RollingStockUnitSpecsGrid.svelte';
  import type { RollingStockUnitSpecsFormState } from './rolling-stock-unit-specs-form-state';

  interface Props {
    unit: RollingStock;
    editable: boolean;
    formState: RollingStockUnitSpecsFormState | undefined;
    specLoaded: boolean;
    railwayModelId: RailwayModelId;
    controlOptions: { id: string; label: string }[];
    dccInterfaceOptions: { id: string; label: string }[];
    couplingSocketOptions: { id: string; label: string }[];
    onSaveIdentification: (
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveSpec: (field: string, value: string) => Promise<void>;
    onSaveBoolSpec: (
      field: 'closeCouplers' | 'digitalShunting',
      value: boolean | null
    ) => Promise<void>;
    onSaveLength: (value: string) => Promise<void>;
    onSaveCategory: (category: string) => Promise<void>;
    onSaveSubcategory: (subcategory: string) => Promise<void>;
    onSaveServiceLevel: (serviceLevel: string) => Promise<void>;
    onDelete?: (unitId: string) => Promise<void>;
    deletePending?: boolean;
    onSpecsSaved?: () => Promise<void> | void;
  }

  const {
    unit,
    editable,
    formState,
    specLoaded,
    railwayModelId,
    controlOptions,
    dccInterfaceOptions,
    couplingSocketOptions,
    onSaveIdentification,
    onSaveSpec,
    onSaveBoolSpec,
    onSaveLength,
    onSaveCategory,
    onSaveSubcategory,
    onSaveServiceLevel,
    onDelete,
    deletePending = false,
    onSpecsSaved
  }: Props = $props();

  let specsDrawerOpen = $state(false);
</script>

<div class="overflow-hidden rounded-sm border border-border bg-card">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>

  <div class="border-b border-border bg-card px-3 py-2">
    <RollingStockListItemHeader
      {unit}
      {editable}
      {formState}
      {specLoaded}
      onSaveRoadNumber={(v) => onSaveIdentification('roadNumber', v)}
      {onSaveCategory}
      {onSaveSubcategory}
      onEditSpecs={() => {
        specsDrawerOpen = true;
      }}
      {onDelete}
      {deletePending}
    />
  </div>

  <div class="p-4">
    <RollingStockUnitSpecsGrid
      {unit}
      {editable}
      {formState}
      {specLoaded}
      {controlOptions}
      {dccInterfaceOptions}
      {couplingSocketOptions}
      {onSaveIdentification}
      {onSaveSpec}
      {onSaveBoolSpec}
      {onSaveLength}
      {onSaveServiceLevel}
    />
  </div>
</div>

{#if editable}
  <RollingStockSpecsDrawer
    open={specsDrawerOpen}
    {railwayModelId}
    rollingStockId={unit.id}
    ownedRollingStockId={unit.ownedRollingStockId}
    currentCouplerId={unit.currentCouplerId}
    onClose={() => {
      specsDrawerOpen = false;
    }}
    onSaved={() => onSpecsSaved?.()}
  />
{/if}
