<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Input, Button } from '$lib/components';
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
  }

  let { entry = $bindable(), railwayCompanies, canRemove, onRemove, errors }: Props = $props();

  // Show locomotive type field only if category is LOCOMOTIVE
  let showLocomotiveType = $derived(entry.category === 'LOCOMOTIVE');

  // When category changes, reset locomotive type if not a locomotive
  $effect(() => {
    if (!showLocomotiveType && entry.locomotiveType) {
      entry.locomotiveType = null;
    }
  });
</script>

<div
  class="rolling-stock-entry grid grid-cols-1 gap-4 rounded-lg border border-border bg-card p-4 text-card-foreground"
>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Railway Company -->
    <div>
      <label for="railway-company-{entry.uid}" class="block space-y-1">
        <span class="text-sm text-muted-foreground">{m.add_model_railway_company()}</span>
      </label>
      <select
        id="railway-company-{entry.uid}"
        bind:value={entry.railwayCompanyId}
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        class:input-error={errors?.railwayCompanyId}
        aria-describedby={errors?.railwayCompanyId
          ? `railway-company-error-{entry.uid}`
          : undefined}
      >
        <option value={null}>-- {m.add_model_railway_company()} --</option>
        {#each railwayCompanies as company (company.id)}
          <option value={company.id}>{company.name}</option>
        {/each}
      </select>
      {#if errors?.railwayCompanyId}
        <p id="railway-company-error-{entry.uid}" class="text-error-500 mt-1 text-sm">
          {errors.railwayCompanyId}
        </p>
      {/if}
    </div>

    <!-- Series Code -->
    <div>
      <label for="series-code-{entry.uid}" class="block space-y-1">
        <span class="text-sm text-muted-foreground">{m.add_model_series_code()}</span>
      </label>
      <Input
        id="series-code-{entry.uid}"
        type="text"
        bind:value={entry.seriesCode}
        placeholder="e.g., 218, Re 4/4"
        class="w-full font-mono"
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
    <div>
      <label for="category-{entry.uid}" class="block space-y-1">
        <span class="text-sm text-muted-foreground">{m.add_model_rs_category()}</span>
      </label>
      <select
        id="category-{entry.uid}"
        bind:value={entry.category}
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        class:input-error={errors?.category}
        aria-describedby={errors?.category ? `category-error-{entry.uid}` : undefined}
      >
        <option value={null}>-- {m.add_model_rs_category()} --</option>
        {#each rollingStockCategories as cat (cat.id)}
          <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
          <option value={cat.id}>{(m as any)[cat.labelKey]()}</option>
        {/each}
      </select>
      {#if errors?.category}
        <p id="category-error-{entry.uid}" class="text-error-500 mt-1 text-sm">
          {errors.category}
        </p>
      {/if}
    </div>

    <!-- Road Number (optional) -->
    <div>
      <label for="road-number-{entry.uid}" class="block space-y-1">
        <span class="text-sm text-muted-foreground">{m.add_model_road_number()}</span>
        <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
      </label>
      <Input
        id="road-number-{entry.uid}"
        type="text"
        bind:value={entry.roadNumber}
        placeholder="e.g., 218 101-3"
        class="w-full font-mono"
      />
    </div>
  </div>

  <!-- Locomotive Type (conditional) -->
  {#if showLocomotiveType}
    <div>
      <label for="locomotive-type-{entry.uid}" class="block space-y-1">
        <span class="text-sm text-muted-foreground">{m.add_model_locomotive_type()}</span>
      </label>
      <select
        id="locomotive-type-{entry.uid}"
        bind:value={entry.locomotiveType}
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
      >
        <option value={null}>-- {m.add_model_locomotive_type()} --</option>
        {#each locomotiveTypes as type (type.id)}
          <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
          <option value={type.id}>{(m as any)[type.labelKey]()}</option>
        {/each}
      </select>
    </div>
  {/if}

  <!-- Remove Button -->
  <div class="flex justify-end">
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
  </div>
</div>

<style>
  .input-error {
    border-color: rgb(var(--color-error-500));
  }
</style>
