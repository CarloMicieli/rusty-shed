<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RailwayModelId } from '$lib/bindings';
  import { Accordion as AccordionPrimitive } from 'bits-ui';
  import * as Accordion from '$lib/components/ui/accordion';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import * as m from '$lib/paraglide/messages';
  import { untrack } from 'svelte';
  import RollingStockListItemHeader from './RollingStockListItemHeader.svelte';
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
    onDelete?: (unitId: string) => Promise<void>;
    deletePendingId?: string | null;
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
    onDelete,
    deletePendingId = null,
    onSpecsSaved
  }: Props = $props();

  let accordionValue = $state<string | undefined>(untrack(() => units[0]?.id));
  let specsDrawerOpenFor = $state<string | null>(null);
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
        <RollingStockListItemHeader
          {unit}
          {editable}
          {formState}
          {specLoaded}
          onSaveRoadNumber={(v) => onSaveIdentification(unit.id, 'roadNumber', v)}
          onSaveCategory={(c) => onSaveCategory(unit.id, c)}
          onSaveSubcategory={(s) => onSaveSubcategory(unit.id, s)}
          onEditSpecs={() => {
            specsDrawerOpenFor = unit.id;
          }}
          {onDelete}
          deletePending={deletePendingId === unit.id}
        >
          {#snippet extraActions()}
            <AccordionPrimitive.Trigger
              class="flex items-center justify-center rounded-sm border border-border p-0.5 text-muted-foreground transition-colors outline-none hover:border-primary hover:text-primary"
              aria-label={isOpen ? 'Collapse' : 'Expand'}
            >
              <ChevronDown
                class="h-4 w-4 transition-transform duration-300 {isOpen ? 'rotate-180' : ''}"
              />
            </AccordionPrimitive.Trigger>
          {/snippet}
        </RollingStockListItemHeader>
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
