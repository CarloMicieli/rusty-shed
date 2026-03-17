<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Input, Button } from '$lib/components';
  import { FormSelect } from '$lib/components/drawer';
  import type { RailwayCompany } from '$lib/bindings';
  import type { RollingStockFormEntry } from '$lib/features/collection/types/AddModelFormTypes';
  import rollingStockCategories from '$lib/data/constants/rollingStockCategories.json';
  import locomotiveTypes from '$lib/data/constants/locomotiveTypes.json';

  interface Props {
    /** Entry data bound two-way */
    entry: RollingStockFormEntry;
    /** Available railway companies for dropdown */
    railwayCompanies: RailwayCompany[];
    /** Whether remove button is enabled (disabled if only entry) */
    canRemove: boolean;
    /** Callback to remove this entry */
    onRemove: () => void;
    /** Validation errors for this entry */
    errors?: {
      railwayCompanyId?: string;
      seriesCode?: string;
      category?: string;
    };
    /** Enable mechanical dark mode styling */
    dark?: boolean;
  }

  let {
    entry = $bindable(),
    railwayCompanies,
    canRemove,
    onRemove,
    errors,
    dark = false
  }: Props = $props();

  const darkInput =
    'flex h-10 w-full rounded-md border border-[#1F1F1F] bg-transparent px-3 py-2 text-sm text-[#E0E0E0] placeholder:text-[#808080] focus:border-[#D48A42] focus:ring-2 focus:ring-[#D48A42]/30 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50';

  // Show locomotive type field only if category is LOCOMOTIVE
  let showLocomotiveType = $derived(entry.category === 'LOCOMOTIVE');

  // When category changes, reset locomotive type if not a locomotive
  $effect(() => {
    if (!showLocomotiveType && entry.locomotiveType) {
      entry.locomotiveType = null;
    }
  });

  // Map options to { value, label }[] for FormSelect
  const companyOptions = $derived(railwayCompanies.map((c) => ({ value: c.id, label: c.name })));

  const categoryOptions = $derived(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    rollingStockCategories.map((cat) => ({ value: cat.id, label: (m as any)[cat.labelKey]() }))
  );

  const locomotiveTypeOptions = $derived(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    locomotiveTypes.map((t) => ({ value: t.id, label: (m as any)[t.labelKey]() }))
  );
</script>

<div
  class="rolling-stock-entry grid grid-cols-1 gap-4 rounded-lg border p-4"
  class:border-[#1F1F1F]={dark}
  class:bg-[#0F0F0F]={dark}
  class:border-border={!dark}
  class:bg-card={!dark}
  class:text-card-foreground={!dark}
>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Railway Company -->
    <FormSelect
      id="railway-company-{entry.uid}"
      label={m.add_model_railway_company()}
      options={companyOptions}
      bind:value={entry.railwayCompanyId}
      placeholder="-- {m.add_model_railway_company()} --"
      error={errors?.railwayCompanyId}
      required
    />

    <!-- Series Code -->
    <div>
      <label for="series-code-{entry.uid}" class="block space-y-1">
        {#if dark}
          <span class="text-[10px] text-[#808080] uppercase">{m.add_model_series_code()}</span>
        {:else}
          <span class="text-sm text-muted-foreground">{m.add_model_series_code()}</span>
        {/if}
      </label>
      <Input
        id="series-code-{entry.uid}"
        type="text"
        bind:value={entry.seriesCode}
        placeholder="e.g., 218, Re 4/4"
        class={dark ? `${darkInput} font-mono` : 'w-full font-mono'}
        aria-describedby={errors?.seriesCode ? `series-code-error-{entry.uid}` : undefined}
      />
      {#if errors?.seriesCode}
        <p id="series-code-error-{entry.uid}" class="text-error-500 mt-1 text-sm">
          {errors.seriesCode}
        </p>
      {/if}
    </div>
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Category -->
    <FormSelect
      id="category-{entry.uid}"
      label={m.add_model_rs_category()}
      options={categoryOptions}
      bind:value={entry.category}
      placeholder="-- {m.add_model_rs_category()} --"
      error={errors?.category}
      required
    />

    <!-- Road Number (optional) -->
    <div>
      <label for="road-number-{entry.uid}" class="block space-y-1">
        {#if dark}
          <span class="text-[10px] text-[#808080] uppercase">{m.add_model_road_number()}</span>
          <span class="ml-1 text-[#808080]/50">(optional)</span>
        {:else}
          <span class="text-sm text-muted-foreground">{m.add_model_road_number()}</span>
          <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
        {/if}
      </label>
      <Input
        id="road-number-{entry.uid}"
        type="text"
        bind:value={entry.roadNumber}
        placeholder="e.g., 218 101-3"
        class={dark ? `${darkInput} font-mono` : 'w-full font-mono'}
      />
    </div>
  </div>

  <!-- Locomotive Type (conditional) -->
  {#if showLocomotiveType}
    <FormSelect
      id="locomotive-type-{entry.uid}"
      label={m.add_model_locomotive_type()}
      options={locomotiveTypeOptions}
      bind:value={entry.locomotiveType}
      placeholder="-- {m.add_model_locomotive_type()} --"
    />
  {/if}

  <!-- Remove Button -->
  <div class="flex justify-end">
    {#if dark}
      <button
        type="button"
        class="text-[10px] tracking-widest text-[#808080]/60 uppercase hover:text-red-400 disabled:pointer-events-none disabled:opacity-30"
        disabled={!canRemove}
        onclick={onRemove}
        aria-label={m.add_model_remove_rolling_stock()}
      >
        {m.add_model_remove_rolling_stock()}
      </button>
    {:else}
      <Button
        type="button"
        variant="ghost"
        size="sm"
        class="text-destructive hover:bg-destructive/10"
        disabled={!canRemove}
        onclick={onRemove}
        aria-label={m.add_model_remove_rolling_stock()}
      >
        <X size={16} />
        <span>{m.add_model_remove_rolling_stock()}</span>
      </Button>
    {/if}
  </div>
</div>
