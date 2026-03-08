<script lang="ts">
  import type { Snippet } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Plus } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import type { Manufacturer } from '$lib/bindings';
  import epochs from '$lib/data/constants/epochs.json';

  // Form data structure with railway model fields
  interface BaseFormData {
    manufacturerId: string | null;
    productCode: string;
    description: string;
    category: string | null;
    scale: string | null;
    powerMethod: string | null;
    epoch: string | null;
  }

  interface Props {
    /** Reference data */
    manufacturers: Manufacturer[];
    categoryOptions: Array<{ id: string; labelKey: string } | string>;
    scaleOptions: Array<{ id: string; display: string } | string>;
    powerMethodOptions: Array<{ id: string; display: string } | string>;

    /** Form data bindings */
    form: BaseFormData;

    /** Validation errors (optional) */
    validationErrors?: Record<string, string | undefined>;

    /** Callbacks for rolling stock operations */
    onAddRollingStock: () => void;

    /** Helpers for label rendering */
    getCategoryLabelKey?: (category: string) => string;
    getPowerMethodLabelKey?: (method: string) => string;

    /** Slot for rolling stock entries */
    children?: Snippet;
  }

  let {
    manufacturers,
    categoryOptions,
    scaleOptions,
    powerMethodOptions,
    form,
    validationErrors = {},
    onAddRollingStock,
    getCategoryLabelKey = (cat: string) => cat,
    getPowerMethodLabelKey = (method: string) => method,
    children
  }: Props = $props();
</script>

<!-- Railway Model Section -->
<section>
  <h3 class="mb-4 text-lg font-semibold text-foreground">
    {m.add_model_section_model()}
  </h3>
  <div class="space-y-4">
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <!-- Manufacturer -->
      <div>
        <label for="base-manufacturer" class="block space-y-1">
          <span class="text-sm text-muted-foreground">{m.add_model_manufacturer()}</span>
        </label>
        <select
          id="base-manufacturer"
          bind:value={form.manufacturerId}
          class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          class:input-error={validationErrors.manufacturerId}
        >
          <option value={null}>-- {m.add_model_manufacturer()} --</option>
          {#each manufacturers as mfr (mfr.id)}
            <option value={mfr.id}>{mfr.name}</option>
          {/each}
        </select>
        {#if validationErrors.manufacturerId}
          <p class="text-error-500 mt-1 text-sm">{validationErrors.manufacturerId}</p>
        {/if}
      </div>

      <!-- Product Code -->
      <div>
        <label for="base-product-code" class="block space-y-1">
          <span class="text-sm text-muted-foreground">{m.add_model_product_code()}</span>
        </label>
        <input
          id="base-product-code"
          type="text"
          bind:value={form.productCode}
          class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 font-mono text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          class:input-error={validationErrors.productCode}
          placeholder="e.g., 37171"
        />
        {#if validationErrors.productCode}
          <p class="text-error-500 mt-1 text-sm">{validationErrors.productCode}</p>
        {/if}
      </div>
    </div>

    <!-- Description -->
    <div>
      <label for="base-description" class="block space-y-1">
        <span class="text-sm text-muted-foreground">{m.add_model_description()}</span>
      </label>
      <input
        id="base-description"
        type="text"
        bind:value={form.description}
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        class:input-error={validationErrors.description}
        placeholder="e.g., DB BR 218 diesel locomotive"
      />
      {#if validationErrors.description}
        <p class="text-error-500 mt-1 text-sm">{validationErrors.description}</p>
      {/if}
    </div>

    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <!-- Category -->
      <div>
        <label for="base-category" class="block space-y-1">
          <span class="text-sm text-muted-foreground">{m.add_model_category()}</span>
        </label>
        <select
          id="base-category"
          bind:value={form.category}
          class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          class:input-error={validationErrors.category}
        >
          <option value={null}>-- {m.add_model_category()} --</option>
          {#each categoryOptions as cat (typeof cat === 'string' ? cat : cat.id)}
            {#if typeof cat === 'string'}
              <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
              <option value={cat}>{(m as any)[getCategoryLabelKey(cat)]()}</option>
            {:else}
              <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
              <option value={cat.id}>{(m as any)[cat.labelKey]()}</option>
            {/if}
          {/each}
        </select>
        {#if validationErrors.category}
          <p class="text-error-500 mt-1 text-sm">{validationErrors.category}</p>
        {/if}
      </div>

      <!-- Scale -->
      <div>
        <label for="base-scale" class="block space-y-1">
          <span class="text-sm text-muted-foreground">{m.add_model_scale()}</span>
        </label>
        <select
          id="base-scale"
          bind:value={form.scale}
          class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          class:input-error={validationErrors.scale}
        >
          <option value={null}>-- {m.add_model_scale()} --</option>
          {#each scaleOptions as scale (typeof scale === 'string' ? scale : scale.id)}
            {#if typeof scale === 'string'}
              <option value={scale}>{scale}</option>
            {:else}
              <option value={scale.id}>{scale.display}</option>
            {/if}
          {/each}
        </select>
        {#if validationErrors.scale}
          <p class="text-error-500 mt-1 text-sm">{validationErrors.scale}</p>
        {/if}
      </div>
    </div>

    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <!-- Power Method -->
      <div>
        <label for="base-power-method" class="block space-y-1">
          <span class="text-sm text-muted-foreground">{m.add_model_power_method()}</span>
        </label>
        <select
          id="base-power-method"
          bind:value={form.powerMethod}
          class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          class:input-error={validationErrors.powerMethod}
        >
          <option value={null}>-- {m.add_model_power_method()} --</option>
          {#each powerMethodOptions as method (typeof method === 'string' ? method : method.id)}
            {#if typeof method === 'string'}
              <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
              <option value={method}>{(m as any)[getPowerMethodLabelKey(method)]()}</option>
            {:else}
              <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
              <option value={method.id}>{(m as any)[getPowerMethodLabelKey(method.id)]()}</option>
            {/if}
          {/each}
        </select>
        {#if validationErrors.powerMethod}
          <p class="text-error-500 mt-1 text-sm">{validationErrors.powerMethod}</p>
        {/if}
      </div>

      <!-- Epoch -->
      <div>
        <label for="base-epoch" class="block space-y-1">
          <span class="text-sm text-muted-foreground">{m.add_model_epoch()}</span>
        </label>
        <select
          id="base-epoch"
          bind:value={form.epoch}
          class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          class:input-error={validationErrors.epoch}
        >
          <option value={null}>-- {m.add_model_epoch()} --</option>
          {#each epochs as epoch (epoch.id)}
            <option value={epoch.id}>{epoch.display}</option>
          {/each}
        </select>
        {#if validationErrors.epoch}
          <p class="text-error-500 mt-1 text-sm">{validationErrors.epoch}</p>
        {/if}
      </div>
    </div>
  </div>
</section>

<!-- Rolling Stocks Section (slot for parent to provide entries) -->
<section>
  <div class="mb-4 flex items-center justify-between">
    <h3 class="text-lg font-semibold text-foreground">
      {m.add_model_section_rolling_stock()}
    </h3>
    <Button type="button" variant="default" size="sm" onclick={onAddRollingStock}>
      <Plus size={16} />
      <span>{m.add_model_add_rolling_stock()}</span>
    </Button>
  </div>

  {#if validationErrors.rollingStocks}
    <p class="text-error-500 mb-3 text-sm">{validationErrors.rollingStocks}</p>
  {/if}

  <div class="space-y-4">
    {@render children?.()}
  </div>
</section>

<style>
  .input-error {
    border-color: rgb(var(--color-error-500));
  }
</style>
