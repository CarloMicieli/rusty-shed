<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RailwayModelId } from '$lib/bindings';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import RollingStockCardHeaderShell from './RollingStockCardHeaderShell.svelte';
  import RollingStockClassificationCluster from './RollingStockClassificationCluster.svelte';
  import RollingStockIdentityCluster from './RollingStockIdentityCluster.svelte';
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
    onSpecsSaved
  }: Props = $props();

  let specsDrawerOpen = $state(false);

  const editSpecsClass =
    'variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95';
</script>

{#snippet headerIdentity()}
  <div class="min-w-0">
    <RollingStockIdentityCluster
      {unit}
      {editable}
      onSaveRoadNumber={(v) => onSaveIdentification('roadNumber', v)}
    />
  </div>
{/snippet}

{#snippet headerClassification()}
  <RollingStockClassificationCluster
    {unit}
    {editable}
    {specLoaded}
    {formState}
    {onSaveCategory}
    {onSaveSubcategory}
  />
{/snippet}

{#snippet headerActions()}
  <div class="flex justify-end">
    {#if editable}
      <button
        type="button"
        class={editSpecsClass}
        onclick={() => {
          specsDrawerOpen = true;
        }}
      >
        <Settings size={12} />
        {m.rolling_stock_edit_specs_button()}
      </button>
    {/if}
  </div>
{/snippet}

<div class="overflow-hidden rounded-sm border border-border bg-card">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>

  <RollingStockCardHeaderShell
    identity={headerIdentity}
    classification={headerClassification}
    actions={headerActions}
  />

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
