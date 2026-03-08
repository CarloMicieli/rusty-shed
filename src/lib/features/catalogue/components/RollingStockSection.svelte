<script lang="ts">
  import * as Accordion from '$lib/components/ui/accordion';

  import FormField from '$lib/shared/ui/FormField.svelte';
  import { Button, Input } from '$lib/components';
  import * as Select from '$lib/components/ui/select';
  import type { ConstantItem } from '../constants';
  import type { RollingStockForm } from '../utils';
  import { resolveLabel } from '../../../../utils/resolveLabel';

  type Option = ConstantItem | { id: string; name: string };

  type Props = {
    rs: RollingStockForm;
    index: number;
    errorsFn: (field: string) => string | undefined;
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
    errorsFn,
    onDuplicate,
    onDelete,
    rollingStockCategoriesData,
    railwayCompaniesData,
    locomotiveTypesData,
    passengerCarTypesData,
    freightCarTypesData,
    electricMultipleUnitTypesData: _electricMultipleUnitTypesData,
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
    return errorsFn(name);
  }
</script>

{#snippet selectField(
  label: ConstantItem,
  fieldName: keyof RollingStockForm,
  required: boolean,
  options: Option[]
)}
  {@const currentValue = (rs[fieldName] as string) || undefined}
  {@const found = options.find((o) => o.id === currentValue)}
  {@const displayLabel = found ? ('name' in found ? found.name : resolveLabel(found)) : null}
  <FormField {label} error={fieldError(fieldName as string)} {required}>
    <Select.Root
      type="single"
      value={currentValue}
      onValueChange={(v: string) => {
        (rs as Record<string, unknown>)[fieldName as string] = v;
      }}
    >
      <Select.Trigger class="w-full">
        {#if displayLabel}
          {displayLabel}
        {:else}
          <span class="text-muted-foreground">{resolveLabel(formLabels.selectPlaceholder)}</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each options as option (option.id)}
          <Select.Item
            value={option.id}
            label={'name' in option ? option.name : resolveLabel(option)}
          />
        {/each}
      </Select.Content>
    </Select.Root>
  </FormField>
{/snippet}

{#snippet passengerFields()}
  <FormField label={formLabels.friendlyName} error={fieldError('friendly_name')} required>
    <Input type="text" bind:value={rs.friendly_name} />
  </FormField>
  <FormField label={formLabels.seriesCode} error={fieldError('series_code')} required>
    <Input type="text" bind:value={rs.series_code} />
  </FormField>

  {@render selectField(
    formLabels.passengerCarType,
    'passenger_car_type',
    true,
    passengerCarTypesData
  )}

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <Input
      class="border-input bg-background"
      type="text"
      value={rs.road_number || ''}
      oninput={(e) => (rs.road_number = e.currentTarget.value)}
    />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <Input
      class="border-input bg-background"
      type="text"
      value={rs.series || ''}
      oninput={(e) => (rs.series = e.currentTarget.value)}
    />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <Input
      class="border-input bg-background"
      type="text"
      value={rs.depot || ''}
      oninput={(e) => (rs.depot = e.currentTarget.value)}
    />
  </FormField>

  <div class="lg:col-span-2">
    <Accordion.Root type="single">
      <Accordion.Item value={`technical-${index}-passenger`}>
        <Accordion.Trigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
        </Accordion.Trigger>
        <Accordion.Content class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(
              formLabels.serviceLevel,
              'service_level',
              false,
              serviceLevelsData
            )}
          </div>
        </Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  </div>
{/snippet}

