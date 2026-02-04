<script lang="ts">
  import {
    Accordion,
    AccordionItem,
    AccordionItemTrigger,
    AccordionItemContent,
    AccordionItemIndicator
  } from '$lib/components/accordion';
  import { safeInvoke, getErrorMessage, isValidationError } from '$lib/services';
  import {
    createRailwayModelSchema,
    type CreateRailwayModelInput
  } from '$lib/schemas/railway-model';
  import { formLabels } from './constants';
  import { createDefaultRollingStock, normalizeRollingStock, type RollingStockForm } from './utils';
  import FormField from '$lib/components/ui/FormField.svelte';
  import manufacturersData from '$lib/data/manufacturers.json';
  import railwayCompaniesData from '$lib/data/railway-companies.json';
  import availabilityStatusesData from '$lib/data/constants/availabilityStatuses.json';
  import categoriesData from '$lib/data/constants/categories.json';
  import controlsData from '$lib/data/constants/controls.json';
  import dccInterfacesData from '$lib/data/constants/dccInterfaces.json';
  import electricMultipleUnitTypesData from '$lib/data/constants/electricMultipleUnitTypes.json';
  import epochsData from '$lib/data/constants/epochs.json';
  import freightCarTypesData from '$lib/data/constants/freightCarTypes.json';
  import locomotiveTypesData from '$lib/data/constants/locomotiveTypes.json';
  import passengerCarTypesData from '$lib/data/constants/passengerCarTypes.json';
  import powerMethodsData from '$lib/data/constants/powerMethods.json';
  import rollingStockCategoriesData from '$lib/data/constants/rollingStockCategories.json';
  import scalesData from '$lib/data/constants/scales.json';
  import serviceLevelsData from '$lib/data/constants/serviceLevels.json';
  import { resolveLabel } from '../../../utils/resolveLabel';
  import type { ConstantItem } from './constants';
  import RollingStockSection from './components/RollingStockSection.svelte';
  import type { ZodError } from 'zod';

  type FormState = {
    manufacturer_id: string;
    product_code: string;
    description: string;
    details: string | null;
    power_method: CreateRailwayModelInput['power_method'] | '';
    scale: CreateRailwayModelInput['scale'] | '';
    epoch: CreateRailwayModelInput['epoch'] | '';
    category: CreateRailwayModelInput['category'] | '';
    delivery_date: string | null;
    availability_status: CreateRailwayModelInput['availability_status'] | '' | null;
    rolling_stocks: RollingStockForm[];
  };

  let accordionValues = $state<string[]>(['basic-info', 'delivery-availability', 'rolling-stock']);

  let formData = $state<FormState>({
    manufacturer_id: '',
    product_code: '',
    description: '',
    details: null,
    power_method: '',
    scale: '',
    epoch: '',
    category: '',
    delivery_date: null,
    availability_status: null,
    rolling_stocks: []
  });

  let errors = $state<Record<string, string>>({});
  let isSubmitting = $state(false);
  const hasRollingStock = $derived(formData.rolling_stocks.length > 0);

  function addRollingStock() {
    formData.rolling_stocks.push(createDefaultRollingStock());
  }

  function deleteRollingStock(index: number) {
    formData.rolling_stocks.splice(index, 1);
  }

  function duplicateRollingStock(index: number) {
    const copy = structuredClone(formData.rolling_stocks[index]);
    formData.rolling_stocks.push(copy);
  }

  function navigate(path: string) {
    window.location.assign(path);
  }

  function optionLabel(option: ConstantItem | { name: string; id: string }): string {
    if ('name' in option) return option.name;
    return resolveLabel(option as ConstantItem);
  }

  async function handleSubmit() {
    isSubmitting = true;
    errors = {};

    try {
      const payload: CreateRailwayModelInput = {
        manufacturer_id: formData.manufacturer_id,
        product_code: formData.product_code,
        description: formData.description,
        details: formData.details,
        power_method: formData.power_method as CreateRailwayModelInput['power_method'],
        scale: formData.scale as CreateRailwayModelInput['scale'],
        epoch: formData.epoch,
        category: formData.category as CreateRailwayModelInput['category'],
        delivery_date: formData.delivery_date,
        availability_status:
          formData.availability_status === ''
            ? null
            : (formData.availability_status as CreateRailwayModelInput['availability_status']),
        rolling_stocks: formData.rolling_stocks.map(normalizeRollingStock)
      } as CreateRailwayModelInput;

      const validated = createRailwayModelSchema.parse(payload);
      const result = await safeInvoke<string>('create_railway_model', { args: validated });

      if (result.ok) {
        navigate(`/models/${result.data}`);
      } else {
        // Handle validation errors by mapping to form fields
        if (isValidationError(result.error)) {
          errors = { ...result.error.fields };
        }
        // Always set a general error message
        errors.general = getErrorMessage(result.error);
      }
    } catch (err) {
      if ((err as ZodError).issues) {
        const zodErr = err as ZodError;
        zodErr.issues.forEach((issue) => {
          const path = issue.path.join('.');
          errors[path] = issue.message;
        });
      } else {
        errors.general = typeof err === 'string' ? err : 'An unexpected error occurred';
      }
    } finally {
      isSubmitting = false;
    }
  }
