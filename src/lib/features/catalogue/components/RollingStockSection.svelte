<script lang="ts">
  import FormField from '$lib/shared/ui/FormField.svelte';
  import { Button, Input } from '$lib/components';
  import * as Select from '$lib/components/ui/select';
  import type { ConstantItem } from '../constants';
  import type { RollingStockForm } from '../utils';
  import { resolveLabel } from '../../../../utils/resolveLabel';
  import RollingStockCategoryFields from './RollingStockCategoryFields.svelte';

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

    <RollingStockCategoryFields
      {rs}
      {index}
      {errorsFn}
      {formLabels}
      {locomotiveTypesData}
      {passengerCarTypesData}
      {freightCarTypesData}
      {controlsData}
      {dccInterfacesData}
      {serviceLevelsData}
    />
  </div>
</div>