{#snippet freightFields()}
  <FormField label={formLabels.friendlyName} error={fieldError('friendly_name')} required>
    <Input type="text" bind:value={rs.friendly_name} />
  </FormField>

  <FormField label={formLabels.seriesCode} error={fieldError('series_code')} required>
    <Input type="text" bind:value={rs.series_code} />
  </FormField>

  {@render selectField(formLabels.freightCarType, 'freight_car_type', false, freightCarTypesData)}

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <Input type="text" bind:value={rs.road_number} />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <Input type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <Input type="text" bind:value={rs.depot} />
  </FormField>
{/snippet}

{#snippet railcarFields()}
  <FormField label={formLabels.friendlyName} error={fieldError('friendly_name')} required>
    <Input type="text" bind:value={rs.friendly_name} />
  </FormField>

  <FormField label={formLabels.seriesCode} error={fieldError('series_code')} required>
    <Input type="text" bind:value={rs.series_code} />
  </FormField>

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <Input type="text" bind:value={rs.road_number} />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <Input type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <Input type="text" bind:value={rs.depot} />
  </FormField>

  <div class="lg:col-span-2">
    <Accordion.Root type="single">
      <Accordion.Item value={`technical-${index}-railcar`}>
        <Accordion.Trigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
        </Accordion.Trigger>
        <Accordion.Content class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(formLabels.control, 'control', false, controlsData)}
            {@render selectField(
              formLabels.dccInterface,
              'dcc_interface',
              false,
              dccInterfacesData
            )}
          </div>
        </Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  </div>
{/snippet}

{#snippet emuFields()}
  <FormField label={formLabels.friendlyName} error={fieldError('friendly_name')} required>
    <Input type="text" bind:value={rs.friendly_name} />
  </FormField>

  <FormField label={formLabels.seriesCode} error={fieldError('series_code')} required>
    <Input type="text" bind:value={rs.series_code} />
  </FormField>

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')}>
    <Input type="text" bind:value={rs.road_number} />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <Input type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <Input type="text" bind:value={rs.depot} />
  </FormField>

  <label class="label flex items-center gap-2">
    <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
    <span class="text-surface-300 text-sm font-bold tracking-wider uppercase">
      {resolveLabel(formLabels.isDummy)}
    </span>
  </label>

  <div class="lg:col-span-2">
    <Accordion.Root type="single">
      <Accordion.Item value={`technical-${index}-emu`}>
        <Accordion.Trigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
        </Accordion.Trigger>
        <Accordion.Content class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(formLabels.control, 'control', false, controlsData)}
            {@render selectField(
              formLabels.dccInterface,
              'dcc_interface',
              false,
              dccInterfacesData
            )}
          </div>
        </Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  </div>
{/snippet}

{#snippet locomotiveFields()}
  <FormField label={formLabels.friendlyName} error={fieldError('friendly_name')} required>
    <Input type="text" bind:value={rs.friendly_name} />
  </FormField>

  <FormField label={formLabels.seriesCode} error={fieldError('series_code')} required>
    <Input type="text" bind:value={rs.series_code} />
  </FormField>

  <FormField label={formLabels.roadNumber} error={fieldError('road_number')} required>
    <Input type="text" bind:value={rs.road_number} />
  </FormField>

  <FormField label={formLabels.series} error={fieldError('series')}>
    <Input type="text" bind:value={rs.series} />
  </FormField>

  <FormField label={formLabels.depot} error={fieldError('depot')}>
    <Input type="text" bind:value={rs.depot} />
  </FormField>

  {@render selectField(formLabels.type, 'locomotive_type', true, locomotiveTypesData)}

  <label class="label flex items-center gap-2">
    <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
    <span class="text-surface-300 text-sm font-bold tracking-wider uppercase">
      {resolveLabel(formLabels.isDummy)}
    </span>
  </label>

  <div class="lg:col-span-2">
    <Accordion.Root type="single">
      <Accordion.Item value={`technical-${index}-locomotive`}>
        <Accordion.Trigger class="flex w-full items-center justify-between px-2 py-1 text-left">
          <span class="text-sm font-semibold">{resolveLabel(formLabels.technicalDetails)}</span>
        </Accordion.Trigger>
        <Accordion.Content class="px-2 pt-1 pb-2">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(formLabels.control, 'control', false, controlsData)}
            {@render selectField(
              formLabels.dccInterface,
              'dcc_interface',
              false,
              dccInterfacesData
            )}
          </div>
        </Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  </div>
{/snippet}

<div class="rounded-lg border border-border bg-card p-4">
  <div class="mb-4 flex items-center justify-between">
    <h4 class="h5">{resolveLabel(formLabels.rollingStock)} #{index + 1}</h4>
    <div class="flex gap-2">
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8"
        onclick={() => onDuplicate(index)}>📋</Button
      >
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8"
        onclick={() => onDelete(index)}>🗑️</Button
      >
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
      <Input type="text" bind:value={rs.livery} />
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