</script>

{#snippet selectField(
  label: ConstantItem,
  error: string | undefined,
  required: boolean,
  value: string,
  options: Array<ConstantItem | { id: string; name: string }>,
  onChange: (next: string) => void
)}
  {@const selected = value}
  <FormField {label} {error} {required}>
    <select
      class="select border-surface-600 bg-surface-800"
      value={selected}
      onchange={(event) => onChange((event.currentTarget as HTMLSelectElement).value)}
    >
      <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
      {#each options as option (option.id)}
        <option value={option.id}>{optionLabel(option)}</option>
      {/each}
    </select>
  </FormField>
{/snippet}

<div class="container mx-auto p-8">
  <h1 class="mb-8 h2">{resolveLabel(formLabels.title)}</h1>
  {#if errors.general}
    <div class="variant-filled-error mb-4 card p-4">{errors.general}</div>
  {/if}

  <form
    onsubmit={(e) => {
      e.preventDefault();
      handleSubmit();
    }}
  >
    <Accordion
      value={accordionValues}
      onValueChange={(details) => (accordionValues = details.value)}
      multiple
      collapsible
      class="space-y-3"
    >
      <AccordionItem value="basic-info" class="rounded-lg border border-surface-600">
        <AccordionItemTrigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="mb-0 h4">{resolveLabel(formLabels.basicInfo)}</h3>
          <AccordionItemIndicator class="text-muted text-sm" />
        </AccordionItemTrigger>

        <AccordionItemContent class="px-3 pt-1 pb-4">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(
              formLabels.manufacturer,
              errors.manufacturer_id,
              true,
              formData.manufacturer_id,
              manufacturersData,
              (next) => (formData.manufacturer_id = next)
            )}

            <FormField label={formLabels.productCode} error={errors.product_code} required>
              <input
                class="input border-surface-600 bg-surface-800 font-mono"
                type="text"
                bind:value={formData.product_code}
                placeholder={resolveLabel(formLabels.productCodePlaceholder)}
              />
            </FormField>

            <FormField label={formLabels.description} error={errors.description} required>
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.description}
                placeholder={resolveLabel(formLabels.descriptionPlaceholder)}
              />
            </FormField>

            {@render selectField(
              formLabels.category,
              errors.category,
              true,
              formData.category,
              categoriesData,
              (next) => (formData.category = next as CreateRailwayModelInput['category'])
            )}

            {@render selectField(
              formLabels.scale,
              errors.scale,
              true,
              formData.scale,
              scalesData,
              (next) => (formData.scale = next as CreateRailwayModelInput['scale'])
            )}

            {@render selectField(
              formLabels.powerMethod,
              errors.power_method,
              true,
              formData.power_method,
              powerMethodsData,
              (next) => (formData.power_method = next as CreateRailwayModelInput['power_method'])
            )}

            {@render selectField(
              formLabels.epoch,
              errors.epoch,
              true,
              formData.epoch,
              epochsData,
              (next) => (formData.epoch = next)
            )}
          </div>
        </AccordionItemContent>
      </AccordionItem>

      <AccordionItem value="delivery-availability" class="rounded-lg border border-surface-600">
        <AccordionItemTrigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="mb-0 h4">{resolveLabel(formLabels.deliveryAvailability)}</h3>
          <AccordionItemIndicator class="text-muted text-sm" />
        </AccordionItemTrigger>

        <AccordionItemContent class="px-3 pt-1 pb-4">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            <FormField label={formLabels.deliveryDate} error={errors.delivery_date}>
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.delivery_date}
                placeholder={resolveLabel(formLabels.deliveryDatePlaceholder)}
              />
            </FormField>

            {@render selectField(
              formLabels.availabilityStatus,
              errors.availability_status,
              false,
              formData.availability_status ?? '',
              availabilityStatusesData,
              (next) =>
                (formData.availability_status = (next ||
                  null) as CreateRailwayModelInput['availability_status'])
            )}

            <FormField label={formLabels.additionalDetails} error={errors.details}>
              <textarea
                class="textarea border-surface-600 bg-surface-800"
                rows="3"
                bind:value={formData.details}
                placeholder={resolveLabel(formLabels.detailsPlaceholder)}
              ></textarea>
            </FormField>
          </div>
        </AccordionItemContent>
      </AccordionItem>

      <AccordionItem value="rolling-stock" class="rounded-lg border border-surface-600">
        <AccordionItemTrigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="mb-0 h4">
            {resolveLabel(formLabels.rollingStock)}
            <span class="variant-soft-primary badge">{formData.rolling_stocks.length}</span>
          </h3>
          <AccordionItemIndicator class="text-muted text-sm" />
        </AccordionItemTrigger>

        <AccordionItemContent class="px-3 pt-1 pb-4">
          <div class="space-y-4">
            {#if !hasRollingStock}
              <div class="text-muted text-sm">Add at least one rolling stock item to continue.</div>
            {/if}

            {#each formData.rolling_stocks as rs, index (index)}
              <RollingStockSection
                {rs}
                {index}
                {errors}
                {rollingStockCategoriesData}
                {railwayCompaniesData}
                {locomotiveTypesData}
                {passengerCarTypesData}
                {freightCarTypesData}
                {electricMultipleUnitTypesData}
                {controlsData}
                {dccInterfacesData}
                {serviceLevelsData}
                {formLabels}
                onDuplicate={() => duplicateRollingStock(index)}
                onDelete={() => deleteRollingStock(index)}
              />
            {/each}

            <button type="button" class="cta-btn cta-primary btn" onclick={addRollingStock}>
              + {resolveLabel(formLabels.addRollingStock)}
            </button>
          </div>
        </AccordionItemContent>
      </AccordionItem>
    </Accordion>

    <div class="mt-8 flex gap-4">
      <button type="submit" class="cta-btn cta-primary btn" disabled={isSubmitting}>
        {isSubmitting ? `${resolveLabel(formLabels.create)}...` : resolveLabel(formLabels.create)}
      </button>
      <button type="button" class="cta-btn cta-secondary btn" onclick={() => navigate('/')}>
        {resolveLabel(formLabels.cancel)}
      </button>
    </div>
  </form>
</div>

<style>
  .cta-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-weight: 700;
    text-decoration: none;
    border: 1px solid transparent;
    transition:
      background-color 150ms ease,
      border-color 150ms ease,
      color 150ms ease;
  }

  .cta-primary {
    background-color: var(--primary-500, #2563eb);
    color: var(--on-primary, #0b1224);
    border-color: var(--primary-600, #1d4ed8);
  }

  .cta-primary:hover {
    background-color: var(--primary-600, #1d4ed8);
  }

  .cta-secondary {
    background-color: var(--surface-700, #2d2f36);
    color: var(--on-surface, #f8fafc);
    border-color: var(--surface-500, #3f4148);
  }

  .cta-secondary:hover {
    background-color: var(--surface-600, #32343b);
  }
</style>
