<script lang="ts">
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  const AccordionItem = Accordion.Item;
  const AccordionItemTrigger = Accordion.ItemTrigger;
  const AccordionItemContent = Accordion.ItemContent;
  const AccordionItemIndicator = Accordion.ItemIndicator;

  import FormField from '$lib/components/ui/FormField.svelte';
  import type { ConstantItem } from '../constants';
  import type { RollingStockForm } from '../utils';
  import { resolveLabel } from '../../../../utils/resolveLabel';

  type Option = ConstantItem | { id: string; name: string };

  type Props = {
    rs: RollingStockForm;
    index: number;
    errors: Record<string, string>;
    onDuplicate: (i: number) => void;
    onDelete: (i: number) => void;
    rollingStockCategoriesData: ConstantItem[];
    railwayCompaniesData: { id: string; name: string }[];
    locomotiveTypesData: ConstantItem[];
    passengerCarTypesData: ConstantItem[];
    freightCarTypesData: ConstantItem[];
    electricMultipleUnitTypesData: ConstantItem[];
    controlsData: ConstantItem[];
    dccInterfacesData: ConstantItem[];
    serviceLevelsData: ConstantItem[];
    formLabels: Record<string, ConstantItem>;
  };

  const {
    rs,
    index,
    errors,
    onDuplicate,
    onDelete,
    rollingStockCategoriesData,
    railwayCompaniesData,
    locomotiveTypesData,
    passengerCarTypesData,
    freightCarTypesData,
    electricMultipleUnitTypesData,
    controlsData,
    dccInterfacesData,
    serviceLevelsData,
    formLabels
  }: Props = $props();

  const isLocomotive = $derived(rs.category === 'Locomotive');
  const isPassengerCar = $derived(rs.category === 'PassengerCar');
  const isFreightCar = $derived(rs.category === 'FreightCar');
  const isRailcar = $derived(rs.category === 'Railcar');
  const isElectricMultipleUnit = $derived(rs.category === 'ElectricMultipleUnit');

  function fieldError(name: string): string | undefined {
    return errors[`rolling_stocks.${index}.${name}`];
  }
</script>

{#snippet selectField(
  label: ConstantItem,
  fieldName: keyof RollingStockForm,
  required: boolean,
  options: Option[]
)}
  <FormField {label} error={fieldError(fieldName as string)} {required}>
    <select class="select border-surface-600 bg-surface-800" bind:value={rs[fieldName]}>
      <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
      {#each options as option (option.id)}
        <option value={option.id}>{'name' in option ? option.name : resolveLabel(option)}</option>
      {/each}
    </select>
  </FormField>
{/snippet}

{#snippet passengerFields()}
  <FormField label={formLabels.typeName} error={fieldError('type_name')} required>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.type_name} />
  </FormField>

  {@render selectField(
    formLabels.passengerCarType,
    'passenger_car_type',
    true,
    passengerCarTypesData
  )}

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <input
      class="input border-surface-600 bg-surface-800"
      type="text"
      bind:value={rs.road_number}
    />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.depot} />
  </FormField>

  <div class="lg:col-span-2">
    <Accordion collapsible>
      <AccordionItem value={`technical-${index}-passenger`}>
        <AccordionItemTrigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
          <AccordionItemIndicator class="text-muted text-xs" />
        </AccordionItemTrigger>
        <AccordionItemContent class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(
              formLabels.serviceLevel,
              'service_level',
              false,
              serviceLevelsData
            )}
          </div>
        </AccordionItemContent>
      </AccordionItem>
    </Accordion>
  </div>
{/snippet}

{#snippet freightFields()}
  <FormField label={formLabels.typeName} error={fieldError('type_name')} required>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.type_name} />
  </FormField>

  {@render selectField(formLabels.freightCarType, 'freight_car_type', false, freightCarTypesData)}

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <input
      class="input border-surface-600 bg-surface-800"
      type="text"
      bind:value={rs.road_number}
    />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.depot} />
  </FormField>
{/snippet}

