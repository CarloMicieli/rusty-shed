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
  import { Button, Input, Badge } from '$lib/components';
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
  import * as Select from '$lib/components/ui/select';

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
  {@const found = options.find((o) => o.id === value)}
  {@const displayLabel = found ? optionLabel(found) : null}
  <FormField {label} {error} {required}>
    <Select.Root
      type="single"
      value={value || undefined}
      onValueChange={(v: string) => onChange(v)}
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
          <Select.Item value={option.id} label={optionLabel(option)} />
        {/each}
      </Select.Content>
    </Select.Root>
  </FormField>
{/snippet}

{#snippet powerMethodPills()}
  <FormField label={formLabels.powerMethod} error={fieldError('power_method')} required>
    <div class="flex gap-1.5 pt-0.5">
      {#each powerMethodsData as pm (pm.id)}
        <button
          type="button"
          class="rounded-full border px-3 py-1 text-xs font-semibold transition-colors {$form.power_method ===
          pm.id
            ? 'border-amber-500 bg-amber-500/15 text-amber-400'
            : 'border-zinc-700 bg-zinc-800/60 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}"
          onclick={() => ($form.power_method = pm.id as CreateRailwayModelInput['power_method'])}
        >
          {pm.display ?? pm.id}
        </button>
      {/each}
    </div>
  </FormField>
{/snippet}

<div class="container mx-auto p-8">
  <h1 class="h2 mb-6">{resolveLabel(formLabels.title)}</h1>

  <div class="flex items-start gap-6">
    <!-- Main form -->
    <form id="railway-model-form" method="POST" use:enhance class="min-w-0 flex-1">
      {#if generalError}
        <div
          class="mb-4 rounded-lg border border-destructive/40 bg-destructive/10 p-4 text-destructive"
        >
          {generalError}
        </div>
      {/if}

      <Accordion.Root bind:value={accordionValues} type="multiple" class="space-y-3">
        <Accordion.Item value="basic-info" class="rounded-lg border border-border">
          <Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2 text-left">
            <h3 class="h4 mb-0">{resolveLabel(formLabels.basicInfo)}</h3>
          </Accordion.Trigger>

          <Accordion.Content class="px-3 pt-1 pb-4">
            <div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
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
                  class="font-mono tracking-wider"
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

              {@render powerMethodPills()}

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
                <div class="rounded-md border border-dashed border-zinc-700 p-6 text-center">
                  <p class="text-sm text-zinc-500">
                    No rolling stock added yet. Use the Command Center to add items.
                  </p>
                </div>
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
            </div>
          </Accordion.Content>
        </Accordion.Item>
      </Accordion.Root>
    </form>

    <!-- Command Center Sidebar -->
    <aside class="sticky top-8 w-60 shrink-0">
      <div class="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
        <!-- Header -->
        <div class="mb-4 flex items-center gap-2">
          <div class="h-px flex-1 bg-zinc-800"></div>
          <span class="text-[10px] font-semibold tracking-widest text-zinc-500 uppercase"
            >Command Center</span
          >
          <div class="h-px flex-1 bg-zinc-800"></div>
        </div>

        <!-- Model summary -->
        <div class="mb-4 space-y-3">
          {#if $form.product_code}
            <div>
              <div class="mb-0.5 text-[10px] tracking-wider text-zinc-500 uppercase">
                Product Code
              </div>
              <div class="font-mono text-sm tracking-wider text-amber-400">
                {$form.product_code}
              </div>
            </div>
          {/if}
          {#if $form.scale}
            {@const scaleDisplay =
              scalesData.find((s) => s.id === $form.scale)?.display ?? $form.scale}
            <div>
              <div class="mb-0.5 text-[10px] tracking-wider text-zinc-500 uppercase">Scale</div>
              <div class="font-mono text-sm text-zinc-200">{scaleDisplay}</div>
            </div>
          {/if}
          <div>
            <div class="mb-0.5 text-[10px] tracking-wider text-zinc-500 uppercase">Power</div>
            <span
              class="rounded-full border border-amber-500/30 bg-amber-500/10 px-2.5 py-0.5 text-xs font-semibold text-amber-400"
              >{$form.power_method}</span
            >
          </div>
        </div>

        <div class="mb-4 h-px bg-zinc-800"></div>

        <!-- Rolling stock -->
        <div class="mb-4 space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-[10px] tracking-wider text-zinc-500 uppercase">Rolling Stock</span>
            <span class="rounded-full bg-zinc-700 px-2 py-0.5 text-xs font-semibold text-zinc-200"
              >{$form.rolling_stocks.length}</span
            >
          </div>
          <Button
            type="button"
            variant="outline"
            class="w-full border-zinc-700 text-xs hover:border-amber-500/50 hover:text-amber-400"
            onclick={addRollingStock}
          >
            + {resolveLabel(formLabels.addRollingStock)}
          </Button>
        </div>

        <div class="mb-4 h-px bg-zinc-800"></div>

        <!-- Actions -->
        <div class="space-y-2">
          <Button type="submit" form="railway-model-form" class="w-full" disabled={$submitting}>
            {$submitting
              ? `${resolveLabel(formLabels.create)}...`
              : resolveLabel(formLabels.create)}
          </Button>
          <Button
            type="button"
            variant="ghost"
            class="w-full text-zinc-400 hover:text-zinc-200"
            onclick={() => navigate('/')}
          >
            {resolveLabel(formLabels.cancel)}
          </Button>
        </div>
      </div>
    </aside>
  </div>
</div>
