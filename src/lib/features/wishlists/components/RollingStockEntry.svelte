<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Input } from '$lib/components';
  import type { RailwayCompany } from '$lib/bindings';
  import type { RollingStockFormEntry } from '../types';
  import { ROLLING_STOCK_CATEGORIES } from '../constants';

  interface Props {
    /** Entry data bound two-way */
    entry: RollingStockFormEntry;
    /** Available railway companies for dropdown */
    railwayCompanies: RailwayCompany[];
    /** Whether remove button is enabled (disabled if only entry) */
    canRemove: boolean;
    /** Callback to remove this entry */
    onRemove: () => void;
  }

  let { entry = $bindable(), railwayCompanies, canRemove, onRemove }: Props = $props();

  // Helper to get category label key
  function getCategoryLabelKey(category: string): string {
    const labelMap: Record<string, string> = {
      LOCOMOTIVES: 'wishlist_category_locomotives',
      FREIGHT_CARS: 'wishlist_category_freight_cars',
      PASSENGER_CARS: 'wishlist_category_passenger_cars',
      ELECTRIC_MULTIPLE_UNITS: 'wishlist_category_electric_multiple_units',
      RAILCARS: 'wishlist_category_railcars'
    };
    return labelMap[category] || category;
  }
</script>

<div
  class="rolling-stock-entry border-surface-700/60 bg-surface-800 grid grid-cols-1 gap-4 rounded-lg border p-4"
>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Railway Company -->
    <div>
      <label for="railway-company-{entry.id}" class="block space-y-1">
        <span class="text-surface-300 text-sm">{m.wishlist_field_railway_company()}</span>
      </label>
      <select
        id="railway-company-{entry.id}"
        bind:value={entry.railwayCompanyId}
        class="input bg-surface-700 w-full"
        required
      >
        <option value="">-- {m.wishlist_field_railway_company()} --</option>
        {#each railwayCompanies as company (company.id)}
          <option value={company.id}>{company.name}</option>
        {/each}
      </select>
    </div>

    <!-- Series Code -->
    <div>
      <label for="series-code-{entry.id}" class="block space-y-1">
        <span class="text-surface-300 text-sm">{m.wishlist_field_series_code()}</span>
      </label>
      <Input
        id="series-code-{entry.id}"
        type="text"
        bind:value={entry.seriesCode}
        placeholder="e.g., 218, Re 4/4"
        class="w-full font-mono"
        required
      />
    </div>
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Category -->
    <div>
      <label for="category-{entry.id}" class="block space-y-1">
        <span class="text-surface-300 text-sm">{m.wishlist_field_category()}</span>
      </label>
      <select
        id="category-{entry.id}"
        bind:value={entry.category}
        class="input bg-surface-700 w-full"
        required
      >
        <option value="">-- {m.wishlist_field_category()} --</option>
        {#each ROLLING_STOCK_CATEGORIES as cat (cat)}
          <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
          <option value={cat}>{(m as any)[getCategoryLabelKey(cat)]()}</option>
        {/each}
      </select>
    </div>

    <!-- Road Number (optional) -->
    <div>
      <label for="road-number-{entry.id}" class="block space-y-1">
        <span class="text-surface-300 text-sm">{m.wishlist_field_road_number()}</span>
      </label>
      <Input
        id="road-number-{entry.id}"
        type="text"
        bind:value={entry.roadNumber}
        placeholder="e.g., 218 101-3"
        class="w-full font-mono"
      />
    </div>
  </div>

  <!-- Remove Button -->
  <div class="flex justify-end">
    <button
      type="button"
      class="variant-ghost-error btn btn-sm"
      disabled={!canRemove}
      onclick={onRemove}
      aria-label={m.wishlist_rolling_stock_remove()}
    >
      <X size={16} />
      <span>{m.wishlist_rolling_stock_remove()}</span>
    </button>
  </div>
</div>