{#snippet railcarFields()}
  <FormField label={formLabels.typeName} error={fieldError('type_name')} required>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.type_name} />
  </FormField>

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <input
      class="input border-surface-600 bg-surface-800"
      type="text"
      bind:value={rs.road_number}
    />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.depot} />
  </FormField>

  <div class="lg:col-span-2">
    <Accordion collapsible>
      <AccordionItem value={`technical-${index}-railcar`}>
        <AccordionItemTrigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
          <AccordionItemIndicator class="text-muted text-xs" />
        </AccordionItemTrigger>
        <AccordionItemContent class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(formLabels.control, 'control', false, controlsData)}
            {@render selectField(
              formLabels.dccInterface,
              'dcc_interface',
              false,
              dccInterfacesData
            )}
          </div>
        </AccordionItemContent>
      </AccordionItem>
    </Accordion>
  </div>
{/snippet}

{#snippet emuFields()}
  <FormField label={formLabels.typeName} error={fieldError('type_name')} required>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.type_name} />
  </FormField>

  {@render selectField(
    formLabels.emuType,
    'electric_multiple_unit_type',
    true,
    electricMultipleUnitTypesData
  )}

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <input
      class="input border-surface-600 bg-surface-800"
      type="text"
      bind:value={rs.road_number}
    />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.depot} />
  </FormField>

  <label class="label flex items-center gap-2">
    <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase">
      {resolveLabel(formLabels.isDummy)}
    </span>
  </label>

  <div class="lg:col-span-2">
    <Accordion collapsible>
      <AccordionItem value={`technical-${index}-emu`}>
        <AccordionItemTrigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
          <AccordionItemIndicator class="text-muted text-xs" />
        </AccordionItemTrigger>
        <AccordionItemContent class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(formLabels.control, 'control', false, controlsData)}
            {@render selectField(
              formLabels.dccInterface,
              'dcc_interface',
              false,
              dccInterfacesData
            )}
          </div>
        </AccordionItemContent>
      </AccordionItem>
    </Accordion>
  </div>
{/snippet}

{#snippet locomotiveFields()}
  <FormField label={formLabels.className} error={fieldError('class_name')} required>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.class_name} />
  </FormField>

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')} required>
    <input
      class="input border-surface-600 bg-surface-800"
      type="text"
      bind:value={rs.road_number}
    />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.depot} />
  </FormField>

  {@render selectField(formLabels.type, 'locomotive_type', true, locomotiveTypesData)}

  <label class="label flex items-center gap-2">
    <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase">
      {resolveLabel(formLabels.isDummy)}
    </span>
  </label>

  <div class="lg:col-span-2">
    <Accordion collapsible>
      <AccordionItem value={`technical-${index}-locomotive`}>
        <AccordionItemTrigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
          <AccordionItemIndicator class="text-muted text-xs" />
        </AccordionItemTrigger>
        <AccordionItemContent class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(formLabels.control, 'control', false, controlsData)}
            {@render selectField(
              formLabels.dccInterface,
              'dcc_interface',
              false,
              dccInterfacesData
            )}
          </div>
        </AccordionItemContent>
      </AccordionItem>
    </Accordion>
  </div>
{/snippet}

<div class="variant-filled-surface card p-4">
  <div class="mb-4 flex items-center justify-between">
    <h4 class="h5">{resolveLabel(formLabels.rollingStock)} #{index + 1}</h4>
    <div class="flex gap-2">
      <button type="button" class="btn-icon btn-sm" onclick={() => onDuplicate(index)}>📋</button>
      <button type="button" class="btn-icon btn-sm" onclick={() => onDelete(index)}>🗑️</button>
    </div>
  </div>

  <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
    {@render selectField(
      formLabels.railwayCompany,
      'railway_company_id',
      true,
      railwayCompaniesData
    )}
    {@render selectField(
      formLabels.rollingStockCategory,
      'category',
      true,
      rollingStockCategoriesData
    )}

    <FormField label={formLabels.livery} error={fieldError('livery')}>
      <input class="input border-surface-600 bg-surface-800" type="text" bind:value={rs.livery} />
    </FormField>

    {#if isLocomotive}
      {@render locomotiveFields()}
    {:else if isPassengerCar}
      {@render passengerFields()}
    {:else if isFreightCar}
      {@render freightFields()}
    {:else if isRailcar}
      {@render railcarFields()}
    {:else if isElectricMultipleUnit}
      {@render emuFields()}
    {/if}
  </div>
</div>
