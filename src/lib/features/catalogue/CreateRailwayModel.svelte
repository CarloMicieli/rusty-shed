<script lang="ts">
  import { untrack } from 'svelte';
  import * as Accordion from '$lib/components/ui/accordion';
  import { superForm } from 'sveltekit-superforms';
  import { safeInvoke, getErrorMessage } from '$lib/services';
  import { commands } from '$lib/bindings';
  import type { CreateRailwayModelInput } from '$lib/schemas/railway-model';
  import { formLabels } from './constants';
  import { createDefaultRollingStock, normalizeRollingStock, type RollingStockForm } from './utils';
  import FormField from '$lib/components/ui/FormField.svelte';
  import { Input, Badge } from '$lib/components';
  import TranslationsSection from './components/TranslationsSection.svelte';
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

  // Custom form type that uses RollingStockForm for UI state
  type CreateRailwayModelFormInput = Omit<CreateRailwayModelInput, 'rolling_stocks'> & {
    rolling_stocks: RollingStockForm[];
  };

  let accordionValues = $state<string[]>([
    'basic-info',
    'delivery-availability',
    'translations',
    'rolling-stock'
  ]);

  const initialData: CreateRailwayModelFormInput = {
    manufacturer_id: '',
    product_code: '',
    description: '',
    details: null,
    power_method: 'AC',
    scale: 'H0',
    epoch: '',
    category: 'LOCOMOTIVES',
    delivery_date: null,
    availability_status: null,
    rolling_stocks: []
  };

  const formObj = superForm<CreateRailwayModelFormInput>(
    untrack(() => $state.snapshot(initialData)),
    {
      SPA: true,
      dataType: 'json',
      // Skip client-side Zod validation since RollingStockForm doesn't match schema exactly
      // Server-side validation via Tauri will catch any issues
      onUpdate: async ({ form }) => {
        // Validate structure manually
        if (!form.data.manufacturer_id || !form.data.product_code || !enDescription) {
          generalError = 'Please fill in all required fields (including English description)';
          return;
        }

        if (form.data.rolling_stocks.length === 0) {
          generalError = 'At least one rolling stock is required';
          return;
        }

        try {
          // Normalize rolling stocks before submission

          const normalizedData = {
            ...form.data,
            rolling_stocks: form.data.rolling_stocks.map((rs) => normalizeRollingStock(rs))
          };

          const result = await safeInvoke<string>('create_railway_model', { args: normalizedData });

          if (result.ok) {
            // After creation, upsert IT translation if any IT fields were provided
            if (itDescription || itDetails) {
              await commands.upsertRailwayModelTranslation({
                railwayModelId: result.data,
                lang: 'it',
                description: itDescription ?? null,
                details: itDetails ?? null
              });
            }
            navigate(`/models/${result.data}`);
          } else {
            // Set general error
            generalError = getErrorMessage(result.error);
          }
        } catch (err) {
          generalError = err instanceof Error ? err.message : 'An unexpected error occurred';
        }
      }
    }
  );

  const { form, errors, enhance, submitting } = formObj;
  const hasRollingStock = $derived($form.rolling_stocks.length > 0);
  let generalError = $state<string | null>(null);

  // Translation state: EN values stay in $form; IT is local state
  let enDescription = $state<string | null>($form.description ?? '');
  let enDetails = $state<string | null>($form.details ?? null);
  let itDescription = $state<string | null>(null);
  let itDetails = $state<string | null>(null);

  // Sync EN translation state back to superForm fields
  $effect(() => {
    $form.description = enDescription ?? '';
    $form.details = enDetails;
  });

  function addRollingStock() {
    $form.rolling_stocks.push(createDefaultRollingStock());
  }

  function deleteRollingStock(index: number) {
    $form.rolling_stocks.splice(index, 1);
  }

  function duplicateRollingStock(index: number) {
    const copy = structuredClone($form.rolling_stocks[index]);
    $form.rolling_stocks.push(copy);
  }

  function navigate(path: string) {
    window.location.assign(path);
  }

  function optionLabel(option: ConstantItem | { name: string; id: string }): string {
    if ('name' in option) return option.name;
    return resolveLabel(option as ConstantItem);
  }

  // Helper to get field-specific errors
  function fieldError(name: string): string | undefined {
    const err = $errors[name as keyof typeof $errors];
    return err ? String(err) : undefined;
  }

  function rollingStockFieldError(index: number, fieldName: string): string | undefined {
    const key = `rolling_stocks.${index}.${fieldName}` as keyof typeof $errors;
    const err = $errors[key];
    return err ? String(err) : undefined;
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
      class="select border-input bg-background"
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
  <h1 class="h2 mb-8">{resolveLabel(formLabels.title)}</h1>
  {#if generalError}
    <div class="variant-filled-error card mb-4 p-4">{generalError}</div>
  {/if}

  <form method="POST" use:enhance>
    <Accordion.Root bind:value={accordionValues} type="multiple" class="space-y-3">
      <Accordion.Item value="basic-info" class="rounded-lg border border-border">
        <Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="h4 mb-0">{resolveLabel(formLabels.basicInfo)}</h3>
        </Accordion.Trigger>

        <Accordion.Content class="px-3 pt-1 pb-4">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {@render selectField(
              formLabels.manufacturer,
              fieldError('manufacturer_id'),
              true,
              $form.manufacturer_id,
              manufacturersData,
              (next) => ($form.manufacturer_id = next)
            )}

            <FormField label={formLabels.productCode} error={fieldError('product_code')} required>
              <Input
                type="text"
                bind:value={$form.product_code}
                placeholder={resolveLabel(formLabels.productCodePlaceholder)}
                class="font-mono"
              />
            </FormField>

            {@render selectField(
              formLabels.category,
              fieldError('category'),
              true,
              $form.category,
              categoriesData,
              (next) => ($form.category = next as CreateRailwayModelInput['category'])
            )}

            {@render selectField(
              formLabels.scale,
              fieldError('scale'),
              true,
              $form.scale,
              scalesData,
              (next) => ($form.scale = next as CreateRailwayModelInput['scale'])
            )}

            {@render selectField(
              formLabels.powerMethod,
              fieldError('power_method'),
              true,
              $form.power_method,
              powerMethodsData,
              (next) => ($form.power_method = next as CreateRailwayModelInput['power_method'])
            )}

            {@render selectField(
              formLabels.epoch,
              fieldError('epoch'),
              true,
              $form.epoch,
              epochsData,
              (next) => ($form.epoch = next)
            )}
          </div>
        </Accordion.Content>
      </Accordion.Item>

      <Accordion.Item value="delivery-availability" class="rounded-lg border border-border">
        <Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="h4 mb-0">{resolveLabel(formLabels.deliveryAvailability)}</h3>
        </Accordion.Trigger>

        <Accordion.Content class="px-3 pt-1 pb-4">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            <FormField label={formLabels.deliveryDate} error={fieldError('delivery_date')}>
              <Input
                type="text"
                bind:value={$form.delivery_date}
                placeholder={resolveLabel(formLabels.deliveryDatePlaceholder)}
              />
            </FormField>

            {@render selectField(
              formLabels.availabilityStatus,
              fieldError('availability_status'),
              false,
              $form.availability_status ?? '',
              availabilityStatusesData,
              (next) =>
                ($form.availability_status = (next ||
                  null) as CreateRailwayModelInput['availability_status'])
            )}
          </div>
        </Accordion.Content>
      </Accordion.Item>

      <Accordion.Item value="translations" class="rounded-lg border border-border">
        <Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="h4 mb-0">{resolveLabel(formLabels.description)}</h3>
        </Accordion.Trigger>
        <Accordion.Content class="px-3 pt-1 pb-4">
          <TranslationsSection
            bind:enDescription
            bind:enDetails
            bind:itDescription
            bind:itDetails
          />
        </Accordion.Content>
      </Accordion.Item>

      <Accordion.Item value="rolling-stock" class="rounded-lg border border-border">
        <Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="h4 mb-0">
            {resolveLabel(formLabels.rollingStock)}
            <Badge variant="default" class="ml-2">{$form.rolling_stocks.length}</Badge>
          </h3>
        </Accordion.Trigger>

        <Accordion.Content class="px-3 pt-1 pb-4">
          <div class="space-y-4">
            {#if !hasRollingStock}
              <div class="text-sm text-muted">Add at least one rolling stock item to continue.</div>
            {/if}

            {#each $form.rolling_stocks as rs, index (index)}
              <RollingStockSection
                {rs}
                {index}
                errorsFn={(field) => rollingStockFieldError(index, field)}
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
        </Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>

    <div class="mt-8 flex gap-4">
      <button type="submit" class="cta-btn cta-primary btn" disabled={$submitting}>
        {$submitting ? `${resolveLabel(formLabels.create)}...` : resolveLabel(formLabels.create)}
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
