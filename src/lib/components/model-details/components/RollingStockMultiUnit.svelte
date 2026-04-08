<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RailwayModelId } from '$lib/bindings';
  import { Accordion as AccordionPrimitive } from 'bits-ui';
  import * as Accordion from '$lib/components/ui/accordion';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { untrack } from 'svelte';
  import RollingStockClassificationCluster from './RollingStockClassificationCluster.svelte';
  import RollingStockIdentityCluster from './RollingStockIdentityCluster.svelte';
  import RollingStockUnitSpecsGrid from './RollingStockUnitSpecsGrid.svelte';
  import type { RollingStockUnitSpecsFormState } from './rolling-stock-unit-specs-form-state';

  interface Props {
    units: RollingStock[];
    editable: boolean;
    railwayModelId: RailwayModelId;
    rollingStockFormState: Map<string, RollingStockUnitSpecsFormState>;
    rollingStockSpecLoaded: Set<string>;
    controlOptions: { id: string; label: string }[];
    dccInterfaceOptions: { id: string; label: string }[];
    couplingSocketOptions: { id: string; label: string }[];
    onSaveIdentification: (
      unitId: string,
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveSpec: (unitId: string, field: string, value: string) => Promise<void>;
    onSaveBoolSpec: (
      unitId: string,
      field: 'closeCouplers' | 'digitalShunting',
      value: boolean | null
    ) => Promise<void>;
    onSaveLength: (unitId: string, value: string) => Promise<void>;
    onSaveCategory: (unitId: string, category: string) => Promise<void>;
    onSaveSubcategory: (unitId: string, subcategory: string) => Promise<void>;
    onSaveServiceLevel: (unitId: string, serviceLevel: string) => Promise<void>;
    onSpecsSaved?: (unitId: string) => Promise<void> | void;
  }

  const {
    units,
    editable,
    railwayModelId,
    rollingStockFormState,
    rollingStockSpecLoaded,
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

  let accordionValue = $state<string | undefined>(untrack(() => units[0]?.id));
  let specsDrawerOpenFor = $state<string | null>(null);

  const editSpecsClass =
    'variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95';
</script>

<Accordion.Root type="single" bind:value={accordionValue} class="space-y-2">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>
  {#each units as unit (unit.id)}
    {@const formState = rollingStockFormState.get(unit.id)}
    {@const specLoaded = rollingStockSpecLoaded.has(unit.id)}
    <AccordionPrimitive.Item
      value={unit.id}
      class="overflow-hidden rounded-sm border border-border bg-card"
    >
      <AccordionPrimitive.Header
        class="border-b border-border bg-card px-3 py-2 data-[state=closed]:rounded-b-sm data-[state=closed]:border-b-transparent"
      >
        {@const isOpen = accordionValue === unit.id}
        <div class="grid grid-cols-3 items-center gap-2">
          <div class="flex min-w-0 items-center">
            <RollingStockIdentityCluster
              {unit}
              {editable}
              onSaveRoadNumber={(v) => onSaveIdentification(unit.id, 'roadNumber', v)}
            />
          </div>

          <div class="flex justify-center">
            <RollingStockClassificationCluster
              {unit}
              {editable}
              {specLoaded}
              {formState}
              onSaveCategory={(c) => onSaveCategory(unit.id, c)}
              onSaveSubcategory={(s) => onSaveSubcategory(unit.id, s)}
            />
          </div>

          <div class="flex items-center justify-end gap-2">
            {#if editable}
              <button
                type="button"
                class={editSpecsClass}
                onclick={() => {
                  specsDrawerOpenFor = unit.id;
                }}
              >
                <Settings size={12} />
                {m.rolling_stock_edit_specs_button()}
              </button>
            {/if}
            <AccordionPrimitive.Trigger
              class="flex items-center justify-center rounded-sm border border-border p-0.5 text-muted-foreground transition-colors outline-none hover:border-primary hover:text-primary"
              aria-label={isOpen ? 'Collapse' : 'Expand'}
            >
              <ChevronDown
                class="h-4 w-4 transition-transform duration-300 {isOpen ? 'rotate-180' : ''}"
              />
            </AccordionPrimitive.Trigger>
          </div>
        </div>
      </AccordionPrimitive.Header>

      <Accordion.Content class="px-4 pt-2.5 pb-4">
        <RollingStockUnitSpecsGrid
          {unit}
          {editable}
          {formState}
          {specLoaded}
          {controlOptions}
          {dccInterfaceOptions}
          {couplingSocketOptions}
          onSaveIdentification={(field, value) => onSaveIdentification(unit.id, field, value)}
          onSaveSpec={(field, value) => onSaveSpec(unit.id, field, value)}
          onSaveBoolSpec={(field, value) => onSaveBoolSpec(unit.id, field, value)}
          onSaveLength={(value) => onSaveLength(unit.id, value)}
          onSaveServiceLevel={(sl) => onSaveServiceLevel(unit.id, sl)}
          liveryTruncate={true}
        />
      </Accordion.Content>
    </AccordionPrimitive.Item>
  {/each}
</Accordion.Root>

{#each units as unit (unit.id)}
  {#if editable}
    <RollingStockSpecsDrawer
      open={specsDrawerOpenFor === unit.id}
      {railwayModelId}
      rollingStockId={unit.id}
      ownedRollingStockId={unit.ownedRollingStockId}
      currentCouplerId={unit.currentCouplerId}
      onClose={() => {
        specsDrawerOpenFor = null;
      }}
      onSaved={() => onSpecsSaved?.(unit.id)}
    />
  {/if}
{/each}
