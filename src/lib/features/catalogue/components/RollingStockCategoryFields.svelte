<script lang="ts">
  import * as Accordion from '$lib/components/ui/accordion';
  import FormField from '$lib/shared/ui/FormField.svelte';
  import * as Select from '$lib/components/ui/select';
  import type { ConstantItem } from '../constants';
  import type { RollingStockForm } from '../utils';
  import { resolveLabel } from '../../../../utils/resolveLabel';
  import RollingStockScaleFields from './RollingStockScaleFields.svelte';

  type Option = ConstantItem | { id: string; name: string };

  interface Props {
    rs: RollingStockForm;
    index: number;
    errorsFn: (field: string) => string | undefined;
    formLabels: Record<string, ConstantItem>;
    locomotiveTypesData: ConstantItem[];
    passengerCarTypesData: ConstantItem[];
    freightCarTypesData: ConstantItem[];
    controlsData: ConstantItem[];
    dccInterfacesData: ConstantItem[];
    serviceLevelsData: ConstantItem[];
  }

  const {
    rs,
    index,
    errorsFn,
    formLabels,
    locomotiveTypesData,
    passengerCarTypesData,
    freightCarTypesData,
    controlsData,
    dccInterfacesData,
    serviceLevelsData
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
  <RollingStockScaleFields {rs} {errorsFn} {formLabels} />

  {@render selectField(
    formLabels.passengerCarType,
    'passenger_car_type',
    true,
    passengerCarTypesData
  )}

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
  <RollingStockScaleFields {rs} {errorsFn} {formLabels} />

  {@render selectField(formLabels.freightCarType, 'freight_car_type', false, freightCarTypesData)}
{/snippet}

{#snippet railcarFields()}
  <RollingStockScaleFields {rs} {errorsFn} {formLabels} />

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
  <RollingStockScaleFields {rs} {errorsFn} {formLabels} />

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
  <RollingStockScaleFields {rs} {errorsFn} {formLabels} roadNumberRequired />

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

<!-- Render category-specific fields based on current category -->
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
